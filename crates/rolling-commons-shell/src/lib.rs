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

mod acts_frame;
mod asset_catalog;

use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

/// QualiaDB revision checked for this game pass. Cargo still uses a sibling
/// path dependency; verify the checkout before calling any build reproducible.
/// `f101983e` is `fix/qg12-town-frame-firstpaint` on top of tag `v0.0.40.11`
/// (`19e2abe`). Town uploads can keep authored coordinates. A scene receipt
/// is not paint; canvas soft-rise waits on visual confirm.
pub const QUALIADB_PINNED_REVISION: &str = "f101983e";

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
    bridge_open: bool,
    bridge_braced: bool,
    high_tide: bool,
    pump_online: bool,
    orchard_active: bool,
    vibe_scene: &str,
) -> Result<JsValue, JsValue> {
    use qualia_core_db::container_10d::provenance_section::ProvenanceSidecar;
    use qualia_core_db::render::assets::Mesh;
    use qualia_core_db::render::compile_10d::compile_mesh_to_10d_with_provenance;
    use qualia_core_db::render::scene_primitives::{
        assemble_into, portal_point, recipe_write, Primitive,
    };
    use qualia_core_db::specialized_libs::computational_geometry::geometry_workspace::{
        Cancellation, GeometryWorkspace,
    };

    fn organ(recipe: &asset_catalog::AssetRecipe) -> Result<(Mesh, JsValue), JsValue> {
        let (mesh, source, mime) = if let Some(spec) = &recipe.parametric {
            let mesh = spec
                .compile()
                .map_err(|e| JsValue::from_str(&format!("parametric geometry: {e}")))?;
            if let Some(src) = &recipe.vibe_source {
                (
                    mesh,
                    src.as_bytes().to_vec(),
                    "application/vnd.rolling-commons.vibe-scene;version=1",
                )
            } else {
                (
                    mesh,
                    spec.source_bytes(),
                    "application/vnd.rolling-commons.parametric-recipe;version=1",
                )
            }
        } else {
            let parts: &[Primitive] = &recipe.parts;
            let mut positions = vec![[0.0; 3]; parts.len() * 8];
            let mut triangles = vec![[0; 3]; parts.len() * 12];
            let mut arena = vec![0u8; 64 + parts.len() * 64];
            let cancel = Cancellation::new();
            let mut workspace = GeometryWorkspace::new(&mut arena, &cancel);
            let receipt = assemble_into(parts, &mut positions, &mut triangles, &mut workspace)
                .map_err(|e| JsValue::from_str(&format!("scene geometry: {e:?}")))?;
            positions.truncate(receipt.vertex_count);
            triangles.truncate(receipt.triangle_count);
            let mesh = Mesh {
                positions,
                triangles,
                min: receipt.min,
                max: receipt.max,
            };
            let mut source = vec![0u8; 6 + parts.len() * 25];
            recipe_write(parts, &mut source)
                .map_err(|e| JsValue::from_str(&format!("scene recipe: {e:?}")))?;
            (
                mesh,
                source,
                "application/vnd.qualia.scene-primitives;version=1",
            )
        };
        // The scene is original authored data with no external reuse grant.
        // This records that status; it does not alter the QualiaDB licence.
        let provenance =
            ProvenanceSidecar::new(source, mime, "All rights reserved (licence not assigned)");
        let bytes = compile_mesh_to_10d_with_provenance(&mesh, Some(&provenance))
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let obj = js_sys::Object::new();
        js_sys::Reflect::set(&obj, &"id".into(), &JsValue::from_str(recipe.id))?;
        js_sys::Reflect::set(
            &obj,
            &"bytes".into(),
            &js_sys::Uint8Array::from(bytes.as_slice()),
        )?;
        for (key, value) in ["r", "g", "b", "a"].iter().zip(recipe.color) {
            js_sys::Reflect::set(&obj, &(*key).into(), &JsValue::from_f64(value as f64))?;
        }
        // Authored town-frame centre. The portal ignores these; the page aims
        // the camera here when the upload keeps town coordinates.
        let center = [
            (mesh.min[0] + mesh.max[0]) * 0.5,
            (mesh.min[1] + mesh.max[1]) * 0.5,
            (mesh.min[2] + mesh.max[2]) * 0.5,
        ];
        for (key, value) in ["cx", "cy", "cz"].iter().zip(center) {
            js_sys::Reflect::set(&obj, &(*key).into(), &JsValue::from_f64(value as f64))?;
        }
        Ok((mesh, obj.into()))
    }

    let organs = js_sys::Array::new();
    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];
    let mut recipes = asset_catalog::kestrel_flats(
        upgrades,
        parts,
        online,
        approved,
        water_online,
        garden_active,
    );
    recipes.extend(asset_catalog::saltwind_reach(
        bridge_open,
        bridge_braced,
        high_tide,
        pump_online,
        orchard_active,
    ));
    if !vibe_scene.trim().is_empty() {
        recipes.push(vibe_scene_recipe(vibe_scene).map_err(|e| JsValue::from_str(&e))?);
    }
    for recipe in recipes {
        let (mesh, obj) = organ(&recipe)?;
        for axis in 0..3 {
            min[axis] = min[axis].min(mesh.min[axis]);
            max[axis] = max[axis].max(mesh.max[axis]);
        }
        organs.push(&obj);
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
    ];
    let mut nodes = Vec::with_capacity(places.len() * 10);
    for point in places {
        let p = portal_point(min, max, point);
        nodes.extend_from_slice(&[0.0, 0.0, 4.0, p[0], p[1], p[2], 0.0, 1.0, 0.7, 0.5]);
    }
    let tensor = tensor_buffer_build(&nodes)?;
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &"organs".into(), &organs)?;
    js_sys::Reflect::set(
        &result,
        &"tensor".into(),
        &js_sys::Uint8Array::from(tensor.as_slice()),
    )?;
    let tiles = js_sys::Array::new();
    for (id, center) in [("kestrel", [0.0, 0.0, 0.0]), ("saltwind", [16.0, 0.0, 0.0])] {
        let p = portal_point(min, max, center);
        let tile = js_sys::Object::new();
        js_sys::Reflect::set(&tile, &"id".into(), &JsValue::from_str(id))?;
        for (key, value) in ["x", "y", "z"].iter().zip(p) {
            js_sys::Reflect::set(&tile, &(*key).into(), &JsValue::from_f64(value as f64))?;
        }
        tiles.push(&tile);
    }
    js_sys::Reflect::set(&result, &"tiles".into(), &tiles)?;
    Ok(result.into())
}

// --- Portal: WebGPU scene ingest, frame, semantic pick ---

/// Thin shell over `QualiaPortal` — the QualiaDB browser render surface.
/// A canvas is supplied by the page; all ingest/pick goes through the engine.
///
/// Call order matters: `init_webgpu(canvas)` BEFORE `GamePortal::new` — the
/// canvas must still be context-free so the WebGPU surface can bind.
#[wasm_bindgen]
pub struct GamePortal {
    inner: qualia_core_db::QualiaPortal,
}

/// Arm the WebGPU path. Await once before constructing the portal; on
/// `false`/throw the portal keeps its canvas2d (tier-1) fallback.
#[wasm_bindgen]
pub async fn init_webgpu(canvas: HtmlCanvasElement) -> Result<bool, JsValue> {
    qualia_core_db::render::portal::portal_init_webgpu(canvas).await
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

    /// Load a multi-object scene: `Array<{bytes: .10d, r,g,b,a}>` where each
    /// mesh keeps its authored position in the shared coordinate space — one
    /// global normalisation across all objects. This is how a town of
    /// distinct props composes into one viewport mesh.
    pub fn load_scene(&mut self, organs: &js_sys::Array) -> Result<JsValue, JsValue> {
        self.inner.load_body_organs_colored(organs)
    }

    /// Render tier: 0 = no webgpu, 1 = canvas2d fallback, 2 = GPU path.
    pub fn tier(&self) -> u8 {
        self.inner.tier()
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
    }

    /// Leave the next scene upload in authored town coordinates instead of
    /// the portal orbit frame. One camera then aims at that mesh.
    pub fn set_preserve_authored_frame(&mut self, on: bool) {
        self.inner.set_preserve_authored_frame(on);
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
        let mut session = GameSession {
            lines,
            actions,
            events: events.into_iter().map(|(_, id)| id).collect(),
            seq,
        };
        // The canal clock is a fact in the world, not a page-side guess.
        // Reloaded documents are corrected to the same rule replay uses.
        session.sync_canal_clock();
        Ok(session)
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
            Ok(vibe::Value::String(id)) => self.propose(&id),
            Ok(_) => err_obj("VibeScript", "command cell must return an action id string"),
            Err(e) => err_obj("VibeScript", &e),
        }
    }

    /// Propose an action by catalogue id. The engine validates the gate
    /// against the current world; on `conforms` the declared triple edits are
    /// applied and an `ev:` event line is appended (event-sourced, so a
    /// replay of the tape must reproduce this exact world text).
    pub fn propose(&mut self, action_id: &str) -> JsValue {
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
        self.lines.push(format!("ev:e{seq} a ev:Event ."));
        self.lines
            .push(format!("ev:e{seq} ev:action \"{action_id}\" ."));
        self.lines.push(format!("ev:e{seq} ev:seq {seq} ."));
        self.events.push(action.id.clone());
        self.sync_canal_clock();
        let mut o = js_sys::Object::new();
        set(&mut o, "ok", JsValue::TRUE);
        set(&mut o, "action", JsValue::from_str(&action.id));
        set(&mut o, "seq", JsValue::from_f64(seq as f64));
        o.into()
    }

    /// Day five (four recorded rests) raises the canal. The bridge may
    /// already be passable; high water stays a fact either way. No-op when
    /// the seed has no canal subject, so the rule cannot invent one.
    fn sync_canal_clock(&mut self) {
        let has_canal = self.lines.iter().any(|line| {
            let mut tokens = line.split_whitespace();
            tokens.next() == Some("rc:canal") && tokens.next() == Some("rc:tide")
        });
        if !has_canal {
            return;
        }
        let turns = self
            .lines
            .iter()
            .filter(|line| {
                let mut tokens = line.split_whitespace();
                tokens.next() == Some("rc:player") && tokens.next() == Some("rc:turn")
            })
            .count();
        let tide = if turns >= 4 { "high" } else { "low" };
        let written = format!("rc:canal rc:tide \"{tide}\" .");
        for line in &mut self.lines {
            let mut tokens = line.split_whitespace();
            if tokens.next() == Some("rc:canal") && tokens.next() == Some("rc:tide") {
                *line = written.clone();
                break;
            }
        }
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

fn eval_vibe(src: &str, session: Option<&GameSession>) -> Result<vibe::Value, String> {
    let mut host = vibe::LocalHost::default();
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
        let bare = |s: &str, p: &str| -> String {
            session
                .lines
                .iter()
                .find_map(|line| {
                    let mut tokens = line.split_whitespace();
                    if tokens.next() == Some(s) && tokens.next() == Some(p) {
                        tokens
                            .next()
                            .map(|token| token.trim_end_matches('.').trim_matches('"').to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or_default()
        };
        let litres = bare("rc:waterTank", "rc:waterLitres")
            .parse::<i64>()
            .unwrap_or(0);
        world.insert("canal_tide".into(), vibe::Value::String(bare("rc:canal", "rc:tide")));
        world.insert(
            "bridge_status".into(),
            vibe::Value::String(bare("rc:bridge", "rc:status")),
        );
        world.insert(
            "water_status".into(),
            vibe::Value::String(bare("rc:waterTank", "rc:status")),
        );
        world.insert("water_litres".into(), vibe::Value::I64(litres));
        world.insert(
            "wallet_instrument".into(),
            vibe::Value::String(if has(
                "rc:scenarioWallet",
                "rc:instrumentOn",
                "rc:sessionHandle",
            ) {
                "rc:scenarioWallet".into()
            } else {
                String::new()
            }),
        );
        world.insert(
            "treasury_instrument".into(),
            vibe::Value::String(
                if has(
                    "rc:committeeTreasury",
                    "rc:instrumentOn",
                    "rc:sessionHandle",
                ) && has("rc:committeeTreasury", "rc:ownedBy", "rc:committee")
                {
                    "rc:committeeTreasury".into()
                } else {
                    String::new()
                },
            ),
        );
        // Coins remain the scenario credits the gates already count. The
        // wallet and treasury are instruments on the session handle, not
        // the player and not a spendable player balance.
        world.insert(
            "instruments_on_player".into(),
            vibe::Value::Bool(
                has("rc:player", "a", "rc:Wallet") || has("rc:player", "a", "rc:Treasury"),
            ),
        );
        env.vars.insert("world".into(), vibe::Value::Record(world));
    }
    vibe::eval_cell(src, &mut host, &mut env).map_err(|e| e.to_string())
}

fn vibe_value_to_js(value: &vibe::Value) -> JsValue {
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
        _ => return Err("scene kind must be sphere or cylinder".into()),
    };
    Ok(AssetRecipe {
        id: "rc:asset/vibe-preview",
        parts: Vec::new(),
        parametric: Some(shape),
        color,
        vibe_source: Some(src.to_string()),
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

fn err_obj(kind: &str, detail: &str) -> JsValue {
    let mut o = js_sys::Object::new();
    set(&mut o, "ok", JsValue::FALSE);
    set(
        &mut o,
        "error",
        JsValue::from_str(&format!("{kind}: {detail}")),
    );
    o.into()
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
