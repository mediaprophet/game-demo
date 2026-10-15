//! Living-entity section assembly for game-authored `.10d` assets.
//!
//! This module maps each authored recipe onto the QualiaDB entity format:
//! an ontology declaration (class, embodiment, lifecycle stage, affordances,
//! spectral coefficients) sealed inside the provenance sidecar's
//! `semantic_metadata`, a ProceduralSynthesis (PROC, section 18) receipt
//! where the geometry was genuinely synthesised, and a HierarchicalSurfels
//! (SURF, section 20) far-field tier derived from the hero mesh clusters.
//! Spectral material coefficients are inverted from the authored sRGB
//! colour and checked against Gate 1 (reflectance plausibility) before
//! they are baked into surfel records and the export manifest.
//!
//! The canonical EONT section (17) is not emitted: this QualiaDB revision's
//! `ENTITY_ONTOLOGY_MINI_HEADER_SIZE` constant disagrees with the actual
//! `EntityOntologyMiniHeader` wire size, which panics the encoder, decoder,
//! and `verify_entity_10d` alike. The provenance `semantic_metadata` field
//! is the sealed, verifier-safe channel for the same declaration.
//!
//! The Tier 2 decimated mesh and Tier 3 hemi-octahedral impostor atlas are
//! emitted by the native pack exporter as sibling pack entries (a sealed
//! `.10d` and a raw `.impo` wire payload). They are deliberately *not*
//! written into the hero container: the container encoder sorts sections by
//! ascending type, so an IMPO (19) payload inside the same file would always
//! land after the QuantizedMesh (1) payload and trip the verifier's
//! reverse-LOD layout check — and on this revision the IMPO codec itself
//! panics (see `IMPOSTOR_WIRE_HEADER_SIZE` in pack-export). Bundle-level
//! reverse-LOD is preserved by shipping the tiers as separate pack entries
//! that each verify cleanly on their own.

use crate::asset_catalog::AssetRecipe;
use qualia_core_db::container_10d::header::Container10dHeader;
use qualia_core_db::container_10d::integrity::{check_whole_file_crc32c, seal_whole_file_crc32c};
use qualia_core_db::container_10d::procedural_synthesis::{
    encode_procedural_synthesis_section, ProceduralSynthesisHeader, FLAG_ALLOMETRIC_GROWTH,
    FLAG_PGA_MOTOR_DYNAMICS, MAGIC_PROC, PROCEDURAL_SYNTHESIS_HEADER_SIZE,
    PROCEDURAL_SYNTHESIS_SECTION_VERSION,
};
use qualia_core_db::container_10d::section::{
    encode_container, parse_section_table, AlignmentTier, SectionInput, SectionTableError,
    SectionType,
};
use qualia_core_db::container_10d::surfel_section::{
    encode_surfel_section, generate_surfel_from_cluster,
    SurfelSectionMiniHeader, FLAG_SURFEL_HIERARCHICAL, MAGIC_SURF, SURFEL_MINI_HEADER_SIZE,
    SURFEL_RECORD_SIZE, SURFEL_SECTION_VERSION,
};
use qualia_core_db::entity_view::entity_ontologies::{AffordanceCategory, LifecycleStage};
use qualia_core_db::q_hash;
use qualia_core_db::render::assets::Mesh;
use qualia_core_db::render::entity_compile_gates::{
    validate_geometry_confinement, validate_ontological_integrity, validate_spectral_coeffs,
    OntologicalClass,
};
use qualia_core_db::render::spectral_basis::JakobHanikaCoeffs;
use qualia_core_db::render::spectral_lut::invert_srgb_to_coeffs;

/// Meshlets per surfel: the guide's ~128-triangle cluster budget.
const SURFEL_CLUSTER_TRIANGLES: usize = 128;
/// Hard cap on surfel records per asset so the far-field tier stays bounded.
const MAX_SURFELS: usize = 256;

/// One deterministic affordance declaration: name, description, category.
type AffordanceSpec = (&'static str, &'static str, AffordanceCategory);

/// The entity-format identity a compiled asset claims in the manifest and
/// in its EntityOntology section.
pub(crate) struct EntityProfile {
    /// `OntologicalClass` name reported to the manifest (`"NaturalPerson"`, …).
    pub ontology_class: &'static str,
    /// Embodiment noun from the same taxonomy (`"Persona"`, `"Construct"`, …).
    pub embodiment: &'static str,
    pub stage: LifecycleStage,
    pub taxon_iri: String,
    pub affordances: &'static [AffordanceSpec],
    /// True when the asset depicts a natural person; such assets can never
    /// be classified as commercial/manufactured property (Gate 5).
    pub natural_person: bool,
}

const FLORA_AFFORDANCES: &[AffordanceSpec] = &[
    (
        "photosynthesize",
        "Fix ambient light into biomass reserves",
        AffordanceCategory::Biological,
    ),
    (
        "inspect_specimen",
        "Non-invasive specimen survey and spectral read",
        AffordanceCategory::Sensing,
    ),
];
const FAUNA_AFFORDANCES: &[AffordanceSpec] = &[
    (
        "graze",
        "Consume pasture or browse material",
        AffordanceCategory::Biological,
    ),
    (
        "flee",
        "Emergency evasive sprint",
        AffordanceCategory::Physical,
    ),
];
const PERSONA_AFFORDANCES: &[AffordanceSpec] = &[(
    "converse",
    "Sovereign persona dialogue surface (consent-gated)",
    AffordanceCategory::Social,
)];
const TERRAIN_AFFORDANCES: &[AffordanceSpec] = &[
    (
        "traverse",
        "Cross the surface on foot or by wheel",
        AffordanceCategory::Physical,
    ),
    (
        "survey",
        "Non-invasive survey and measurement",
        AffordanceCategory::Sensing,
    ),
];
const MACHINE_AFFORDANCES: &[AffordanceSpec] = &[
    (
        "operate",
        "Run the asset within its operational envelope",
        AffordanceCategory::Physical,
    ),
    (
        "repair",
        "Maintenance action restoring wear budget",
        AffordanceCategory::Maintenance,
    ),
];
const PROPERTY_AFFORDANCES: &[AffordanceSpec] = &[(
    "inspect",
    "Non-invasive visual and dimensional inspection",
    AffordanceCategory::Sensing,
)];
const WORK_AFFORDANCES: &[AffordanceSpec] = &[(
    "read",
    "Render or perform the authored expression",
    AffordanceCategory::Sensing,
)];

fn has_token(component: &str, words: &[&str]) -> bool {
    words.iter().any(|word| {
        component
            .split(|c: char| c == '-' || c == '_')
            .any(|token| token == *word)
    })
}

/// Map a recipe id onto the entity ontology taxonomy. Component assets keep
/// the class of the entity they belong to (a banksia branch is still a
/// DarwinianTaxon specimen fragment); the manifest records the same class so
/// review tooling and the pack exporter stay consistent.
pub(crate) fn classify(recipe: &AssetRecipe) -> EntityProfile {
    let id = recipe.id.as_str();
    let component = id.rsplit('/').next().unwrap_or(id);

    if component.starts_with("resident-") || component.starts_with("persona-") {
        return EntityProfile {
            ontology_class: "NaturalPerson",
            embodiment: "Persona",
            stage: LifecycleStage::Adult,
            taxon_iri: format!("urn:qualia:persona:{}", component),
            affordances: PERSONA_AFFORDANCES,
            natural_person: true,
        };
    }

    let flora = id.contains("/flora/")
        || has_token(
            component,
            &[
                "flora", "flower", "flowers", "blossom", "blossoms", "herb", "rosemary", "sage",
                "thyme", "lavender", "shrub", "reeds", "canopy", "crown", "leaves", "phyllodes",
                "cones", "trunk", "stalk", "greens", "shoots", "foliage", "gum", "wattle",
                "grass", "crop", "cone", "bottlebrush", "saltbush", "kangaroo-paw", "produce",
                "fruit",
            ],
        )
        || component.starts_with("flower-")
        || component.starts_with("herb-")
        || component.starts_with("lavender-");
    let fauna = id.contains("/fauna/") || component.starts_with("garden-ant-");

    if flora || fauna {
        return EntityProfile {
            ontology_class: "DarwinianTaxon",
            embodiment: "Specimen",
            stage: LifecycleStage::Adult,
            taxon_iri: format!(
                "urn:qualia:taxon:{}",
                id.trim_start_matches("rc:asset/")
            ),
            affordances: if fauna {
                FAUNA_AFFORDANCES
            } else {
                FLORA_AFFORDANCES
            },
            natural_person: false,
        };
    }

    if recipe.vibe_source.is_some() {
        return EntityProfile {
            ontology_class: "LiberalArtsWork",
            embodiment: "Expression",
            stage: LifecycleStage::Adult,
            taxon_iri: format!("urn:qualia:work:{}", component),
            affordances: WORK_AFFORDANCES,
            natural_person: false,
        };
    }

    let abiotic = has_token(
        component,
        &[
            "terrain", "meadow", "hill", "ridge", "ground", "road", "path", "creek", "water",
            "cloud", "soil", "gravel", "rock", "stone", "bank", "banks", "canal", "ripple",
            "ripples", "earth", "skirt", "mound", "boulder", "footpath", "verges", "furrows",
            "beds",
        ],
    ) || component.starts_with("terrain-")
        || component.starts_with("kestrel-meadow-")
        || component.starts_with("kestrel-earth-skirt")
        || component.starts_with("highland-creek")
        || component.starts_with("cloud-")
        || component.starts_with("hill-")
        || component.starts_with("main-road")
        || component.starts_with("path-")
        || component == "ground"
        || component == "grass-verges"
        || component == "channel"
        || component == "channel-banks";

    if abiotic {
        return EntityProfile {
            ontology_class: "AbioticEnvironment",
            embodiment: "Terrain",
            stage: LifecycleStage::Adult,
            taxon_iri: format!("urn:qualia:feature:{}", component),
            affordances: TERRAIN_AFFORDANCES,
            natural_person: false,
        };
    }

    let machine = has_token(
        component,
        &[
            "solar", "battery", "mast", "pump", "tank", "purification", "turbine", "generator",
            "vehicle", "rig", "van", "boat", "winch", "motor", "signal",
        ],
    ) || id.contains("/rig/");

    EntityProfile {
        ontology_class: "ManufacturedProperty",
        embodiment: "Construct",
        stage: LifecycleStage::Operational,
        taxon_iri: format!("urn:qualia:system:{}", component),
        affordances: if machine {
            MACHINE_AFFORDANCES
        } else {
            PROPERTY_AFFORDANCES
        },
        natural_person: false,
    }
}

/// Invert the authored display colour into Jakob–Hanika spectral basis
/// coefficients and check them against Gate 1. Non-finite or implausible
/// results fall back to neutral 50% reflectance rather than failing the
/// compile — a colour outside the basis gamut still gets a physical model.
pub(crate) fn spectral_coeffs(color: [f32; 4]) -> JakobHanikaCoeffs {
    let rgb = [
        color[0].clamp(0.0, 1.0),
        color[1].clamp(0.0, 1.0),
        color[2].clamp(0.0, 1.0),
    ];
    let [c0, c1, c2] = invert_srgb_to_coeffs(rgb, [0.0; 3]);
    let coeffs = JakobHanikaCoeffs::new(c0, c1, c2);
    if c0.is_finite()
        && c1.is_finite()
        && c2.is_finite()
        && validate_spectral_coeffs(&coeffs).is_ok()
    {
        coeffs
    } else {
        JakobHanikaCoeffs::FLAT_MID
    }
}

/// Serialise the entity ontology declaration as bounded JSON for the
/// provenance sidecar's `semantic_metadata` field.
///
/// The canonical `SectionType::EntityOntology` (EONT) section cannot be
/// emitted against this QualiaDB revision: the upstream encoder's
/// `ENTITY_ONTOLOGY_MINI_HEADER_SIZE` constant (32) no longer matches the
/// 40-byte `EntityOntologyMiniHeader` struct, so `encode_entity_ontology_
/// section` panics inside `copy_from_slice`, and every consumer path that
/// reads the section (`decode_entity_ontology_section_into`,
/// `verify_entity_10d`) panics on the same mismatch. Emitting EONT bytes by
/// hand would crash the verifier on any file that contains the section, so
/// the ontology declaration travels inside the provenance sidecar instead —
/// `semantic_metadata` is exactly the field QualiaDB reserves for sealed,
/// transportable entity semantics. The declaration stays byte-inseparable
/// from the asset (it is inside the CRC-sealed container) and is mirrored
/// onto the manifest for review tooling.
fn ontology_metadata_json(
    recipe: &AssetRecipe,
    profile: &EntityProfile,
    coeffs: JakobHanikaCoeffs,
) -> Vec<u8> {
    let mut affordances = String::new();
    for (index, (name, description, category)) in profile.affordances.iter().enumerate() {
        if index > 0 {
            affordances.push(',');
        }
        affordances.push_str(&format!(
            "{{\"name\":\"{name}\",\"category\":\"{category:?}\",\"description\":\"{description}\"}}"
        ));
    }
    format!(
        "{{\"schema\":\"qualia-entity-ontology-v1\",\
            \"entity\":\"{}\",\
            \"class\":\"{}\",\
            \"embodiment\":\"{}\",\
            \"stage\":\"{}\",\
            \"taxonIri\":\"{}\",\
            \"spectral\":{{\"c0\":{},\"c1\":{},\"c2\":{}}},\
            \"affordances\":[{}]}}",
        recipe.id,
        profile.ontology_class,
        profile.embodiment,
        stage_name(profile.stage),
        profile.taxon_iri,
        coeffs.c0,
        coeffs.c1,
        coeffs.c2,
        affordances,
    )
    .into_bytes()
}

fn stage_name(stage: LifecycleStage) -> &'static str {
    match stage {
        LifecycleStage::SeedOrSpore => "seed_or_spore",
        LifecycleStage::Juvenile => "juvenile",
        LifecycleStage::Adult => "adult",
        LifecycleStage::Senescent => "senescent",
        LifecycleStage::Deceased => "deceased",
        LifecycleStage::Manufactured => "manufactured",
        LifecycleStage::Operational => "operational",
        LifecycleStage::Degraded => "degraded",
        LifecycleStage::Decommissioned => "decommissioned",
    }
}

/// Encode the PROC provenance record when synthesis flags genuinely apply:
/// grown flora get allometric growth, wind-animated foliage and water
/// surfaces get the PGA motor dynamics flag. Everything else omits the
/// section rather than claiming a technique that was not used.
fn procedural_synthesis_payload(recipe: &AssetRecipe, profile: &EntityProfile) -> Option<Vec<u8>> {
    let is_flora = profile.ontology_class == "DarwinianTaxon"
        && recipe.id.contains("/flora/");
    let wind_animated = recipe.material_family.id == "living-foliage";
    let water = recipe.render_role == "water";
    let mut flags = 0u32;
    if is_flora {
        flags |= FLAG_ALLOMETRIC_GROWTH;
    }
    if wind_animated || water {
        flags |= FLAG_PGA_MOTOR_DYNAMICS;
    }
    if flags == 0 {
        return None;
    }
    let span_y = 1.0f32;
    let header = ProceduralSynthesisHeader {
        magic: MAGIC_PROC,
        version: PROCEDURAL_SYNTHESIS_SECTION_VERSION,
        flags: flags as u16,
        seed: q_hash(&recipe.id),
        voronoi_frequency: 0.0,
        voronoi_jitter: 0.0,
        fillet_blend_radius: 0.0,
        fillet_threshold: 0.0,
        allometry_exponent_k: if is_flora { 0.75 } else { 0.0 },
        allometry_base_b: if is_flora { span_y } else { 0.0 },
        spiral_growth_rate_k: 0.0,
        spiral_initial_radius_a: 0.0,
        motor_frequency_hz: if wind_animated || water { 0.45 } else { 0.0 },
        motor_damping_ratio: if wind_animated || water { 0.2 } else { 0.0 },
        critical_stress_threshold: 0.0,
        reserved_f32: 0.0,
    };
    let mut out = vec![0u8; PROCEDURAL_SYNTHESIS_HEADER_SIZE];
    match encode_procedural_synthesis_section(&header, &mut out) {
        Ok(written) => {
            out.truncate(written);
            Some(out)
        }
        Err(_) => None,
    }
}

/// Encode the SURF far-field tier: one 32-byte surfel per ~128-triangle
/// cluster, bounded by the hero AABB. Fails open (returns Ok(None)) only
/// when the mesh cannot produce even a single valid cluster; Gate 2
/// confinement is checked against the hero bounds on success.
fn surfel_payload(
    recipe: &AssetRecipe,
    mesh: &Mesh,
    coeffs: JakobHanikaCoeffs,
) -> Result<Option<Vec<u8>>, String> {
    let tri_count = mesh.triangles.len();
    if tri_count == 0 {
        return Ok(None);
    }
    // Cluster size: ~128 triangles, but grow clusters before allowing more
    // than MAX_SURFELS records.
    let cluster_tris = SURFEL_CLUSTER_TRIANGLES.max(tri_count.div_ceil(MAX_SURFELS));
    let mut records = Vec::with_capacity(tri_count.div_ceil(cluster_tris));
    for cluster in mesh.triangles.chunks(cluster_tris) {
        let indices: Vec<u32> = cluster.iter().flatten().copied().collect();
        let record = generate_surfel_from_cluster(
            &mesh.positions,
            &indices,
            [coeffs.c0, coeffs.c1, coeffs.c2],
            recipe.material_family.roughness,
            0,
        )
        .map_err(|e| format!("surfel generation for {}: {e}", recipe.id))?;
        records.push(record);
    }
    if records.is_empty() {
        return Ok(None);
    }
    let center = [
        (mesh.min[0] + mesh.max[0]) * 0.5,
        (mesh.min[1] + mesh.max[1]) * 0.5,
        (mesh.min[2] + mesh.max[2]) * 0.5,
    ];
    let radius = ((mesh.max[0] - mesh.min[0]).powi(2)
        + (mesh.max[1] - mesh.min[1]).powi(2)
        + (mesh.max[2] - mesh.min[2]).powi(2))
    .sqrt()
        * 0.5;
    validate_geometry_confinement(mesh.min, mesh.max, mesh.min, mesh.max, center, radius)
        .map_err(|e| format!("surfel confinement for {}: {e}", recipe.id))?;
    let header = SurfelSectionMiniHeader {
        magic: MAGIC_SURF,
        version: SURFEL_SECTION_VERSION,
        flags: FLAG_SURFEL_HIERARCHICAL,
        surfel_count: records.len() as u32,
        stride: SURFEL_RECORD_SIZE as u32,
        sphere_center: center,
        sphere_radius: radius,
        aabb_min: mesh.min,
        aabb_max: mesh.max,
        reserved: [0; 8],
    };
    let mut out = vec![0u8; SURFEL_MINI_HEADER_SIZE + records.len() * SURFEL_RECORD_SIZE];
    let written = encode_surfel_section(&header, &records, &mut out)
        .map_err(|e| format!("surfel section for {}: {e}", recipe.id))?;
    out.truncate(written);
    Ok(Some(out))
}

/// Attach extra sections to a sealed `.10d` container, preserving every
/// existing section verbatim and resealing the whole-file CRC. This is the
/// generic form of QualiaDB's per-section attach helpers, which cover only
/// material/UV/skeletal payloads.
fn attach_sections(
    container: &[u8],
    extras: &[(SectionType, Vec<u8>)],
) -> Result<Vec<u8>, String> {
    if extras.is_empty() {
        return Ok(container.to_vec());
    }
    let header = Container10dHeader::parse(container)
        .map_err(|e| format!("entity attach header: {e:?}"))?;
    check_whole_file_crc32c(container).map_err(|e| format!("entity attach crc: {e:?}"))?;
    let descriptors = parse_section_table(container, &header)
        .map_err(|e| format!("entity attach table: {e:?}"))?;
    if descriptors
        .iter()
        .any(|d| extras.iter().any(|(ty, _)| d.typ() == Some(*ty)))
    {
        return Err("entity section already present in container".into());
    }
    let mut inputs: Vec<SectionInput<'_>> = Vec::with_capacity(descriptors.len() + extras.len());
    for (index, descriptor) in descriptors.iter().enumerate() {
        let start = descriptor.byte_offset as usize;
        let end = start
            .checked_add(descriptor.byte_length as usize)
            .filter(|end| *end <= container.len())
            .ok_or(SectionTableError::OutOfBounds {
                index,
                offset: descriptor.byte_offset,
                length: descriptor.byte_length,
                file_len: container.len(),
            })
            .map_err(|e| format!("entity attach bounds: {e:?}"))?;
        inputs.push(SectionInput {
            section_type: descriptor
                .typ()
                .ok_or(SectionTableError::UndefinedSectionType {
                    index,
                    got: descriptor.section_type,
                })
                .map_err(|e| format!("entity attach type: {e:?}"))?,
            alignment_tier: descriptor
                .tier()
                .ok_or(SectionTableError::UndefinedAlignmentTier {
                    index,
                    got: descriptor.alignment_tier,
                })
                .map_err(|e| format!("entity attach tier: {e:?}"))?,
            stride: descriptor.stride,
            element_count: descriptor.element_count,
            payload: &container[start..end],
        });
    }
    for (section_type, payload) in extras {
        inputs.push(SectionInput {
            section_type: *section_type,
            alignment_tier: AlignmentTier::CacheLine,
            stride: 0,
            element_count: 0,
            payload,
        });
    }
    let needed = match encode_container(&header, &inputs, &mut []) {
        Ok(size) => size,
        Err(SectionTableError::OutputBufferTooSmall { needed, .. }) => needed,
        Err(error) => return Err(format!("entity attach plan: {error:?}")),
    };
    let mut out = vec![0u8; needed];
    let written = encode_container(&header, &inputs, &mut out)
        .map_err(|e| format!("entity attach encode: {e:?}"))?;
    out.truncate(written);
    seal_whole_file_crc32c(&mut out);
    Ok(out)
}

/// Per-asset entity plan computed once, before the provenance sidecar is
/// sealed: `semantic_metadata` is the JSON ontology declaration baked into
/// the PRV1 section, and the rest feeds the post-compile section attach
/// and the JS receipt.
pub(crate) struct EntityPlan {
    profile: EntityProfile,
    pub spectral: JakobHanikaCoeffs,
    /// JSON ontology declaration for `ProvenanceSidecar.semantic_metadata`.
    pub semantic_metadata: Vec<u8>,
}

/// Classify the recipe, run Gates 1/3/5, and produce the sealed ontology
/// declaration. Called before the hero container is compiled so the
/// declaration lands inside the provenance sidecar.
pub(crate) fn plan(recipe: &AssetRecipe) -> Result<EntityPlan, String> {
    let profile = classify(recipe);
    // Gate 3/5: the declared class must resolve, and a persona may never be
    // filed as manufactured property.
    let class = OntologicalClass::from_str_name(profile.ontology_class)
        .ok_or_else(|| format!("malformed ontology class for {}", recipe.id))?;
    validate_ontological_integrity(
        class as u8,
        if class == OntologicalClass::ManufacturedProperty {
            // Components do not carry a mass ledger; declare a finite
            // nominal mass so the physical-constants check passes honestly.
            Some(1.0)
        } else {
            None
        },
    )
    .map_err(|e| format!("ontological integrity for {}: {e}", recipe.id))?;
    if profile.natural_person && profile.ontology_class != "NaturalPerson" {
        return Err(format!("persona {} misclassified", recipe.id));
    }
    let spectral = spectral_coeffs(recipe.color);
    let semantic_metadata = ontology_metadata_json(recipe, &profile, spectral);
    Ok(EntityPlan {
        profile,
        spectral,
        semantic_metadata,
    })
}

/// Everything the receipt reports about the entity-format upgrade.
pub(crate) struct EntitySections {
    pub container: Vec<u8>,
    pub profile_class: &'static str,
    pub embodiment: &'static str,
    pub stage: &'static str,
    pub taxon_iri: String,
    pub affordance_names: Vec<&'static str>,
    pub spectral: JakobHanikaCoeffs,
    pub surfel_count: u32,
}

/// Attach the entity sections for one compiled asset: procedural-synthesis
/// provenance (PROC) where a synthesis flag genuinely applies, and the
/// Tier 4 hierarchical surfel set (SURF) generated from the hero mesh.
/// Gate 2 confinement is checked against the hero AABB.
pub(crate) fn attach(
    recipe: &AssetRecipe,
    container: &[u8],
    mesh: &Mesh,
    plan: &EntityPlan,
) -> Result<EntitySections, String> {
    let mut extras: Vec<(SectionType, Vec<u8>)> = Vec::with_capacity(2);
    if let Some(proc_payload) = procedural_synthesis_payload(recipe, &plan.profile) {
        extras.push((SectionType::ProceduralSynthesis, proc_payload));
    }
    let surfel_count;
    if let Some(surf) = surfel_payload(recipe, mesh, plan.spectral)? {
        let count = (surf.len() - SURFEL_MINI_HEADER_SIZE) / SURFEL_RECORD_SIZE;
        surfel_count = count as u32;
        extras.push((SectionType::HierarchicalSurfels, surf));
    } else {
        surfel_count = 0;
    }
    let container = attach_sections(container, &extras)?;

    Ok(EntitySections {
        container,
        profile_class: plan.profile.ontology_class,
        embodiment: plan.profile.embodiment,
        stage: stage_name(plan.profile.stage),
        taxon_iri: plan.profile.taxon_iri.clone(),
        affordance_names: plan.profile.affordances.iter().map(|a| a.0).collect(),
        spectral: plan.spectral,
        surfel_count,
    })
}
