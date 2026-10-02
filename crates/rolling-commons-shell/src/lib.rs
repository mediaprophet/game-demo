//! Rolling Commons — QualiaDB-backed WASM game shell.
//!
//! Per the QualiaDB-only development contract, this crate exposes game-facing
//! bindings over published QualiaDB surfaces only. It owns no renderer, rules
//! engine, graph store, or persistence of its own.
//!
//! Browser profile: `wasm-webcivics` (semantic interchange + SHACL + modal
//! logic + Q42 kernels; no GPU/LLM weight). Rendering is a separate profile
//! concern pending the Phase 0 `portal`/`webizen-render` spike.

use wasm_bindgen::prelude::*;

/// Upstream QualiaDB revision this build was verified against
/// (branch `0.0.40.5`). Keep in sync with the capability ledger.
pub const QUALIADB_PINNED_REVISION: &str = "6356bb5a";

#[wasm_bindgen]
pub fn pinned_qualiadb_revision() -> String {
    QUALIADB_PINNED_REVISION.to_string()
}

/// Parse a Turtle world description into the QualiaDB graph representation.
/// Returns the engine's parse receipt (triple count / errors) as a JS value.
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

/// Validate a data graph (N3/N-Triples) against a SHACL shapes document.
/// Returns the validation report including per-constraint explanations —
/// the basis of the player-facing "Why?" view.
#[wasm_bindgen]
pub fn action_validate_shacl(data_n3: &str, shapes_json: &str) -> Result<JsValue, JsValue> {
    qualia_core_db::wasm_bridge::validate_shacl_json_wasm(data_n3, shapes_json)
}

/// Persist world bytes via QualiaDB's OPFS-backed virtual filesystem.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn world_save(path: &str, data: &[u8]) -> Result<(), JsValue> {
    use qualia_core_db::storage::{OpfsVfs, VirtualFileSystem};
    OpfsVfs
        .write_chunk(path, data)
        .await
        .map_err(|e| JsValue::from_str(&e))
}

/// Load world bytes previously persisted through `world_save`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn world_load(path: &str) -> Result<Vec<u8>, JsValue> {
    use qualia_core_db::storage::{OpfsVfs, VirtualFileSystem};
    OpfsVfs
        .read_chunk(path)
        .await
        .map_err(|e| JsValue::from_str(&e))
}
