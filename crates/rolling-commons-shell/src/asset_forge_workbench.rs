//! Live Rust Asset Forge & Online-Source Metadata Workbench (`.vibe` · `.10d` · `.q42` · `.m64` · `.hmc`).
//!
//! Synthesizes procedural PBR surface textures at runtime in Rust (replacing embedded `.png` files),
//! evaluates canonical `.vibe` biological/engineering scripts via `vibe::session::SourceSession`,
//! and compiles/verifies online-source metadata into 48-byte `.q42` `NQuin` Super-Quins,
//! 64-byte `.m64` Matter-64 soil/allometric containers (`M64Reader::parse`), `.10d` Living
//! Containers (`verify_whole_file_crc32c`), and `.hmc` Hypermedia Bundles (`BundleWriter` / `BundleReader`).

use crate::{json_to_vibe, GamePortal};
use qualia_core_db::bundle::{BundleReader, BundleWriter};
use qualia_core_db::container_10d::integrity::{
    compute_whole_file_crc32c, seal_whole_file_crc32c, verify_whole_file_crc32c,
};
use qualia_core_db::container_10d::{parse_section_table, Container10dHeader, SectionType};
use qualia_core_db::q42::m64_matter::{
    M64ChunkDirectoryEntry, M64Header, M64HoloRecord, M64PayloadInterval, M64ReadLimits,
    M64Reader, M64StrataColumnRecord, M64_MAGIC, M64_MODE_HOLO, M64_MODE_STRATA,
};
use qualia_core_db::render::surface_authoring::{
    encode_surface_png_into, generate_surface_maps_into, SurfaceRecipe,
};
use qualia_core_db::render::texture_decode::TextureDecodeLimits;
use qualia_core_db::render::texture_ingestion::ingest_texture_payload;
use qualia_core_db::resolver::{INLINE_TAG_DECIMAL, INLINE_TAG_INTEGER};
use qualia_core_db::{q_hash, NQuin};

const CORYMBIA_VIBE: &str =
    include_str!("../../../../ecopunk/entities/taxa/corymbia-aparrerinja/corymbia-aparrerinja.vibe");
const WHITE_GUM_VIBE: &str = include_str!(
    "../../../../ecopunk/assets/production-studies/white-gum-review-r1/authoring/white-gum-r2.vibe"
);
const SWAMP_WALLABY_VIBE: &str = include_str!(
    "../../../../ecopunk/assets/production-studies/swamp-wallaby-review-r1/authoring/swamp-wallaby-r3.vibe"
);

const LEGACY_SHA_SOIL: [u8; 32] = [
    0x9a, 0x37, 0x7f, 0x61, 0x7d, 0xca, 0x2d, 0x67, 0xff, 0xe7, 0x20, 0x00, 0xb2, 0x8a, 0xd1, 0x04,
    0x18, 0x0d, 0x9e, 0xf4, 0x45, 0x62, 0x46, 0xa4, 0x10, 0x9e, 0x9e, 0x31, 0x6a, 0x89, 0x7c, 0x42,
];
const LEGACY_SHA_FOLIAGE: [u8; 32] = [
    0xa5, 0x92, 0x53, 0xae, 0x98, 0xd6, 0xf5, 0xd0, 0xa4, 0xd1, 0x0c, 0xe6, 0x92, 0x84, 0x91, 0xd4,
    0x2b, 0xca, 0xb5, 0xa5, 0xc5, 0x12, 0xf9, 0x96, 0x8a, 0x3d, 0x3c, 0x55, 0x15, 0x7b, 0x92, 0x80,
];
const LEGACY_SHA_PEBBLES: [u8; 32] = [
    0xdc, 0x43, 0xb6, 0x3f, 0x44, 0x48, 0x48, 0x33, 0xbd, 0x01, 0x64, 0xc2, 0x76, 0x80, 0xb5, 0xf4,
    0x62, 0xa9, 0x0d, 0xd3, 0xc8, 0x03, 0x0f, 0x5f, 0x18, 0x5f, 0xde, 0x4d, 0x7e, 0xcf, 0x22, 0xa8,
];
const LEGACY_SHA_BARK: [u8; 32] = [
    0x17, 0xa1, 0x80, 0x3e, 0x01, 0x75, 0xe9, 0xb9, 0x45, 0x65, 0x5c, 0x06, 0x39, 0x64, 0x05, 0x9f,
    0x09, 0xad, 0xc3, 0x28, 0x4c, 0x3f, 0xf8, 0x93, 0xc3, 0x06, 0x0c, 0x9e, 0xd0, 0x8d, 0xb3, 0xbc,
];

/// Synthesize 4 procedural PBR surface textures (`soil`, `foliage`, `pebbles`, `bark`) in Rust
/// via `qualia_core_db::render::surface_authoring` (zero static `.png` files embedded) and return
/// their runtime `[u8; 32]` SHA-256 digests so `.10d` `MAT1` records can be remapped on admission.
pub fn synthesize_and_admit_procedural_pbr_textures(
    portal: &mut GamePortal,
) -> Option<[[u8; 32]; 4]> {
    const SIDE: u32 = 128;
    let recipes: [(&str, SurfaceRecipe); 4] = [
        (
            "slga-soil-strata-litter",
            SurfaceRecipe {
                seed: q_hash("ecopunk:surface:slga_soil_strata_litter") as u32,
                tint: [0.66, 0.50, 0.36],
                contrast: 0.24,
                relief: 0.0042,
                bands: [5, 3],
                band_strength: 0.22,
                roughness: 0.94,
                metallic: 0.02,
            },
        ),
        (
            "foliage-leaf-venation",
            SurfaceRecipe {
                seed: q_hash("ecopunk:surface:foliage_leaf_venation") as u32,
                tint: [0.42, 0.58, 0.38],
                contrast: 0.16,
                relief: 0.0012,
                bands: [4, 18],
                band_strength: 0.36,
                roughness: 0.68,
                metallic: 0.02,
            },
        ),
        (
            "riverbed-pebbles-lithology",
            SurfaceRecipe {
                seed: q_hash("ecopunk:surface:riverbed_pebbles_lithology") as u32,
                tint: [0.58, 0.56, 0.52],
                contrast: 0.26,
                relief: 0.0048,
                bands: [8, 8],
                band_strength: 0.18,
                roughness: 0.86,
                metallic: 0.04,
            },
        ),
        (
            "bark-fibrous-mottled",
            SurfaceRecipe {
                seed: q_hash("ecopunk:surface:bark_fibrous_mottled") as u32,
                tint: [0.76, 0.72, 0.66],
                contrast: 0.22,
                relief: 0.0036,
                bands: [20, 3],
                band_strength: 0.64,
                roughness: 0.88,
                metallic: 0.02,
            },
        ),
    ];

    let size = (SIDE * SIDE * 4) as usize;
    let mut colour = vec![0u8; size];
    let mut normal = vec![0u8; size];
    let mut orm = vec![0u8; size];
    let mut png_buf = vec![0u8; size + SIDE as usize + 4096];
    let mut out_digests = [[0u8; 32]; 4];

    for (idx, (_name, recipe)) in recipes.iter().enumerate() {
        generate_surface_maps_into(*recipe, SIDE, &mut colour, &mut normal, &mut orm).ok()?;
        let used = encode_surface_png_into(SIDE, &colour, &mut png_buf).ok()?;
        let ingested = ingest_texture_payload(
            "image/png",
            Some(&png_buf[..used]),
            TextureDecodeLimits::default(),
        )
        .ok()?;
        let ok = portal
            .inner
            .load_texture_encoded(&ingested.digest, "image/png", &ingested.bytes, 0, 0.5)
            .ok()?;
        if !ok {
            return None;
        }
        out_digests[idx] = ingested.digest;
    }
    Some(out_digests)
}

/// Replace any legacy 32-byte texture SHA-256 digests in a `.10d` `MAT1` section with the runtime
/// procedurally generated texture digests, update the `MAT1` `SectionDescriptor.crc32c`, and
/// reseal the whole-file `CRC-32C`.
pub fn remap_and_reseal_10d_textures(bytes: &mut [u8], runtime_digests: &[[u8; 32]; 4]) {
    let Ok(hdr) = Container10dHeader::parse(bytes) else {
        return;
    };
    let table_off = hdr.section_table_offset as usize;
    let sec_count = hdr.section_count as usize;
    let legacy = [
        LEGACY_SHA_SOIL,
        LEGACY_SHA_FOLIAGE,
        LEGACY_SHA_PEBBLES,
        LEGACY_SHA_BARK,
    ];
    let mut any_patched = false;

    for s_idx in 0..sec_count {
        let desc_off = table_off.saturating_add(s_idx * 24);
        if desc_off + 24 > bytes.len() {
            break;
        }
        let sec_type = bytes[desc_off];
        if sec_type != SectionType::Materials as u8 {
            continue;
        }
        let b_off = u32::from_le_bytes([
            bytes[desc_off + 4],
            bytes[desc_off + 5],
            bytes[desc_off + 6],
            bytes[desc_off + 7],
        ]) as usize;
        let b_len = u32::from_le_bytes([
            bytes[desc_off + 8],
            bytes[desc_off + 9],
            bytes[desc_off + 10],
            bytes[desc_off + 11],
        ]) as usize;
        let end = b_off.saturating_add(b_len);
        if end > bytes.len() || b_len < 32 {
            continue;
        }

        let mut sec_patched = false;
        let mut i = b_off;
        while i + 32 <= end {
            for (slot, old_sha) in legacy.iter().enumerate() {
                if &bytes[i..i + 32] == old_sha {
                    bytes[i..i + 32].copy_from_slice(&runtime_digests[slot]);
                    sec_patched = true;
                    break;
                }
            }
            i += 1;
        }
        if sec_patched {
            let new_crc = qualia_core_db::container_10d::crc32c(&bytes[b_off..end]);
            bytes[desc_off + 20..desc_off + 24].copy_from_slice(&new_crc.to_le_bytes());
            any_patched = true;
        }
    }
    if any_patched {
        let _ = seal_whole_file_crc32c(bytes);
    }
}

/// Canonical online-source metadata groups available to attach to any specimen or civic asset.
pub const ONLINE_SOURCE_GROUPS: [(&str, &str); 5] = [
    ("0", "GBIF / ALA / APNI / AFD Taxonomy (dwc:taxonID)"),
    ("1", "AusTraits / AVONET / QUDT SI Biophysical Traits"),
    ("2", "Plant Ontology (PO:) / UBERON & Spectral Albedo"),
    ("3", "CSIRO SLGA 6-Horizon Soil & ELVIS DEM (.m64)"),
    ("4", "AIATSIS Country / ABS SEIFA / dev.civics.au (.q42)"),
];

/// Summary of a live asset compilation & container verification pass across `.vibe`, `.10d`, `.q42`, `.m64`, and `.hmc`.
#[derive(Clone, Default)]
pub struct AssetForgeReport {
    pub asset_id: String,
    pub scientific_name: String,
    pub vernacular_and_indigenous: String,
    pub vibe_module: String,
    pub vibe_revision: u64,
    pub vibe_eval_summary: String,
    pub ten_d_bytes: usize,
    pub ten_d_crc32c_hex: String,
    pub ten_d_crc_valid: bool,
    pub ten_d_sections: String,
    pub ten_d_dimensions_m: [f32; 3],
    pub q42_quin_count: usize,
    pub q42_bytes: usize,
    pub q42_ecc_all_valid: bool,
    pub q42_sample_triples: Vec<String>,
    pub m64_bytes: usize,
    pub m64_header_crc_hex: String,
    pub m64_verified: bool,
    pub m64_summary: String,
    pub hmc_bytes: usize,
    pub hmc_entry_count: usize,
    pub hmc_all_verified: bool,
}

fn make_quin(
    subject_iri: &str,
    predicate_iri: &str,
    opcode: u8,
    object_raw: u64,
    context_iri: &str,
    routing_lane_bits: u64,
    lamport_clock: u32,
) -> NQuin {
    let mut q = NQuin {
        subject: q_hash(subject_iri),
        predicate: (q_hash(predicate_iri) << 8) | (opcode as u64),
        object: object_raw,
        context: q_hash(context_iri),
        metadata: (routing_lane_bits << 61) | ((lamport_clock as u64) & 0xFFFF_FFFF),
        parity: 0,
    };
    q.recalculate_parity();
    q
}

fn tag_integer(val: u64) -> u64 {
    INLINE_TAG_INTEGER | (val & 0x0FFF_FFFF_FFFF_FFFF)
}

fn tag_decimal_micro(val_f64: f64) -> u64 {
    let micro = (val_f64.max(0.0) * 1_000_000.0).round() as u64;
    INLINE_TAG_DECIMAL | (micro & 0x0FFF_FFFF_FFFF_FFFF)
}

/// Evaluate the canonical `.vibe` script for the selected specimen, build and verify its
/// `.q42` Super-Quin volume, `.m64` Matter-64 soil/allometric container, `.10d` Living Container,
/// and `.hmc` Hypermedia Bundle.
pub fn compile_and_verify_asset_containers(
    specimen_key: &str,
    dissection_scale: u8,
    dissection_archetype: u8,
    explode_01: f32,
    cutaway_01: f32,
    anthesis_01: f32,
    peel_01: f32,
    ten_d_container: &[u8],
    enabled_sources: &[bool; 5],
) -> AssetForgeReport {
    let (asset_id, sci_name, vern_name, vibe_src, vibe_mod) = match specimen_key {
        "swamp_wallaby" => (
            "ecopunk:taxon:wallabia-bicolor",
            "Wallabia bicolor (Desmarest, 1804)",
            "Swamp Wallaby · Black Wallaby (Ngarigo / Wiradjuri Bushland)",
            SWAMP_WALLABY_VIBE,
            "ecopunk.studies.swamp_wallaby_r3",
        ),
        "eucalypt_spray" => (
            "ecopunk:taxon:eucalyptus-dalrympleana",
            "Eucalyptus dalrympleana Maiden",
            "Mountain White Gum · Ribbon Gum (Montane Snowy Valleys)",
            WHITE_GUM_VIBE,
            "ecopunk.studies.white_gum_r2",
        ),
        "growing_station" => (
            "civics:facility:hydroponic-growing-station",
            "Community Grounds NFT Hydroponic Station (42.5 kg/wk)",
            "Cooperative Food Security Module · did:web:dev.civics.au",
            CORYMBIA_VIBE,
            "civics.grounds.hydroponic_station",
        ),
        "dissection_scale" if dissection_scale == 5 => {
            let (id, sci, vern) = match dissection_archetype % 5 {
                0 => (
                    "ecopunk:taxon:macropus-giganteus",
                    "Macropus giganteus / Wallabia bicolor",
                    "Saltatorial Macropod · 3-Layer Osteology/Myology/Integument",
                ),
                1 => (
                    "ecopunk:taxon:vombatus-ursinus",
                    "Vombatus ursinus (Shaw, 1800)",
                    "Common Wombat (Fossorial Plantigrade · Humeral Deltoid Crest)",
                ),
                2 => (
                    "ecopunk:taxon:ornithorhynchus-anatinus",
                    "Ornithorhynchus anatinus (Shaw, 1799)",
                    "Platypus (Monotreme Electroreceptive Bill & Webbed Paddles)",
                ),
                3 => (
                    "ecopunk:taxon:aquila-audax",
                    "Aquila audax (Latham, 1801)",
                    "Wedge-Tailed Eagle · Maliyan (Keeled Sternum & Remiges)",
                ),
                _ => (
                    "ecopunk:taxon:phascolarctos-cinereus",
                    "Phascolarctos cinereus (Goldfuss, 1817)",
                    "Koala (Zygodactyl Scansorial Forelimb & Caecal Viscera)",
                ),
            };
            (
                id,
                sci,
                vern,
                SWAMP_WALLABY_VIBE,
                "ecopunk.anatomy.scale5_vertebrate",
            )
        }
        _ => (
            "ecopunk:taxon:corymbia-aparrerinja",
            "Corymbia aparrerinja (Blakely) K.D.Hill & L.A.S.Johnson",
            "Ghost Gum · Ilwempe (Arrernte Country, AIATSIS C8)",
            CORYMBIA_VIBE,
            "ecopunk.taxa.corymbia_aparrerinja",
        ),
    };

    // 1. Evaluate `.vibe` via `vibe::load_program` + `vibe::session::SourceSession`
    let mut session = vibe::session::SourceSession::default();
    let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
    let _ = session.submit(vibe_src, &mut host, vibe::Budget::default());
    let param_expr = format!(
        "let live_peel = {:.3}; let live_explode = {:.3}; let live_cutaway = {:.3}; let live_anthesis = {:.3}; live_peel + live_explode",
        peel_01, explode_01, cutaway_01, anthesis_01
    );
    let _ = session.submit(&param_expr, &mut host, vibe::Budget::default());

    let mut vibe_eval_summary = format!(
        "rev #{} · peel={:.0}% exp={:.0}% cut={:.0}%",
        session.revision(),
        peel_01 * 100.0,
        explode_01 * 100.0,
        cutaway_01 * 100.0,
    );
    if let Ok(prog) = vibe::load_program(CORYMBIA_VIBE) {
        let mut env = vibe::Env::default();
        if let Ok(r_val) = vibe::eval_function(
            &prog,
            "stem_radius_at_height",
            vec![
                json_to_vibe(&serde_json::json!(1.3)),
                json_to_vibe(&serde_json::json!(18.0)),
                json_to_vibe(&serde_json::json!(0.34)),
                json_to_vibe(&serde_json::json!(0.04)),
            ],
            &mut host,
            &mut env,
        ) {
            if let Ok(m_val) = vibe::eval_function(
                &prog,
                "murray_daughter_radius",
                vec![
                    json_to_vibe(&serde_json::json!(0.34)),
                    json_to_vibe(&serde_json::json!(0.72)),
                ],
                &mut host,
                &mut env,
            ) {
                vibe_eval_summary = format!(
                    "rev #{} · r_bh={:?} · r_d={:?}",
                    session.revision(),
                    r_val,
                    m_val,
                );
            }
        }
    }

    // 2. Verify `.10d` Living Container header, sections, bounds, and whole-file CRC-32C
    let mut ten_d_copy = ten_d_container.to_vec();
    let ten_d_crc_valid = verify_whole_file_crc32c(&mut ten_d_copy).is_ok();
    let crc_val = compute_whole_file_crc32c(&ten_d_copy);
    let ten_d_crc32c_hex = format!("0x{:08X}", crc_val);
    let mut ten_d_sections = String::new();
    let mut ten_d_dimensions_m = [1.80_f32, 2.40, 1.80];

    if let Ok(hdr) = Container10dHeader::parse(ten_d_container) {
        if let Ok(descs) = parse_section_table(ten_d_container, &hdr) {
            let mut tags = Vec::new();
            for d in descs {
                let tag = match d.typ() {
                    Some(SectionType::QuantizedMesh) => {
                        let start = d.byte_offset as usize;
                        let end = start.saturating_add(d.byte_length as usize);
                        if let Some(payload) = ten_d_container.get(start..end) {
                            if let Ok((mh, _)) =
                                qualia_core_db::container_10d::parse_mesh_header(payload)
                            {
                                ten_d_dimensions_m = [
                                    (mh.max[0] - mh.min[0]).abs(),
                                    (mh.max[1] - mh.min[1]).abs(),
                                    (mh.max[2] - mh.min[2]).abs(),
                                ];
                            }
                        }
                        "MESH"
                    }
                    Some(SectionType::Materials) => "MAT1",
                    Some(SectionType::TextureCoordinates) => "UV01",
                    Some(SectionType::FieldSidecar) => "FNRM",
                    Some(SectionType::EntityOntology) => "EONT",
                    Some(SectionType::SkeletalAnimation) => "SKL1",
                    Some(SectionType::ProceduralSynthesis) => "PROC",
                    Some(SectionType::ProvenanceSidecar) => "PROV",
                    Some(SectionType::SurfaceReading) => "SURF",
                    _ => "SECT",
                };
                if !tags.contains(&tag) {
                    tags.push(tag);
                }
            }
            ten_d_sections = tags.join("+");
        }
    }

    // 3. Compile enabled Online-Source Metadata Groups into 48-byte `.q42` `NQuin` Super-Quins
    let mut quins = Vec::with_capacity(24);
    let mut sample_triples = Vec::new();
    let ctx = "did:web:dev.civics.au#biosphere-commons";
    let mut clock = 1001u32;

    if enabled_sources[0] {
        // Group 0: GBIF / ALA / APNI / AFD Taxonomy
        let gbif_key = if asset_id.contains("wallabia") || dissection_scale == 5 {
            2440154u64
        } else {
            5418392u64
        };
        quins.push(make_quin(
            asset_id,
            "http://rs.tdwg.org/dwc/terms/scientificName",
            0x20,
            q_hash(sci_name),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "https://www.gbif.org/species/usageKey",
            0x20,
            tag_integer(gbif_key),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "https://biodiversity.org.au/apni/nameID",
            0x20,
            q_hash("apni:taxon:australian-national-species-list"),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        sample_triples.push(format!(
            "[ALA/GBIF] dwc:usageKey {gbif_key} (xsd:integer)"
        ));
    }

    if enabled_sources[1] {
        // Group 1: AusTraits / AVONET / PanTHERIA QUDT SI Biophysical Traits
        quins.push(make_quin(
            asset_id,
            "https://traitecoevo.github.io/austraits/wood_density_kg_m3",
            0x20,
            tag_decimal_micro(850.0),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "https://traitecoevo.github.io/austraits/leaf_mass_per_area_g_m2",
            0x20,
            tag_decimal_micro(215.0),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "http://qudt.org/schema/qudt/height_m",
            0x20,
            tag_decimal_micro(ten_d_dimensions_m[1] as f64),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        sample_triples.push(format!(
            "[AusTraits] woodDensity 850 kg/m³ · H={:.2}m SI",
            ten_d_dimensions_m[1]
        ));
    }

    if enabled_sources[2] {
        // Group 2: Plant Ontology (PO:) / UBERON Anatomy & Spectral Albedo
        let onto_curie = if dissection_scale == 5 || asset_id.contains("wallabia") {
            "http://purl.obolibrary.org/obo/UBERON_0001434"
        } else {
            "http://purl.obolibrary.org/obo/PO_0005848"
        };
        let short_curie = if dissection_scale == 5 || asset_id.contains("wallabia") {
            "UBERON:0001434"
        } else {
            "PO:0005848"
        };
        quins.push(make_quin(
            asset_id,
            "http://purl.obolibrary.org/obo/RO_0002180",
            0x20,
            q_hash(onto_curie),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "urn:ecopunk:spectral:bark_albedo_nir_800nm",
            0x20,
            tag_decimal_micro(0.76),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        sample_triples.push(format!(
            "[PO/UBERON] ro:hasComponent {short_curie} · NIR=0.76"
        ));
    }

    if enabled_sources[3] {
        // Group 3: CSIRO SLGA Soil / GA ELVIS DEM / BoM Climate
        quins.push(make_quin(
            asset_id,
            "https://www.clw.csiro.au/aclep/soilandlandscapegrid/ph_cacl2",
            0x20,
            tag_decimal_micro(6.2),
            ctx,
            0b11,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "https://elevation.fsdf.org.au/epsg_7844_ahd_m",
            0x20,
            tag_decimal_micro(385.0),
            ctx,
            0b11,
            clock,
        ));
        clock += 1;
        sample_triples.push(
            "[SLGA/ELVIS] slga:pH 6.2 · EPSG:7844 / 385m AHD".to_string(),
        );
    }

    if enabled_sources[4] {
        // Group 4: AIATSIS Custodial Country / ABS SEIFA / dev.civics.au Commons
        quins.push(make_quin(
            asset_id,
            "https://collection.aiatsis.gov.au/austlang/custodialCountry",
            0x10,
            q_hash("aiatsis:C8_Arrernte_and_S68_Ngarigo"),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "https://www.abs.gov.au/seifa/irsd_index",
            0x20,
            tag_integer(964),
            ctx,
            0b01,
            clock,
        ));
        clock += 1;
        quins.push(make_quin(
            asset_id,
            "https://dev.civics.au/ns/commonsObligationPool",
            0x10,
            q_hash("civics:PermissiveCommons_NonUsurious_Pool"),
            ctx,
            0b01,
            clock,
        ));
        sample_triples.push(
            "[AIATSIS/ABS] C8/S68 · SEIFA=964 · CommonsPool".to_string(),
        );
    }

    let q42_ecc_all_valid = quins.iter().all(|q| q.verify_ecc_parity());
    let mut q42_bytes_vec = Vec::with_capacity(quins.len() * 48);
    for q in &quins {
        q42_bytes_vec.extend_from_slice(bytemuck::bytes_of(q));
    }

    // 4. Build & Verify `.m64` Matter-64 Binary Container (64B Header + 2×64B Directory + 64B Strata + 64B Holo = 320B)
    let mut strata = M64StrataColumnRecord {
        surface_elevation_mm: 385_000,
        water_table_depth_mm: 1_200,
        surface_fluid_depth_mm: 0,
        fluid_velocity_xy: [0, 0],
        fluid_sediment_load_ppm: 18,
        surface_roughness_mm: 24,
        layer_thickness_mm: [50, 100, 150, 300, 400, 1000],
        layer_lut_index: [1, 2, 3, 4, 5, 6],
        layer_moisture_pct: [64, 118, 165, 140, 82, 24],
        layer_compaction_pct: [32, 76, 138, 175, 218, 250],
        layer_artefact_mask: [1, 1, 2, 4, 0, 0],
        bulk_shear_strength_kpa: 48,
        angle_of_repose_deg10: 345,
        gpr_dielectric_perm_x100: 660,
        acoustic_p_wave_m_s: 1890,
        record_crc32c: 0,
    };
    strata.update_crc();

    let holo = M64HoloRecord {
        provenance_did_hash: q_hash(asset_id),
        ..M64HoloRecord::default()
    };

    let mut dir0 = M64ChunkDirectoryEntry {
        chunk_uuid: [0x6D, 0x36, 0x34, 0x01, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        spatial_hilbert_index: 0x0001_0042,
        byte_offset: 192,
        byte_length: 64,
        record_count: 1,
        srid_geodetic_code: 7844,
        lut_bank_epoch_id: 1,
        chunk_record_mode: M64_MODE_STRATA,
        compression_flags: 0,
        bao_root_hash_prefix: [0xEC; 12],
        entry_crc32c: 0,
    };
    dir0.update_crc();

    let mut dir1 = M64ChunkDirectoryEntry {
        chunk_uuid: [0x6D, 0x36, 0x34, 0x02, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2],
        spatial_hilbert_index: 0x0001_0043,
        byte_offset: 256,
        byte_length: 64,
        record_count: 1,
        srid_geodetic_code: 7844,
        lut_bank_epoch_id: 1,
        chunk_record_mode: M64_MODE_HOLO,
        compression_flags: 0,
        bao_root_hash_prefix: [0x10; 12],
        entry_crc32c: 0,
    };
    dir1.update_crc();

    let mut m64_hdr = M64Header {
        magic: M64_MAGIC,
        version: 2,
        header_size: 64,
        total_file_size: 320,
        spatial_origin: [148.293, -35.576],
        elevation_datum: [385.0, 0.001],
        chunk_table_offset: 64,
        chunk_count: 2,
        header_crc32c: 0,
        container_flags: 0,
    };
    m64_hdr.update_crc();

    let mut m64_bytes = Vec::with_capacity(320);
    m64_bytes.extend_from_slice(bytemuck::bytes_of(&m64_hdr));
    m64_bytes.extend_from_slice(bytemuck::bytes_of(&dir0));
    m64_bytes.extend_from_slice(bytemuck::bytes_of(&dir1));
    m64_bytes.extend_from_slice(bytemuck::bytes_of(&strata));
    m64_bytes.extend_from_slice(bytemuck::bytes_of(&holo));

    let mut m64_scratch = [M64PayloadInterval::default(); 4];
    let m64_parsed = M64Reader::parse(
        &m64_bytes,
        M64ReadLimits {
            max_file_bytes: 65536,
            max_chunks: 8,
            max_records: 256,
        },
        &mut m64_scratch,
    );
    let m64_verified = m64_parsed.is_ok();
    let m64_summary = format!(
        "320B (SLGA 6-Horizon Soil CRC=0x{:08X} + Holo)",
        strata.record_crc32c
    );

    // 5. Package `.vibe` + `.10d` + `.q42` + `.m64` into a page-aligned `.hmc` Hypermedia Bundle & verify
    let mut hmc_writer = BundleWriter::new();
    let _ = hmc_writer.add_file("authoring.vibe", "vibe", vibe_src.as_bytes().to_vec(), None);
    let _ = hmc_writer.add_file("embodiment.10d", "10d", ten_d_container.to_vec(), None);
    let _ = hmc_writer.add_file("metadata.q42", "q42", q42_bytes_vec.clone(), None);
    let _ = hmc_writer.add_file("substrate.m64", "m64", m64_bytes.clone(), None);
    let hmc_bytes_vec = hmc_writer.build().unwrap_or_default();
    let (hmc_entry_count, hmc_all_verified) = match BundleReader::parse(&hmc_bytes_vec) {
        Ok(reader) => {
            let count = reader.entries().len();
            let all_ok = reader.entries().iter().all(|e| reader.verify_entry(&e.key));
            (count, all_ok)
        }
        Err(_) => (0, false),
    };

    AssetForgeReport {
        asset_id: asset_id.to_string(),
        scientific_name: sci_name.to_string(),
        vernacular_and_indigenous: vern_name.to_string(),
        vibe_module: vibe_mod.to_string(),
        vibe_revision: session.revision(),
        vibe_eval_summary,
        ten_d_bytes: ten_d_container.len(),
        ten_d_crc32c_hex,
        ten_d_crc_valid,
        ten_d_sections,
        ten_d_dimensions_m,
        q42_quin_count: quins.len(),
        q42_bytes: q42_bytes_vec.len(),
        q42_ecc_all_valid,
        q42_sample_triples: sample_triples,
        m64_bytes: m64_bytes.len(),
        m64_header_crc_hex: format!("0x{:08X}", m64_hdr.header_crc32c),
        m64_verified,
        m64_summary,
        hmc_bytes: hmc_bytes_vec.len(),
        hmc_entry_count,
        hmc_all_verified,
    }
}
