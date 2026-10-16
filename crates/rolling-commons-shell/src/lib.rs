//! Maslows Challenge — QualiaDB-backed WASM game shell.
//!
//! Per the QualiaDB-only development contract, this crate exposes game-facing
//! bindings over published QualiaDB surfaces only. It owns no renderer, rules
//! engine, graph store, or persistence of its own.
//!
//! Profile: QualiaDB `wasm-full`, including graph/logic, scientific geometry,
//! WebGPU portal rendering, and the local inference runtime. Core play remains
//! fully available without loading a model.

#![cfg(target_arch = "wasm32")]

mod activity_presentation;
mod animation_bridge;
mod ants;
mod asset_catalog;
mod asset_compile;
mod camera_presentation;
mod camp_home;
mod campaign_simulation;
mod character_simulation;
mod drive_contact_integration;
mod drive_contacts;
mod drive_session;
mod entity_sections;
mod fauna;
mod fauna_animation;
mod flora;
mod macropod_models;
mod material_library;
mod mature_gums;
mod pack_validation;
mod quadruped_animation;
mod quadruped_models;
mod resident_animation;
mod resident_props;
mod rig_workshop;
mod semantic_slice;
mod session_journal;
mod talbingo_scenes;
pub(crate) mod talbingo_terrain;
mod talbingo_vegetation;
mod taxon_bindings;
mod terrain;
mod variant_resolver;
mod vehicle_animation;
mod vehicle_cabins;
mod vehicle_camera;
mod vehicle_exterior;
mod vehicles;
mod water_presentation;
mod workshop;
mod workshop_assembly;
mod workshop_structure;

pub use campaign_simulation::CampaignSimulation;
pub use character_simulation::CharacterSimulation;
pub use session_journal::GameSessionJournal;
pub use variant_resolver::EntityFamilyManifestWrapper;

/// Six authored, illustrative campsite scenes compiled by QualiaDB.
#[wasm_bindgen]
pub fn talbingo_scene(site: u8) -> Result<JsValue, JsValue> {
    talbingo_scenes::build(site)
}

/// Present a VibeScript accepted Talbingo vehicle pose through Qualia's
/// skeletal scene actor. This does not implement movement or rendering.
#[wasm_bindgen]
pub fn talbingo_vehicle_pose(
    portal: &mut GamePortal,
    x: f64,
    z: f64,
    heading: f64,
) -> Result<u32, JsValue> {
    talbingo_vehicle_pose_y(portal, x, 0.0, z, heading)
}

#[wasm_bindgen]
pub fn talbingo_vehicle_pose_y(
    portal: &mut GamePortal,
    x: f64,
    y: f64,
    z: f64,
    heading: f64,
) -> Result<u32, JsValue> {
    use qualia_core_db::render::skeletal::Motor;
    if !x.is_finite()
        || !y.is_finite()
        || !z.is_finite()
        || !heading.is_finite()
        || x < -6.0
        || x > 23.0
        || z.abs() > 8.5
    {
        return Err(JsValue::from_str("Talbingo vehicle pose outside scene"));
    }
    let yaw = heading * 0.5;
    let root = Motor::from_translation(x, y, z)
        .mul(&Motor::from_rotation_translation(
            [yaw.cos(), 0.0, yaw.sin(), 0.0],
            [0.0; 3],
        ))
        .mul(&Motor::from_translation(-2.35, 0.0, -4.65));
    portal
        .inner
        .set_scene_actor_root(crate::vehicle_animation::root_id(0), root)
        .map_err(|e| JsValue::from_str(&format!("Talbingo actor pose: {e:?}")))
}

#[wasm_bindgen]
pub fn talbingo_road_z(site: u8, x: f32) -> f32 {
    talbingo_scenes::road_z(site.min(5), x)
}

#[wasm_bindgen]
pub fn talbingo_terrain_elevation(site: u8, x: f32, z: f32) -> f32 {
    crate::talbingo_terrain::elevation(site.min(5), x, z)
}

#[wasm_bindgen]
pub fn talbingo_walker_pose(portal: &mut GamePortal, x: f64, z: f64) -> Result<u32, JsValue> {
    talbingo_walker_pose_y(portal, x, 0.0, z)
}

#[wasm_bindgen]
pub fn talbingo_walker_pose_y(
    portal: &mut GamePortal,
    x: f64,
    y: f64,
    z: f64,
) -> Result<u32, JsValue> {
    use qualia_core_db::render::skeletal::Motor;
    if !x.is_finite()
        || !y.is_finite()
        || !z.is_finite()
        || !(13.0..=24.0).contains(&x)
        || !(-7.0..=7.0).contains(&z)
    {
        return Err(JsValue::from_str("Talbingo walker pose outside site"));
    }
    let root = Motor::from_translation(x, y, z).mul(&Motor::from_translation(-18.0, 0.0, -3.5));
    portal
        .inner
        .set_scene_actor_root(talbingo_scenes::walker_root(), root)
        .map_err(|e| JsValue::from_str(&format!("Talbingo walker pose: {e:?}")))
}

use std::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

/// Full QualiaDB engine revision used by the reproducible game build.
/// The sibling checkout is verified by scripts/build-game.ps1 and CI.
pub const QUALIADB_PINNED_REVISION: &str = include_str!("../../../qualia.ref");

thread_local! { static ART_STYLE: Cell<u8> = const { Cell::new(0) }; }
thread_local! { static VEHICLE_RIG: Cell<u8> = const { Cell::new(0) }; }
// Place profiles carry stable render keys, so the ecology pipeline can add
// taxa without squeezing Australia's species catalogue into a numeric mask.
thread_local! { static FAUNA_SELECTION: RefCell<Vec<String>> = RefCell::new(vec![
    "goat".into(), "horse".into(), "rabbit".into(), "kangaroo".into(),
    "wallaby".into(), "wombat".into(), "deer".into(), "roadkill-crows".into(),
    "kookaburra".into(), "snake".into(), "goanna".into(),
]); }
thread_local! { static FLORA_SELECTION: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) }; }

/// Presentation preference only; world state and game rules are unchanged.
#[wasm_bindgen]
pub fn set_art_style(style: u8) {
    ART_STYLE.with(|active| active.set(style.min(2)));
}

/// Select a presentation-only touring combination: campervan, 4WD/caravan,
/// car/camper-trailer, or 4WD/roof-top tent.
#[wasm_bindgen]
pub fn set_vehicle_rig(rig: u8) {
    VEHICLE_RIG.with(|active| active.set(rig.min(3)));
}

/// Select fauna render keys from a reviewed locality profile. Unknown keys
/// stay in the source profile as data and do not become guessed 3D species.
#[wasm_bindgen]
pub fn set_fauna_selection(selection_json: &str) -> Result<(), JsValue> {
    if selection_json.len() > 256_000 {
        return Err(JsValue::from_str("ecology selection exceeds bound"));
    }
    let selection: Vec<String> = serde_json::from_str(selection_json)
        .map_err(|error| JsValue::from_str(&format!("ecology selection: {error}")))?;
    if selection.len() > 4096
        || selection
            .iter()
            .any(|key| key.trim().is_empty() || key.len() > 128)
    {
        return Err(JsValue::from_str(
            "ecology selection entry is outside bounds",
        ));
    }
    FAUNA_SELECTION.with(|active| *active.borrow_mut() = selection);
    Ok(())
}

/// Select original Australian plant groups from a reviewed locality profile.
/// Unknown keys remain source-profile data and do not receive guessed models.
#[wasm_bindgen]
pub fn set_flora_selection(selection_json: &str) -> Result<(), JsValue> {
    if selection_json.len() > 256_000 {
        return Err(JsValue::from_str("flora selection exceeds bound"));
    }
    let selection: Vec<String> = serde_json::from_str(selection_json)
        .map_err(|error| JsValue::from_str(&format!("flora selection: {error}")))?;
    if selection.len() > 4096
        || selection.iter().any(|key| {
            key.trim().is_empty()
                || key.len() > 128
                || !key
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        })
    {
        return Err(JsValue::from_str("flora selection entry is outside bounds"));
    }
    FLORA_SELECTION.with(|active| *active.borrow_mut() = selection);
    Ok(())
}

/// Community Grounds: the muted sage, olive, warm tan, terracotta and cream
/// script sampled from the civics.au community-ground concept illustration.
/// Object families keep their role colours, so state readings such as
/// powered windows and online indicators stay legible at overview scale.
fn community_grounds_color(id: &str, color: [f32; 4]) -> [f32; 4] {
    // Hero families now have explicit material palettes. A family-name tint
    // must not turn green steel roofing orange or wash out blue solar cells.
    if id.contains("/workshop")
        || id.contains("/solar-")
        || id.contains("/camp-shelter")
        || id.contains("-attachment-")
    {
        return color;
    }
    let has = |words: &[&str]| words.iter().any(|word| id.contains(word));
    // State-bearing and character surfaces keep their authored reading.
    let keep = has(&[
        "window",
        "glow",
        "signal",
        "indicator",
        "node",
        "sign",
        "eyes",
        "face",
        "hair",
        "boot",
        "door",
    ]);
    let (target, mix) = if keep {
        ([0.58, 0.53, 0.47], 0.10)
    } else if has(&["solar", "panel"]) {
        ([0.15, 0.24, 0.34], 0.42)
    } else if has(&["trunk", "stem", "branch", "bark", "fork"]) {
        ([0.64, 0.52, 0.38], 0.30)
    } else if has(&["awning", "roof", "chimney", "band", "banner", "fascia"])
        || id.contains("market-canopy")
    {
        ([0.75, 0.44, 0.25], 0.42)
    } else if has(&[
        "canopy",
        "crown",
        "foliage",
        "leaf",
        "shoot",
        "shrub",
        "herb",
        "grass",
        "wattle",
        "lavender",
        "orchard",
        "gum",
        "flower",
        "banksia",
        "bottlebrush",
        "grass-tree",
        "saltbush",
        "kangaroo-paw",
        "reeds",
        "windbreak",
        "veggie",
        "spray",
        "bloom",
        "crop",
    ]) {
        ([0.50, 0.56, 0.42], 0.46)
    } else if has(&["water", "canal", "ripple", "channel", "creek", "trough"]) {
        ([0.45, 0.57, 0.62], 0.26)
    } else if has(&[
        "mast", "cable", "rack", "drum", "wheel", "chassis", "light", "pole", "pipe",
    ]) {
        ([0.55, 0.54, 0.52], 0.22)
    } else if has(&[
        "deck", "boards", "fence", "basket", "crate", "stand", "rail", "plank",
    ]) {
        ([0.64, 0.52, 0.38], 0.30)
    } else if has(&[
        "workshop", "hall", "barn", "market", "stall", "shelter", "facade",
    ]) {
        ([0.92, 0.89, 0.83], 0.22)
    } else if has(&[
        "ground", "terrain", "earth", "road", "bank", "meadow", "hill", "skirt", "soil", "furrow",
        "mound", "boulder", "stone", "path", "bed", "quay",
    ]) {
        ([0.62, 0.54, 0.36], 0.40)
    } else {
        ([0.60, 0.53, 0.45], 0.15)
    };
    let luminance = color[0] * 0.25 + color[1] * 0.58 + color[2] * 0.17;
    let mut result = color;
    for axis in 0..3 {
        let muted = luminance * 0.28 + color[axis] * 0.72;
        result[axis] = (muted * (1.0 - mix) + target[axis] * mix).clamp(0.0, 1.0);
    }
    result
}

fn style_color(id: &str, color: [f32; 4], style: u8) -> [f32; 4] {
    // Preserve natural hide/coat palettes across the stylised built-world packs.
    if id.contains("/fauna/") {
        return color;
    }
    if style == 0 {
        return color;
    }
    if style == 2 {
        return community_grounds_color(id, color);
    }
    // Earthlight: a restrained earth/foliage colour script. The same Qualia
    // geometry and `.10d` surface-reading path carries both styles.
    let woody = ["trunk", "stem", "branch", "bark", "fork"]
        .iter()
        .any(|word| id.contains(word));
    let foliage = [
        "crown",
        "canopy",
        "foliage",
        "leaf",
        "shoot",
        "shrub",
        "herb",
        "grass",
        "wattle",
        "lavender",
        "orchard",
        "gum",
        "cone",
        "bottlebrush",
        "saltbush",
        "kangaroo-paw",
    ]
    .iter()
    .any(|word| id.contains(word));
    let earth = [
        "ground", "terrain", "earth", "road", "bank", "meadow", "hill",
    ]
    .iter()
    .any(|word| id.contains(word));
    let target = if foliage && !woody {
        [0.36, 0.52, 0.33]
    } else if earth {
        [0.55, 0.43, 0.31]
    } else {
        [0.65, 0.52, 0.40]
    };
    let mix = if (foliage && !woody) || earth {
        0.38
    } else {
        0.18
    };
    let luminance = color[0] * 0.25 + color[1] * 0.58 + color[2] * 0.17;
    let mut result = color;
    for axis in 0..3 {
        let muted = luminance * 0.28 + color[axis] * 0.72;
        result[axis] = (muted * (1.0 - mix) + target[axis] * mix).clamp(0.0, 1.0);
    }
    result
}

#[wasm_bindgen]
pub fn pinned_qualiadb_revision() -> String {
    QUALIADB_PINNED_REVISION.trim().to_string()
}

/// Package intact game assets through Qualia's transparent QBDL/HMC writer.
/// The application owns entry names and its manifest; Qualia owns the format,
/// checksums, alignment, and reader verification.
#[wasm_bindgen]
pub fn pack_game_hmc(entries: &js_sys::Array, manifest_json: &str) -> Result<Vec<u8>, JsValue> {
    use qualia_core_db::bundle::{BundleReader, BundleWriter};

    if entries.length() > 16384 || manifest_json.len() > 16_777_216 {
        return Err(JsValue::from_str(
            "game pack exceeds bounded entry or manifest limit",
        ));
    }
    let mut writer = BundleWriter::new();
    writer
        .add_file(
            "manifest.json",
            "manifest",
            manifest_json.as_bytes().to_vec(),
            None,
        )
        .map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    for item in entries.iter() {
        let key = js_sys::Reflect::get(&item, &"key".into())?
            .as_string()
            .ok_or_else(|| JsValue::from_str("HMC entry key must be a string"))?;
        let texture = key.strip_prefix("textures/sha256/").map_or(false, |hex| {
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        });
        // Semantic entries carry the canonical .q42 unified volume
        // (object-sorted SuperBlocks, embedded Q42LEX lexicon, BIDX +
        // field-range + postings indexes). These are how ontology data
        // reaches the browser — the native ontology loader is cfg !wasm32,
        // so packs are the delivery path (G50). Loose .quins/.lex entries
        // are retired.
        let semantic = key.strip_prefix("sem/").map_or(false, |name| {
            name.bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'.')
                && name.ends_with(".q42")
        });
        if ((!key.starts_with("10d/") || !key.ends_with(".10d")) && !texture && !semantic)
            || key.contains("..")
        {
            return Err(JsValue::from_str("invalid game HMC entry key"));
        }
        let value = js_sys::Reflect::get(&item, &"bytes".into())?;
        let bytes = value
            .dyn_into::<js_sys::Uint8Array>()
            .map_err(|_| JsValue::from_str("HMC entry bytes must be Uint8Array"))?
            .to_vec();
        if key.ends_with(".q42")
            && (bytes.len() < 4 || bytes[0..4] != qualia_core_db::q42_volume::Q42_MAGIC)
        {
            return Err(JsValue::from_str("semantic q42 entry has a bad magic"));
        }
        let kind = if texture {
            "image/png"
        } else if semantic {
            "q42"
        } else {
            "10d"
        };
        writer
            .add_file(key, kind, bytes, None)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
    }
    let bytes = writer
        .build()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let reader = BundleReader::parse(&bytes).map_err(|e| JsValue::from_str(&e.to_string()))?;
    if reader
        .entries()
        .iter()
        .any(|entry| !reader.verify_entry(&entry.key))
    {
        return Err(JsValue::from_str("HMC entry digest verification failed"));
    }
    Ok(bytes)
}

/// Validate a game pack using Qualia's reader and report its verified entry count.
#[wasm_bindgen]
pub fn verify_game_hmc(bytes: &[u8]) -> Result<u32, JsValue> {
    use qualia_core_db::bundle::BundleReader;
    let reader =
        BundleReader::parse(bytes).map_err(|error| JsValue::from_str(&error.to_string()))?;
    if reader.entries().len() > 16385
        || reader
            .entries()
            .iter()
            .any(|entry| !reader.verify_entry(&entry.key))
    {
        return Err(JsValue::from_str(
            "HMC entry count or digest verification failed",
        ));
    }
    Ok(reader.entries().len() as u32)
}

/// Hot-path pack inspection for scene hydration: structural validation
/// (magic, version, whole-file CRC, entry bounds) plus the entry count.
/// Per-entry SHA-256 is left to `load_hmc_scene`, which digests each asset
/// and texture it actually resolves; the cold-path `verify_game_hmc` /
/// `validate_game_hmc` gates still hash every entry.
#[wasm_bindgen]
pub fn inspect_game_hmc(bytes: &[u8]) -> Result<u32, JsValue> {
    use qualia_core_db::bundle::BundleReader;
    let reader =
        BundleReader::parse(bytes).map_err(|error| JsValue::from_str(&error.to_string()))?;
    if reader.entries().len() > 16385 {
        return Err(JsValue::from_str("HMC entry count exceeds bound"));
    }
    Ok(reader.entries().len() as u32)
}

/// Cold-path release gate: verify every embedded .10d mesh and material graph.
#[wasm_bindgen]
pub fn validate_game_hmc(bytes: &[u8]) -> Result<u32, JsValue> {
    pack_validation::verify_game_pack(bytes).map_err(|error| JsValue::from_str(&error))
}

#[wasm_bindgen]
pub fn init_panic_hook_shell() {
    qualia_core_db::init_panic_hook();
}

/// Parse a Turtle world description into the QualiaDB graph representation.
/// Returns the engine's parse receipt (triple list / errors) as a JS value.
#[wasm_bindgen]
pub fn world_load_turtle(turtle: &str) -> JsValue {
    qualia_core_db::parse_turtle_wasm(turtle)
}

/// Parse N3Logic rules. Rules are content; they validate and explain commands,
/// they never mutate state directly.
#[wasm_bindgen]
pub fn rules_load_n3(n3: &str) -> JsValue {
    qualia_core_db::parse_n3logic_wasm(n3)
}

/// Validate a data graph (N3 token form) against JSON SHACL ShapeSpecs.
/// Returns the validation report (`conforms`, per-constraint results) —
/// the basis of the player-facing "Why?" view.
#[wasm_bindgen]
pub fn action_validate_shacl(data_n3: &str, shapes_json: &str) -> Result<JsValue, JsValue> {
    qualia_core_db::wasm_bridge::validate_shacl_json_wasm(data_n3, shapes_json)
}

/// Persist world bytes via QualiaDB's OPFS-backed virtual filesystem.
/// `path` is currently a bare filename — nested paths are not supported by
/// `OpfsVfs` (recorded in the capability ledger).
#[wasm_bindgen]
pub async fn world_save(path: &str, data: &[u8]) -> Result<(), JsValue> {
    use qualia_core_db::storage::{OpfsVfs, VirtualFileSystem};
    OpfsVfs
        .write_chunk(path, data)
        .await
        .map_err(|e| JsValue::from_str(&e))
}

/// Load world bytes previously persisted through `world_save`.
#[wasm_bindgen]
pub async fn world_load(path: &str) -> Result<Vec<u8>, JsValue> {
    use qualia_core_db::storage::{OpfsVfs, VirtualFileSystem};
    OpfsVfs
        .read_chunk(path)
        .await
        .map_err(|e| JsValue::from_str(&e))
}

// --- Asset path: agent-authored geometry → `.10d` → semantic manifest ---

/// Compile a game-authored prop into a sealed `.10d` container using
/// QualiaDB's mesh pipeline. `positions` is flat xyz triples (f32),
/// `triangles` flat vertex-index triples (u32, CCW), `nodes_flat` is
/// optional Tensor10D nodes as groups of 10 floats
/// `[q, v, w, x, y, z, t, alpha, mu, sigma]` — the semantic pick targets.
#[wasm_bindgen]
pub fn asset_compile_10d(
    positions: &[f32],
    triangles: &[u32],
    nodes_flat: &[f32],
) -> Result<Vec<u8>, JsValue> {
    use qualia_core_db::render::assets::Mesh;
    use qualia_core_db::render::compile_10d::compile_mesh_to_10d_with_nodes;

    if positions.len() % 3 != 0 || positions.is_empty() {
        return Err(JsValue::from_str("positions must be non-empty xyz triples"));
    }
    if triangles.len() % 3 != 0 || triangles.is_empty() {
        return Err(JsValue::from_str(
            "triangles must be non-empty index triples",
        ));
    }
    if nodes_flat.len() % 10 != 0 {
        return Err(JsValue::from_str("nodes must be groups of 10 floats"));
    }
    let positions: Vec<[f32; 3]> = positions
        .chunks_exact(3)
        .map(|c| [c[0], c[1], c[2]])
        .collect();
    let triangles: Vec<[u32; 3]> = triangles
        .chunks_exact(3)
        .map(|c| [c[0], c[1], c[2]])
        .collect();
    if triangles
        .iter()
        .flatten()
        .any(|&i| i as usize >= positions.len())
    {
        return Err(JsValue::from_str("triangle index out of range"));
    }
    let mut min = [f32::MAX; 3];
    let mut max = [f32::MIN; 3];
    for p in &positions {
        for k in 0..3 {
            min[k] = min[k].min(p[k]);
            max[k] = max[k].max(p[k]);
        }
    }
    let mesh = Mesh {
        positions,
        triangles,
        min,
        max,
    };
    let nodes: Vec<qualia_core_db::tensor::Tensor10D> = nodes_flat
        .chunks_exact(10)
        .map(|n| {
            qualia_core_db::tensor::Tensor10D::new(
                n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9],
            )
        })
        .collect();
    compile_mesh_to_10d_with_nodes(&mesh, &nodes).map_err(|e| JsValue::from_str(&format!("{e:?}")))
}

/// Compile authored review geometry with an explicit source attestation so
/// QualiaPortal can admit it through its normal governance check.
#[wasm_bindgen]
pub fn asset_compile_10d_with_provenance(
    positions: &[f32],
    triangles: &[u32],
    source: &str,
) -> Result<Vec<u8>, JsValue> {
    use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
    use qualia_core_db::render::compile_10d::{
        compile_mesh_to_10d_with_extras_and_frames, decode_10d_mesh, Compile10dExtras,
    };

    if source.trim().is_empty() || source.len() > 64 * 1024 {
        return Err(JsValue::from_str(
            "source attestation must be non-empty and at most 64 KiB",
        ));
    }
    let bare = asset_compile_10d(positions, triangles, &[])?;
    let mesh = decode_10d_mesh(&bare).map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    let provenance = ProvenanceSidecar::new(
        source.as_bytes().to_vec(),
        "application/vnd.ecopunk.model-source+json;version=1",
        "All rights reserved (licence not assigned)",
    );
    compile_mesh_to_10d_with_extras_and_frames(
        &mesh,
        &[],
        Some(&provenance),
        Compile10dExtras::default(),
        None,
        None,
        None,
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Compile one feature from a caller-supplied OSM Overpass JSON snapshot into
/// a provenance-bearing `.10d` mesh. This function performs no network access;
/// acquisition and consent belong to the caller, and the snapshot can be
/// authored offline. `style` is `marker`, `line`, or `area`; `size_m` is the
/// marker radius or line width and is ignored for area fills.
#[wasm_bindgen]
pub fn compile_osm_feature_10d(
    overpass_json: &str,
    feature_type: &str,
    feature_id: u64,
    origin_lat: f64,
    origin_lon: f64,
    origin_alt_m: f64,
    style: &str,
    size_m: f32,
    vertical_offset_m: f32,
) -> Result<Vec<u8>, JsValue> {
    let frame = qualia_core_db::domains::geospatial::vector_scene::VectorSceneFrame::new(
        origin_lat,
        origin_lon,
        origin_alt_m,
    );
    compile_osm_feature_10d_with_frame(
        overpass_json,
        feature_type,
        feature_id,
        style,
        size_m,
        vertical_offset_m,
        frame,
    )
}

/// Compile one OSM feature and clip every emitted triangle to an exact local
/// tile rectangle. Bounds are `[min_east, min_north, max_east, max_north]` in
/// metres relative to the supplied WGS84 origin. No network access occurs.
#[wasm_bindgen]
pub fn compile_osm_feature_10d_in_tile(
    overpass_json: &str,
    feature_type: &str,
    feature_id: u64,
    origin_lat: f64,
    origin_lon: f64,
    origin_alt_m: f64,
    style: &str,
    size_m: f32,
    vertical_offset_m: f32,
    min_east: f64,
    min_north: f64,
    max_east: f64,
    max_north: f64,
) -> Result<Vec<u8>, JsValue> {
    let frame = qualia_core_db::domains::geospatial::vector_scene::VectorSceneFrame::new(
        origin_lat,
        origin_lon,
        origin_alt_m,
    )
    .with_tile_bounds([min_east, min_north, max_east, max_north]);
    compile_osm_feature_10d_with_frame(
        overpass_json,
        feature_type,
        feature_id,
        style,
        size_m,
        vertical_offset_m,
        frame,
    )
}

fn compile_osm_feature_10d_with_frame(
    overpass_json: &str,
    feature_type: &str,
    feature_id: u64,
    style: &str,
    size_m: f32,
    vertical_offset_m: f32,
    frame: qualia_core_db::domains::geospatial::vector_scene::VectorSceneFrame,
) -> Result<Vec<u8>, JsValue> {
    use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
    use qualia_core_db::domains::geospatial::adapters::OsmAdapter;
    use qualia_core_db::domains::geospatial::vector_scene::{
        compile_osm_feature_mesh, OsmVectorStyle,
    };
    use qualia_core_db::render::compile_10d::compile_mesh_to_10d_with_surface_reading;

    const MAX_SNAPSHOT_BYTES: usize = 16 * 1024 * 1024;
    if overpass_json.len() > MAX_SNAPSHOT_BYTES {
        return Err(JsValue::from_str(
            "OSM snapshot exceeds the 16 MiB feature compile bound",
        ));
    }
    let vector_style = match style {
        "marker" => OsmVectorStyle::Marker {
            radius_m: size_m,
            vertical_offset_m,
        },
        "line" => OsmVectorStyle::Line {
            width_m: size_m,
            vertical_offset_m,
        },
        "area" => OsmVectorStyle::Area { vertical_offset_m },
        _ => return Err(JsValue::from_str("OSM style must be marker, line, or area")),
    };
    let adapter = OsmAdapter::new("offline_osm_snapshot", "", "");
    let features = adapter
        .parse_vector_features(overpass_json)
        .map_err(|error| JsValue::from_str(&format!("OSM snapshot: {error}")))?;
    let feature = features
        .iter()
        .find(|feature| feature.element_type == feature_type && feature.id == feature_id)
        .ok_or_else(|| JsValue::from_str("OSM feature identity was not present in the snapshot"))?;
    let rendered = compile_osm_feature_mesh(feature, frame, vector_style)
        .map_err(|error| JsValue::from_str(&format!("OSM feature mesh: {error}")))?;
    let source_record = serde_json::json!({
        "source": rendered.source_iri,
        "elementType": feature_type,
        "elementId": feature_id,
        "version": rendered.source_version,
        "timestamp": rendered.source_timestamp,
        "tags": rendered.tags,
        "attribution": "© OpenStreetMap contributors",
        "license": "ODbL-1.0"
    });
    let source_bytes = serde_json::to_vec(&source_record)
        .map_err(|error| JsValue::from_str(&format!("OSM provenance: {error}")))?;
    let provenance = ProvenanceSidecar::new(
        source_bytes,
        "application/vnd.openstreetmap.feature+json",
        "ODbL-1.0 (OpenStreetMap contributors)",
    );
    let surface_reading = vec![[1.0; 4]; rendered.mesh.positions.len()];
    compile_mesh_to_10d_with_surface_reading(&rendered.mesh, Some(&provenance), &surface_reading)
        .map_err(|error| JsValue::from_str(&format!("OSM .10d compile: {error}")))
}

/// Build a pickable tensor buffer (32B header + N×40 `Tensor10D`) from the
/// same authored nodes — the surface `QualiaPortal::upload_tensor_buffer`
/// ingests for semantic picking. Groups of 10 floats as in
/// `asset_compile_10d`.
#[wasm_bindgen]
pub fn tensor_buffer_build(nodes_flat: &[f32]) -> Result<Vec<u8>, JsValue> {
    use qualia_core_db::tensor::buffer_export::{write_tensor_buffer, TensorBufferHeader};
    use qualia_core_db::tensor::Tensor10D;

    if nodes_flat.len() % 10 != 0 || nodes_flat.is_empty() {
        return Err(JsValue::from_str(
            "nodes must be non-empty groups of 10 floats",
        ));
    }
    let nodes: Vec<Tensor10D> = nodes_flat
        .chunks_exact(10)
        .map(|n| Tensor10D::new(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9]))
        .collect();
    let mut buf = vec![0u8; TensorBufferHeader::total_bytes(nodes.len())];
    write_tensor_buffer(&nodes, &mut buf).map_err(|e| JsValue::from_str(e))?;
    Ok(buf)
}

// --- Deterministic scene presentation bridge -----------------------------

/// Pick the checked-in HMC scene whose authored geometry matches the current
/// story milestone. This is presentation state only; accepted actions and
/// their replay tape remain owned by `GameSession` below.
fn bridge_scene_key(
    upgrades: u32,
    parts: u32,
    online: bool,
    water_online: bool,
    bridge_open: bool,
    bridge_braced: bool,
    high_tide: bool,
    pump_online: bool,
    orchard_active: bool,
    orchard_harvested: bool,
    signal_online: bool,
) -> &'static str {
    if orchard_harvested {
        "harvested"
    } else if bridge_open && pump_online && orchard_active {
        "thriving"
    } else if bridge_open {
        "bridge-open"
    } else if high_tide && bridge_braced {
        "braced-crossing"
    } else if high_tide {
        "high-tide"
    } else if signal_online {
        "signal-online"
    } else if online {
        "workshop-online"
    } else if upgrades > 0 || parts > 0 || water_online {
        "early-progress"
    } else {
        "opening"
    }
}

fn bridge_seed(scene: &str, day: u32, water_online: bool, high_tide: bool) -> u32 {
    // A small integer mixer is deliberate here: no wall clock, random source,
    // GPU counter, or iteration order enters scene effects.
    let mut seed = 0x9e37_79b9_u32 ^ day.wrapping_mul(0x85eb_ca6b);
    for byte in scene.bytes() {
        seed = (seed ^ u32::from(byte)).wrapping_mul(0xc2b2_ae35);
        seed = seed.rotate_left(13);
    }
    if water_online {
        seed ^= 0x51ed_0a7e;
    }
    if high_tide {
        seed ^= 0xa771_de5e;
    }
    seed ^ (seed >> 16)
}

#[derive(serde::Serialize)]
struct SceneQualityBridge {
    scene_key: &'static str,
    terrain_phase: &'static str,
    water_state: &'static str,
    water_level: f32,
    water_phase: u32,
    ambient_seed: u32,
    effect_seed: u32,
    water_ripples: bool,
    shore_foam: bool,
    harvest_glow: bool,
    quality_tier: u8,
    ambient_enabled: bool,
    hmc_resident: bool,
    fallback: &'static str,
    camera_cue: &'static str,
    camera_focus: i32,
    temporal_slice: f32,
    temporal_window: f32,
}

/// Derive all renderer-facing presentation choices from the deterministic
/// story snapshot. This never mutates the world and never becomes part of the
/// event log, so saves and replay tapes remain byte-for-byte compatible.
#[wasm_bindgen]
pub fn scene_quality_state(
    upgrades: u32,
    parts: u32,
    online: bool,
    _approved: bool,
    water_online: bool,
    garden_active: bool,
    signal_online: bool,
    bridge_open: bool,
    bridge_braced: bool,
    high_tide: bool,
    pump_online: bool,
    orchard_active: bool,
    orchard_harvested: bool,
    day: u32,
    portal_tier: u8,
    hmc_ready: bool,
) -> Result<JsValue, JsValue> {
    let scene_key = bridge_scene_key(
        upgrades,
        parts,
        online,
        water_online,
        bridge_open,
        bridge_braced,
        high_tide,
        pump_online,
        orchard_active,
        orchard_harvested,
        signal_online,
    );
    let water_state = if high_tide && !bridge_open {
        "high-tide"
    } else if water_online {
        "flowing"
    } else {
        "dry"
    };
    let water_level = match water_state {
        "high-tide" => 1.0,
        "flowing" => 0.62,
        _ => 0.18,
    };
    let quality_tier = if portal_tier >= 2 {
        2
    } else if portal_tier >= 1 {
        1
    } else {
        0
    };
    let camera_focus = if high_tide && !bridge_open {
        7
    } else if !water_online && (upgrades > 0 || parts > 0) {
        5
    } else if bridge_open && !pump_online {
        8
    } else if orchard_active {
        9
    } else {
        -1
    };
    let camera_cue = if camera_focus >= 0 { "survey" } else { "map" };
    let seed = bridge_seed(scene_key, day, water_online, high_tide);
    serde_wasm_bindgen::to_value(&SceneQualityBridge {
        scene_key,
        terrain_phase: if bridge_open {
            "connected"
        } else if high_tide {
            "flooded"
        } else if garden_active {
            "cultivated"
        } else {
            "settled"
        },
        water_state,
        water_level,
        water_phase: day % 8,
        ambient_seed: seed,
        effect_seed: seed.rotate_left(7) ^ 0x6d2b_79f5,
        water_ripples: water_online || high_tide,
        shore_foam: high_tide || bridge_open,
        harvest_glow: orchard_harvested,
        quality_tier,
        ambient_enabled: quality_tier >= 2,
        hmc_resident: hmc_ready,
        fallback: match quality_tier {
            2 => "webgpu",
            1 => "canvas2d",
            _ => "unavailable",
        },
        camera_cue,
        camera_focus,
        temporal_slice: (day % 8) as f32,
        temporal_window: if high_tide { 2.0 } else { 1.0 },
    })
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Verify a `.10d` container read-back: mesh decode + content digest, i.e.
/// the Q42 manifest linkage the game uses to bind semantic IDs to geometry.
#[wasm_bindgen]
pub fn asset_verify_10d(bytes: &[u8]) -> Result<JsValue, JsValue> {
    use qualia_core_db::render::compile_10d::{compiled_digest, decode_10d_mesh};

    let mesh = decode_10d_mesh(bytes).map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    let (rig_joints, animation_clips) =
        crate::pack_validation::skeletal_counts(bytes).map_err(|e| JsValue::from_str(&e))?;
    let normal_vertices =
        qualia_core_db::render::compile_10d::decode_10d_normals(bytes, mesh.vertex_count())
            .map_err(|e| JsValue::from_str(&format!("normal stream: {e:?}")))?
            .map_or(0, |n| n.len());
    let mut uv = vec![[0.0; 2]; mesh.vertex_count()];
    let uv_vertices = match qualia_core_db::container_10d::read_texture_coordinate_section_into(
        bytes,
        mesh.vertex_count(),
        &mut uv,
    ) {
        Ok(n) => n,
        Err(qualia_core_db::container_10d::TextureCoordinateContainerError::Missing) => 0,
        Err(e) => return Err(JsValue::from_str(&format!("UV stream: {e:?}"))),
    };
    let (materials, material_ranges) =
        match qualia_core_db::container_10d::material_container_counts(bytes) {
            Ok(counts) => counts,
            Err(qualia_core_db::container_10d::MaterialContainerError::Missing) => (0, 0),
            Err(error) => return Err(JsValue::from_str(&format!("material section: {error:?}"))),
        };
    // `decode_surfel_section` takes the *section payload*, not the whole
    // container — slice it out of the section table first.
    let (surfel_count, surfel_sphere_radius) = match section_payload(
        bytes,
        qualia_core_db::container_10d::SectionType::HierarchicalSurfels,
    ) {
        Some(payload) => {
            match qualia_core_db::container_10d::surfel_section::decode_surfel_section(payload) {
                Ok((h, _)) => (h.surfel_count as usize, h.sphere_radius),
                Err(_) => (0, 0.0),
            }
        }
        None => (0, 0.0),
    };
    // Upstream `decode_impostor_section` panics on any input: the declared
    // IMPOSTOR_MINI_HEADER_SIZE (64) disagrees with the actual 44-byte
    // `ImpostorAtlasMiniHeader`. Locate the section through the section
    // table and read the wire fields at fixed offsets instead.
    let (impostor_dim, impostor_res) = match impostor_wire_view(bytes) {
        Some((grid, res)) => (grid, res),
        None => (0, 0),
    };
    #[derive(serde::Serialize)]
    struct Verified {
        vertices: usize,
        triangles: usize,
        surfel_count: usize,
        surfel_sphere_radius: f32,
        impostor_grid_dimension: usize,
        impostor_view_resolution: usize,
        materials: usize,
        material_ranges: usize,
        normal_vertices: usize,
        uv_vertices: usize,
        rig_joints: usize,
        animation_clips: usize,
        digest: u32,
        min: [f32; 3],
        max: [f32; 3],
    }
    serde_wasm_bindgen::to_value(&Verified {
        vertices: mesh.vertex_count(),
        triangles: mesh.triangle_count(),
        surfel_count,
        surfel_sphere_radius,
        impostor_grid_dimension: impostor_dim,
        impostor_view_resolution: impostor_res,
        materials,
        material_ranges,
        normal_vertices,
        uv_vertices,
        rig_joints,
        animation_clips,
        digest: compiled_digest(bytes),
        min: mesh.min,
        max: mesh.max,
    })
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Slice one section's payload out of a `.10d` container by type.
fn section_payload<'a>(
    bytes: &'a [u8],
    section_type: qualia_core_db::container_10d::SectionType,
) -> Option<&'a [u8]> {
    let header = qualia_core_db::container_10d::Container10dHeader::parse(bytes).ok()?;
    let sections = qualia_core_db::container_10d::parse_section_table(bytes, &header).ok()?;
    let section = sections.iter().find(|s| s.typ() == Some(section_type))?;
    let start = section.byte_offset as usize;
    let end = start.checked_add(section.byte_length as usize)?;
    bytes.get(start..end)
}

/// Read `(grid_dimension, view_resolution)` from an in-container IMPO
/// section's 44-byte wire header without touching the broken upstream
/// codec. Returns `None` when the container carries no impostor section.
fn impostor_wire_view(bytes: &[u8]) -> Option<(usize, usize)> {
    let payload = section_payload(
        bytes,
        qualia_core_db::container_10d::SectionType::ImpostorAtlas,
    )?;
    if payload.len() < 44
        || payload[..4] != *b"IMPO"
        || u16::from_le_bytes([payload[4], payload[5]]) != 1
    {
        return None;
    }
    let grid = u16::from_le_bytes([payload[8], payload[9]]) as usize;
    let res = u16::from_le_bytes([payload[10], payload[11]]) as usize;
    if grid == 0 || res == 0 {
        return None;
    }
    Some((grid, res))
}

/// Comprehensive ontological entity verification for `.10d` living containers.
#[wasm_bindgen]
pub fn entity_verify_10d(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let report = qualia_core_db::bundle::entity_verifier::verify_entity_10d(bytes);
    serde_wasm_bindgen::to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Comprehensive ontological entity verification for `.hmc` bundles.
#[wasm_bindgen]
pub fn entity_verify_bundle(bytes: &[u8]) -> Result<JsValue, JsValue> {
    let report = qualia_core_db::bundle::entity_verifier::verify_entity_bundle(bytes);
    serde_wasm_bindgen::to_value(&report).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Build the town from QualiaDB computational geometry, seal each coloured
/// mesh as `.10d`, and align semantic pick nodes with the portal projection.
/// JavaScript supplies only game state and displays the result.
#[wasm_bindgen]
pub fn scene_build(
    upgrades: u32,
    parts: u32,
    online: bool,
    approved: bool,
    water_online: bool,
    garden_active: bool,
    signal_online: bool,
    bridge_open: bool,
    bridge_braced: bool,
    high_tide: bool,
    pump_online: bool,
    orchard_active: bool,
    orchard_harvested: bool,
    vibe_scene: &str,
) -> Result<JsValue, JsValue> {
    use crate::asset_compile::compile_asset;
    use qualia_core_db::render::scene_primitives::portal_point;
    let assets = js_sys::Array::new();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    let mut recipes = asset_catalog::kestrel_flats(
        upgrades,
        parts,
        online,
        approved,
        water_online,
        garden_active,
        signal_online,
    );
    recipes.extend(asset_catalog::saltwind_reach(
        bridge_open,
        bridge_braced,
        high_tide,
        pump_online,
        orchard_active,
        orchard_harvested,
    ));
    recipes.extend(asset_catalog::northern_highlands());
    recipes.extend(vehicles::rig_assets(VEHICLE_RIG.with(Cell::get)));
    let fauna_selection = FAUNA_SELECTION.with(|active| active.borrow().clone());
    recipes.extend(fauna::road_hazards(&fauna_selection));
    let flora_selection = FLORA_SELECTION.with(|active| active.borrow().clone());
    recipes.extend(flora::profile_assets(&flora_selection));
    if !vibe_scene.trim().is_empty() {
        recipes.push(vibe_scene_recipe(vibe_scene).map_err(|e| JsValue::from_str(&e))?);
    }
    let art_style = ART_STYLE.with(|active| active.get());
    for recipe in &mut recipes {
        recipe.color = style_color(&recipe.id, recipe.color, art_style);
    }
    for recipe in recipes {
        let (mesh, obj) = compile_asset(&recipe)?;
        for axis in 0..3 {
            min[axis] = min[axis].min(mesh.min[axis]);
            max[axis] = max[axis].max(mesh.max[axis]);
        }
        assets.push(&obj);
    }
    let places = [
        [-3.0, 0.6, 2.6],
        [4.0, 0.6, 3.2],
        [-2.2, 0.8, -3.4],
        [3.2, 1.0, -2.2],
        [0.0, 1.9, -0.6],
        [-5.0, 0.7, -0.5],
        [-3.9, 0.2, -3.8],
        [8.0, 0.4, 0.5],
        [19.0, 1.2, -2.0],
        [17.6, 0.8, 4.0],
        [5.75, 2.1, 5.3],
    ];
    let mut nodes = Vec::with_capacity(places.len() * 10);
    let site_positions = js_sys::Array::new();
    let pick_targets = js_sys::Array::new();
    for (place_index, point) in places.into_iter().enumerate() {
        let p = portal_point(min, max, point).map(|v| v * (1.7 / 1.6));
        nodes.extend_from_slice(&[0.0, 0.0, 4.0, p[0], p[1], p[2], 0.0, 1.0, 0.7, 0.5]);
        let site = js_sys::Object::new();
        for (key, value) in ["x", "y", "z"].iter().zip(p) {
            js_sys::Reflect::set(&site, &(*key).into(), &JsValue::from_f64(value as f64))?;
        }
        site_positions.push(&site);
        let target = js_sys::Object::new();
        js_sys::Reflect::set(
            &target,
            &"placeIndex".into(),
            &JsValue::from_f64(place_index as f64),
        )?;
        js_sys::Reflect::set(
            &target,
            &"label".into(),
            &JsValue::from_str("Community site"),
        )?;
        pick_targets.push(&target);
    }
    // Mesh-adjacent semantic nodes turn visible props into direct entry points
    // for the corresponding place/activity instead of relying on the tiny HUD.
    let hotspots: [(&str, [f32; 3], usize, &str, &str); 10] = [
        (
            "touring-vehicle",
            [2.35, 0.85, 4.65],
            0,
            "Touring Rig · Click to Drive",
            "vehicle-drive",
        ),
        (
            "workshop-repair-bench",
            [3.24, 0.92, -0.91],
            3,
            "Workshop repair bench",
            "workshop-repair",
        ),
        (
            "hydroponic-grow-bed",
            [-3.0, 0.48, -3.30],
            6,
            "Hydroponic growing table",
            "garden-produce",
        ),
        (
            "touring-camp-rig",
            [-3.0, 0.95, 2.70],
            0,
            "Touring camp and rig",
            "",
        ),
        (
            "orchard-harvest-crates",
            [18.3, 0.46, 4.45],
            9,
            "Orchard harvest",
            "fruit-harvest",
        ),
        (
            "orchard-preparation-table",
            [16.7, 0.78, 3.55],
            9,
            "Produce preparation table",
            "shared-meal",
        ),
        (
            "cistern-water-tank",
            [-5.0, 0.7, -0.5],
            5,
            "Community water cistern",
            "water-inspect",
        ),
        (
            "solar-power-array",
            [-2.2, 0.8, -3.4],
            2,
            "Solar photovoltaic array",
            "solar-inspect",
        ),
        (
            "wind-pump-station",
            [19.0, 1.2, -2.0],
            8,
            "Saltwind bore wind pump",
            "pump-inspect",
        ),
        (
            "signal-mast-repeater",
            [5.75, 2.1, 5.3],
            10,
            "Highlands signal mast",
            "mast-inspect",
        ),
    ];
    for (id, point, place_index, label, activity_id) in hotspots {
        let p = portal_point(min, max, point).map(|v| v * (1.7 / 1.6));
        nodes.extend_from_slice(&[0.0, 0.0, 1.2, p[0], p[1], p[2], 0.0, 1.0, 0.7, 0.5]);
        let target = js_sys::Object::new();
        js_sys::Reflect::set(&target, &"id".into(), &JsValue::from_str(id))?;
        js_sys::Reflect::set(
            &target,
            &"placeIndex".into(),
            &JsValue::from_f64(place_index as f64),
        )?;
        js_sys::Reflect::set(&target, &"label".into(), &JsValue::from_str(label))?;
        js_sys::Reflect::set(
            &target,
            &"activityId".into(),
            &JsValue::from_str(activity_id),
        )?;
        pick_targets.push(&target);
    }
    let tensor = tensor_buffer_build(&nodes)?;
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"assets".into(), &assets)?;
    let textures = crate::material_library::texture_receipts()?;
    js_sys::Reflect::set(&result, &"textures".into(), &JsValue::from(textures))?;
    js_sys::Reflect::set(
        &result,
        &"tensor".into(),
        &js_sys::Uint8Array::from(tensor.as_slice()),
    )?;
    let tiles = js_sys::Array::new();
    for (id, center) in [
        ("kestrel", [0.0, 0.0, 0.0]),
        ("saltwind", [16.0, 0.0, 0.0]),
        ("highlands", [8.0, 0.0, -14.0]),
    ] {
        let p = portal_point(min, max, center).map(|v| v * (1.7 / 1.6));
        let tile = js_sys::Object::new();
        js_sys::Reflect::set(&tile, &"id".into(), &JsValue::from_str(id))?;
        for (key, value) in ["x", "y", "z"].iter().zip(p) {
            js_sys::Reflect::set(&tile, &(*key).into(), &JsValue::from_f64(value as f64))?;
        }
        tiles.push(&tile);
    }
    js_sys::Reflect::set(&result, &"tiles".into(), &tiles)?;
    let center = js_sys::Object::new();
    let center_world = [
        (min[0] + max[0]) * 0.5,
        (min[1] + max[1]) * 0.5,
        (min[2] + max[2]) * 0.5,
    ];
    for (key, value) in ["x", "y", "z"].iter().zip(center_world) {
        js_sys::Reflect::set(&center, &(*key).into(), &JsValue::from_f64(value as f64))?;
    }
    let span = (0..3)
        .map(|axis| max[axis] - min[axis])
        .fold(1e-6_f32, f32::max);
    js_sys::Reflect::set(&result, &"worldCenter".into(), &center)?;
    js_sys::Reflect::set(
        &result,
        &"worldScale".into(),
        &JsValue::from_f64((1.7 / span) as f64),
    )?;
    js_sys::Reflect::set(&result, &"places".into(), &site_positions)?;
    js_sys::Reflect::set(&result, &"pickTargets".into(), &pick_targets)?;
    Ok(result.into())
}

/// Camera presentation sample in source-world metres, from the same authored
/// height function used to generate the Qualia terrain patches.
#[wasm_bindgen]
pub fn terrain_height_world(x: f32, z: f32) -> f32 {
    let dist_kestrel = (x - 0.0).abs().max(z.abs());
    let dist_saltwind = (x - 16.0).abs().max(z.abs());
    let dist_ridge = (x - 8.0).abs().max((z + 14.0).abs());
    if dist_kestrel <= 6.5 {
        terrain::settlement_height(0.0, x, z)
    } else if dist_saltwind <= 6.5 {
        terrain::settlement_height(16.0, x, z)
    } else if dist_ridge <= 8.5 && z <= -7.0 {
        terrain::height(x, z)
    } else {
        terrain::continuous_landscape_height(x, z)
    }
}

// --- Portal: WebGPU scene ingest, frame, semantic pick ---

/// Qualia's generic in-viewport HUD. Game rules remain in `GameSession`.
#[wasm_bindgen]
pub struct GameHud {
    inner: qualia_core_db::render::hud::QualiaHud,
}

#[wasm_bindgen]
impl GameHud {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<GameHud, JsValue> {
        Ok(Self {
            inner: qualia_core_db::render::hud::QualiaHud::new(canvas)?,
        })
    }

    pub fn set_document_json(&mut self, json: &str) -> Result<(), JsValue> {
        self.inner.set_document_json(json)
    }

    pub fn set_camera_target(&mut self, yaw: f32, pitch: f32, zoom: f32, x: f32, y: f32, z: f32) {
        self.inner.set_camera_target(yaw, pitch, zoom, x, y, z);
    }

    pub fn hit_test(&mut self, x: f64, y: f64) -> String {
        self.inner.hit_test(x, y)
    }
    pub fn focus_next(&mut self, reverse: bool) -> String {
        self.inner.focus_next(reverse)
    }
    pub fn focused_action(&self) -> String {
        self.inner.focused_action()
    }
}

/// Thin shell over `QualiaPortal` — the QualiaDB browser render surface.
/// A canvas is supplied by the page; all ingest/pick goes through the engine.
///
/// Call order matters: `init_webgpu(canvas)` BEFORE `GamePortal::new` — the
/// canvas must still be context-free so the WebGPU surface can bind.
#[wasm_bindgen]
pub struct GamePortal {
    inner: qualia_core_db::QualiaPortal,
}

fn is_water_surface_role(id: &str, render_role: &str) -> bool {
    render_role == "water" || matches!(id, "rc:asset/channel" | "rc:asset/highland-creek")
}

fn append_water_mesh(
    mesh: &qualia_core_db::render::assets::Mesh,
    positions: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
) -> Result<(), JsValue> {
    let base = u32::try_from(positions.len())
        .map_err(|_| JsValue::from_str("water vertex count exceeds the scene index range"))?;
    for triangle in &mesh.triangles {
        for &index in triangle {
            indices.push(
                base.checked_add(index).ok_or_else(|| {
                    JsValue::from_str("water index overflows the scene index range")
                })?,
            );
        }
    }
    positions.extend_from_slice(&mesh.positions);
    Ok(())
}

fn read_10d_bounds(bytes: &[u8]) -> Option<([f32; 3], [f32; 3])> {
    use qualia_core_db::container_10d::header::Container10dHeader;
    use qualia_core_db::container_10d::section::{parse_section_table, SectionType};
    let header = Container10dHeader::parse(bytes).ok()?;
    let descs = parse_section_table(bytes, &header).ok()?;
    for d in descs {
        if d.typ() == Some(SectionType::QuantizedMesh) {
            let start = d.byte_offset as usize;
            let payload = bytes.get(start..start + 40)?;
            let min = [
                f32::from_le_bytes(payload[12..16].try_into().ok()?),
                f32::from_le_bytes(payload[16..20].try_into().ok()?),
                f32::from_le_bytes(payload[20..24].try_into().ok()?),
            ];
            let max = [
                f32::from_le_bytes(payload[24..28].try_into().ok()?),
                f32::from_le_bytes(payload[28..32].try_into().ok()?),
                f32::from_le_bytes(payload[32..36].try_into().ok()?),
            ];
            return Some((min, max));
        }
    }
    None
}

fn load_scene_assets(
    inner: &mut qualia_core_db::QualiaPortal,
    assets: &js_sys::Array,
    mut water_positions: Vec<[f32; 3]>,
    water_indices: Vec<u32>,
    water_surface_count: usize,
    water_colour: [f32; 4],
) -> Result<JsValue, JsValue> {
    // The browser camera uses the portal's shared orbit frame. Normalise the
    // complete scene together; retaining metre coordinates puts the camera
    // inside the terrain and also suppresses the portal's CPU fallback.
    inner.set_preserve_authored_frame(false);
    // Water is uploaded separately and does not pass through the body's orbit
    // normalisation. Use the same opaque bounds and 1.7-span transform here.
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    for asset in assets.iter() {
        let bytes = js_get(&asset, "bytes")
            .ok_or_else(|| JsValue::from_str("scene asset missing bytes"))?;
        let bytes = js_sys::Uint8Array::new(&bytes).to_vec();
        if let Some((b_min, b_max)) = read_10d_bounds(&bytes) {
            for axis in 0..3 {
                min[axis] = min[axis].min(b_min[axis]);
                max[axis] = max[axis].max(b_max[axis]);
            }
        } else {
            let mesh = qualia_core_db::render::compile_10d::decode_10d_mesh(&bytes)
                .map_err(|e| JsValue::from_str(&format!("scene bounds: {e}")))?;
            for position in &mesh.positions {
                for axis in 0..3 {
                    min[axis] = min[axis].min(position[axis]);
                    max[axis] = max[axis].max(position[axis]);
                }
            }
        }
    }
    let span = (0..3).map(|i| max[i] - min[i]).fold(1e-6_f32, f32::max);
    for position in &mut water_positions {
        for axis in 0..3 {
            position[axis] = (position[axis] - (min[axis] + max[axis]) * 0.5) * 1.7 / span;
        }
    }
    let receipt = inner.load_assets_colored(assets)?;
    let flat_positions: Vec<f32> = water_positions.iter().flat_map(|p| *p).collect();
    inner.set_water_geometry(&flat_positions, &water_indices)?;
    if water_surface_count > 0 {
        inner.set_water_surface(
            water_colour[0],
            water_colour[1],
            water_colour[2],
            water_colour[3].clamp(0.0, 1.0),
            0.025,
            1.0,
            1,
        );
    }
    let organs = js_get(&receipt, "assets_loaded")
        .or_else(|| js_get(&receipt, "organs_loaded"))
        .and_then(|value| value.as_f64())
        .unwrap_or(0.0);
    let material_pipeline = js_get(&receipt, "material_pipeline")
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let materials_loaded = js_get(&receipt, "materials_loaded")
        .and_then(|value| value.as_f64())
        .unwrap_or(0.0);
    let material_ranges = js_get(&receipt, "material_ranges")
        .and_then(|value| value.as_f64())
        .unwrap_or(0.0);
    let object = js_sys::Object::new();
    for (key, value) in [
        (
            "assets_loaded",
            JsValue::from_f64(organs + water_surface_count as f64),
        ),
        (
            "organs_loaded",
            JsValue::from_f64(organs + water_surface_count as f64),
        ),
        ("opaque_meshes_loaded", JsValue::from_f64(organs)),
        (
            "water_surfaces",
            JsValue::from_f64(water_surface_count as f64),
        ),
        (
            "water_triangles",
            JsValue::from_f64((water_indices.len() / 3) as f64),
        ),
    ] {
        js_sys::Reflect::set(&object, &key.into(), &value)?;
    }
    js_sys::Reflect::set(
        &object,
        &"material_pipeline".into(),
        &JsValue::from_bool(material_pipeline),
    )?;
    js_sys::Reflect::set(
        &object,
        &"materials_loaded".into(),
        &JsValue::from_f64(materials_loaded),
    )?;
    js_sys::Reflect::set(
        &object,
        &"material_ranges".into(),
        &JsValue::from_f64(material_ranges),
    )?;
    Ok(object.into())
}

fn split_scene_assets(
    source: &js_sys::Array,
) -> Result<(js_sys::Array, Vec<[f32; 3]>, Vec<u32>, usize, [f32; 4]), JsValue> {
    let mut opaque = js_sys::Array::new();
    let mut water_positions = Vec::new();
    let mut water_indices = Vec::new();
    let mut water_surfaces = 0usize;
    let mut water_colour_sum = [0.0; 4];
    for value in source.iter() {
        let id = js_sys::Reflect::get(&value, &"id".into())?
            .as_string()
            .unwrap_or_default();
        let render_role = js_sys::Reflect::get(&value, &"renderRole".into())?
            .as_string()
            .unwrap_or_default();
        if is_water_surface_role(&id, &render_role) {
            let source_bytes = js_sys::Reflect::get(&value, &"bytes".into())?;
            let bytes = js_sys::Uint8Array::new(&source_bytes).to_vec();
            let mesh = qualia_core_db::render::compile_10d::decode_10d_mesh(&bytes)
                .map_err(|error| JsValue::from_str(&format!("water .10d decode: {error}")))?;
            append_water_mesh(&mesh, &mut water_positions, &mut water_indices)?;
            for (axis, key) in ["r", "g", "b", "a"].iter().enumerate() {
                water_colour_sum[axis] += js_sys::Reflect::get(&value, &(*key).into())?
                    .as_f64()
                    .unwrap_or(1.0) as f32;
            }
            water_surfaces += 1;
        } else {
            opaque.push(&value);
        }
    }
    let water_colour = if water_surfaces == 0 {
        [0.16, 0.38, 0.42, 0.78]
    } else {
        water_colour_sum.map(|channel| channel / water_surfaces as f32)
    };
    Ok((
        opaque,
        water_positions,
        water_indices,
        water_surfaces,
        water_colour,
    ))
}

/// Arm the WebGPU path. Await once before constructing the portal; on
/// `false`/throw the portal keeps its CPU/canvas fallback.
#[wasm_bindgen]
pub async fn init_webgpu(canvas: HtmlCanvasElement) -> Result<bool, JsValue> {
    qualia_core_db::render::portal::portal_init_webgpu(canvas).await
}

#[wasm_bindgen]
impl GamePortal {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<GamePortal, JsValue> {
        let mut inner = qualia_core_db::QualiaPortal::new(canvas)?;
        inner.set_preserve_authored_frame(false);
        Ok(GamePortal { inner })
    }

    /// Load a sealed `.10d` container into the scene (CRC + provenance
    /// verified inside the engine; fail-closed on malformed input).
    pub fn load_10d(&mut self, bytes: &[u8]) -> Result<JsValue, JsValue> {
        self.inner.load_10d(bytes)
    }

    /// Load a multi-asset scene: `Array<{bytes: .10d, r,g,b,a}>` where each
    /// mesh keeps its authored position in the shared coordinate space — one
    /// authored scale across all objects. This is how a town of
    /// distinct props composes into one viewport mesh. The engine binding
    /// still carries its legacy anatomy-era name (`load_body_organs_colored`);
    /// the upstream rename is tracked as QG-20 in
    /// `docs/planning/19-qualiadb-upstream-gate-work-orders.md`.
    pub fn load_scene(&mut self, assets: &js_sys::Array) -> Result<JsValue, JsValue> {
        let (opaque, water_positions, water_indices, water_surfaces, water_colour) =
            split_scene_assets(assets)?;
        load_scene_assets(
            &mut self.inner,
            &opaque,
            water_positions,
            water_indices,
            water_surfaces,
            water_colour,
        )
    }

    /// Hydrate the dependencies before creating material bind groups.
    pub fn load_scene_textured(
        &mut self,
        assets: &js_sys::Array,
        textures: &js_sys::Array,
    ) -> Result<JsValue, JsValue> {
        let loaded = crate::material_library::hydrate(&mut self.inner, textures)?;
        let receipt = self.load_scene(assets)?;
        js_sys::Reflect::set(
            &receipt,
            &"texture_bindings_requested".into(),
            &JsValue::from_f64(textures.length() as f64),
        )?;
        js_sys::Reflect::set(
            &receipt,
            &"texture_bindings_resident".into(),
            &JsValue::from_f64(loaded as f64),
        )?;
        Ok(receipt)
    }

    /// Load one game-authored scene snapshot from its verified HMC pack. The
    /// manifest selects a bounded scene, while Qualia's bundle reader verifies
    /// every referenced entry before the portal sees it. If this optional path
    /// is unavailable in the browser, callers can keep using `load_scene`.
    pub fn load_hmc_scene(
        &mut self,
        hmc_bytes: &[u8],
        manifest_json: &str,
        scene_key: &str,
    ) -> Result<JsValue, JsValue> {
        fn default_asset_rgba() -> [f32; 4] {
            [1.0; 4]
        }
        #[derive(serde::Deserialize)]
        struct Manifest {
            scenes: Vec<Scene>,
            #[serde(default)]
            assets: Vec<SceneAsset>,
            #[serde(default)]
            textures: Vec<TextureAsset>,
        }
        #[derive(serde::Deserialize)]
        struct TextureAsset {
            file: String,
            mime: String,
            role: u8,
        }
        #[derive(serde::Deserialize)]
        struct Scene {
            name: String,
            #[serde(default)]
            assets: Vec<SceneAsset>,
            #[serde(rename = "assetIndices", default)]
            asset_indices: Vec<usize>,
        }
        #[derive(Clone, serde::Deserialize)]
        struct SceneAsset {
            id: String,
            file: String,
            #[serde(default = "default_asset_rgba")]
            rgba: [f32; 4],
            #[serde(rename = "renderRole", default)]
            render_role: String,
        }

        if manifest_json.len() > 16_777_216 {
            return Err(JsValue::from_str("HMC scene manifest exceeds bound"));
        }
        let manifest: Manifest = serde_json::from_str(manifest_json)
            .map_err(|e| JsValue::from_str(&format!("HMC scene manifest: {e}")))?;
        let scene = manifest
            .scenes
            .into_iter()
            .find(|scene| scene.name == scene_key)
            .ok_or_else(|| JsValue::from_str("HMC scene key not found"))?;
        if manifest.assets.len() > 65_536 {
            return Err(JsValue::from_str("HMC manifest asset table exceeds bound"));
        }
        let scene_assets =
            if scene.assets.is_empty() {
                if scene.asset_indices.len() > 4096 {
                    return Err(JsValue::from_str("HMC scene asset count is outside bounds"));
                }
                scene
                    .asset_indices
                    .into_iter()
                    .map(|index| {
                        manifest.assets.get(index).cloned().ok_or_else(|| {
                            JsValue::from_str("HMC scene asset index is outside table")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?
            } else {
                scene.assets
            };
        if scene_assets.is_empty() || scene_assets.len() > 4096 {
            return Err(JsValue::from_str("HMC scene asset count is outside bounds"));
        }

        use qualia_core_db::bundle::BundleReader;
        let reader = BundleReader::parse(hmc_bytes)
            .map_err(|e| JsValue::from_str(&format!("HMC scene bundle: {e}")))?;
        if manifest.textures.len() > 512 {
            return Err(JsValue::from_str("HMC texture count exceeds bound"));
        }
        let mut resident_textures = 0u32;
        for texture in &manifest.textures {
            let entry = reader
                .entry(&texture.file)
                .ok_or_else(|| JsValue::from_str("HMC texture missing"))?;
            let digest: [u8; 32] = entry
                .sha256
                .as_slice()
                .try_into()
                .map_err(|_| JsValue::from_str("HMC texture digest invalid"))?;
            let resource = qualia_core_db::render::asset_package::resolve_hmc_texture_resource(
                &reader, &digest,
            )
            .map_err(|e| JsValue::from_str(&format!("HMC texture: {e:?}")))?;
            if resource.mime_type != texture.mime
                || texture.file != qualia_core_db::render::asset_package::texture_hmc_key(&digest)
            {
                return Err(JsValue::from_str("HMC texture metadata mismatch"));
            }
            if self.inner.load_texture_encoded(
                &digest,
                &texture.mime,
                resource.bytes,
                texture.role,
                0.5,
            )? {
                resident_textures += 1;
            }
        }
        let assets = js_sys::Array::new();
        let mut water_positions = Vec::new();
        let mut water_indices = Vec::new();
        let mut water_surfaces = 0usize;
        let mut water_colour_sum = [0.0; 4];
        for asset in scene_assets {
            if !asset.file.starts_with("10d/")
                || !asset.file.ends_with(".10d")
                || asset.file.contains("..")
                || !asset.rgba.iter().all(|v| v.is_finite())
            {
                return Err(JsValue::from_str("invalid HMC scene asset reference"));
            }
            let entry = reader
                .entries()
                .iter()
                .find(|entry| entry.key == asset.file)
                .ok_or_else(|| JsValue::from_str("HMC scene asset entry not found"))?;
            if entry.kind != "10d" || !reader.verify_entry(&entry.key) {
                return Err(JsValue::from_str("HMC scene asset verification failed"));
            }
            let bytes = reader
                .get(&entry.key)
                .ok_or_else(|| JsValue::from_str("HMC scene asset bytes not found"))?;
            if is_water_surface_role(&asset.id, &asset.render_role) {
                let mesh = qualia_core_db::render::compile_10d::decode_10d_mesh(bytes)
                    .map_err(|error| JsValue::from_str(&format!("water .10d decode: {error}")))?;
                append_water_mesh(&mesh, &mut water_positions, &mut water_indices)?;
                for (axis, value) in asset.rgba.iter().enumerate() {
                    water_colour_sum[axis] += *value;
                }
                water_surfaces += 1;
                continue;
            }
            let object = js_sys::Object::new();
            js_sys::Reflect::set(&object, &"id".into(), &JsValue::from_str(&asset.id))?;
            js_sys::Reflect::set(&object, &"bytes".into(), &js_sys::Uint8Array::from(bytes))?;
            for (key, value) in ["r", "g", "b", "a"].iter().zip(asset.rgba) {
                js_sys::Reflect::set(&object, &(*key).into(), &JsValue::from_f64(value as f64))?;
            }
            assets.push(&object);
        }
        let water_colour = if water_surfaces == 0 {
            [0.16, 0.38, 0.42, 0.78]
        } else {
            water_colour_sum.map(|channel| channel / water_surfaces as f32)
        };
        let receipt = load_scene_assets(
            &mut self.inner,
            &assets,
            water_positions,
            water_indices,
            water_surfaces,
            water_colour,
        )?;
        js_sys::Reflect::set(
            &receipt,
            &"texture_bindings_requested".into(),
            &JsValue::from_f64(manifest.textures.len() as f64),
        )?;
        js_sys::Reflect::set(
            &receipt,
            &"texture_bindings_resident".into(),
            &JsValue::from_f64(resident_textures as f64),
        )?;
        Ok(receipt)
    }

    /// Render tier: 0 = CPU fallback, 1 = WebGL2 fallback, 2 = WebGPU path.
    pub fn tier(&self) -> u8 {
        self.inner.tier()
    }

    pub fn set_exposure_compensation(&mut self, ev: f32) -> bool {
        self.inner.set_exposure_compensation(ev)
    }

    pub fn set_auto_temporal_enabled(&mut self, enabled: bool) {
        self.inner.set_auto_temporal_enabled(enabled);
    }

    pub fn set_shadows_enabled(&mut self, enabled: bool) {
        self.inner.set_shadows_enabled(enabled);
    }

    pub fn set_screen_space_ao_enabled(&mut self, enabled: bool) {
        self.inner.set_screen_space_ao_enabled(enabled);
    }

    pub fn set_screen_space_ao(&mut self, radius: f32, strength: f32, bias: f32) {
        self.inner.set_screen_space_ao(radius, strength, bias);
    }

    pub fn set_screen_space_ao_sample_count(&mut self, samples: u32) {
        self.inner.set_screen_space_ao_sample_count(samples);
    }

    pub fn set_environment_probe_data(
        &mut self,
        packed_probes: &[f32],
        fallback_radiance: &[f32],
    ) -> Result<bool, JsValue> {
        self.inner
            .set_environment_probe_data(packed_probes, fallback_radiance)
    }

    /// Report active renderer resources and actual temporal execution receipts.
    pub fn renderer_producer_report(&self) -> Result<JsValue, JsValue> {
        let receipt = self.inner.scene_render_receipt()?;
        let renderer = js_get(&receipt, "renderer")
            .and_then(|value| value.as_string())
            .unwrap_or_else(|| "none".to_string());
        let mesh_admitted = js_get(&receipt, "uploaded")
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
            && js_get(&receipt, "vertex_count")
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0)
                > 0.0
            && js_get(&receipt, "index_count")
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0)
                > 0.0;
        let frame_presented = js_get(&receipt, "frames_presented")
            .and_then(|value| value.as_f64())
            .unwrap_or(0.0)
            > 0.0;
        let gpu_policy_admitted = renderer == "webgpu" || self.inner.tier() >= 2;
        let gpu_execution_observed = renderer == "webgpu" && frame_presented;

        let temporal = self.inner.temporal_graphics_receipt();
        let producers = js_get(&temporal, "producer_views")
            .unwrap_or_else(|| JsValue::from(js_sys::Object::new()));
        let temporal_producer_ready = js_get(&temporal, "producer_ready")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let temporal_execution_observed = js_get(&temporal, "execution_observed")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let fallback = if temporal_execution_observed {
            "temporal-resolve"
        } else if renderer == "webgpu" {
            "ordinary-tick"
        } else {
            renderer.as_str()
        };
        let mut report = js_sys::Object::new();
        set(&mut report, "mesh_receipt", receipt);
        set(&mut report, "backend", JsValue::from_str(&renderer));
        set(
            &mut report,
            "portal_tier",
            JsValue::from_f64(self.inner.tier() as f64),
        );
        set(
            &mut report,
            "renderer_admitted",
            JsValue::from_bool(mesh_admitted),
        );
        set(
            &mut report,
            "frame_presented",
            JsValue::from_bool(frame_presented),
        );
        set(
            &mut report,
            "gpu_policy_admitted",
            JsValue::from_bool(gpu_policy_admitted),
        );
        set(
            &mut report,
            "gpu_execution_observed",
            JsValue::from_bool(gpu_execution_observed),
        );
        set(&mut report, "producer_views", producers);
        set(
            &mut report,
            "temporal_producer_ready",
            JsValue::from_bool(temporal_producer_ready),
        );
        set(
            &mut report,
            "temporal_execution_observed",
            JsValue::from_bool(temporal_execution_observed),
        );
        set(
            &mut report,
            "temporal_execution_count",
            js_get(&temporal, "execution_count").unwrap_or(JsValue::from_f64(0.0)),
        );
        set(
            &mut report,
            "screen_space_ao_available",
            JsValue::from_bool(self.inner.screen_space_ao_available()),
        );
        set(
            &mut report,
            "shadows_available",
            JsValue::from_bool(self.inner.shadows_available()),
        );
        set(&mut report, "fallback", JsValue::from_str(fallback));
        Ok(report.into())
    }

    /// Return a bounded temporal schedule with current renderer-owned producer evidence.
    pub fn temporal_schedule_contract(
        &mut self,
        quality_tier: u8,
        scene_changed: bool,
        camera_changed: bool,
    ) -> Result<JsValue, JsValue> {
        let renderer = self.renderer_producer_report()?;
        let gpu_policy_admitted = js_get(&renderer, "gpu_policy_admitted")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let temporal_receipt = self.inner.temporal_graphics_receipt();
        let producer_ready = js_get(&temporal_receipt, "producer_ready")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let temporal_enabled = js_get(&temporal_receipt, "enabled")
            .and_then(|value| value.as_bool())
            .unwrap_or(false);
        let policy_admitted =
            quality_tier >= 2 && gpu_policy_admitted && producer_ready && temporal_enabled;
        let reset_history = scene_changed || camera_changed;
        if reset_history {
            self.inner.invalidate_temporal_history();
        }
        let reads_history = policy_admitted && !reset_history;
        let publish_history = policy_admitted;
        let final_output = policy_admitted;
        let schedule_valid =
            policy_admitted && publish_history && final_output && !(reset_history && reads_history);

        let mut schedule = js_sys::Object::new();
        set(
            &mut schedule,
            "enabled",
            JsValue::from_bool(policy_admitted),
        );
        set(
            &mut schedule,
            "reset_history",
            JsValue::from_bool(reset_history),
        );
        set(
            &mut schedule,
            "reads_history",
            JsValue::from_bool(reads_history),
        );
        set(
            &mut schedule,
            "publish_history",
            JsValue::from_bool(publish_history),
        );
        set(
            &mut schedule,
            "final_output",
            JsValue::from_bool(final_output),
        );

        let mut contract = js_sys::Object::new();
        set(&mut contract, "version", JsValue::from_f64(1.0));
        set(
            &mut contract,
            "policy_admitted",
            JsValue::from_bool(policy_admitted),
        );
        set(
            &mut contract,
            "schedule_valid",
            JsValue::from_bool(schedule_valid),
        );
        set(&mut contract, "schedule", schedule.into());
        set(
            &mut contract,
            "producer_views",
            js_get(&temporal_receipt, "producer_views")
                .unwrap_or_else(|| JsValue::from(js_sys::Object::new())),
        );
        set(
            &mut contract,
            "producer_ready",
            JsValue::from_bool(producer_ready),
        );
        set(
            &mut contract,
            "execution_admitted",
            JsValue::from_bool(policy_admitted),
        );
        set(
            &mut contract,
            "executed",
            js_get(&temporal_receipt, "execution_observed").unwrap_or(JsValue::FALSE),
        );
        set(
            &mut contract,
            "fallback",
            JsValue::from_str(if policy_admitted {
                "temporal-resolve"
            } else {
                "ordinary-tick"
            }),
        );
        Ok(contract.into())
    }

    /// Set the deterministic story time slice used by the renderer's
    /// standpoint. This does not create temporal history or producer views.
    pub fn set_temporal_slice(&mut self, t_slice: f32, t_window: f32) {
        self.inner.set_temporal_slice(t_slice, t_window);
    }

    /// Render one frame; `dt_ms` is elapsed milliseconds since the last tick.
    pub fn tick(&mut self, canvas: &HtmlCanvasElement, dt_ms: f32) -> Result<(), JsValue> {
        self.inner.tick(canvas.clone(), dt_ms)
    }

    /// Upload a tensor buffer (see `tensor_buffer_build`) as the pickable
    /// semantic node set. Also drives `last_tensor`, which the CPU pick
    /// fallback reads when WebGPU is unavailable.
    pub fn upload_tensor(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        self.inner.upload_tensor_buffer(bytes)?;
        // Retain pickable semantic nodes without the bright debug particle
        // field. The frame guard in the page keeps both presentation paths
        // off even if the engine re-uploads the tensor on GPU re-init.
        self.inner.set_ambient_enabled(false);
        self.inner.set_tensor_projection_enabled(false);
        Ok(())
    }

    /// Toggle tensor point rendering independently from semantic picking.
    pub fn set_tensor_projection_enabled(&mut self, enabled: bool) {
        self.inner.set_tensor_projection_enabled(enabled);
    }

    /// Toggle the ambient particle field (tensor-node particle cloud).
    /// Off in this game; exposed so the frame guard can assert it stays off.
    pub fn set_ambient_enabled(&mut self, enabled: bool) {
        self.inner.set_ambient_enabled(enabled);
    }

    /// Queue a pick at canvas pixel (x, y). CPU fallback resolves
    /// synchronously; the GPU path resolves on a subsequent `tick`.
    pub fn queue_pick(&mut self, x: f32, y: f32, canvas_w: u32, canvas_h: u32) -> bool {
        self.inner.select_node_at(x, y, canvas_w, canvas_h).is_ok()
    }

    /// Poll the queued pick result: semantic node index or -1 when nothing
    /// is hit / the GPU readback is still in flight.
    pub fn poll_pick(&self) -> i32 {
        self.inner.poll_selected_node()
    }

    pub fn set_camera(&mut self, yaw: f32, pitch: f32, zoom: f32) -> Result<(), JsValue> {
        self.inner.set_camera(yaw, pitch, zoom)
    }

    pub fn set_camera_target(
        &mut self,
        yaw: f32,
        pitch: f32,
        zoom: f32,
        x: f32,
        y: f32,
        z: f32,
    ) -> Result<(), JsValue> {
        self.inner.set_camera_target(yaw, pitch, zoom, x, y, z)
    }

    pub fn set_sky_preset(&mut self, preset: u32) {
        self.inner.set_sky_preset(preset);
        // Storybook strategy art needs legible shadow faces at overview scale.
        let (sun, intensity, ambient) = match preset {
            2 => ([0.85, 0.25, 0.45], 1.50, 0.70),
            3 => ([0.20, 0.40, 0.80], 0.95, 0.48),
            _ => ([0.45, 0.85, 0.35], 1.60, 0.82),
        };
        self.inner
            .set_lighting(sun[0], sun[1], sun[2], intensity, ambient);
    }

    /// Tunable key/fill/ambient light from the QualiaPortal surface, so a
    /// visual style can present its own daylight reading without touching
    /// campaign or rule state.
    pub fn set_lighting(
        &mut self,
        sun_r: f32,
        sun_g: f32,
        sun_b: f32,
        intensity: f32,
        ambient: f32,
    ) {
        self.inner
            .set_lighting(sun_r, sun_g, sun_b, intensity, ambient);
    }

    /// Tune the engine-owned animated water pass without rebuilding the scene.
    pub fn set_water_surface(
        &mut self,
        r: f32,
        g: f32,
        b: f32,
        alpha: f32,
        wave_amplitude: f32,
        wave_frequency: f32,
        quality: u32,
    ) {
        self.inner
            .set_water_surface(r, g, b, alpha, wave_amplitude, wave_frequency, quality);
    }
}

// --- Session: deterministic world document + event log ---
//
// The session holds the world as an N3 document (one triple per line) and an
// ordered event log. It owns no rules: every action's gate is a SHACL
// ShapeSpec list evaluated by the engine, and every effect is the declared
// add/remove triple set from the action catalogue (countable token triples,
// so no arithmetic lives here). Rejected actions spend nothing.

#[derive(serde::Deserialize, Clone)]
struct ActionDef {
    id: String,
    #[serde(default)]
    label: String,
    #[serde(default)]
    gate: serde_json::Value, // ShapeSpec list, passed verbatim to the engine
    #[serde(rename = "removeN", default)]
    remove_n: Vec<RemoveSpec>,
    #[serde(rename = "removeAll", default)]
    remove_all: Vec<RemoveSpec>,
    #[serde(default)]
    add: Vec<String>,
    #[serde(default)]
    explain: String,
    /// Declarative delayed consequences checked after a world-changing action.
    /// Conditions are SHACL shapes and effects are the same token-triple edits
    /// as direct actions, so time-gated outcomes remain replayable.
    #[serde(default, rename = "afterTurn")]
    after_turn: Vec<TransitionDef>,
}

#[derive(serde::Deserialize, Clone)]
struct TransitionDef {
    #[serde(default)]
    gate: serde_json::Value,
    #[serde(rename = "removeN", default)]
    remove_n: Vec<RemoveSpec>,
    #[serde(rename = "removeAll", default)]
    remove_all: Vec<RemoveSpec>,
    #[serde(default)]
    add: Vec<String>,
}

#[derive(serde::Deserialize, Clone)]
struct RemoveSpec {
    s: String,
    p: String,
    #[serde(default)]
    o: Option<String>,
    #[serde(default)]
    n: usize,
}

#[wasm_bindgen]
pub struct GameSession {
    /// World document, one `s p o .` triple per line (token form).
    lines: Vec<String>,
    actions: Vec<ActionDef>,
    /// Accepted action ids in order — the replay tape.
    events: Vec<String>,
    seq: u32,
    drive_verified: [bool; 4],
    epistemic_mode: String,
}

/// Epistemic runtime type alias: `WorldSession` represents the general-purpose
/// semantic state machine for non-fiction field data, digital twin simulations,
/// scientific hypothesis testing, and collaborative civics stewardship.
pub type WorldSession = GameSession;

/// Scenario-scoped alias for narrative, speculative worldbuilding, and training runs.
pub type ScenarioSession = GameSession;

/// A checked VibeScript program that owns the driving simulation rules.
/// The browser submits sampled world facts and control intent, then renders
/// the returned state; it does not implement the vehicle or thermal model.
#[wasm_bindgen]
pub struct GameDriveSimulation {
    program: vibe::Program,
    policy_hash: u64,
}

/// Epistemic alias for physical vehicle and thermal simulation runtimes.
pub type WorldDriveSimulation = GameDriveSimulation;

#[wasm_bindgen]
impl GameDriveSimulation {
    #[wasm_bindgen(constructor)]
    pub fn new(source: &str) -> Result<GameDriveSimulation, JsValue> {
        if source.len() > 131_072 {
            return Err(JsValue::from_str("drive policy exceeds 128 KiB bound"));
        }
        let program = vibe::load_program(source)
            .map_err(|error| JsValue::from_str(&format!("VibeScript drive module: {error}")))?;
        Ok(GameDriveSimulation {
            program,
            policy_hash: qualia_core_db::q_hash(source),
        })
    }

    /// Return the initial driving state and handling profile authored by the
    /// loaded VibeScript module. The browser maps this record to HUD fields;
    /// route, rig and thermal defaults stay with the simulation rules.
    pub fn setup(&self, rig_id: u8) -> JsValue {
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "drive_setup",
            vec![vibe::Value::F64(rig_id as f64)],
            &mut host,
            &mut env,
        ) {
            Ok(value) => vibe_value_to_js(&value),
            Err(error) => err_obj("VibeScript drive setup", &error.to_string()),
        }
    }

    /// Evaluate authored on-foot movement for a Talbingo campsite preview.
    pub fn camp_step(&self, input_json: &str) -> JsValue {
        if input_json.len() > 1024 {
            return err_obj("VibeScript camp input", "record exceeds bound");
        }
        let input: serde_json::Value = match serde_json::from_str(input_json) {
            Ok(value) => value,
            Err(error) => return err_obj("VibeScript camp input", &error.to_string()),
        };
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "camp_step",
            vec![json_to_vibe(&input)],
            &mut host,
            &mut env,
        ) {
            Ok(value) => vibe_value_to_js(&value),
            Err(error) => err_obj("VibeScript camp step", &error.to_string()),
        }
    }

    /// Evaluate exactly one deterministic simulation tick using the checked
    /// VibeScript module loaded at construction. Input and output use records
    /// so the authored policy can grow without expanding the WASM ABI.
    pub fn step(&self, input_json: &str) -> JsValue {
        if input_json.len() > 16_384 {
            return err_obj("VibeScript drive input", "record exceeds the 16 KiB bound");
        }
        let mut input: serde_json::Value = match serde_json::from_str(input_json) {
            Ok(value) => value,
            Err(error) => return err_obj("VibeScript drive input", &error.to_string()),
        };
        // Older callers predate workshop integration. Their unchanged rig is
        // roadworthy with baseline cooling; current callers submit saved state.
        if let Some(record) = input.as_object_mut() {
            record
                .entry("rig_roadworthy")
                .or_insert(serde_json::Value::Bool(true));
            record
                .entry("cooling_efficiency")
                .or_insert(serde_json::json!(1.0));
            record
                .entry("wheel_angle")
                .or_insert(serde_json::json!(0.0));
            let heading = record
                .get("heading")
                .cloned()
                .unwrap_or(serde_json::json!(0.0));
            record.entry("trailer_heading").or_insert(heading);
            record
                .entry("trailer_wheel_angle")
                .or_insert(serde_json::json!(0.0));
        }
        let mut host = qualia_core_db::poet_host::PoetSnapshot::default();
        let mut env = vibe::Env::default();
        match vibe::eval_function(
            &self.program,
            "drive_step",
            vec![json_to_vibe(&input)],
            &mut host,
            &mut env,
        ) {
            Ok(value) => vibe_value_to_js(&value),
            Err(error) => err_obj("VibeScript drive tick", &error.to_string()),
        }
    }
}

pub(crate) fn json_to_vibe(value: &serde_json::Value) -> vibe::Value {
    match value {
        serde_json::Value::Null => vibe::Value::Null,
        serde_json::Value::Bool(value) => vibe::Value::Bool(*value),
        serde_json::Value::Number(value) => vibe::Value::F64(value.as_f64().unwrap_or_default()),
        serde_json::Value::String(value) => vibe::Value::String(value.clone()),
        serde_json::Value::Array(values) => {
            vibe::Value::List(values.iter().map(json_to_vibe).collect())
        }
        serde_json::Value::Object(values) => vibe::Value::Record(
            values
                .iter()
                .map(|(key, value)| (key.clone(), json_to_vibe(value)))
                .collect(),
        ),
    }
}

#[wasm_bindgen]
impl GameSession {
    #[wasm_bindgen(constructor)]
    pub fn new(seed_n3: &str, actions_json: &str) -> Result<GameSession, JsValue> {
        let catalog: serde_json::Value = serde_json::from_str(actions_json)
            .map_err(|e| JsValue::from_str(&format!("actions json: {e}")))?;
        let actions: Vec<ActionDef> =
            serde_json::from_value(catalog.get("actions").cloned().unwrap_or_default())
                .map_err(|e| JsValue::from_str(&format!("actions list: {e}")))?;
        let lines: Vec<String> = seed_n3
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#') && !l.starts_with("@prefix"))
            .collect();
        // A document loaded from a save already carries `ev:` lines — recover
        // the tape and the sequence counter so new events continue, not collide.
        let seq = lines.iter().filter(|l| l.ends_with("a ev:Event .")).count() as u32;
        let mut events: Vec<(u32, String)> = Vec::new();
        for l in &lines {
            // line form: `ev:eN ev:action "id" .`
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() >= 3 && parts[1] == "ev:action" {
                let n = parts[0]
                    .trim_start_matches("ev:e")
                    .parse::<u32>()
                    .unwrap_or(0);
                let id = parts[2]
                    .trim_matches('"')
                    .trim_end_matches('.')
                    .trim_matches('"');
                events.push((n, id.to_string()));
            }
        }
        events.sort_by_key(|(n, _)| *n);
        let epistemic_mode = catalog
            .get("epistemicMode")
            .or_else(|| catalog.get("epistemic_mode"))
            .and_then(|v| v.as_str())
            .unwrap_or("simulation")
            .to_string();
        Ok(GameSession {
            lines,
            actions,
            events: events.into_iter().map(|(_, id)| id).collect(),
            seq,
            drive_verified: [false; 4],
            epistemic_mode,
        })
    }

    /// Active epistemic modality mode ("fiction", "non-fiction", "simulation", "hypothesis", "civics").
    pub fn epistemic_mode(&self) -> String {
        self.epistemic_mode.clone()
    }

    /// Set the epistemic modality mode.
    pub fn set_epistemic_mode(&mut self, mode: &str) -> Result<(), JsValue> {
        let normalized = match mode.to_ascii_lowercase().as_str() {
            "fiction" => "fiction",
            "non-fiction" | "nonfiction" => "non-fiction",
            "simulation" => "simulation",
            "hypothesis" => "hypothesis",
            "civics" => "civics",
            _ => {
                return Err(JsValue::from_str(
                    "invalid epistemic_mode: must be fiction | non-fiction | simulation | hypothesis | civics",
                ))
            }
        };
        self.epistemic_mode = normalized.to_string();
        Ok(())
    }

    /// Current world document as N3 text (includes `ev:` event lines).
    pub fn world(&self) -> String {
        self.lines.join("\n") + "\n"
    }

    /// Accepted action ids — the replay tape.
    pub fn events(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.events).unwrap_or(JsValue::NULL)
    }

    /// Evaluate a pure VibeScript cell against a read-only projection of the
    /// current world. Q42-backed live host queries remain an upstream gate.
    pub fn vibe_query(&self, src: &str) -> JsValue {
        match eval_vibe(src, Some(self)) {
            Ok(value) => {
                let mut o = js_sys::Object::new();
                set(&mut o, "ok", JsValue::TRUE);
                set(&mut o, "value", vibe_value_to_js(&value));
                o.into()
            }
            Err(e) => err_obj("VibeScript", &e),
        }
    }

    /// A VibeScript cell may select one catalogued action id. It cannot edit
    /// triples: the existing Qualia SHACL gate remains the only proposal path.
    pub fn vibe_propose(&mut self, src: &str) -> JsValue {
        match eval_vibe(src, Some(self)) {
            Ok(vibe::Value::String(id)) => self.apply_action(&id),
            Ok(_) => err_obj("VibeScript", "command cell must return an action id string"),
            Err(e) => err_obj("VibeScript", &e),
        }
    }

    /// Compatibility entry point for UI hosts and deterministic replay. Even
    /// this action-id form is converted to a VibeScript command cell before it
    /// can reach the QualiaDB SHACL gate.
    pub fn propose(&mut self, action_id: &str) -> JsValue {
        let source = format!("= {}", serde_json::to_string(action_id).unwrap_or_default());
        self.vibe_propose(&source)
    }

    /// The only path that applies a catalogued game action.
    fn apply_action(&mut self, action_id: &str) -> JsValue {
        let Some(action) = self.actions.iter().find(|a| a.id == action_id).cloned() else {
            return err_obj("unknown action", action_id);
        };
        let gate_json = serde_json::to_string(&action.gate).unwrap_or_else(|_| "[]".into());
        let report = match qualia_core_db::wasm_bridge::validate_shacl_json_wasm(
            &self.world(),
            &gate_json,
        ) {
            Ok(r) => r,
            Err(e) => return err_obj("validation error", &js_err(&e)),
        };
        let conforms = js_get(&report, "conforms")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !conforms {
            let mut o = js_sys::Object::new();
            set(&mut o, "ok", JsValue::FALSE);
            set(&mut o, "action", JsValue::from_str(&action.id));
            set(&mut o, "explain", JsValue::from_str(&action.explain));
            set(&mut o, "report", report);
            return o.into();
        }
        for spec in &action.remove_all {
            self.remove_matching(spec, usize::MAX);
        }
        for spec in &action.remove_n {
            self.remove_matching(spec, spec.n.max(1));
        }
        self.seq += 1;
        let seq = self.seq;
        for add in &action.add {
            self.lines.push(add.replace("{seq}", &seq.to_string()));
        }
        // Time-delayed outcomes (for example, a grant decision arriving the
        // evening after submission) are data-driven transitions. QualiaDB
        // SHACL validates each condition against the updated world; effects
        // share this event's sequence and replay deterministically.
        for transition in &action.after_turn {
            let gate_json = serde_json::to_string(&transition.gate).unwrap_or_else(|_| "[]".into());
            let report = match qualia_core_db::wasm_bridge::validate_shacl_json_wasm(
                &self.world(),
                &gate_json,
            ) {
                Ok(r) => r,
                Err(_) => continue,
            };
            let ready = js_get(&report, "conforms")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            if !ready {
                continue;
            }
            for spec in &transition.remove_all {
                self.remove_matching(spec, usize::MAX);
            }
            for spec in &transition.remove_n {
                self.remove_matching(spec, spec.n.max(1));
            }
            for add in &transition.add {
                self.lines.push(add.replace("{seq}", &seq.to_string()));
            }
        }
        self.lines.push(format!("ev:e{seq} a ev:Event ."));
        self.lines
            .push(format!("ev:e{seq} ev:action \"{action_id}\" ."));
        self.lines.push(format!("ev:e{seq} ev:seq {seq} ."));
        self.events.push(action.id.clone());
        self.drive_verified = [false; 4];
        let mut o = js_sys::Object::new();
        set(&mut o, "ok", JsValue::TRUE);
        set(&mut o, "action", JsValue::from_str(&action.id));
        set(&mut o, "seq", JsValue::from_f64(seq as f64));
        o.into()
    }

    fn remove_matching(&mut self, spec: &RemoveSpec, limit: usize) {
        let mut removed = 0usize;
        self.lines.retain(|line| {
            if removed >= limit {
                return true;
            }
            let hit = line_matches(line, spec);
            if hit {
                removed += 1;
            }
            !hit
        });
    }
}

fn line_to_quin(line: &str) -> Option<qualia_core_db::NQuin> {
    use qualia_core_db::lexicon::generate_60bit_token;
    let mut parts = line.split_whitespace();
    let s = parts.next()?;
    let p = parts.next()?;
    let o = parts.next()?;
    if s == "." || p == "." || o == "." {
        return None;
    }
    let s_clean = s.trim_matches(|c| c == '<' || c == '>');
    let p_clean = p.trim_matches(|c| c == '<' || c == '>');
    let o_clean = o.trim_matches(|c| c == '<' || c == '>' || c == '"');
    let subj = generate_60bit_token(s_clean.as_bytes());
    let pred = generate_60bit_token(p_clean.as_bytes());
    let obj = generate_60bit_token(o_clean.as_bytes());
    Some(qualia_core_db::NQuin {
        subject: subj,
        predicate: pred,
        object: obj,
        context: 0,
        metadata: 0,
        parity: subj ^ pred ^ obj,
    })
}

fn eval_vibe(src: &str, session: Option<&GameSession>) -> Result<vibe::Value, String> {
    let mut quins = Vec::new();
    if let Some(session) = session {
        for line in &session.lines {
            if let Some(q) = line_to_quin(line) {
                quins.push(q);
            }
        }
    }
    let mut host = qualia_core_db::poet_host::PoetSnapshot::with_seed(quins);
    let mut env = vibe::Env::default();
    if let Some(session) = session {
        let mut world = BTreeMap::new();
        let count = |s: &str, p: &str| {
            session
                .lines
                .iter()
                .filter(|line| {
                    let mut tokens = line.split_whitespace();
                    tokens.next() == Some(s) && tokens.next() == Some(p)
                })
                .count()
        };
        let has = |s: &str, p: &str, o: &str| {
            session
                .lines
                .iter()
                .any(|line| line.trim() == format!("{s} {p} {o} ."))
        };
        world.insert(
            "coins".into(),
            vibe::Value::I64(count("rc:player", "rc:coin") as i64),
        );
        world.insert(
            "labour_ready".into(),
            vibe::Value::Bool(count("rc:player", "rc:labourToken") > 0),
        );
        world.insert(
            "parts_installed".into(),
            vibe::Value::I64(count("rc:solarProject", "rc:partInstalled") as i64),
        );
        world.insert(
            "shelter_upgrades".into(),
            vibe::Value::I64(count("rc:swag", "rc:upgrade") as i64),
        );
        world.insert(
            "workshop_online".into(),
            vibe::Value::Bool(has("rc:workshop", "rc:status", "\"online\"")),
        );
        world.insert(
            "water_online".into(),
            vibe::Value::Bool(has("rc:waterTank", "rc:status", "\"online\"")),
        );
        world.insert(
            "garden_growing".into(),
            vibe::Value::Bool(has("rc:garden", "rc:status", "\"growing\"")),
        );
        world.insert(
            "approved".into(),
            vibe::Value::Bool(count("rc:solarProject", "rc:approvedBy") > 0),
        );
        world.insert(
            "bridge_open".into(),
            vibe::Value::Bool(has("rc:bridge", "rc:status", "\"passable\"")),
        );
        world.insert(
            "bridge_braced".into(),
            vibe::Value::Bool(count("rc:bridge", "rc:bracedBy") > 0),
        );
        world.insert(
            "pump_online".into(),
            vibe::Value::Bool(has("rc:windPump", "rc:status", "\"online\"")),
        );
        world.insert(
            "orchard_growing".into(),
            vibe::Value::Bool(has("rc:orchard", "rc:status", "\"growing\"")),
        );
        world.insert(
            "day".into(),
            vibe::Value::I64((count("rc:player", "rc:turn") + 1) as i64),
        );
        world.insert(
            "workshop_order_delivered".into(),
            vibe::Value::Bool(count("rc:workshop", "rc:deliveredOrder") > 0),
        );
        world.insert(
            "commons_open".into(),
            vibe::Value::Bool(has("rc:community", "rc:finale", "\"commons\"")),
        );
        world.insert(
            "trade_route_open".into(),
            vibe::Value::Bool(has("rc:community", "rc:finale", "\"trade\"")),
        );
        env.vars.insert("world".into(), vibe::Value::Record(world));
    }
    vibe::eval_cell(src, &mut host, &mut env).map_err(|e| e.to_string())
}

pub(crate) fn vibe_value_to_js(value: &vibe::Value) -> JsValue {
    match value {
        vibe::Value::Null => JsValue::NULL,
        vibe::Value::Bool(v) => JsValue::from_bool(*v),
        vibe::Value::I64(v) => JsValue::from_f64(*v as f64),
        vibe::Value::U64(v) => JsValue::from_f64(*v as f64),
        vibe::Value::F64(v) => JsValue::from_f64(*v),
        vibe::Value::String(v) => JsValue::from_str(v),
        vibe::Value::List(items) => {
            let a = js_sys::Array::new();
            for item in items {
                a.push(&vibe_value_to_js(item));
            }
            a.into()
        }
        vibe::Value::Record(fields) => {
            let o = js_sys::Object::new();
            for (key, item) in fields {
                let _ = js_sys::Reflect::set(&o, &JsValue::from_str(key), &vibe_value_to_js(item));
            }
            o.into()
        }
        other => JsValue::from_str(&format!("{other:?}")),
    }
}

fn vibe_scene_recipe(src: &str) -> Result<asset_catalog::AssetRecipe, String> {
    use asset_catalog::{AssetRecipe, ParametricRecipe};
    let value = eval_vibe(src, None)?;
    let vibe::Value::Record(fields) = value else {
        return Err("scene cell must return a record".into());
    };
    let number = |key: &str, low: f64, high: f64| -> Result<f32, String> {
        let n = fields
            .get(key)
            .and_then(vibe::Value::as_f64)
            .ok_or_else(|| format!("scene field {key} must be numeric"))?;
        if !n.is_finite() || n < low || n > high {
            return Err(format!(
                "scene field {key} must be between {low} and {high}"
            ));
        }
        Ok(n as f32)
    };
    let center = [
        number("x", -6.0, 6.0)?,
        number("y", 0.1, 3.0)?,
        number("z", -6.0, 6.0)?,
    ];
    let color = [
        number("r", 0.0, 1.0)?,
        number("g", 0.0, 1.0)?,
        number("b", 0.0, 1.0)?,
        1.0,
    ];
    let shape = match fields.get("kind") {
        Some(vibe::Value::String(kind)) if kind == "sphere" => ParametricRecipe::Sphere {
            center,
            radius: number("radius", 0.08, 1.2)?,
            latitude: 16,
            longitude: 24,
        },
        Some(vibe::Value::String(kind)) if kind == "cylinder" => ParametricRecipe::Cylinder {
            center,
            radius: number("radius", 0.08, 1.2)?,
            height: number("height", 0.1, 2.4)?,
            segments: 24,
        },
        Some(vibe::Value::String(kind)) if kind == "water" || kind == "water_plane" => {
            let width = number("width", 0.5, 20.0).unwrap_or(8.0);
            let length = number("length", 0.5, 20.0).unwrap_or(8.0);
            ParametricRecipe::WaterPlane {
                center,
                size: [width, length],
                subdivisions: 16,
            }
        }
        Some(vibe::Value::String(kind)) if kind == "vegetation" || kind == "grass" => {
            let radius = number("radius", 0.2, 8.0).unwrap_or(2.0);
            let density = number("density", 4.0, 128.0).unwrap_or(32.0) as u32;
            let blade_height = number("blade_height", 0.1, 4.0).unwrap_or(1.0);
            ParametricRecipe::VegetationPatch {
                center,
                radius,
                density,
                blade_height,
            }
        }
        Some(vibe::Value::String(kind)) if kind == "box" || kind == "cube" => {
            let size_x = number("size_x", 0.1, 6.0)
                .or_else(|_| number("size", 0.1, 6.0))
                .unwrap_or(1.0);
            let size_y = number("size_y", 0.1, 6.0)
                .or_else(|_| number("size", 0.1, 6.0))
                .unwrap_or(1.0);
            let size_z = number("size_z", 0.1, 6.0)
                .or_else(|_| number("size", 0.1, 6.0))
                .unwrap_or(1.0);
            ParametricRecipe::TiltedBox {
                center,
                size: [size_x, size_y, size_z],
                pitch_deg: 0.0,
                roll_deg: 0.0,
                yaw_deg: 0.0,
            }
        }
        Some(vibe::Value::String(kind)) if kind == "probe" || kind == "light_probe" => {
            let radius = number("radius", 0.1, 3.0).unwrap_or(0.3);
            ParametricRecipe::Sphere {
                center,
                radius,
                latitude: 12,
                longitude: 16,
            }
        }
        _ => {
            return Err(
                "scene kind must be sphere, cylinder, water, vegetation, box, or probe".into(),
            )
        }
    };
    Ok(AssetRecipe {
        id: "rc:asset/vibe-preview".to_string(),
        parts: Vec::new(),
        parametric: Some(shape),
        color,
        material_family: asset_catalog::material_family_for("rc:asset/vibe-preview"),
        vibe_source: Some(src.to_string()),
        render_role: "opaque".to_string(),
    })
}

fn line_matches(line: &str, spec: &RemoveSpec) -> bool {
    let mut it = line.split_whitespace();
    if it.next() != Some(spec.s.as_str()) || it.next() != Some(spec.p.as_str()) {
        return false;
    }
    match &spec.o {
        Some(o) => it.next().map(|t| t.trim_end_matches('.')) == Some(o.as_str()),
        None => true,
    }
}

fn js_get(v: &JsValue, key: &str) -> Option<JsValue> {
    let k = JsValue::from_str(key);
    // serde_wasm_bindgen returns engine reports as JS Maps — property access
    // via Reflect is undefined on those; use Map.get.
    if v.is_instance_of::<js_sys::Map>() {
        let r = v.unchecked_ref::<js_sys::Map>().get(&k);
        if r.is_undefined() {
            None
        } else {
            Some(r)
        }
    } else {
        js_sys::Reflect::get(v, &k).ok()
    }
}

fn js_err(e: &JsValue) -> String {
    e.as_string().unwrap_or_else(|| format!("{e:?}"))
}

fn set(o: &mut js_sys::Object, k: &str, v: JsValue) {
    let _ = js_sys::Reflect::set(o, &JsValue::from_str(k), &v);
}

pub(crate) fn err_obj(kind: &str, detail: &str) -> JsValue {
    let mut o = js_sys::Object::new();
    set(&mut o, "ok", JsValue::FALSE);
    set(
        &mut o,
        "error",
        JsValue::from_str(&format!("{kind}: {detail}")),
    );
    o.into()
}

#[cfg(test)]
mod scene_bridge_tests {
    use super::{bridge_scene_key, bridge_seed};

    #[test]
    fn scene_key_advances_only_from_story_facts() {
        assert_eq!(
            bridge_scene_key(0, 0, false, false, false, false, false, false, false, false, false),
            "opening"
        );
        assert_eq!(
            bridge_scene_key(1, 1, false, false, false, false, false, false, false, false, false),
            "early-progress"
        );
        assert_eq!(
            bridge_scene_key(1, 2, true, true, true, false, false, true, true, false, false),
            "thriving"
        );
        assert_eq!(
            bridge_scene_key(1, 2, true, true, true, false, false, true, true, true, false),
            "harvested"
        );
    }

    #[test]
    fn effect_seed_is_repeatable_and_state_sensitive() {
        let a = bridge_seed("high-tide", 5, false, true);
        assert_eq!(a, bridge_seed("high-tide", 5, false, true));
        assert_ne!(a, bridge_seed("high-tide", 6, false, true));
        assert_ne!(a, bridge_seed("high-tide", 5, true, true));
    }
}

/// Replay a tape of accepted action ids against the seed and return the
/// resulting world text — the caller compares it to the live world to prove
/// determinism. Any rejected proposal marks the divergence point.
#[wasm_bindgen]
pub fn session_replay(
    seed_n3: &str,
    actions_json: &str,
    event_ids: JsValue,
) -> Result<JsValue, JsValue> {
    let ids: Vec<String> = serde_wasm_bindgen::from_value(event_ids)
        .map_err(|e| JsValue::from_str(&format!("event ids: {e}")))?;
    let mut s = GameSession::new(seed_n3, actions_json)?;
    let mut diverged_at = JsValue::NULL;
    for (i, id) in ids.iter().enumerate() {
        let r = s.propose(id);
        if js_get(&r, "ok").and_then(|v| v.as_bool()) != Some(true) {
            diverged_at = JsValue::from_f64(i as f64);
            break;
        }
    }
    let mut o = js_sys::Object::new();
    set(&mut o, "world", JsValue::from_str(&s.world()));
    set(&mut o, "diverged_at", diverged_at);
    Ok(o.into())
}

// --- VibeScript: bounded authored content ---

/// Evaluate a VibeScript cell through the pinned host. Scripts propose and
/// evaluate through published capabilities only — no direct state mutation.
#[wasm_bindgen]
pub fn vibe_eval_cell(src: &str) -> JsValue {
    vibe_wasm::eval_cell_src(src)
}

/// Invoke a VibeScript host capability by id. Granted stdlib kernels
/// (`Econ.*`, `Statistics.*`, `PhysicalUnits.*`, …) evaluate; unknown or
/// ungranted ids fail closed — the boundary QG-04 requires.
#[wasm_bindgen]
pub fn vibe_invoke(id: &str, args_json: &str) -> JsValue {
    vibe_wasm::capability_invoke(id, args_json)
}

#[cfg(test)]
mod vibe_scene_expansion_tests {
    use super::*;

    #[test]
    fn test_vibe_scene_recipe_water_and_vegetation() {
        let water_vibe = r#"{
            kind: "water",
            x: 0.0, y: 0.2, z: 0.0,
            width: 6.0, length: 6.0,
            r: 0.1, g: 0.5, b: 0.8
        }"#;
        let recipe = vibe_scene_recipe(water_vibe).expect("water vibe recipe should parse");
        assert!(recipe.parametric.is_some());
        let mesh = recipe
            .parametric
            .unwrap()
            .compile()
            .expect("water plane compiles");
        assert!(!mesh.positions.is_empty());
        assert!(!mesh.triangles.is_empty());

        let veg_vibe = r#"{
            kind: "vegetation",
            x: 1.0, y: 0.1, z: -1.0,
            radius: 2.5, density: 16.0, blade_height: 1.2,
            r: 0.2, g: 0.8, b: 0.3
        }"#;
        let recipe = vibe_scene_recipe(veg_vibe).expect("veg vibe recipe should parse");
        assert!(recipe.parametric.is_some());
        let mesh = recipe
            .parametric
            .unwrap()
            .compile()
            .expect("veg patch compiles");
        assert_eq!(mesh.positions.len(), 16 * 3);
        assert_eq!(mesh.triangles.len(), 16);

        let box_vibe = r#"{
            kind: "box",
            x: -1.0, y: 0.5, z: 1.0,
            size: 1.2,
            r: 0.9, g: 0.7, b: 0.2
        }"#;
        let recipe = vibe_scene_recipe(box_vibe).expect("box vibe recipe should parse");
        let mesh = recipe.parametric.unwrap().compile().expect("box compiles");
        assert!(!mesh.positions.is_empty());
    }

    #[test]
    fn test_vibe_eval_session_with_modalities() {
        let mut session = GameSession {
            lines: vec![
                "rc:player rc:coin 10 .".into(),
                "rc:workshop rc:status \"online\" .".into(),
                "ev:e1 a ev:Event .".into(),
            ],
            actions: Vec::new(),
            events: vec!["init".into()],
            seq: 1,
            drive_verified: [false; 4],
            epistemic_mode: "simulation".into(),
        };
        assert_eq!(session.epistemic_mode(), "simulation");
        assert!(session.set_epistemic_mode("civics").is_ok());
        assert_eq!(session.epistemic_mode(), "civics");
        assert!(session.set_epistemic_mode("invalid-mode").is_err());
        // Verify line_to_quin converts Turtle triples
        assert!(line_to_quin("rc:player rc:coin 10 .").is_some());
        assert!(line_to_quin(".").is_none());

        // Verify VibeScript evaluation with session world record
        let val = eval_vibe("= world.coins", Some(&session)).expect("world.coins evaluates");
        assert_eq!(val, vibe::Value::I64(1));

        let online = eval_vibe("= world.workshop_online", Some(&session))
            .expect("workshop_online evaluates");
        assert_eq!(online, vibe::Value::Bool(true));

        // Verify CharacterSimulation loads and evaluates character-arcs.vibe
        let char_src = include_str!("../../../web/fixtures/character-arcs.vibe");
        let char_sim = CharacterSimulation::new(char_src).expect("character simulation loads");
        assert_eq!(char_sim.nearby_radius("observant"), 7.0);
        assert_eq!(char_sim.nearby_radius("connector"), 5.0);
        assert_eq!(char_sim.walk_speed("trailwise", false), 2.6 * 1.25);
        assert_eq!(char_sim.walk_speed("steadfast", true), 5.0 * 1.15);
        assert!(char_sim.bio("Ada").contains("tools"));

        // Verify CampaignSimulation loads and evaluates campaign-rules.vibe
        let camp_src = include_str!("../../../web/fixtures/campaign-rules.vibe");
        let _camp_sim = CampaignSimulation::new(camp_src).expect("campaign simulation loads");
    }
}
