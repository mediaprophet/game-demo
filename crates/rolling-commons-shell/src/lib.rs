//! Rolling Commons — QualiaDB-backed WASM game shell.
//!
//! Per the QualiaDB-only development contract, this crate exposes game-facing
//! bindings over published QualiaDB surfaces only. It owns no renderer, rules
//! engine, graph store, or persistence of its own.
//!
//! Profiles: `wasm-webcivics` (semantic interchange + SHACL + modal logic +
//! Q42 kernels) and `portal` (WebGPU render path incl. `.10d` ingest and
//! semantic picking). No `wasm-llm` — the no-LLM baseline per D-007/D-020.

#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// Upstream QualiaDB revision this build was verified against
/// (`0.0.40.5` / `rolling-commons/phase0`). Keep in sync with the ledger.
pub const QUALIADB_PINNED_REVISION: &str = "27d1644a";

#[wasm_bindgen]
pub fn pinned_qualiadb_revision() -> String {
    QUALIADB_PINNED_REVISION.to_string()
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
        return Err(JsValue::from_str("triangles must be non-empty index triples"));
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
    if triangles.iter().flatten().any(|&i| i as usize >= positions.len()) {
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
    compile_mesh_to_10d_with_nodes(&mesh, &nodes)
        .map_err(|e| JsValue::from_str(&format!("{e:?}")))
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
        return Err(JsValue::from_str("nodes must be non-empty groups of 10 floats"));
    }
    let nodes: Vec<Tensor10D> = nodes_flat
        .chunks_exact(10)
        .map(|n| Tensor10D::new(n[0], n[1], n[2], n[3], n[4], n[5], n[6], n[7], n[8], n[9]))
        .collect();
    let mut buf = vec![0u8; TensorBufferHeader::total_bytes(nodes.len())];
    write_tensor_buffer(&nodes, &mut buf)
        .map_err(|e| JsValue::from_str(e))?;
    Ok(buf)
}

/// Verify a `.10d` container read-back: mesh decode + content digest, i.e.
/// the Q42 manifest linkage the game uses to bind semantic IDs to geometry.
#[wasm_bindgen]
pub fn asset_verify_10d(bytes: &[u8]) -> Result<JsValue, JsValue> {
    use qualia_core_db::render::compile_10d::{compiled_digest, decode_10d_mesh};

    let mesh = decode_10d_mesh(bytes).map_err(|e| JsValue::from_str(&format!("{e:?}")))?;
    #[derive(serde::Serialize)]
    struct Verified {
        vertices: usize,
        triangles: usize,
        digest: u32,
        min: [f32; 3],
        max: [f32; 3],
    }
    serde_wasm_bindgen::to_value(&Verified {
        vertices: mesh.vertex_count(),
        triangles: mesh.triangle_count(),
        digest: compiled_digest(bytes),
        min: mesh.min,
        max: mesh.max,
    })
    .map_err(|e| JsValue::from_str(&e.to_string()))
}

// --- Portal: WebGPU scene ingest, frame, semantic pick ---

/// Thin shell over `QualiaPortal` — the QualiaDB browser render surface.
/// A canvas is supplied by the page; all ingest/pick goes through the engine.
#[wasm_bindgen]
pub struct GamePortal {
    inner: qualia_core_db::QualiaPortal,
}

#[wasm_bindgen]
impl GamePortal {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: HtmlCanvasElement) -> Result<GamePortal, JsValue> {
        Ok(GamePortal {
            inner: qualia_core_db::QualiaPortal::new(canvas)?,
        })
    }

    /// Load a sealed `.10d` container into the scene (CRC + provenance
    /// verified inside the engine; fail-closed on malformed input).
    pub fn load_10d(&mut self, bytes: &[u8]) -> Result<JsValue, JsValue> {
        self.inner.load_10d(bytes)
    }

    /// Render one frame; `dt_ms` is elapsed milliseconds since the last tick.
    pub fn tick(&mut self, canvas: &HtmlCanvasElement, dt_ms: f32) -> Result<(), JsValue> {
        self.inner.tick(canvas.clone(), dt_ms)
    }

    /// Upload a tensor buffer (see `tensor_buffer_build`) as the pickable
    /// semantic node set. Also drives `last_tensor`, which the CPU pick
    /// fallback reads when WebGPU is unavailable.
    pub fn upload_tensor(&mut self, bytes: &[u8]) -> Result<(), JsValue> {
        self.inner.upload_tensor_buffer(bytes)
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
