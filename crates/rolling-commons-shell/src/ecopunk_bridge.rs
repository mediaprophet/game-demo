//! High-grade Ecopunk `.10d` Living Container synthesis, physically energy-conserving
//! Australian IBL `EnvironmentProbeSet` configuration, and 5-Scale Botanical/Zoological
//! Dissection bridge for `rolling-commons-shell` (`dev.civics.au` & Talbingo Continuum).

use crate::botanical_anatomy::{
    build_scale1_whole_organism_10d, build_scale2_stem_bark_wedge_10d,
    build_scale3_floral_and_gumnut_dissection_10d, build_scale4_leaf_venation_and_histology_10d,
    DevelopmentalStage,
};
use crate::fauna_animation_rig::FaunaBiomechanicalArchetype;
use crate::zoological_anatomy::build_scale5_vertebrate_anatomy_10d;
use qualia_core_db::render::lighting::{
    EnvironmentProbe, EnvironmentProbeSet, ENVIRONMENT_SPECULAR_LEVELS,
};
use wasm_bindgen::prelude::*;

/// Number of `f32` values per packed `EnvironmentProbe` record in `QualiaPortal::set_environment_probe_data`.
pub const PORTAL_PROBE_RECORD_FLOATS: usize = 3 + 1 + 3 + ENVIRONMENT_SPECULAR_LEVELS * 3 + 3;

/// Map one of the 8 continuum study sites (`0..=7`) to its canonical `.10d` sector key and title in `ecopunk-terrain`.
pub fn sector_key_for_study_site(site_index: u8) -> (&'static str, &'static str) {
    match site_index {
        0 | 1 => ("talbingo", "Talbingo & Blowering Riparian Corridor"),
        2 => ("jounama_karst", "Jounama Creek Valley & Bogong Tors"),
        3 | 4 => ("yarrangobilly", "Yarrangobilly Silurian Limestone Karst Gorge"),
        5 => ("three_mile_dam", "Three Mile Dam Subalpine Snow Gum & Sphagnum Bog"),
        6 => ("sydney_sandstone", "Sydney Hawkesbury Sandstone Gully"),
        _ => ("noosa_headland", "Noosa Coastal Pandanus Basalt Headland"),
    }
}

/// Build a physically energy-conserving 2-probe `EnvironmentProbeSet` tuned per Australian sector
/// (covering both Community Grounds `dev.civics.au` settlements and Kosciuszko/Talbingo continuum sectors):
/// - Probe 0: Cool Zenith Sky Hemisphere fill (`dominant_direction = [0.0, 1.0, 0.0]`) with
///   roughness-attenuated specular pyramid (`0..5`) so water, rover glass/paint, solar arrays, and waxy leaves
///   catch crisp specular highlights while rough terrain soil and bark receive zero grazing haze.
/// - Probe 1: Warm Australian Solar-Canopy / Ground Bounce (`dominant_direction` aligned with the
///   sun/valley bounce) so shaded relief retains natural 3D depth without any additive gray veil.
pub fn build_sector_environment_probe_set(sector_id: &str) -> EnvironmentProbeSet {
    let (zenith_irr, zenith_spec0, bounce_irr, bounce_dir) = match sector_id {
        "talbingo" => (
            [0.34_f32, 0.42, 0.52],
            [0.16_f32, 0.21, 0.27],
            [0.32_f32, 0.28, 0.20],
            [0.48_f32, 0.36, 0.44],
        ),
        "yarrangobilly" => (
            [0.30_f32, 0.44, 0.52],
            [0.15_f32, 0.22, 0.27],
            [0.28_f32, 0.30, 0.26],
            [-0.22_f32, 0.38, 0.56],
        ),
        "three_mile_dam" => (
            [0.36_f32, 0.46, 0.58],
            [0.18_f32, 0.23, 0.30],
            [0.30_f32, 0.29, 0.22],
            [0.48_f32, 0.35, 0.48],
        ),
        "sydney_sandstone" => (
            [0.36_f32, 0.43, 0.50],
            [0.17_f32, 0.20, 0.25],
            [0.38_f32, 0.29, 0.19],
            [0.40_f32, 0.38, 0.44],
        ),
        "noosa_headland" | "saltwind_reach" => (
            [0.32_f32, 0.48, 0.58],
            [0.17_f32, 0.24, 0.30],
            [0.34_f32, 0.31, 0.24],
            [0.32_f32, 0.40, 0.44],
        ),
        "jounama_karst" | "northern_highlands" => (
            [0.33_f32, 0.41, 0.52],
            [0.16_f32, 0.20, 0.26],
            [0.33_f32, 0.28, 0.21],
            [0.54_f32, 0.36, 0.36],
        ),
        "dissection_lab" | "workshop" => (
            [0.38_f32, 0.44, 0.52],
            [0.20_f32, 0.23, 0.28],
            [0.34_f32, 0.31, 0.25],
            [0.42_f32, 0.45, 0.38],
        ),
        // Default: Community Grounds ("kestrel_flats" / cooperative settlement daylight)
        _ => (
            [0.35_f32, 0.44, 0.54],
            [0.17_f32, 0.22, 0.28],
            [0.35_f32, 0.30, 0.22],
            [0.45_f32, 0.38, 0.42],
        ),
    };

    let build_spec_pyramid = |spec0: [f32; 3]| -> [[f32; 3]; ENVIRONMENT_SPECULAR_LEVELS] {
        [
            spec0,
            [spec0[0] * 0.68, spec0[1] * 0.68, spec0[2] * 0.68],
            [spec0[0] * 0.42, spec0[1] * 0.42, spec0[2] * 0.42],
            [spec0[0] * 0.22, spec0[1] * 0.22, spec0[2] * 0.22],
            [spec0[0] * 0.10, spec0[1] * 0.10, spec0[2] * 0.10],
        ]
    };

    let mut set = EnvironmentProbeSet::new();
    let _ = set.push(EnvironmentProbe {
        position: [0.0, 260.0, 0.0],
        radius: 2400.0,
        irradiance: zenith_irr,
        specular_levels: build_spec_pyramid(zenith_spec0),
        dominant_direction: [0.0, 1.0, 0.0],
        valid: true,
    });

    let bounce_spec0 = [
        bounce_irr[0] * 0.28,
        bounce_irr[1] * 0.28,
        bounce_irr[2] * 0.28,
    ];
    let _ = set.push(EnvironmentProbe {
        position: [0.0, -40.0, 0.0],
        radius: 2400.0,
        irradiance: bounce_irr,
        specular_levels: build_spec_pyramid(bounce_spec0),
        dominant_direction: bounce_dir,
        valid: true,
    });
    set
}

/// Pack an `EnvironmentProbeSet` into the flat `f32` buffer format required by
/// `QualiaPortal::set_environment_probe_data(packed_probes, fallback_radiance)`.
pub fn pack_sector_environment_probes(sector_id: &str) -> (Vec<f32>, [f32; 3]) {
    let probe_set = build_sector_environment_probe_set(sector_id);
    let slice = probe_set.as_slice();
    let mut packed = Vec::with_capacity(slice.len() * PORTAL_PROBE_RECORD_FLOATS);
    for probe in slice {
        if !probe.valid || probe.radius <= 0.0 {
            continue;
        }
        packed.extend_from_slice(&probe.position);
        packed.push(probe.radius);
        packed.extend_from_slice(&probe.irradiance);
        for level in &probe.specular_levels {
            packed.extend_from_slice(level);
        }
        packed.extend_from_slice(&probe.dominant_direction);
    }
    // Keep fallback_radiance strictly [0.0, 0.0, 0.0] because `sample_environment_probe` adds
    // `fallback_radiance * (1.0 - roughness * 0.55)` on top of weighted probe radiance.
    (packed, [0.0, 0.0, 0.0])
}

/// Configure a `QualiaPortal` instance with physically energy-conserving Australian IBL
/// environment probes, directional sun shadows, and screen-space ambient occlusion.
pub fn apply_sector_environment_to_portal(
    portal: &mut qualia_core_db::QualiaPortal,
    sector_id: &str,
) {
    let (packed_probes, fallback_radiance) = pack_sector_environment_probes(sector_id);
    let _ = portal.set_environment_probe_data(&packed_probes, &fallback_radiance);
    portal.set_shadows_enabled(true);
    portal.set_screen_space_ao_enabled(true);
    portal.set_screen_space_ao(0.45, 0.85, 0.02);
}

/// Build the sealed `.10d` Living Container bytes and metadata for any of the 5 educational dissection scales.
pub fn build_dissection_scale_10d(
    scale: u8,
    explode_01: f32,
    cutaway_01: f32,
    anthesis_01: f32,
    peel_01: f32,
    archetype_idx: u8,
) -> Result<(Vec<u8>, serde_json::Value), String> {
    match scale.clamp(1, 5) {
        1 => {
            let organ = build_scale1_whole_organism_10d(
                "corymbia-aparrerinja",
                DevelopmentalStage::Mature,
            )?;
            let meta = serde_json::json!({
                "scale": 1,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        2 => {
            let organ = build_scale2_stem_bark_wedge_10d(
                "corymbia-aparrerinja",
                explode_01.clamp(0.0, 1.0),
            )?;
            let meta = serde_json::json!({
                "scale": 2,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        3 => {
            let organ = build_scale3_floral_and_gumnut_dissection_10d(
                "corymbia-aparrerinja",
                anthesis_01.clamp(0.0, 1.0),
                cutaway_01.clamp(0.0, 1.0),
            )?;
            let meta = serde_json::json!({
                "scale": 3,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        4 => {
            let organ = build_scale4_leaf_venation_and_histology_10d(
                "corymbia-aparrerinja",
                cutaway_01.clamp(0.0, 1.0),
            )?;
            let meta = serde_json::json!({
                "scale": 4,
                "scale_name": organ.scale.name(),
                "physical_range": organ.scale.physical_range(),
                "vertex_count": organ.vertex_count(),
                "triangle_count": organ.triangle_count(),
                "tissue_layer_count": organ.tissue_slices.len(),
                "container_bytes": organ.container_bytes.len(),
            });
            Ok((organ.container_bytes, meta))
        }
        _ => {
            let arch = match archetype_idx % 5 {
                0 => FaunaBiomechanicalArchetype::MacropodSaltatorial,
                1 => FaunaBiomechanicalArchetype::VombatidFossorial,
                2 => FaunaBiomechanicalArchetype::MonotremeTeleostAmphibian,
                3 => FaunaBiomechanicalArchetype::RaptorSoaring,
                _ => FaunaBiomechanicalArchetype::PhascolarctidArboreal,
            };
            let peel = peel_01.clamp(0.0, 1.0);
            let osteo = 1.0_f32;
            let myo = (1.0 - (peel - 0.45).max(0.0) * 1.5).clamp(0.15, 1.0);
            let integ = (1.0 - peel * 1.6).clamp(0.0, 1.0);
            let vert = build_scale5_vertebrate_anatomy_10d(
                arch,
                osteo,
                myo,
                integ,
                explode_01.clamp(0.0, 1.0),
            );
            let meta = serde_json::json!({
                "scale": 5,
                "scale_name": "Scale 5: Comparative Vertebrate 3-Layer Anatomy (Osteology / Myology / Integument)",
                "physical_range": "150 mm – 2.2 m",
                "archetype": format!("{arch:?}"),
                "vertex_count": vert.vertex_count,
                "triangle_count": vert.triangle_count,
                "tissue_layer_count": vert.submesh_ranges.len(),
                "container_bytes": vert.container_bytes.len(),
            });
            Ok((vert.container_bytes, meta))
        }
    }
}

/// WASM export: build and return a sealed `.10d` Living Container + metadata for any of the 5 dissection scales.
#[wasm_bindgen]
pub fn talbingo_dissection_10d(
    scale: u8,
    explode_01: f32,
    cutaway_01: f32,
    anthesis_01: f32,
    peel_01: f32,
    archetype_idx: u8,
) -> Result<JsValue, JsValue> {
    let (bytes, meta) = build_dissection_scale_10d(
        scale,
        explode_01,
        cutaway_01,
        anthesis_01,
        peel_01,
        archetype_idx,
    )
    .map_err(|e| JsValue::from_str(&e))?;
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &"bytes".into(),
        &js_sys::Uint8Array::from(bytes.as_slice()),
    )?;
    let meta_js = serde::Serialize::serialize(
        &meta,
        &serde_wasm_bindgen::Serializer::json_compatible(),
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
    js_sys::Reflect::set(&obj, &"meta".into(), &meta_js)?;
    Ok(obj.into())
}

/// WASM export: build and return the high-grade Ecopunk `.10d` scene bundle + 256×256 PBR textures
/// + Tensor10D semantic pick buffer for a Talbingo / Kosciuszko study site (`0..=5`).
#[wasm_bindgen]
pub fn talbingo_high_grade_sector_10d(site_index: u8) -> Result<JsValue, JsValue> {
    let (sector_key, sector_title) = sector_key_for_study_site(site_index);
    let scene = crate::talbingo_scenes::build(site_index.min(5))?;
    js_sys::Reflect::set(&scene, &"sectorKey".into(), &JsValue::from_str(sector_key))?;
    js_sys::Reflect::set(
        &scene,
        &"sectorTitle".into(),
        &JsValue::from_str(sector_title),
    )?;
    Ok(scene)
}
