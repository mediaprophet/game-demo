/* tslint:disable */
/* eslint-disable */

/**
 * Persistent compiled cell for the browser binding.
 *
 * Keeps the compiled [`bytecode::Chunk`] so timed / repeated runs can call
 * [`CompiledCell::run`] without re-decoding VBC1 bytes. Prefer this over
 * [`decode_and_run`] when the job is "run what we already compiled."
 */
export class CompiledCell {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compile a cell expression (`= expr`) into a handle. Does not run it.
     */
    static compile(src: string): CompiledCell;
    /**
     * Human-readable disassembly of the held chunk (inspect only).
     */
    disassembly(): string;
    /**
     * Decode a VBC1 byte buffer once into a handle. Later [`Self::run`] calls
     * do not decode again.
     */
    static from_bytes(bytes: Uint8Array): CompiledCell;
    /**
     * Run the held chunk on a fresh local VM. Does not decode.
     */
    run(): any;
    readonly code_size: number;
    readonly constants: number;
    readonly functions: number;
    readonly top_locals: number;
}

/**
 * The Federated Node Manager handles discovery and WebRTC offloading
 */
export class FederatedNodeManager {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Probes the local network/IPC for an installed 64-bit native daemon
     */
    discover_capabilities(): boolean;
    constructor();
    /**
     * Attempts to route a heavy mathematical payload to the native daemon
     */
    offload_intent(intent: WasmOffloadIntent): string;
}

/**
 * Qualia's generic in-viewport HUD. Game rules remain in `GameSession`.
 */
export class GameHud {
    free(): void;
    [Symbol.dispose](): void;
    focus_next(reverse: boolean): string;
    focused_action(): string;
    hit_test(x: number, y: number): string;
    constructor(canvas: HTMLCanvasElement);
    set_camera_target(yaw: number, pitch: number, zoom: number, x: number, y: number, z: number): void;
    set_document_json(json: string): void;
}

/**
 * Thin shell over `QualiaPortal` — the QualiaDB browser render surface.
 * A canvas is supplied by the page; all ingest/pick goes through the engine.
 *
 * Call order matters: `init_webgpu(canvas)` BEFORE `GamePortal::new` — the
 * canvas must still be context-free so the WebGPU surface can bind.
 */
export class GamePortal {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Load a sealed `.10d` container into the scene (CRC + provenance
     * verified inside the engine; fail-closed on malformed input).
     */
    load_10d(bytes: Uint8Array): any;
    /**
     * Load a multi-object scene: `Array<{bytes: .10d, r,g,b,a}>` where each
     * mesh keeps its authored position in the shared coordinate space — one
     * global normalisation across all objects. This is how a town of
     * distinct props composes into one viewport mesh.
     */
    load_scene(organs: Array<any>): any;
    constructor(canvas: HTMLCanvasElement);
    /**
     * Poll the queued pick result: semantic node index or -1 when nothing
     * is hit / the GPU readback is still in flight.
     */
    poll_pick(): number;
    /**
     * Queue a pick at canvas pixel (x, y). CPU fallback resolves
     * synchronously; the GPU path resolves on a subsequent `tick`.
     */
    queue_pick(x: number, y: number, canvas_w: number, canvas_h: number): boolean;
    set_camera(yaw: number, pitch: number, zoom: number): void;
    set_camera_target(yaw: number, pitch: number, zoom: number, x: number, y: number, z: number): void;
    set_sky_preset(preset: number): void;
    /**
     * Render one frame; `dt_ms` is elapsed milliseconds since the last tick.
     */
    tick(canvas: HTMLCanvasElement, dt_ms: number): void;
    /**
     * Render tier: 0 = no webgpu, 1 = canvas2d fallback, 2 = GPU path.
     */
    tier(): number;
    /**
     * Upload a tensor buffer (see `tensor_buffer_build`) as the pickable
     * semantic node set. Also drives `last_tensor`, which the CPU pick
     * fallback reads when WebGPU is unavailable.
     */
    upload_tensor(bytes: Uint8Array): void;
}

export class GameSession {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Accepted action ids — the replay tape.
     */
    events(): any;
    constructor(seed_n3: string, actions_json: string);
    /**
     * Propose an action by catalogue id. The engine validates the gate
     * against the current world; on `conforms` the declared triple edits are
     * applied and an `ev:` event line is appended (event-sourced, so a
     * replay of the tape must reproduce this exact world text).
     */
    propose(action_id: string): any;
    /**
     * A VibeScript cell may select one catalogued action id. It cannot edit
     * triples: the existing Qualia SHACL gate remains the only proposal path.
     */
    vibe_propose(src: string): any;
    /**
     * Evaluate a pure VibeScript cell against a read-only projection of the
     * current world. Q42-backed live host queries remain an upstream gate.
     */
    vibe_query(src: string): any;
    /**
     * Current world document as N3 text (includes `ev:` event lines).
     */
    world(): string;
}

/**
 * Browser canvas HUD. Place its transparent canvas above the Portal canvas.
 * Coordinates are in canvas pixels, independent of CSS scaling.
 */
export class QualiaHud {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Move focus between enabled buttons and return the focused action ID.
     */
    focus_next(reverse: boolean): string;
    focused_action(): string;
    /**
     * Return an action ID for a button under the pointer, in canvas pixels.
     */
    hit_test(x: number, y: number): string;
    constructor(canvas: HTMLCanvasElement);
    /**
     * Repaint the current document. Uses no Rust heap allocations.
     */
    paint(): void;
    /**
     * Use the same orbit camera as Portal to project world markers into HUD pixels.
     */
    set_camera_target(yaw: number, pitch: number, zoom: number, x: number, y: number, z: number): void;
    /**
     * Replace the presentation document. Bounds and size are checked before adoption.
     */
    set_document_json(json: string): void;
}

/**
 * Portal tier: 0 = CPU canvas2d fallback, 1 = tensor projection, 2 = WebGPU ambient.
 */
export class QualiaPortal {
    free(): void;
    [Symbol.dispose](): void;
    acoustic_enabled(): boolean;
    /**
     * SharedArrayBuffer byte length for zero-copy U3 handoff (requires COOP/COEP).
     */
    acoustic_sab_byte_length(): number;
    /**
     * Whether a baked STFT/CQT sidecar is pinned on the portal.
     */
    acoustic_sidecar_pinned(): boolean;
    /**
     * Serialized `AcousticUniform` bytes for AudioWorklet `SharedArrayBuffer` handoff.
     */
    acoustic_uniform_bytes(): Uint8Array;
    /**
     * Phenomenal U3 float uniform count (18 scalars + 64 preview bins).
     */
    acoustic_uniform_float_count(): number;
    /**
     * Flat `f32` uniform for AudioWorklet message port (18 scalars + 64 preview bins).
     */
    acoustic_uniform_floats(): Float32Array;
    ambient_intensity(): number;
    /**
     * Phase 2 — drive the loaded mesh artefact with a kinematic joint (visible physics). `kind` is
     * `"prismatic"` (slide) or anything else = `"revolute"` (spin); `(ax,ay,az)` is the axis
     * (normalised here; defaults to +Y if zero); `rate` is rad/s (revolute) or units/s (prismatic).
     */
    animate_artefact(kind: string, ax: number, ay: number, az: number, rate: number): void;
    /**
     * Phase 2 — whether the artefact's proposed motion is currently being refused (clamped).
     */
    artefact_refused(): boolean;
    /**
     * Cold-bake CQT sidecar (log-spaced bins) for selected tensor node.
     */
    bake_cqt_sidecar_demo(frames: number): Uint8Array;
    /**
     * Cold-bake STFT sidecar for selected tensor node; pins bytes for hot frame reads.
     */
    bake_stft_sidecar_demo(frames: number): Uint8Array;
    /**
     * Cold-path Anatomy lifecycle receipt. Success requires a retained upload
     * and at least one presented renderer frame.
     */
    body_render_receipt(): any;
    /**
     * Phase 5 (affordability rail) — whether a device tier (`0`=Full, `1`=Eco, `2`=Reserve)
     * collapses a qapp's 3D scene to its 2D pane under the budget rule. Pure (no state change);
     * the qapp planner (`render::authoring`) uses the same `OperationalMode::supports_3d` source.
     */
    budget_collapses_3d(mode_code: number): boolean;
    camera_pitch(): number;
    camera_target_x(): number;
    camera_target_y(): number;
    camera_target_z(): number;
    camera_yaw(): number;
    camera_zoom(): number;
    /**
     * Wavefunction collapse — set node `q` to 0 in the resident session manifold.
     */
    collapse_node_q(index: number): void;
    /**
     * Pending ICP commands in the SPSC ring.
     */
    control_pending(): number;
    /**
     * Allocate zeroed acoustic SAB with Q3AS header.
     */
    create_acoustic_sab(): SharedArrayBuffer;
    /**
     * Phase 2 — visible **deterministic refusal**: slide the artefact along +X (prismatic joint)
     * into a world bound; the admission gate refuses poses that would leave the bound, so the
     * artefact deterministically halts at the wall instead of passing through.
     */
    demo_artefact_refusal(): void;
    /**
     * Drain up to `max` control commands and apply to this portal. Returns count applied.
     */
    drain_control_commands(max: number): number;
    /**
     * Drain pending sonic tokens into a JS `BigUint64Array` or `Array` of token raw values.
     */
    drain_sonic_tokens(max: number): any;
    encode_geometry(json: string): any;
    epistemic_q(): number;
    last_parsed(): any | undefined;
    /**
     * P9.2 — Load a `.10d` container asset: parse the section table, extract
     * the QuantizedMesh and Tensor10DNodes (provenance) sections, upload the
     * mesh to the GPU, and report node/triangle counts.
     *
     * **Governance fail-closed:** if the header carries
     * `FLAG_DEFAULT_DISPOSITION_REFUSE` (bit 0) and no attestation section is
     * present, the mesh is loaded for display but `description` marks it as
     * governance-refused — not citable as provenance until attested.
     *
     * Returns a JS object `{ vertex_count, triangle_count, provenance_mu, tier }`.
     */
    load_10d(bytes: Uint8Array): any;
    /**
     * S5.1 colour-by-load — like [`load_10d`] but paints the whole organ mesh a single uniform linear
     * RGBA. The host resolves each organ's body-system percept
     * (`qualia-client-core … AnatomyViewReport::paint_organs`) and passes that system's σ-derived colour
     * (`OrganPercept.percept.rgba`) here, so the 3D body is coloured by accumulated burden. Same
     * governance fail-closed as `load_10d`. (Deliberately parallels `load_10d` rather than sharing a
     * refactored helper: the portal path is wasm+GPU-only and not runtime-testable here, so the proven
     * `load_10d` is left untouched — unify them in the browser-test pass when the anatomy GLBs land.)
     */
    load_10d_colored(bytes: Uint8Array, r: number, g: number, b: number, a: number): any;
    /**
     * S5.8 (web) — load the whole body directly from a `.hmc` **anatomy pack**
     * bundle (see [`crate::bundle`]). Parses the bundle with the *shared* Rust
     * reader (the same code the native host uses — "one reader, both channels"),
     * reads each organ's sealed `.10d` plus its
     * [`AnatomyOrganMeta`](crate::render::anatomy_pack::AnatomyOrganMeta) (system
     * colour + anatomical position), and hands them to
     * [`Self::load_body_organs_colored`]. This is the pure-web render path — no
     * Tauri host / `webizen://` needed: the browser fetches one `.hmc` file and
     * renders the real body. Returns the same `{organs_loaded, organs_refused,
     * total_triangles}` summary.
     */
    load_body_from_qualia_bundle(bytes: Uint8Array): any;
    /**
     * Like [`Self::load_body_from_qualia_bundle`] but honours the **mixer's per-body-system
     * channels**: `system_levels` is a JS object `{ <system_id>: <level 0..1> }`. An organ whose
     * system level is ≤ 0 is omitted (muted); otherwise its colour alpha is scaled by the level.
     * (The mesh pipeline is currently opaque, so a nonzero level acts as show; smooth opacity lands
     * when the mesh pipeline gains alpha blending — mixer plan P2.) An absent/empty map shows every
     * system at full — so `load_body_from_qualia_bundle` is exactly this with no mixer applied.
     *
     * Decodes organs **in Rust** from the pack buffer — no per-organ JS `Uint8Array` materialisation.
     * That cut peak heap by ~1–2× pack size and is the phone-safe path.
     */
    load_body_from_qualia_bundle_mixed(bytes: Uint8Array, system_levels: any, disabled_parts: any): any;
    /**
     * S5.8 — load the **whole body** as a set of per-organ `.10d` meshes, each painted its body-system's
     * σ-derived RGBA, accumulated into one combined GPU mesh. This is the real-mesh render path.
     *
     * The CCF/HRA reference organs are authored in ONE shared body coordinate space (a brain's vertices
     * sit at the head, a bladder's at the pelvis, skin envelops the whole body), so this **preserves
     * each organ's TRUE position and relative size**: it accumulates the whole-body bounds across all
     * organs and applies **one global centre + scale**, rather than normalising each organ separately
     * (which would flatten proportions and shrink the full-body skin mesh to a dot). Governance
     * fail-closed per organ, as in `load_10d_colored`.
     *
     * `organs` is a JS `Array` of objects: `{ bytes: Uint8Array, r: f32, g: f32, b: f32, a: f32 }`
     * (per-organ colour). Any `x/y/z` fields are ignored — the mesh already carries its position.
     * Returns `{ organs_loaded, organs_refused, total_triangles }`.
     *
     * Prefer [`Self::load_body_from_qualia_bundle_mixed`] for packs — that path never materialises a
     * per-organ JS `Uint8Array` copy (critical on phones).
     */
    load_body_organs_colored(organs: Array<any>): any;
    load_json_scene(json: string): any;
    load_q42(bytes: Uint8Array): any;
    mount_qapp(root_id: string): void;
    /**
     * Frame the camera on a tensor node (`Maps_to_node`).
     */
    navigate_to_node(index: number): void;
    constructor(canvas: HTMLCanvasElement);
    /**
     * Select at pixel; returns index immediately on CPU fallback, else `-1` until next `tick`.
     */
    observe_node_at(x: number, y: number, canvas_w: number, canvas_h: number): number;
    operational_mode(): number;
    /**
     * Read a `.hmc` pack's **manifest** without rendering — the list of parts the UI builds its
     * dynamic system + part selectors from. Returns a JS array of `{ key, label, system, systems }`
     * (one per `.10d` entry), so the demo can offer per-system *and* per-part select/deselect driven by
     * what is actually in the loaded pack, not a hardcoded list. Read-only.
     */
    pack_manifest(bytes: Uint8Array): any;
    /**
     * Returns selected tensor index, or `-1` if none / pick still pending.
     */
    poll_selected_node(): number;
    /**
     * Phase 1.4 — the **2D view** of the resident manifold: each tensor node's `project(.., Plane2D)`
     * shadow as a flat `[x0,y0,x1,y1,...]` array (world units, ~[-1,1]). The 3D scene draws the same
     * nodes through the GPU projector (the `Volume3D` view); both are the *one* manifold projection
     * seen two ways (see `manifold_project`). JS paints this on the 2D companion canvas.
     */
    project_resident_plane2d(time: number): Float32Array;
    /**
     * Publish phenomenal uniform + pending sonic tokens into SAB.
     */
    publish_acoustic_sab(sab: SharedArrayBuffer): void;
    /**
     * Push a packed Interface Control Plane command (`PortalControlCommand` raw `u64`).
     */
    push_control_command(raw: bigint): boolean;
    push_sonic_token_raw(raw: bigint): boolean;
    resize(canvas: HTMLCanvasElement, width: number, height: number): void;
    sample_telemetry(): any;
    /**
     * Queue GPU picking at canvas pixel `(x, y)`. Result available after the next `tick`.
     */
    select_node_at(x: number, y: number, canvas_w: number, canvas_h: number): void;
    selected_node_index(): number;
    /**
     * Enable or mute U3 AcousticPlane (automatically off in Reserve mode).
     */
    set_acoustic_enabled(enabled: boolean): void;
    /**
     * Enable/disable the **ambient particle field** — the mixer's "ambient" channel. Off by default
     * (a plain mesh/anatomy view has no use for the decorative random cloud); a Tensor10D upload
     * turns it on automatically because the particles then encode epistemic nodes.
     */
    set_ambient_enabled(on: boolean): void;
    /**
     * Replace the person-authored body fit. Pass JSON matching wellfare `BodyFit`.
     * Applied on the next `load_body_*` upload. Empty / invalid JSON resets to identity.
     */
    set_body_fit_json(json: string): void;
    /**
     * Orbit camera IPC from the UI shell (yaw/pitch in radians, zoom = eye distance).
     */
    set_camera(yaw: number, pitch: number, zoom: number): void;
    /**
     * RTS camera pan IPC (moves look-at center point in world space).
     */
    set_camera_pan(target_x: number, target_y: number, target_z: number): void;
    /**
     * Full camera placement (yaw, pitch, zoom, and world-space target center).
     */
    set_camera_target(yaw: number, pitch: number, zoom: number, target_x: number, target_y: number, target_z: number): void;
    /**
     * Configure viewport background / sky clear color (r, g, b, a in 0.0..1.0).
     */
    set_clear_color(r: number, g: number, b: number, a: number): void;
    set_display_mode(mode: string): void;
    /**
     * Configure directional sun lighting (direction vector, sun intensity, and ambient intensity).
     */
    set_lighting(sun_x: number, sun_y: number, sun_z: number, sun_intensity: number, ambient_intensity: number): void;
    /**
     * Apply an authored atmospheric sky preset:
     * - 0: Cyber-Dark / Deep Void
     * - 1: Daylight (clear sky, high warm sun, balanced ambient)
     * - 2: Sunset / Golden Hour (warm orange sky, low golden sun)
     * - 3: Night / Moonlight (dark indigo sky, cold moonlight)
     * Keep the next `load_body_*` upload in authored coordinates.
     * Anatomy stays false (orbit frame). A town scene sets this so the
     * camera and the mesh share one frame.
     */
    set_preserve_authored_frame(on: boolean): void;
    set_sky_preset(preset: number): void;
    /**
     * Human-Centric observer standpoint IPC (independent of camera lens).
     *
     * `standpoint_class`: 0=spectator, 1=ephemeral, 2=identifier (DID), 3=vault.
     * `identifier_did`: empty for spectator/ephemeral; supply DID IRI to bind a verified
     * identifier. Vault standpoints require a sealed local data plane (not exposed here).
     */
    set_standpoint(standpoint_class: number, epistemic_q: number, t_slice: number, t_window: number, identifier_did: string): void;
    set_telemetry(floats: Float32Array): void;
    set_temporal_slice(t_slice: number, t_window: number): void;
    sonic_token_pending(): number;
    spatial_encode(json: string): any;
    standpoint_class(): number;
    /**
     * Phase 2 — freeze the artefact (joint → identity, no world clamp).
     */
    stop_artefact_animation(): void;
    sun_intensity(): number;
    t_slice(): number;
    t_window(): number;
    tick(canvas: HTMLCanvasElement, dt_ms: number): void;
    tier(): number;
    /**
     * Import a 3D mesh asset (OBJ / STL / GLB bytes) and render it as a solid surface (Phase 1.2).
     * The mesh is centred on its bounding-box centroid and scaled so its largest extent is ~1.6
     * units — fitting the orbit camera's default frame (eye at distance 3.5, looking at the origin)
     * — then uploaded to the GPU. `hint` is an optional lowercase extension ("obj"/"stl"/"glb");
     * empty = sniff from the bytes. Returns the triangle count (0 if the GPU path isn't active).
     */
    upload_mesh_asset(bytes: Uint8Array, hint: string): number;
    upload_tensor_buffer(bytes: Uint8Array): void;
    /**
     * Write posed positions into one vertex span of the resident body mesh.
     * Generic: any app can move a part over time without a full re-upload.
     */
    write_part_vertices(start: number, xyz: Float32Array): void;
}

export class QualiaStore {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Clear all stored quints.
     */
    clear(): void;
    /**
     * Parse a CBOR-LD byte array (CBOR array of 4 or 5 unsigned integers) and
     * insert the resulting quin. Returns true on success, false on parse error.
     *
     * The qualiaDB binary gatekeeper (cbor_compiler.rs) requires this format:
     *   CBOR array header (0x84 or 0x85) followed by 4–5 CBOR unsigned integers.
     * All values are Lexicon-compressed u64 IDs assigned by the JS Lexicon.
     */
    insert_from_cbor_ld(data: Uint8Array): boolean;
    /**
     * Insert a quint (s, p, o, c, m).  Returns true on success.
     */
    insert_quin(s: bigint, p: bigint, o: bigint, c: bigint, m: bigint): boolean;
    /**
     * Total number of quints stored.
     */
    len(): number;
    constructor();
    /**
     * Return all quints in the given context as a flat Float64Array.
     */
    query_context(c: bigint): Float64Array;
    /**
     * Return all quints with the given predicate as a flat Float64Array.
     */
    query_predicate(p: bigint): Float64Array;
    /**
     * Return all quints with the given subject as a flat Float64Array
     * (groups of 5: [s,p,o,c,m, s,p,o,c,m, ...]).
     */
    query_subject(s: bigint): Float64Array;
}

/**
 * In-memory RDF store exposed to JS.  Load Turtle, run SPARQL SELECT/ASK/CONSTRUCT.
 */
export class WasmHealthStore {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Load a Turtle document into the store (appends — call on a fresh store to replace).
     */
    load_turtle(turtle: string): void;
    constructor();
    /**
     * Execute a SPARQL query; returns JSON SPARQL results string.
     */
    query(sparql: string): string;
}

/**
 * WASM edge offload descriptor — distinct from governance [`crate::llm_agent::AgentIntent`].
 */
export class WasmOffloadIntent {
    free(): void;
    [Symbol.dispose](): void;
    constructor(opcode: number, priority: number, payload_size: number);
    static with_string_payload(opcode: number, priority: number, payload: string): WasmOffloadIntent;
    opcode: number;
    payload_size: number;
    priority: number;
}

export class WasmQ42Session {
    free(): void;
    [Symbol.dispose](): void;
    active_quin_count(): number;
    advance_tick(tick: bigint): void;
    commit_transaction(command_hash: bigint, actor_did: bigint): number;
    current_tick(): bigint;
    /**
     * Export all currently active Quins as a flat contiguous 48-byte buffer.
     */
    export_active_quins_bytes(): Uint8Array;
    export_journal_bytes(): Uint8Array;
    /**
     * Load an existing durable Q42 session from journal bytes and optional base Quin snapshot bytes.
     */
    static load_from_bytes(journal_bytes: Uint8Array, base_quins_bytes: Uint8Array): WasmQ42Session;
    constructor(world_did_hash: bigint, base_digest_bytes: Uint8Array);
    /**
     * Check if a Quin matching the given pattern exists in the active session (0 = wildcard).
     */
    query_has_quin(s: bigint, p: bigint, o: bigint): boolean;
    /**
     * Query the object value for a given (subject, predicate) pair.
     */
    query_object_for_predicate(s: bigint, p: bigint): bigint | undefined;
    /**
     * Rewind session state to a specific simulation/historical tick.
     */
    rewind_to_tick(base_quins_bytes: Uint8Array, target_tick: bigint): void;
    rollback_staged(): void;
    stage_add_quin(s: bigint, p: bigint, o: bigint, c: bigint, m: bigint): void;
    stage_remove_quin(s: bigint, p: bigint, o: bigint, c: bigint, m: bigint): void;
}

export class WasmSimulationWorld {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Compute group formation destinations for `count` agents (QG-11).
     * Returns interleaved destination coordinates: [x0, y0, x1, y1, ...].
     */
    compute_group_formation(target_x: number, target_y: number, count: number, obstacles: Uint8Array, width: number, height: number): Int32Array;
    compute_state_hash(): bigint;
    current_tick(): bigint;
    /**
     * Deterministic 2D grid pathfinder (QG-11).
     * Returns interleaved waypoint coordinates: [x0, y0, x1, y1, ...].
     */
    find_path_grid(start_x: number, start_y: number, target_x: number, target_y: number, obstacles: Uint8Array, width: number, height: number): Int32Array;
    get_agent_pos_x_mm(entity_id: bigint): bigint | undefined;
    get_agent_pos_y_mm(entity_id: bigint): bigint | undefined;
    constructor(seed: bigint);
    /**
     * Spatial box query returning matching entity IDs as strings (for u64 JS precision).
     */
    query_agents_in_bounds(min_x_mm: bigint, min_y_mm: bigint, max_x_mm: bigint, max_y_mm: bigint): string[];
    register_agent(entity_id: bigint, x_mm: bigint, y_mm: bigint): boolean;
    step_tick(): number;
    submit_command(target_tick: bigint, sequence: number, actor_did: bigint, command_opcode: number, target_entity: bigint, arg0: bigint, arg1: bigint): void;
}

/**
 * Legacy alias — prefer `QualiaPortal`.
 */
export class WebEngine {
    free(): void;
    [Symbol.dispose](): void;
    last_parsed(): any | undefined;
    load_json_scene(json: string): any;
    load_q42(bytes: Uint8Array): any;
    mount_qapp(root_id: string): void;
    constructor();
    render_to_canvas(): void;
}

/**
 * Validate a data graph (N3 token form) against JSON SHACL ShapeSpecs.
 * Returns the validation report (`conforms`, per-constraint results) —
 * the basis of the player-facing "Why?" view.
 */
export function action_validate_shacl(data_n3: string, shapes_json: string): any;

export function align_sequences_wasm(val: any): any;

/**
 * Apply a structural edit to a VibeScript program and project the result.
 *
 * The edit is specified as a JSON object with an `op` field and
 * operation-specific fields. This enables LLMs and browsers to
 * edit program structure without text patching.
 *
 * Supported ops:
 * - `add_field`: { op, name, ty, unit?, support?, representation? }
 * - `add_material`: { op, name, properties: [{name, value}] }
 * - `add_law`: { op, name, condition, consequence }
 * - `remove_item`: { op, index }
 * - `rename_item`: { op, index, new_name }
 * - `set_field_unit`: { op, index, unit? }
 * - `set_field_support`: { op, index, support }
 * - `set_field_representation`: { op, index, representation }
 * - `add_material_property`: { op, index, name, value }
 * - `remove_material_property`: { op, index, name }
 * - `add_prefix`: { op, prefix, iri }
 * - `remove_prefix`: { op, prefix }
 */
export function apply_structural_edit(src: string, edit_json: string): any;

/**
 * Apply multiple structural edits in sequence.
 * `edits_json` is a JSON array of edit objects.
 */
export function apply_structural_edits(src: string, edits_json: string): any;

/**
 * Compile a game-authored prop into a sealed `.10d` container using
 * QualiaDB's mesh pipeline. `positions` is flat xyz triples (f32),
 * `triangles` flat vertex-index triples (u32, CCW), `nodes_flat` is
 * optional Tensor10D nodes as groups of 10 floats
 * `[q, v, w, x, y, z, t, alpha, mu, sigma]` — the semantic pick targets.
 */
export function asset_compile_10d(positions: Float32Array, triangles: Uint32Array, nodes_flat: Float32Array): Uint8Array;

/**
 * Verify a `.10d` container read-back: mesh decode + content digest, i.e.
 * the Q42 manifest linkage the game uses to bind semantic IDs to geometry.
 */
export function asset_verify_10d(bytes: Uint8Array): any;

/**
 * Get the JSON schema for the AST.
 */
export function ast_schema_json(): string;

/**
 * Black-Scholes European option pricing with full Greeks.
 */
export function black_scholes_wasm(val: any): any;

/**
 * Evaluates input-output multipliers and total requirements via the Leontief inverse (I - A)^(-1).
 */
export function calculate_leontief_multipliers_wasm(val: any): any;

/**
 * Evaluates distributional and welfare metrics (Gini, Atkinson index, Palma ratio,
 * mean, median, P10, P90) and emits an auditable `CalculationReceipt`.
 */
export function calculate_welfare_metrics_wasm(val: any): any;

/**
 * Capability invoke — routes through `LocalHost` so Civics/stdlib kernels
 * (`Econ.*`, `Statistics.ols`, `PhysicalUnits.convert`, …) work under WASM.
 * Ungranted / unknown ids still fail closed with E300 / E100.
 */
export function capability_invoke(id: string, args_json: string): any;

/**
 * Symbolic derivative. Input `{ expr, var }` (e.g. `{ "expr":"x^3 - 2*x^2 + 5",
 * "var":"x" }`) → `{ derivative }`. The result is simplified, then rendered with the
 * `Expr` `Display` (fully parenthesised). Errors on a parse failure.
 */
export function cas_differentiate_wasm(val: any): any;

/**
 * Numerically evaluate an expression given variable bindings. Input
 * `{ expr, bindings }` where `bindings` is an object of `name -> number`
 * (e.g. `{ "expr":"x^2 + 3*x + 2", "bindings":{ "x":4 } }`) → `{ value }`.
 * Errors if a referenced variable is unbound, or the result is non-finite
 * (division by zero, √negative, ln of a non-positive value).
 */
export function cas_evaluate_wasm(val: any): any;

/**
 * Distribute products over sums and expand small (≤ 8) positive integer powers, so the
 * result has no product/power over an additive child. Value-preserving. Input
 * `{ expr }` → `{ expanded }`. Errors on a parse failure.
 */
export function cas_expand_wasm(val: any): any;

/**
 * Factor a real quadratic `a·x² + b·x + c` into `a·(x − r₁)·(x — r₂)` (roots snapped to
 * integers/halves when numerically close). Input `{ a, b, c, var }` (`var` defaults to
 * `"x"`) → `{ factored }`. Errors when `a = 0` or the discriminant is negative (no real
 * factorisation).
 */
export function cas_factor_wasm(val: any): any;

/**
 * Algebraic simplification (constant folding + identity elimination, to a bounded
 * fixpoint). Input `{ expr }` → `{ simplified }`. Errors on a parse failure.
 */
export function cas_simplify_wasm(val: any): any;

/**
 * Symbolic roots of `a·x² + b·x + c = 0` as `(-b ± √(b²−4ac)) / (2a)` (simplified
 * `Expr` strings), plus their numeric values when the discriminant is non-negative.
 * Input `{ a, b, c }` → `{ roots:[{ expr, value }] }`. For `a = 0, b ≠ 0` returns the
 * single linear root `-c/b`; for `a = 0, b = 0` returns an empty list. A complex /
 * non-finite root value is reported as `null`.
 */
export function cas_solve_quadratic_wasm(val: any): any;

/**
 * But-for / reachability causation (`causal::caused`).
 */
export function causal_caused_wasm(val: any): any;

/**
 * Check a cell expression.
 */
export function check_cell_src(src: string): any;

export function check_drug_interactions_wasm(val: any): any;

/**
 * Check a full program.
 */
export function check_program_src(src: string): any;

/**
 * Description-logic subsumption check (`check_subsumption_quin`).
 */
export function check_subsumption_wasm(val: any): any;

export function clinical_risk(input_json: string): string;

/**
 * Compile a flat GGUF byte image into a canonical P64 LLM-weight container.
 * Run once at ingest and cache the result in OPFS.
 */
export function compileGgufToP64(gguf: Uint8Array, page_log2: number): Uint8Array;

/**
 * Historical export retained for browser compatibility. Emits P64 bytes.
 */
export function compileGgufToQ42(gguf: Uint8Array, page_log2: number): Uint8Array;

/**
 * Compile a cell expression to bytecode and return chunk metadata.
 *
 * For repeated execution without re-decode, use [`CompiledCell::compile`] and
 * [`CompiledCell::run`] instead. This export stays for playground inspect /
 * size reporting.
 */
export function compile_cell_bytecode(src: string): any;

/**
 * Compiles a query string (SPARQL WHERE-clause or N-Triples pattern) to a JSON
 * description of the Webizen VM bytecode program.  Useful for playground inspection
 * and benchmarking the compilation pipeline without supplying a database.
 */
export function compile_query_to_json(query: string): string;

/**
 * Compile Turtle / N3 `sh:NodeShape` documents into ShapeSpec-compatible JSON (UE-050).
 */
export function compile_shacl_turtle_wasm(turtle: string): any;

export function compute_framingham_risk_wasm(val: any): any;

export function compute_molecular_descriptors_wasm(val: any): any;

/**
 * Evaluates ordinary least squares regression with complete diagnostics and receipt.
 */
export function compute_ols_diagnostics_wasm(val: any): any;

/**
 * Stateless PID controller step.
 * Returns { output, new_error, new_integral } for chaining into the next step.
 */
export function compute_pid_step_wasm(val: any): any;

export function compute_reaction_metrics_wasm(val: any): any;

export function compute_thermochemistry_wasm(val: any): any;

export function create_canvas(width: number, height: number): HTMLCanvasElement;

/**
 * AEAD decrypt + verify. Input `{ algorithm, key:{text|hex}, nonce:{text|hex},
 * ciphertext:{text|hex}, aad?:{text|hex} }` → `{ algorithm, plaintext_hex,
 * plaintext_utf8?, bytes }`. Fails closed on a bad tag / wrong key, nonce, or aad.
 */
export function crypto_aead_decrypt(val: any): any;

/**
 * AEAD encrypt. Input `{ algorithm, key:{text|hex}, nonce:{text|hex},
 * plaintext:{text|hex}, aad?:{text|hex} }` → `{ algorithm, ciphertext_hex, bytes }`.
 * `algorithm` ∈ aes256gcm | chacha20poly1305 | xchacha20poly1305. Key is 32 bytes;
 * nonce 12 (24 for xchacha). The caller owns the nonce — NEVER reuse a (key, nonce).
 */
export function crypto_aead_encrypt(val: any): any;

/**
 * BLAKE3 digest (256-bit).
 */
export function crypto_blake3(val: any): any;

/**
 * HKDF-SHA256 key derivation (RFC 5869). Input
 * `{ ikm:{text|hex}, salt?:{text|hex}, info?:{text|hex}, length }` →
 * `{ algorithm, okm_hex, length }`. `length` is output bytes (1..=8160).
 */
export function crypto_hkdf_sha256(val: any): any;

/**
 * SHA-256 digest of `{ text } | { hex }` → `{ algorithm, hex, bytes }`.
 */
export function crypto_sha256(val: any): any;

/**
 * SHA3-256 (Keccak) digest.
 */
export function crypto_sha3_256(val: any): any;

/**
 * SHA-512 digest.
 */
export function crypto_sha512(val: any): any;

/**
 * Decode a binary bytecode chunk and run it once.
 *
 * Compat wrapper: each call decodes again. Prefer [`CompiledCell::from_bytes`]
 * then [`CompiledCell::run`] when the same bytes will run more than once.
 */
export function decode_and_run(bytes: Uint8Array): any;

/**
 * Decodes tagged CBOR-LD AST bytes (Tag 4200) into a canonical representation,
 * returning the reconstructed source code, AST hash, and provenance metadata.
 */
export function decode_program_cborld_wasm(bytes: Uint8Array): any;

/**
 * Dummy design. `{ categories:number[] }` → matrix + labels.
 */
export function design_dummies_wasm(val: any): any;

export function design_encode_wasm(json: string): any;

export function detect_functional_groups_wasm(val: any): any;

export function device_storage_policy_wasm(): any;

/**
 * Parse + check a module; collect up to eight diagnostics.
 */
export function diagnose_src(src: string): any;

/**
 * Get the JSON schema for diagnostics.
 */
export function diagnostic_schema_json(): string;

/**
 * Get the EBNF grammar string.
 */
export function ebnf_grammar(): string;

/**
 * Encode a cell's bytecode to a binary Uint8Array.
 */
export function encode_cell_bytecode(src: string): any;

/**
 * Encodes a VibeScript source program into tagged CBOR-LD AST bytes (Tag 4200).
 * Validates syntax and type consistency before serializing.
 */
export function encode_program_cborld_wasm(src: string): Uint8Array;

/**
 * Enforces the rights ontology prior to transmission (e.g., checking DID constraints)
 */
export function enforce_rights_ontology(subject_did: bigint): boolean;

/**
 * Enumerate ASP stable-model world contexts (`enumerate_stable_models`).
 */
export function enumerate_stable_models_wasm(val: any): any;

/**
 * Query the browser's storage quota and current OPFS usage (bytes).
 *
 * Returns `{ quota: number, usage: number, available: number }`.
 * On mobile PWA the quota is typically 60 % of free disk space (Chrome) or
 * up to 1 GB on iOS Safari. Call this before a large ingest to check headroom.
 */
export function estimate_browser_storage(): Promise<any>;

/**
 * Evaluate a cell and return the result as a JSON-compatible JS value.
 * Playground Run uses `eval_program_src` (module + optional `main`), not this.
 */
export function eval_cell_json(src: string): any;

/**
 * Evaluate a cell expression (`= expr`) with the in-process local host.
 */
export function eval_cell_src(src: string): any;

/**
 * Evaluate a full program on LocalHost (workshop dialect).
 *
 * Runs preamble items, then `main` when present — same as `vibe eval FILE main`.
 */
export function eval_program_src(src: string): any;

/**
 * Evaluates a Vibe program and returns the evaluated value together with a signed,
 * reproducible execution receipt (containing source hash, AST hash, and engine metadata).
 */
export function eval_program_with_receipt_wasm(src: string): any;

/**
 * Evaluate deontic norms in a quin frame (`evaluate_deontic_contract`).
 */
export function evaluate_deontic_wasm(val: any): any;

/**
 * Evaluate epistemic claims (`evaluate_epistemic_frame`).
 */
export function evaluate_epistemic_wasm(val: any): any;

export function evaluate_inference_guard_wasm(value: any): any;

export function evaluate_lipinski_wasm(val: any): any;

/**
 * Evaluate an LTL formula against a quin trace (`evaluate_ltl_trace`).
 */
export function evaluate_ltl_trace_wasm(val: any): any;

/**
 * Evaluate all 7 N3 clinical rules against a Turtle document.
 *
 * Returns a JSON array of triggered patterns:
 * `[{"pattern":"ChronicSleepDebt","confidence":"high","routingLane":2,"n3Source":"sleep_debt.n3"},...]`
 *
 * Empty array = no concerns found in the supplied health data.
 * Routing lane 2 = BilateralMicroCommons (N3Logic implication rules requiring identity context).
 * Routing lane 0 = PassthroughStandard (simple threshold flags).
 */
export function evaluate_n3_rules(turtle: string): string;

/**
 * Exact sum `a + b`. Input `{ a: String, b: String }` -> `{ result }`.
 */
export function exact_bigint_add(val: any): any;

/**
 * Truncated division with remainder: `a = quotient*b + remainder`, remainder
 * taking the sign of `a` (toward-zero truncation, matching Rust `/` and `%`).
 * Input `{ a: String, b: String }` -> `{ quotient, remainder }`. Fails closed
 * (`Err`) when `b` is zero.
 */
export function exact_bigint_divmod(val: any): any;

/**
 * Factorial `n!` as an exact decimal string. Input `{ n: u32 }` ->
 * `{ result }`. Computed from the wasm-clean `BigInt` primitives (the same
 * `mul` loop the solver's `factorial_100_known_value` test uses), so e.g.
 * `n = 100` returns the full 158-digit value with no overflow.
 */
export function exact_bigint_factorial(val: any): any;

/**
 * Greatest common divisor `gcd(a, b)` (always non-negative; `gcd(0,0) = 0`).
 * Input `{ a: String, b: String }` -> `{ result }`.
 */
export function exact_bigint_gcd(val: any): any;

/**
 * Exact product `a * b`. Input `{ a: String, b: String }` -> `{ result }`.
 */
export function exact_bigint_mul(val: any): any;

/**
 * Exact integer power `base ^ exp`. Input `{ base: String, exp: u32 }` ->
 * `{ result }`. `base` is an arbitrary-precision decimal string; e.g.
 * `base = "2", exp = 100` returns `1267650600228229401496703205376`.
 */
export function exact_bigint_pow(val: any): any;

/**
 * Exact rational sum `a + b`, returned reduced and sign-normalised as `"p/q"`
 * (q > 0). Inputs are `"p/q"` strings (a bare `"p"` is read as `p/1`). Input
 * `{ a: String, b: String }` -> `{ result }`. E.g. `"1/3" + "1/6" = "1/2"`.
 */
export function exact_rational_add(val: any): any;

/**
 * Exact rational product `a * b`, returned reduced and sign-normalised as
 * `"p/q"` (q > 0). Inputs are `"p/q"` strings (a bare `"p"` is read as `p/1`).
 * Input `{ a: String, b: String }` -> `{ result }`. E.g. `"3/4" * "1/4" =
 * "3/16"`.
 */
export function exact_rational_mul(val: any): any;

export function execute_ntriples_query(query: string, db_bytes: Uint8Array, max_results: number): string;

export function export_tensor_buffer_wasm(json: string): any;

export function export_tensor_slice_wasm(max_nodes: number): any;

/**
 * Forward-chaining defeasible inference engine.
 * Input: `{ facts: ["bird", "penguin"], rules: [{ head: "flies", body: ["bird"], defeaters: ["penguin"] }, ...] }`
 * Output: `{ inferred: ["swims"] }`
 */
export function forward_chain_wasm(val: any): any;

/**
 * Fuzzy t-norm (Gödel min / Łukasiewicz / product).
 */
export function fuzzy_t_norm_wasm(val: any): any;

/**
 * Get the GBNF grammar (for LLM constrained decoding).
 */
export function gbnf_grammar(): string;

export function geometric_algebra_operation(input_json: string): string;

/**
 * Compute the 2-D convex hull of a point set.
 *
 * `points` is a flat `[x0, y0, x1, y1, ...]` array.
 * Returns `{ indices, vertex_count, hull_points }` where `hull_points`
 * is a flat `[x0, y0, ...]` array of hull vertices in order.
 *
 * Over the 5-point fixture `[[0,0],[1,0],[0.5,0.5],[1,1],[0,1]]` this
 * returns `indices = [0,1,3,4]`, `vertex_count = 4` — identical to the
 * native `execute_geometry_tool_json` test.
 */
export function geometry_convex_hull_2(val: any): any;

/**
 * Compute the Delaunay triangulation of a 2-D point set.
 */
export function geometry_delaunay_2(val: any): any;

/**
 * Execute any geometry tool via the JSON boundary — same function as
 * `execute_geometry_tool_json` on native. This is the full op surface
 * (`orientation_2`, `convex_hull_2`, `triangle_topology`, `mesh_topology`,
 * `delaunay_2`, `voronoi_2`, `nearest_site`).
 */
export function geometry_execute_json(args: string): string;

/**
 * Find the nearest site to a query point (brute-force).
 *
 * Returns the index of the nearest site, or -1 if the point set is empty.
 */
export function geometry_nearest_site(points: Float64Array, qx: number, qy: number): number;

/**
 * Robust 2-D orientation predicate.
 *
 * Returns `"clockwise"`, `"collinear"`, or `"counter_clockwise"` —
 * identical to the native `orientation_2` sign.
 */
export function geometry_orientation_2(ax: number, ay: number, bx: number, by: number, cx: number, cy: number): string;

/**
 * Numeric orientation sign (-1, 0, 1) for machine consumption.
 */
export function geometry_orientation_2_sign(ax: number, ay: number, bx: number, by: number, cx: number, cy: number): number;

/**
 * Compute the Voronoi diagram of a 2-D point set.
 */
export function geometry_voronoi_2(val: any): any;

export function geosparql_operation_wasm(json: string): any;

/**
 * Stable cold-path receipt for the resident browser execution plan.
 */
export function getBrowserExecutionReceipt(): any;

/**
 * Crate semver baked in at compile time.
 */
export function getEngineVersion(): string;

/**
 * Diagnostic: vocab size of the resident model tokenizer (0 if engine empty / parse failed).
 * Used by the online demo to show "garbage risk" before generate.
 */
export function getResidentTokenizerVocab(): number;

/**
 * Name of the selected resident browser backend.
 */
export function getWasmBackend(): string;

/**
 * Current stage of WebGPU engine init (empty when idle/done). Poll from JS so the
 * status line advances on phones while pipelines/weights load.
 */
export function getWebgpuInitStatus(): string;

/**
 * Structured engine metadata for browser UIs and diagnostics.
 */
export function get_engine_info(): any;

/**
 * Returns the qualia-core-db crate version baked in at compile time (matches daemon `/health`).
 */
export function get_engine_version(): string;

/**
 * Machine-readable SHACL capability and constraint coverage manifest (QW-05).
 */
export function get_shacl_capability_manifest_wasm(): any;

/**
 * Fuzzy RDF graph similarity (Ma, Li & Ma) — degree-aware Jaccard and Dice over two
 * sets of weighted triples. Terms are interned term ids (non-negative integers);
 * degrees are membership values in `[0,1]`. Two empty graphs are defined as 1.0.
 *
 * Input `{ g1:[[s,p,o,degree],..], g2:[[s,p,o,degree],..] }` ->
 * `{ jaccard, dice }`.
 */
export function graph_fuzzy_similarity(val: any): any;

/**
 * Knowledge-graph link prediction: score a set of candidate tails for a fixed
 * (head, relation) under TransE / DistMult / ComplEx / RotatE and rank them by
 * plausibility (higher = better). Input
 * `{ model, head:[f64], relation:[f64], candidates:[[f64],…], p?, top_k? }` →
 * `{ model, rank, ranking:[{index, score}] }` sorted best-first.
 */
export function graph_kge_predict(val: any): any;

/**
 * Knowledge-graph embedding plausibility score for a single triple
 * `(head, relation, tail)` under one of the four embedding families. Higher = more
 * plausible (translational models return the negative distance). Vector layout by
 * model (rank `k`):
 * * `transe` / `distmult` — head, relation, tail are length `k`.
 * * `complex` — all three length `2k` (`[re(0..k), im(k..2k)]`).
 * * `rotate` — head/tail length `2k` (`[re, im]`); relation length `k` (phase angles).
 *
 * `k` is inferred from the vector lengths; mismatched lengths fail closed.
 *
 * Input `{ model, head:[f64], relation:[f64], tail:[f64], p? }` -> `{ score, model,
 * rank }`. `p` (1 or 2) is the TransE norm order (default 2); ignored by other models.
 */
export function graph_kge_score(val: any): any;

/**
 * Single-source single-target shortest path over a directed, non-negative weighted
 * graph (Dijkstra, the engine's exact reference). The distance comes straight from
 * `solvers::graph_opt::dijkstra`; the node sequence is reconstructed by backtracking
 * on that distance field (`dist[u] + w == dist[v]`), so the math stays owned by the
 * solver.
 *
 * Input `{ edges:[[u,v,w],..], source, target, n? }` ->
 * `{ distance, reachable, path:[node,..] }` (path empty and reachable=false when
 * `target` is unreachable; `distance` is then null).
 */
export function graph_shortest_path(val: any): any;

/**
 * Spreading activation (Kornai, *Vector Semantics*) — propagate activation from seed
 * concepts through directed weighted edges, decaying each hop and pruning below a
 * threshold. Returns per-node total activation and a top-k relevance ranking.
 *
 * Input `{ edges:[[u,v,w],..], seeds:[[node,activation],..], decay, threshold?,
 * max_hops?, top_k?, n? }` -> `{ activation:[f64;n], ranking:[node,..] }`.
 */
export function graph_spreading_activation(val: any): any;

export function heart_rate_turtle_from_csv(content: string): string;

/**
 * Frozen host ABI stamp (`vibe-host-0.1`).
 */
export function host_version(): string;

/**
 * Phase 2B: async WebGPU decode — yields to the browser event loop on every `map_async`.
 * Returns a JS `Promise`; use `await inferWasmAsync(...)` from module code.
 */
export function inferWasmAsync(prompt: string, on_token: Function): Promise<string>;

/**
 * Async WebGPU decode with an explicit, bounded token budget and exact token count.
 *
 * This is the benchmark-safe API: callers can compare engines at the same decode
 * budget without estimating model tokens from whitespace.
 */
export function inferWasmAsyncMeasured(prompt: string, max_tokens: number, on_token: Function): Promise<any>;

/**
 * Stream token deltas to `on_token` (UTF-8 string chunks) while decoding.
 */
export function inferWasmStreaming(prompt: string, on_token: Function): Promise<string>;

/**
 * Streaming inference that additionally accepts grounded chat-graph thread
 * context (e.g. from `format_thread_context_to`), injected as a typed
 * `RequestPart::chat_graph` ahead of the user prompt.
 */
export function inferWasmStreamingWithChatGraph(prompt: string, graph_context: string, chat_graph_context: string | null | undefined, on_token: Function): Promise<string>;

/**
 * Streaming inference with optional graph context for provenance hashing.
 */
export function inferWasmStreamingWithContext(prompt: string, graph_context: string, on_token: Function): Promise<string>;

/**
 * Same as `infer_wasm` but accepts optional graph-context bytes for provenance hashing.
 */
export function inferWasmWithContext(prompt: string, graph_context: string): Promise<string>;

/**
 * Run autoregressive inference (non-streaming). Prompt must include any chat template tokens.
 */
export function infer_wasm(prompt: string): Promise<string>;

/**
 * Construct the volumetric renderer on the shared WebGPU device (no canvas).
 */
export function init_offscreen_renderer(width: number, height: number, particle_cap: number): Promise<void>;

export function init_panic_hook(): void;

export function init_panic_hook_shell(): void;

/**
 * Create the process-wide WebGPU device used by graph accel, offscreen
 * rendering, and LLM decode. Idempotent.
 */
export function init_shared_webgpu(): Promise<void>;

/**
 * Arm the WebGPU path. Await once before constructing the portal; on
 * `false`/throw the portal keeps its canvas2d (tier-1) fallback.
 */
export function init_webgpu(canvas: HTMLCanvasElement): Promise<boolean>;

/**
 * Prepare Qualia's independent CPU-WASM engine without creating a GPU adapter,
 * device, pipeline, or buffer. WebGPU remains an optional faster backend.
 */
export function initializeCpuWasmEngine(model_data: Uint8Array): Promise<void>;

/**
 * Prepare CPU-WASM with an explicit LLM context allocation. This memory domain
 * is independent from the 42 MiB semantic/SLG Sentinel arena.
 */
export function initializeCpuWasmEngineWithContext(model_data: Uint8Array, max_context: number): Promise<void>;

/**
 * Load a GGUF or P64 model into the resident browser WebGPU engine.
 *
 * Single-argument ABI (stable for demos). Progress stages are published via
 * [`get_webgpu_init_status`] while this future runs; the implementation yields to
 * the browser between major phases so the status line can update.
 */
export function initialize_webgpu_engine(model_data: Uint8Array): Promise<void>;

/**
 * Intercepts heavy computational opcodes and constructs a WASM offload intent.
 */
export function intercept_computational_opcode(opcode: number, payload_size: number): WasmOffloadIntent | undefined;

export function intercept_pharmacogenomics_intent(smiles: string): WasmOffloadIntent;

/**
 * True when either first-party Qualia browser backend has a resident model.
 */
export function isWasmEngineReady(): boolean;

/**
 * Returns true when a GGUF or P64 model has been loaded via `initialize_webgpu_engine`.
 */
export function isWebgpuEngineReady(): boolean;

/**
 * Check whether a SuperBlock is cached in the OPFS vault.
 * Returns `true` if the `.qblk` file exists, `false` otherwise.
 */
export function is_opfs_block_cached(block_index: number): Promise<boolean>;

/**
 * Return the pinned Qualia JSON-LD 1.1 context + SHA-256 digest (UE-012).
 *
 * Packages should embed `context` and record `digest` on receipts — do not
 * fetch remote `@context` URLs at admission time.
 */
export function jsonld_context_digest_wasm(): any;

/**
 * Hohfeld correlative position for a jural opcode.
 */
export function jural_correlative_wasm(val: any): any;

/**
 * Determinant of a square matrix via LU (partial pivoting).
 * Input `{ rows, cols, data }` (rows==cols) → `{ determinant }`.
 */
export function la_determinant_wasm(val: any): any;

/**
 * Symmetric eigendecomposition (cyclic Jacobi). Input `{ rows, cols, data }`
 * (square, symmetric) → `{ eigenvalues:[..], eigenvectors:{rows,cols,data} }`
 * where eigenvector `j` is column `j` of the row-major `eigenvectors` matrix.
 */
export function la_eigen_symmetric_wasm(val: any): any;

/**
 * General (non-symmetric) eigenvalues via the characteristic polynomial.
 * Input `{ rows, cols, data }` (square) → `{ eigenvalues:[{re,im}] }`.
 */
export function la_eigenvalues_wasm(val: any): any;

/**
 * `C = A · B`. Input `{ a:{rows,cols,data}, b:{rows,cols,data} }`,
 * output `{ rows, cols, data }`. Errors on a shape mismatch (`a.cols != b.rows`).
 */
export function la_matmul_wasm(val: any): any;

/**
 * All complex roots of a real polynomial (Durand–Kerner). Input
 * `{ coeffs:[cₙ,…,c₁,c₀] }` (descending) → `{ degree, roots:[{re,im}] }`.
 */
export function la_polynomial_roots_wasm(val: any): any;

/**
 * Solve `A · x = b` for a square `A` via LU. Input `{ a:{rows,cols,data}, b:[..] }`
 * (b length == a.rows) → `{ x:[..] }`. Errors if `A` is singular.
 */
export function la_solve_wasm(val: any): any;

/**
 * Thin SVD `A = U·Σ·Vᵀ`. Input `{ rows, cols, data }` →
 * `{ singular_values:[..], u:{rows,cols,data}, v:{rows,cols,data} }`
 * (`u` is m×n, `v` is n×n; singular vectors are columns; values descending).
 */
export function la_svd_wasm(val: any): any;

/**
 * Transpose. Input `{ rows, cols, data }` → output `{ rows:cols, cols:rows, data }`.
 */
export function la_transpose_wasm(val: any): any;

/**
 * Get the VibeScript language version string.
 */
export function language_version(): string;

/**
 * Capability names available in this WASM build.
 */
export function list_capabilities_wasm(): any;

export function list_hmc_bundle_entries_wasm(bundle_bytes: Uint8Array): any;

/**
 * Names that are intentionally native-only (UE-035/044/045) — never stubbed in browser.
 */
export function list_native_only_capabilities_wasm(): any;

/**
 * Airy functions `Ai(x)` and `Bi(x)` (both, from one Maclaurin-series evaluation).
 * Input `{ x }` -> `{ ai, bi }`.
 */
export function num_airy_wasm(val: any): any;

/**
 * Euler's totient `phi(n)`, the Mobius `mu(n)`, divisor count `d(n)` and divisor sum
 * `sigma(n)` — the classic multiplicative arithmetic functions, all from the prime
 * factorization. Input `{ n }` -> `{ totient, mobius, divisor_count, divisor_sum }`.
 */
export function num_arithmetic_functions_wasm(val: any): any;

/**
 * Modified Bessel function of the first kind `I_n(x)`, integer order. Defined for all
 * real `x`. Input `{ n, x }` -> `{ value }`.
 */
export function num_bessel_i_wasm(val: any): any;

/**
 * Bessel function of the first kind `J_n(x)`, integer order (any sign), defined for all
 * real `x`. Input `{ n, x }` -> `{ value }`.
 */
export function num_bessel_j_wasm(val: any): any;

/**
 * Modified Bessel function of the second kind `K_n(x)`, integer order `n >= 0`. Requires
 * `x > 0`. Input `{ n, x }` -> `{ value }`; errors for `x <= 0`.
 */
export function num_bessel_k_wasm(val: any): any;

/**
 * Bessel function of the second kind `Y_n(x)`, integer order `n >= 0`. Requires `x > 0`
 * (singular at the origin) and `J_0(x) != 0`. Input `{ n, x }` -> `{ value }`; errors
 * for `x <= 0` or an ill-posed Wronskian solve.
 */
export function num_bessel_y_wasm(val: any): any;

/**
 * Binomial coefficient `C(n, k)` (exact integer at every step). Result is returned as a
 * decimal **string** since it may exceed `f64`/`u53` precision. Errors (fail closed) on
 * `u128` overflow. Input `{ n, k }` -> `{ value }` (value is a string).
 */
export function num_binomial_wasm(val: any): any;

/**
 * The `n`-th Catalan number, plus the Stirling numbers `S(n,k)` (second kind) and
 * `c(n,k)` (unsigned first kind). All exact integers as decimal **strings**; errors
 * (fail closed) on `u128` overflow. Input `{ n, k }` ->
 * `{ catalan, stirling_second, stirling_first }`.
 */
export function num_combinatorics_wasm(val: any): any;

/**
 * Natural cubic spline through `(xs, ys)` (xs strictly increasing), evaluated at each
 * query in `queries`. Errors on insufficient data, unsorted/duplicate nodes, or a
 * singular tridiagonal system. Input `{ xs:[..], ys:[..], queries:[..] }` ->
 * `{ values:[..] }`.
 */
export function num_cubic_spline_wasm(val: any): any;

/**
 * All positive divisors of `n`, ascending. Input `{ n }` -> `{ divisors:[..] }`.
 */
export function num_divisors_wasm(val: any): any;

/**
 * Factorial `n!` as an exact integer (decimal **string**; `f64` cannot hold it).
 * Errors (fail closed) for `n >= 35` (`35!` overflows `u128`).
 * Input `{ n }` -> `{ value }` (value is a string).
 */
export function num_factorial_wasm(val: any): any;

/**
 * Greatest common divisor and least common multiple of `a` and `b`.
 * Input `{ a, b }` -> `{ gcd, lcm }`.
 */
export function num_gcd_lcm_wasm(val: any): any;

/**
 * Deterministic Miller-Rabin primality test (exact for all `u64`).
 * Input `{ n }` -> `{ prime }`.
 */
export function num_is_prime_wasm(val: any): any;

/**
 * Evaluate the Lagrange interpolating polynomial through `(xs, ys)` at `x`. Errors on
 * empty/mismatched data or duplicate nodes. Input `{ xs:[..], ys:[..], x }` -> `{ value }`.
 */
export function num_lagrange_eval_wasm(val: any): any;

/**
 * Piecewise-linear interpolation of `(xs, ys)` (xs strictly increasing) at `x` (clamped
 * to the endpoints outside the range). Input `{ xs:[..], ys:[..], x }` -> `{ value }`.
 */
export function num_linear_interp_wasm(val: any): any;

/**
 * Minimize a built-in benchmark objective with the Nelder-Mead simplex method
 * (derivative-free, deterministic, zero-allocation `[f64; 4]` simplex).
 *
 * `objective` is one of `"sphere" | "rosenbrock" | "booth" | "matyas" | "sum_abs"`.
 * `start` is the initial 4-D point (missing components default to 0, extras ignored).
 * `max_iterations` (optional, default 1000) and `tolerance` (optional, default 1e-6)
 * configure the solver. Input
 * `{ objective, start:[..], max_iterations?, tolerance? }` ->
 * `{ best_point:[4], best_value, iterations, converged }`. Errors on an unknown objective.
 */
export function num_minimize_wasm(val: any): any;

/**
 * Modular multiplicative inverse: the `x` with `a*x ≡ 1 (mod m)`. Errors (fail closed)
 * when `gcd(a, m) != 1`. Input `{ a, m }` -> `{ inverse }`.
 */
export function num_mod_inverse_wasm(val: any): any;

/**
 * `(base^exp) mod modulus` by repeated squaring (overflow-safe via `u128`).
 * Input `{ base, exp, modulus }` -> `{ value }`.
 */
export function num_mod_pow_wasm(val: any): any;

/**
 * Newton divided-difference interpolation: build the coefficients from `(xs, ys)` and
 * evaluate the interpolant at `x`. Input `{ xs:[..], ys:[..], x }` ->
 * `{ value, coefficients:[..] }`.
 */
export function num_newton_eval_wasm(val: any): any;

/**
 * Smallest prime strictly greater than `n`. Input `{ n }` -> `{ next_prime }`.
 */
export function num_next_prime_wasm(val: any): any;

/**
 * Classical orthogonal polynomial `P_n(x)` by three-term recurrence. `kind` is one of
 * `"legendre" | "chebyshev_t" | "chebyshev_u" | "hermite" | "laguerre"`.
 * Input `{ kind, n, x }` -> `{ value }`; errors on an unknown kind.
 */
export function num_orthopoly_wasm(val: any): any;

/**
 * Number of integer partitions `p(n)` (ways to write `n` as an unordered sum of positive
 * integers). Input `{ n }` -> `{ value }`.
 */
export function num_partitions_wasm(val: any): any;

/**
 * Least-squares polynomial fit of degree `degree` to `(xs, ys)` (via the normal
 * equations). Returns coefficients in **ascending** order `[c0, c1, ..., c_degree]` (so
 * the polynomial is `sum c_k x^k`). Optionally evaluates the fit at each `queries` value.
 * Errors on too few points, `degree + 1 > n`, or a singular system.
 * Input `{ xs:[..], ys:[..], degree, queries?:[..] }` ->
 * `{ coefficients:[..], values:[..] }`.
 */
export function num_poly_fit_wasm(val: any): any;

/**
 * Prime factorization (trial division then Pollard's rho), correct across all `u64`.
 * Input `{ n }` -> `{ factors:[{ prime, exponent }] }`. Empty for `n < 2`.
 */
export function num_prime_factorize_wasm(val: any): any;

/**
 * Riemann zeta function `zeta(s)` for real `s > 1` (Euler-Maclaurin). Input `{ s }` ->
 * `{ value }`; errors for `s <= 1` (needs analytic continuation, out of this domain).
 */
export function num_zeta_wasm(val: any): any;

export function ode_solver(input_json: string): string;

/**
 * Multiple OLS. Input `{ x:number[][], y:number[], fit_intercept?:bool }` →
 * coefficients, SE, t, p, R², adj-R², F, residuals, fitted, n, k.
 */
export function ols_multiple_wasm(val: any): any;

export function organic_chemistry(input_json: string): string;

/**
 * Current P64 container version for OPFS cache invalidation.
 */
export function p64FormatVersion(): number;

/**
 * Package intact game assets through Qualia's transparent QBDL/HMC writer.
 * The application owns entry names and its manifest; Qualia owns the format,
 * checksums, alignment, and reader verification.
 */
export function pack_game_hmc(entries: Array<any>, manifest_json: string): Uint8Array;

/**
 * Pack raw NQuin field bytes into a fully-structured SuperBlock with correct ECC parity.
 *
 * `raw_quin_bytes` must be `N × 48` bytes where each 48-byte chunk contains the
 * five semantic `u64` fields (40 bytes) followed by 8 placeholder bytes (ignored —
 * ECC is computed here). `N` must not exceed `QUINS_PER_BLOCK` (850).
 *
 * Returns exactly `BLOCK_MULTIPLIER_SIZE` (40 960) bytes, ready to write to OPFS.
 * This is the canonical packing path — **the JS ingest worker must call this**
 * instead of reimplementing the SuperBlock layout in JavaScript.
 */
export function pack_quins_into_superblock(seq_id: bigint, owner_did: bigint, raw_quin_bytes: Uint8Array): Uint8Array;

/**
 * Package exposure receipt: context + shapes + vibe AST digests (UE-053).
 */
export function package_exposure_manifest_wasm(shapes_json: string, vibe_program_cbor?: Uint8Array | null): any;

export function parse_cbor_ld_wasm(payload: Uint8Array): any;

/**
 * Parse a cell expression (`= expr`).
 * Returns `{ ok: true, ast: ... }` or `{ ok: false, error: ... }`.
 */
export function parse_cell_src(src: string): any;

export function parse_csv_wasm(val: any): any;

export function parse_heart_rate_csv_json(content: string): any;

export function parse_json_mapping_wasm(val: any): any;

export function parse_json_wasm(payload: string): any;

/**
 * Parse JSON-LD 1.1 text into packed quins (Civics primary semantic format).
 *
 * Profile: `application/ld+json`. Context must be embedded/pinned by the caller;
 * this binding does not fetch remote contexts.
 */
export function parse_jsonld_wasm(payload: string): any;

export function parse_n3logic_wasm(payload: string): any;

/**
 * Parse a full VibeScript program (module).
 */
export function parse_program_src(src: string): any;

/**
 * Parse an RDF document by Solid/LDP Content-Type (or Qualia format id).
 */
export function parse_rdf_document_wasm(content_type: string, payload: string): any;

export function parse_sleep_csv_json(content: string): any;

export function parse_steps_csv_json(content: string): any;

export function parse_turtle_wasm(payload: string): any;

export function parse_weight_csv_json(content: string): any;

/**
 * Parse and compile a yaml-ld-q42 document (workspace pages or HCF HypermediaDocument) into quins and lexicon.
 */
export function parse_yaml_ld_q42_wasm(source: string, namespace?: bigint | null, lamport?: bigint | null): any;

export function pinned_qualiadb_revision(): string;

export function plan_device_storage_wasm(val: any): any;

/**
 * Page-side timeout. Stops a late surface from being adopted. If the canvas
 * was already claimed, [`portal_webgpu_canvas_claimed`] is true and the page
 * must replace that element before the proof tick.
 */
export function portal_abort_webgpu(): void;

/**
 * Bind a hardware WebGL2 Anatomy renderer before `QualiaPortal` construction.
 * This is selected only after capability probing proves that WebGPU has no
 * usable adapter and WebGL2 context creation succeeds.
 */
export function portal_init_webgl2(canvas: HTMLCanvasElement): boolean;

/**
 * Create the WebGPU device + surface asynchronously and stash it for the render loop to adopt.
 * JS calls this **once, awaited**, right after constructing the portal and **before** the render
 * loop starts — the canvas must still be context-free (no 2d context yet) so the WebGPU surface
 * can bind to it. Returns `true` if the GPU path is now armed; on `false`/throw the portal keeps
 * the canvas2d fallback.
 */
export function portal_init_webgpu(canvas: HTMLCanvasElement): Promise<boolean>;

/**
 * True once WebGPU has called `getContext("webgpu")` on the init canvas.
 * A 2d tick on that same element cannot paint; replace it.
 */
export function portal_webgpu_canvas_claimed(): boolean;

export function predict_receptor_binding_wasm(): number;

/**
 * Project a VibeScript program source to canonical form.
 * Parses the source, then re-projects it from the AST.
 * This is the core of projectional authoring: structure → text.
 */
export function project_source(src: string): any;

/**
 * Performs topological pruning and validates meshes prior to physics offloading
 */
export function prune_and_validate_mesh(mesh_id: bigint): boolean;

/**
 * Historical export retained for browser compatibility. Returns P64_VERSION.
 */
export function q42FormatVersion(): number;

/**
 * RDFC-1.0 graph hash — **honest fail-closed** until a conforming implementation ships (UE-013).
 *
 * Never returns a digest labelled as RDFC-1.0. Optionally includes a
 * `provisional_spo_sha256` under profile `qualia:provisional-spo-sha256-v1`
 * for scaffolding only.
 */
export function rdfc10_graph_hash_wasm(val: any): any;

export function read_hmc_bundle_entry_wasm(bundle_bytes: Uint8Array, key: string): Uint8Array;

/**
 * Read a cached SuperBlock from the OPFS vault.
 *
 * Returns the raw 40 960 bytes as `Uint8Array`, or `null` if the block has not
 * been written yet (cache miss). Callers should fall back to an HTTP Range
 * request (see the JS `VFS` class) on cache miss.
 */
export function read_opfs_block(block_index: number): Promise<any>;

/**
 * Release resident model weights and tear down the WebGPU engine instance.
 */
export function releaseWebgpuEngine(): Promise<void>;

/**
 * Resolves two conflicting NQuin entries using Last-Writer-Wins semantics.
 * The Lamport clock is encoded in the metadata field; on ties, higher object wins.
 */
export function resolve_lww_wasm(local_val: any, remote_val: any): any;

/**
 * Route contradictions into an isolated context (`route_paraconsistent`).
 */
export function route_paraconsistent_wasm(val: any): any;

/**
 * Parse N3Logic rules. Rules are content; they validate and explain commands,
 * they never mutate state directly.
 */
export function rules_load_n3(n3: string): any;

/**
 * Compile a cell to bytecode, run it on the VM, and return the result.
 */
export function run_cell_bytecode(src: string): any;

/**
 * Compile a program to bytecode, encode it to binary, decode it, and run
 * a named function.  Demonstrates the full bytecode round-trip.
 */
export function run_program_bytecode(src: string, fn_name: string, args: any[]): any;

export function run_semantic_simulation(val: any): any;

export function sample_browser_telemetry_wasm(): any;

/**
 * Bounded stride sample of packed 48-byte Quins. Browser graphs cannot mmap
 * `.q42` files; this is the WASM-safe equivalent of `mmap_sample_quins`.
 */
export function sample_packed_quins_wasm(db_bytes: Uint8Array, max_quins: number): Uint8Array;

/**
 * Build the town from QualiaDB computational geometry, seal each coloured
 * mesh as `.10d`, and align semantic pick nodes with the portal projection.
 * JavaScript supplies only game state and displays the result.
 */
export function scene_build(upgrades: number, parts: number, online: boolean, approved: boolean, water_online: boolean, garden_active: boolean, bridge_open: boolean, bridge_braced: boolean, high_tide: boolean, pump_online: boolean, orchard_active: boolean, vibe_scene: string): any;

export function sequence_alignment(input_json: string): string;

export function serialize_csv_wasm(val: any): any;

/**
 * Continuous Mathematical Serialization into Float64Array
 */
export function serialize_float64_array(data: Float64Array): Float64Array;

/**
 * Packs an array of floats into a Uint8Array strictly typed buffer to avoid IEEE-754 truncation
 */
export function serialize_float_array(data: Float32Array): Uint8Array;

export function serialize_json_wasm(val: any): any;

/**
 * Serialize quins to RDF. `format` accepts Qualia ids (`turtle`, `jsonld`, `n3`)
 * or Solid MIME types (`text/turtle`, `application/ld+json`, `text/n3`).
 */
export function serialize_rdf_wasm(val: any): any;

/**
 * Replay a tape of accepted action ids against the seed and return the
 * resulting world text — the caller compares it to the live world to prove
 * determinism. Any rejected proposal marks the divergence point.
 */
export function session_replay(seed_n3: string, actions_json: string, event_ids: any): any;

/**
 * Simulates a GBM price path and returns the full series together with
 * min_price, max_price, and final_price.
 */
export function simulate_gbm_path_wasm(val: any): any;

export function sleep_turtle_from_csv(content: string): string;

/**
 * Negotiate Solid `Accept` → preferred RDF Content-Type.
 */
export function solid_negotiate_accept_wasm(accept: string): any;

/**
 * Solves dy/dt = -k·y via classical RK4, returning t_values, y_values, and final_y.
 */
export function solve_ode_exponential_decay_wasm(val: any): any;

/**
 * Bounded DPLL SAT solver.
 * Input: `{ clauses: [[1, 2, -3], [-1, 3], ...] }` (signed literal convention).
 * Output: `{ satisfiable: bool, assignment: { "1": true, "2": false, ... } }`
 */
export function solve_sat_wasm(val: any): any;

export function spatial_encode_wasm(json: string): any;

/**
 * One-way ANOVA F-test for equality of `k` group means. Input
 * `{ groups:[[..],[..],..] }` (≥ 2 groups, each non-empty, total > k) →
 * `{ f_statistic, p_value, df_between, df_within, ss_between, ss_within,
 * ms_between, ms_within }`. Errors on degenerate input.
 */
export function stats_anova_wasm(val: any): any;

/**
 * Breusch–Pagan. `{ residuals, x:number[][] }` (x = original predictors).
 */
export function stats_breusch_pagan_wasm(val: any): any;

/**
 * Pearson χ² goodness-of-fit test, `Σ(Oᵢ−Eᵢ)²/Eᵢ`, dof = k−1. Input
 * `{ observed:[..], expected:[..] }` (equal length ≥ 2, all expected > 0) →
 * `{ statistic, p_value, dof }`. Errors on length mismatch, len < 2, or a
 * non-positive expected count.
 */
export function stats_chi_square_gof_wasm(val: any): any;

/**
 * χ² test of independence on an R×C contingency table of counts. Input
 * `{ table:[[..],[..],..] }` (≥ 2 rows, ≥ 2 cols, rectangular, grand total > 0) →
 * `{ statistic, p_value, dof }` with `dof = (R−1)(C−1)`. Errors on a ragged or
 * undersized table.
 */
export function stats_chi_square_independence_wasm(val: any): any;

/**
 * χ² (chi-squared) distribution pdf/cdf at `x` with `k` degrees of freedom, plus
 * the upper-tail p-value. Input `{ x:f64, k:f64, p?:f64 }` (`k` > 0, `x` ≥ 0) →
 * `{ pdf, cdf, upper_p, quantile }`. `quantile` is the inverse-cdf at `p` when
 * supplied (0<p<1), else `null`.
 */
export function stats_chi_squared_dist_wasm(val: any): any;

/**
 * Chow structural break. `{ x, y, break_index }`.
 */
export function stats_chow_test_wasm(val: any): any;

/**
 * Pearson, Spearman, and Kendall correlation of two equal-length series, plus the
 * two-sided p-value for the Pearson coefficient. Input `{ x:[..], y:[..] }` →
 * `{ pearson, spearman, kendall, pearson_p_value }`. Each coefficient is `null`
 * when undefined (lengths differ, or n < 2); `pearson_p_value` is `null` for n < 3.
 */
export function stats_correlation_wasm(val: any): any;

/**
 * Full descriptive summary of a sample. Input `{ data:[..], sample?:bool }`
 * (`sample` defaults to `true` → Bessel-corrected variance/std) →
 * `{ n, sum, mean, variance, std_dev, min, max, median, q1, q3, skewness, kurtosis }`.
 * `variance`/`std_dev` are `null` when n < 2 in sample mode (no residual dof);
 * `skewness`/`kurtosis` are excess-kurtosis (Fisher) conventions.
 */
export function stats_describe_wasm(val: any): any;

/**
 * Durbin–Watson. `{ residuals:[..] }` → `{ statistic, approx_p_value }`.
 */
export function stats_durbin_watson_wasm(val: any): any;

/**
 * Fisher–Snedecor F-distribution: pdf and cdf at x with (d1, d2) degrees of
 * freedom, plus the inverse-cdf quantile when an optional `p` is supplied.
 * Input `{ x, d1, d2, p? }` → `{ pdf, cdf, quantile? }`.
 */
export function stats_fisher_f_wasm(val: any): any;

/**
 * Friedman test for k treatments across n blocks (e.g. classifiers × datasets).
 * Input `{ blocks:[[m1,…,mk], …] }` (each block length k, higher = better) →
 * `{ chi_square, chi_p_value, df, iman_davenport_f, f_p_value }`.
 */
export function stats_friedman_wasm(val: any): any;

/**
 * Influence (leverage / Cook / studentized). `{ x, y }` — flags only, never drops.
 */
export function stats_influence_wasm(val: any): any;

/**
 * Jarque–Bera on a residual vector. `{ residuals:[..] }` → `{ statistic, p_value, skewness, excess_kurtosis }`.
 */
export function stats_jarque_bera_wasm(val: any): any;

/**
 * LDA. `{ x:number[][], y:number[] (int class labels) }` → classes + predictions.
 */
export function stats_lda_wasm(val: any): any;

/**
 * Simple (one-predictor) OLS linear regression of `y` on `x`. Input
 * `{ x:[..], y:[..] }` (equal length, n ≥ 3, x not constant) →
 * `{ slope, intercept, r_squared, residual_std_error, slope_std_error, slope_t,
 * slope_p_value, intercept_std_error, intercept_p_value, n }`. Errors on length
 * mismatch, n < 3, or zero-variance `x`.
 */
export function stats_linear_regression_wasm(val: any): any;

/**
 * Binary logit. `{ x:number[][], y:number[] (0/1), fit_intercept?:bool }`.
 */
export function stats_logit_wasm(val: any): any;

/**
 * Mahalanobis outliers. `{ x:number[][], alpha?:number }`.
 */
export function stats_mahalanobis_outliers_wasm(val: any): any;

/**
 * McNemar's test for two paired binary classifiers. Input `{ b, c }` — the
 * discordant counts (b = first right / second wrong, c = first wrong / second
 * right) — → `{ statistic, p_value, dof }`. Continuity-corrected χ², dof 1.
 */
export function stats_mcnemar_wasm(val: any): any;

/**
 * Normal (Gaussian) distribution pdf/cdf/quantile at one point. Input
 * `{ x:f64, mu?:f64, sigma?:f64, p?:f64 }` (`mu` defaults 0, `sigma` defaults 1,
 * must be > 0) → `{ pdf, cdf, quantile }`. `pdf`/`cdf` are evaluated at `x`;
 * `quantile` is `Φ⁻¹(p)` when `p` is supplied (0<p<1), else `null`.
 */
export function stats_normal_wasm(val: any): any;

/**
 * One-sample t-test of the sample mean against `mu`. Input `{ data:[..], mu:f64 }`
 * → `{ t_statistic, p_value, degrees_of_freedom, ci_lower, ci_upper }`
 * (95% CI around the sample mean, t critical value). Errors if n < 2.
 */
export function stats_one_sample_t_wasm(val: any): any;

/**
 * Univariate outlier screen. `{ data:[..] }` → mean/median + 2σ/3σ indices.
 */
export function stats_outlier_screen_univariate_wasm(val: any): any;

/**
 * Paired t-test (one-sample t-test of the paired differences against 0). Input
 * `{ a:[..], b:[..] }` (equal length) → `{ t_statistic, p_value,
 * degrees_of_freedom, ci_lower, ci_upper }`. Errors if lengths differ or n < 2.
 */
export function stats_paired_t_wasm(val: any): any;

/**
 * Linear-interpolated quantile (numpy "linear" / R type-7). Input
 * `{ data:[..], q:0.0..1.0 }` → `{ quantile }`. `q` is clamped to `[0,1]`.
 */
export function stats_quantile_wasm(val: any): any;

/**
 * Ramsey RESET. `{ x, y, power_max?:number }`.
 */
export function stats_ramsey_reset_wasm(val: any): any;

/**
 * Residual runs test. `{ residuals:[..] }`.
 */
export function stats_residual_runs_wasm(val: any): any;

/**
 * Residual symmetry. `{ residuals:[..] }`.
 */
export function stats_residual_symmetry_wasm(val: any): any;

/**
 * Spurious-regression guard. `{ y, x?:number[] }`.
 */
export function stats_spurious_guard_wasm(val: any): any;

/**
 * Backward stepwise (exploratory). `{ x, y, exit_alpha?, max_steps? }`.
 */
export function stats_stepwise_backward_wasm(val: any): any;

/**
 * Student's t-distribution pdf/cdf at `t` with `nu` degrees of freedom, plus the
 * two-sided p-value. Input `{ t:f64, nu:f64, p?:f64 }` (`nu` > 0) →
 * `{ pdf, cdf, two_sided_p, quantile }`. `quantile` is the inverse-cdf at `p`
 * when supplied (0<p<1), else `null`.
 */
export function stats_students_t_wasm(val: any): any;

/**
 * Two-sample t-test of `mean(a) − mean(b) = 0`. Input
 * `{ a:[..], b:[..], equal_var?:bool }` (`equal_var` defaults to `false` → the
 * Welch test; `true` → pooled Student) → `{ t_statistic, p_value,
 * degrees_of_freedom, mean_difference, ci_lower, ci_upper }`. Errors if either
 * sample has n < 2.
 */
export function stats_two_sample_t_wasm(val: any): any;

/**
 * VIF per predictor column. `{ x:number[][] }` → `{ vif:[..] }`.
 */
export function stats_vif_wasm(val: any): any;

export function steps_turtle_from_csv(content: string): string;

/**
 * STIT: did agent bring about content?
 */
export function stit_brought_about_wasm(val: any): any;

/**
 * Build a pickable tensor buffer (32B header + N×40 `Tensor10D`) from the
 * same authored nodes — the surface `QualiaPortal::upload_tensor_buffer`
 * ingests for semantic picking. Groups of 10 floats as in
 * `asset_compile_10d`.
 */
export function tensor_buffer_build(nodes_flat: Float32Array): Uint8Array;

export function thermodynamics_mcmc(input_json: string): string;

/**
 * Series transform. `{ values, kind: "log"|"log1p"|"sqrt"|"square"|"reciprocal"|"exp" }`.
 */
export function transform_series_wasm(val: any): any;

/**
 * Look up a CODATA / SI-2019 physical constant by name, returning its value (in coherent
 * SI base units) and its physical dimension as the 7-vector.
 *
 * Input `{ name }` → `{ name, symbol, description, value, dimension:{..} }`.
 * Accepted names are those from `units_list_constants` (canonical name or symbol alias).
 */
export function units_constant(val: any): any;

/**
 * Convert a magnitude between two named units of the **same** physical dimension.
 * Affine (Celsius/Fahrenheit) and linear scales are both handled. Fails closed if the
 * units have different dimensions (e.g. `m` → `s`).
 *
 * Input `{ value, from, to }` → `{ value, from, to, dimension:{..} }`.
 */
export function units_convert(val: any): any;

/**
 * List every CODATA constant available to `units_constant`, with value, symbol,
 * description and dimension. Takes an empty object `{}`.
 * Input `{}` → `{ constants:[{name,symbol,description,value,dimension}] }`.
 */
export function units_list_constants(_val: any): any;

/**
 * List every unit the engine can convert between, with a human label and its dimension
 * 7-vector. Takes an empty object `{}`. Input `{}` → `{ units:[{symbol,label,dimension}] }`.
 */
export function units_list_units(_val: any): any;

/**
 * Multiply or divide two dimensioned quantities, composing their dimensions. Each
 * quantity is `{ value, unit }`; the unit string is resolved to its SI factor so the
 * result value is in coherent SI base units, and the result dimension is returned as the
 * 7-vector. `divide` fails closed on a zero divisor.
 *
 * Input `{ a:{value,unit}, b:{value,unit}, op:"multiply"|"divide" }`
 * → `{ value, dimension:{..} }`.
 */
export function units_quantity_op(val: any): any;

export function validate_fasta_wasm(val: any): any;

export function validate_fhir_observation_wasm(val: any): any;

/**
 * Evaluate a named policy constraint against a single quint (s,p,o,c,m).
 *
 * Supported constraint names:
 *   "cooperative_obligation" — PermissiveCommons work obligation gate (lane 1)
 *   "guardian_identity"      — BilateralMicroCommons guardian auth gate (lane 2)
 *   "commercial_block"       — BilateralMicroCommons anti-commercial gate (lane 2)
 *
 * Returns JSON: `{"passed":bool,"routingLane":N}`
 */
export function validate_health_quin(constraint: string, s: bigint, p: bigint, o: bigint, c: bigint, m: bigint): string;

/**
 * Validate a Turtle document against built-in health shapes (SPARQL ASK constraints).
 * Returns a JSON string: `{"valid":bool,"checked":N,"violations":[{"shape":"...","message":"..."}]}`
 */
export function validate_health_turtle(turtle: string): string;

export function validate_shacl_constraint_wasm(val: any): any;

/**
 * Validates raw packed 48-byte Quins against a list of JSON ShapeSpecs.
 */
export function validate_shacl_graph_wasm(db_bytes: Uint8Array, shapes_json: string): any;

/**
 * Full graph SHACL validation from N3/N-Triples data and JSON ShapeSpecs.
 * Returns the complete `ValidationReport` preserving conforms, focus node, path,
 * severity, and constraint component.
 */
export function validate_shacl_json_wasm(data_n3: string, shapes_json: string): any;

/**
 * Values abuse-check (agency.n3 G1/G1' personhood guard) — WASM surface for Civics.
 */
export function values_check_wasm(val: any): any;

/**
 * Consent non-coerced guard (`capacity::detect_duress` inverted).
 */
export function values_consent_non_coerced_wasm(val: any): any;

/**
 * Harm-below-ceiling guard (wasm-safe numeric; CAS marginal-harm stays native).
 */
export function values_harm_below_ceiling_wasm(val: any): any;

/**
 * Serialize vault biometric records (JSON array from wf-biometrics IDB store) → Turtle.
 */
export function vault_biometrics_to_turtle(json: string): string;

/**
 * Serialize vault diet log entries (JSON array from wf-dl IDB store) → Turtle.
 */
export function vault_diet_to_turtle(json: string): string;

/**
 * Serialize vault medication records (JSON array from wf-meds IDB store) → Turtle.
 */
export function vault_meds_to_turtle(json: string): string;

/**
 * Differential WebGPU/CPU probe for the first layer's Q projection.
 *
 * This is intentionally exposed to the browser debug surface so automated
 * agents can distinguish model/package failures from quantized-kernel
 * failures without asking a user to inspect opaque generated text.
 */
export function verifyFirstLayerQuant(): Promise<any>;

export function verify_backup_manifest_wasm(manifest: any, payload_sha256_hex: string): any;

/**
 * Validate a game pack using Qualia's reader and report its verified entry count.
 */
export function verify_game_hmc(bytes: Uint8Array): number;

/**
 * Verify a signed law package (JSON) against an Ed25519 public key.
 */
export function verify_law_package_wasm(json: string, public_key: Uint8Array): boolean;

/**
 * Multiple OLS + verification report with Civics calculation receipt.
 */
export function verify_regression_model_receipt_wasm(val: any): any;

/**
 * Full verification report + soft/hard flags. `{ x, y, alpha?, strict? }`.
 */
export function verify_regression_model_wasm(val: any): any;

/**
 * Validate ECC parity for every NQuin in a raw SuperBlock.
 *
 * Returns JSON: `{"valid":bool,"total":N,"bad":[indices...]}`
 * A non-empty `bad` array indicates sector corruption.
 */
export function verify_superblock_ecc(block_bytes: Uint8Array): string;

/**
 * Evaluate a VibeScript cell through the pinned host. Scripts propose and
 * evaluate through published capabilities only — no direct state mutation.
 */
export function vibe_eval_cell(src: string): any;

/**
 * Invoke a VibeScript host capability by id. Granted stdlib kernels
 * (`Econ.*`, `Statistics.*`, `PhysicalUnits.*`, …) evaluate; unknown or
 * ungranted ids fail closed — the boundary QG-04 requires.
 */
export function vibe_invoke(id: string, args_json: string): any;

export function wasm_convex_hull_2d(points_flat: Float64Array): Uint32Array;

export function wasm_delaunay_triangulation_2d(points_flat: Float64Array): Uint32Array;

/**
 * Polls the local Webizen for pending agreements waiting for the user's signature.
 */
export function webizen_poll_agreements(): string;

/**
 * Proposes a new M:N Guardianship agreement to the local WebRTC mesh.
 */
export function webizen_propose_agreement(_nominated_guardians: Array<any>, principal: string, domain: string, threshold: number): bigint;

/**
 * Signs a pending agreement, advancing its state machine and triggering WebRTC peer sync.
 */
export function webizen_sign_agreement(_agreement_id: bigint, _private_key_mock: string): void;

export function weight_turtle_from_csv(content: string): string;

/**
 * Load world bytes previously persisted through `world_save`.
 */
export function world_load(path: string): Promise<Uint8Array>;

/**
 * Parse a Turtle world description into the QualiaDB graph representation.
 * Returns the engine's parse receipt (triple list / errors) as a JS value.
 */
export function world_load_turtle(turtle: string): any;

/**
 * Persist world bytes via QualiaDB's OPFS-backed virtual filesystem.
 * `path` is currently a bare filename — nested paths are not supported by
 * `OpfsVfs` (recorded in the capability ledger).
 */
export function world_save(path: string, data: Uint8Array): Promise<void>;

/**
 * Write a SuperBlock to the OPFS vault at `block_index`.
 *
 * `block_bytes` must be exactly `BLOCK_MULTIPLIER_SIZE` (40 960) bytes — use
 * `pack_quins_into_superblock()` to produce correctly-structured blocks.
 *
 * File name: `block_XXXXXXXX.qblk` (zero-padded 8-digit decimal index).
 * Compatible with the naming convention used by the JS VFS class.
 */
export function write_opfs_block(block_index: number, block_bytes: Uint8Array): Promise<void>;

/**
 * Forward discrete Fourier transform `X[k] = Σ_n x[n] e^{-2πi kn/N}`
 * (un-normalized, forward sign convention). f64-exact CPU reference path.
 *
 * Input `{ data:[..] }` (real signal) OR `{ re:[..], im:[..] }` (complex signal).
 * Output `{ re:[..], im:[..], magnitude:[..], n }`.
 */
export function xform_dft(val: any): any;

/**
 * Inverse discrete Fourier transform `x[n] = (1/N) Σ_k X[k] e^{+2πi kn/N}`.
 * Round-trips `xform_dft` to ~1e-9.
 *
 * Input the spectrum as `{ re:[..], im:[..] }` (complex bins) OR `{ data:[..] }`
 * (real bins → imaginary parts taken as 0).
 * Output `{ re:[..], im:[..], magnitude:[..], n }` — the recovered samples.
 */
export function xform_idft(val: any): any;

/**
 * Numerical Laplace transform `L{f}(s) = ∫₀^∞ e^{-st} f(t) dt` by Simpson
 * quadrature, for a built-in time-function family (so a deterministic kernel
 * crosses the JS boundary instead of an arbitrary closure):
 * * `"one"`   → f(t)=1            (closed form 1/s)
 * * `"t"`     → f(t)=t            (1/s²)
 * * `"exp"`   → f(t)=e^{a·t}      (1/(s-a) for s>a)
 * * `"poly"`  → f(t)=tⁿ           (n!/s^{n+1}); supply `n`
 * * `"sin"`   → f(t)=sin(a·t)     (a/(s²+a²))
 * * `"cos"`   → f(t)=cos(a·t)     (s/(s²+a²))
 * `a` defaults to 1, `n` defaults to 1. Requires `s>0`, `t_max>0`, even `steps≥2`.
 *
 * Input `{ fn, s, t_max, steps, a?, n? }`. Output `{ value, s, t_max, steps }`.
 */
export function xform_laplace_numeric(val: any): any;

/**
 * Symbolic Laplace transform of a polynomial in `t` from the table the CAS can
 * represent: a sum of `coeff · t^power` terms (constants are `power = 0`).
 * Returns the resulting `Expr` in `s` as a pretty string and, when `s` is
 * supplied, its numeric value `L{f}(s)`. Fails closed (`NotTransformable`) on
 * anything outside constants / integer powers / their linear combinations.
 *
 * Input `{ terms:[{coeff, power}, ..], s? }`. Output `{ expr, value? }`.
 */
export function xform_laplace_table(val: any): any;

/**
 * Closed form of the geometric `aⁿ u[n]` Z-transform `X(z) = 1/(1 - a z^{-1})`
 * (valid for `|z| > |a|`). Fails closed where the denominator vanishes / at `z = 0`.
 *
 * Input `{ a, z_re, z_im }`. Output `{ re, im, magnitude }`.
 */
export function xform_z_geometric(val: any): any;

/**
 * Z-transform of a finite causal sequence evaluated at a complex point `z`:
 * `X(z) = Σ_{n=0}^{N-1} x[n] z^{-n}`. Fails closed at `z = 0`.
 *
 * Input `{ x:[..], z_re, z_im }`. Output `{ re, im, magnitude }`.
 */
export function xform_z_transform(val: any): any;

/**
 * Closed form of the unit-step `u[n]` Z-transform `X(z) = z/(z-1)`
 * (valid for `|z| > 1`). Fails closed at `z = 0` or `z = 1`.
 *
 * Input `{ z_re, z_im }`. Output `{ re, im, magnitude }`.
 */
export function xform_z_unit_step(val: any): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_gamehud_free: (a: number, b: number) => void;
    readonly __wbg_gameportal_free: (a: number, b: number) => void;
    readonly __wbg_gamesession_free: (a: number, b: number) => void;
    readonly action_validate_shacl: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly asset_compile_10d: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number, number, number];
    readonly asset_verify_10d: (a: number, b: number) => [number, number, number];
    readonly gamehud_focus_next: (a: number, b: number) => [number, number];
    readonly gamehud_focused_action: (a: number) => [number, number];
    readonly gamehud_hit_test: (a: number, b: number, c: number) => [number, number];
    readonly gamehud_new: (a: any) => [number, number, number];
    readonly gamehud_set_camera_target: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly gamehud_set_document_json: (a: number, b: number, c: number) => [number, number];
    readonly gameportal_load_10d: (a: number, b: number, c: number) => [number, number, number];
    readonly gameportal_load_scene: (a: number, b: any) => [number, number, number];
    readonly gameportal_new: (a: any) => [number, number, number];
    readonly gameportal_poll_pick: (a: number) => number;
    readonly gameportal_queue_pick: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly gameportal_set_camera: (a: number, b: number, c: number, d: number) => [number, number];
    readonly gameportal_set_camera_target: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
    readonly gameportal_set_sky_preset: (a: number, b: number) => void;
    readonly gameportal_tick: (a: number, b: any, c: number) => [number, number];
    readonly gameportal_tier: (a: number) => number;
    readonly gameportal_upload_tensor: (a: number, b: number, c: number) => [number, number];
    readonly gamesession_events: (a: number) => any;
    readonly gamesession_new: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly gamesession_propose: (a: number, b: number, c: number) => any;
    readonly gamesession_vibe_propose: (a: number, b: number, c: number) => any;
    readonly gamesession_vibe_query: (a: number, b: number, c: number) => any;
    readonly gamesession_world: (a: number) => [number, number];
    readonly init_panic_hook_shell: () => void;
    readonly init_webgpu: (a: any) => any;
    readonly pack_game_hmc: (a: any, b: number, c: number) => [number, number, number, number];
    readonly pinned_qualiadb_revision: () => [number, number];
    readonly rules_load_n3: (a: number, b: number) => any;
    readonly scene_build: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number) => [number, number, number];
    readonly session_replay: (a: number, b: number, c: number, d: number, e: any) => [number, number, number];
    readonly tensor_buffer_build: (a: number, b: number) => [number, number, number, number];
    readonly verify_game_hmc: (a: number, b: number) => [number, number, number];
    readonly vibe_eval_cell: (a: number, b: number) => any;
    readonly vibe_invoke: (a: number, b: number, c: number, d: number) => any;
    readonly world_load: (a: number, b: number) => any;
    readonly world_load_turtle: (a: number, b: number) => any;
    readonly world_save: (a: number, b: number, c: number, d: number) => any;
    readonly __wbg_compiledcell_free: (a: number, b: number) => void;
    readonly apply_structural_edit: (a: number, b: number, c: number, d: number) => any;
    readonly apply_structural_edits: (a: number, b: number, c: number, d: number) => any;
    readonly ast_schema_json: () => [number, number];
    readonly capability_invoke: (a: number, b: number, c: number, d: number) => any;
    readonly check_cell_src: (a: number, b: number) => any;
    readonly check_program_src: (a: number, b: number) => any;
    readonly compile_cell_bytecode: (a: number, b: number) => any;
    readonly compiledcell_code_size: (a: number) => number;
    readonly compiledcell_compile: (a: number, b: number) => [number, number, number];
    readonly compiledcell_constants: (a: number) => number;
    readonly compiledcell_disassembly: (a: number) => [number, number];
    readonly compiledcell_from_bytes: (a: number, b: number) => [number, number, number];
    readonly compiledcell_functions: (a: number) => number;
    readonly compiledcell_run: (a: number) => any;
    readonly compiledcell_top_locals: (a: number) => number;
    readonly decode_and_run: (a: number, b: number) => any;
    readonly decode_program_cborld_wasm: (a: number, b: number) => [number, number, number];
    readonly diagnose_src: (a: number, b: number) => any;
    readonly diagnostic_schema_json: () => [number, number];
    readonly ebnf_grammar: () => [number, number];
    readonly encode_cell_bytecode: (a: number, b: number) => any;
    readonly encode_program_cborld_wasm: (a: number, b: number) => [number, number, number];
    readonly eval_cell_json: (a: number, b: number) => any;
    readonly eval_cell_src: (a: number, b: number) => any;
    readonly eval_program_src: (a: number, b: number) => any;
    readonly eval_program_with_receipt_wasm: (a: number, b: number) => any;
    readonly gbnf_grammar: () => [number, number];
    readonly host_version: () => [number, number];
    readonly language_version: () => [number, number];
    readonly parse_cell_src: (a: number, b: number) => any;
    readonly parse_program_src: (a: number, b: number) => any;
    readonly project_source: (a: number, b: number) => any;
    readonly run_cell_bytecode: (a: number, b: number) => any;
    readonly run_program_bytecode: (a: number, b: number, c: number, d: number, e: number, f: number) => any;
    readonly align_sequences_wasm: (a: any) => [number, number, number];
    readonly causal_caused_wasm: (a: any) => [number, number, number];
    readonly check_subsumption_wasm: (a: any) => [number, number, number];
    readonly compute_molecular_descriptors_wasm: (a: any) => [number, number, number];
    readonly compute_reaction_metrics_wasm: (a: any) => [number, number, number];
    readonly compute_thermochemistry_wasm: (a: any) => [number, number, number];
    readonly detect_functional_groups_wasm: (a: any) => [number, number, number];
    readonly enumerate_stable_models_wasm: (a: any) => [number, number, number];
    readonly evaluate_deontic_wasm: (a: any) => [number, number, number];
    readonly evaluate_epistemic_wasm: (a: any) => [number, number, number];
    readonly evaluate_inference_guard_wasm: (a: any) => [number, number, number];
    readonly evaluate_lipinski_wasm: (a: any) => [number, number, number];
    readonly evaluate_ltl_trace_wasm: (a: any) => [number, number, number];
    readonly fuzzy_t_norm_wasm: (a: any) => [number, number, number];
    readonly jural_correlative_wasm: (a: any) => [number, number, number];
    readonly la_determinant_wasm: (a: any) => [number, number, number];
    readonly la_eigen_symmetric_wasm: (a: any) => [number, number, number];
    readonly la_eigenvalues_wasm: (a: any) => [number, number, number];
    readonly la_matmul_wasm: (a: any) => [number, number, number];
    readonly la_polynomial_roots_wasm: (a: any) => [number, number, number];
    readonly la_solve_wasm: (a: any) => [number, number, number];
    readonly la_svd_wasm: (a: any) => [number, number, number];
    readonly la_transpose_wasm: (a: any) => [number, number, number];
    readonly num_airy_wasm: (a: any) => [number, number, number];
    readonly num_arithmetic_functions_wasm: (a: any) => [number, number, number];
    readonly num_bessel_i_wasm: (a: any) => [number, number, number];
    readonly num_bessel_j_wasm: (a: any) => [number, number, number];
    readonly num_bessel_k_wasm: (a: any) => [number, number, number];
    readonly num_bessel_y_wasm: (a: any) => [number, number, number];
    readonly num_binomial_wasm: (a: any) => [number, number, number];
    readonly num_combinatorics_wasm: (a: any) => [number, number, number];
    readonly num_cubic_spline_wasm: (a: any) => [number, number, number];
    readonly num_divisors_wasm: (a: any) => [number, number, number];
    readonly num_factorial_wasm: (a: any) => [number, number, number];
    readonly num_gcd_lcm_wasm: (a: any) => [number, number, number];
    readonly num_is_prime_wasm: (a: any) => [number, number, number];
    readonly num_lagrange_eval_wasm: (a: any) => [number, number, number];
    readonly num_linear_interp_wasm: (a: any) => [number, number, number];
    readonly num_minimize_wasm: (a: any) => [number, number, number];
    readonly num_mod_inverse_wasm: (a: any) => [number, number, number];
    readonly num_mod_pow_wasm: (a: any) => [number, number, number];
    readonly num_newton_eval_wasm: (a: any) => [number, number, number];
    readonly num_next_prime_wasm: (a: any) => [number, number, number];
    readonly num_orthopoly_wasm: (a: any) => [number, number, number];
    readonly num_partitions_wasm: (a: any) => [number, number, number];
    readonly num_poly_fit_wasm: (a: any) => [number, number, number];
    readonly num_prime_factorize_wasm: (a: any) => [number, number, number];
    readonly num_zeta_wasm: (a: any) => [number, number, number];
    readonly route_paraconsistent_wasm: (a: any) => [number, number, number];
    readonly stats_anova_wasm: (a: any) => [number, number, number];
    readonly stats_chi_square_gof_wasm: (a: any) => [number, number, number];
    readonly stats_chi_square_independence_wasm: (a: any) => [number, number, number];
    readonly stats_chi_squared_dist_wasm: (a: any) => [number, number, number];
    readonly stats_correlation_wasm: (a: any) => [number, number, number];
    readonly stats_describe_wasm: (a: any) => [number, number, number];
    readonly stats_fisher_f_wasm: (a: any) => [number, number, number];
    readonly stats_friedman_wasm: (a: any) => [number, number, number];
    readonly stats_linear_regression_wasm: (a: any) => [number, number, number];
    readonly stats_mcnemar_wasm: (a: any) => [number, number, number];
    readonly stats_normal_wasm: (a: any) => [number, number, number];
    readonly stats_one_sample_t_wasm: (a: any) => [number, number, number];
    readonly stats_paired_t_wasm: (a: any) => [number, number, number];
    readonly stats_quantile_wasm: (a: any) => [number, number, number];
    readonly stats_students_t_wasm: (a: any) => [number, number, number];
    readonly stats_two_sample_t_wasm: (a: any) => [number, number, number];
    readonly stit_brought_about_wasm: (a: any) => [number, number, number];
    readonly validate_fasta_wasm: (a: any) => [number, number, number];
    readonly values_check_wasm: (a: any) => [number, number, number];
    readonly values_consent_non_coerced_wasm: (a: any) => [number, number, number];
    readonly values_harm_below_ceiling_wasm: (a: any) => [number, number, number];
    readonly predict_receptor_binding_wasm: () => number;
    readonly __wbg_qualiahud_free: (a: number, b: number) => void;
    readonly black_scholes_wasm: (a: any) => [number, number, number];
    readonly calculate_leontief_multipliers_wasm: (a: any) => [number, number, number];
    readonly calculate_welfare_metrics_wasm: (a: any) => [number, number, number];
    readonly clinical_risk: (a: number, b: number) => [number, number, number, number];
    readonly compute_ols_diagnostics_wasm: (a: any) => [number, number, number];
    readonly compute_pid_step_wasm: (a: any) => [number, number, number];
    readonly design_dummies_wasm: (a: any) => [number, number, number];
    readonly geometric_algebra_operation: (a: number, b: number) => [number, number, number, number];
    readonly geometry_convex_hull_2: (a: any) => [number, number, number];
    readonly geometry_delaunay_2: (a: any) => [number, number, number];
    readonly geometry_execute_json: (a: number, b: number) => [number, number, number, number];
    readonly geometry_nearest_site: (a: number, b: number, c: number, d: number) => number;
    readonly geometry_orientation_2: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
    readonly geometry_orientation_2_sign: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
    readonly geometry_voronoi_2: (a: any) => [number, number, number];
    readonly get_engine_info: () => [number, number, number];
    readonly get_engine_version: () => [number, number];
    readonly init_offscreen_renderer: (a: number, b: number, c: number) => any;
    readonly init_shared_webgpu: () => any;
    readonly list_capabilities_wasm: () => [number, number, number];
    readonly list_native_only_capabilities_wasm: () => [number, number, number];
    readonly ode_solver: (a: number, b: number) => [number, number, number, number];
    readonly ols_multiple_wasm: (a: any) => [number, number, number];
    readonly organic_chemistry: (a: number, b: number) => [number, number, number, number];
    readonly qualiahud_focus_next: (a: number, b: number) => [number, number];
    readonly qualiahud_focused_action: (a: number) => [number, number];
    readonly qualiahud_hit_test: (a: number, b: number, c: number) => [number, number];
    readonly qualiahud_new: (a: any) => [number, number, number];
    readonly qualiahud_paint: (a: number) => void;
    readonly qualiahud_set_camera_target: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly qualiahud_set_document_json: (a: number, b: number, c: number) => [number, number];
    readonly resolve_lww_wasm: (a: any, b: any) => [number, number, number];
    readonly run_semantic_simulation: (a: any) => [number, number, number];
    readonly sequence_alignment: (a: number, b: number) => [number, number, number, number];
    readonly simulate_gbm_path_wasm: (a: any) => [number, number, number];
    readonly solve_ode_exponential_decay_wasm: (a: any) => [number, number, number];
    readonly solve_sat_wasm: (a: any) => [number, number, number];
    readonly stats_breusch_pagan_wasm: (a: any) => [number, number, number];
    readonly stats_chow_test_wasm: (a: any) => [number, number, number];
    readonly stats_durbin_watson_wasm: (a: any) => [number, number, number];
    readonly stats_influence_wasm: (a: any) => [number, number, number];
    readonly stats_jarque_bera_wasm: (a: any) => [number, number, number];
    readonly stats_lda_wasm: (a: any) => [number, number, number];
    readonly stats_logit_wasm: (a: any) => [number, number, number];
    readonly stats_mahalanobis_outliers_wasm: (a: any) => [number, number, number];
    readonly stats_outlier_screen_univariate_wasm: (a: any) => [number, number, number];
    readonly stats_ramsey_reset_wasm: (a: any) => [number, number, number];
    readonly stats_residual_runs_wasm: (a: any) => [number, number, number];
    readonly stats_residual_symmetry_wasm: (a: any) => [number, number, number];
    readonly stats_spurious_guard_wasm: (a: any) => [number, number, number];
    readonly stats_stepwise_backward_wasm: (a: any) => [number, number, number];
    readonly stats_vif_wasm: (a: any) => [number, number, number];
    readonly thermodynamics_mcmc: (a: number, b: number) => [number, number, number, number];
    readonly transform_series_wasm: (a: any) => [number, number, number];
    readonly verify_law_package_wasm: (a: number, b: number, c: number, d: number) => number;
    readonly verify_regression_model_receipt_wasm: (a: any) => [number, number, number];
    readonly verify_regression_model_wasm: (a: any) => [number, number, number];
    readonly exact_bigint_add: (a: any) => [number, number, number];
    readonly exact_bigint_divmod: (a: any) => [number, number, number];
    readonly exact_bigint_factorial: (a: any) => [number, number, number];
    readonly exact_bigint_gcd: (a: any) => [number, number, number];
    readonly exact_bigint_mul: (a: any) => [number, number, number];
    readonly exact_bigint_pow: (a: any) => [number, number, number];
    readonly exact_rational_add: (a: any) => [number, number, number];
    readonly exact_rational_mul: (a: any) => [number, number, number];
    readonly units_constant: (a: any) => [number, number, number];
    readonly units_convert: (a: any) => [number, number, number];
    readonly units_list_constants: (a: any) => [number, number, number];
    readonly units_list_units: (a: any) => [number, number, number];
    readonly units_quantity_op: (a: any) => [number, number, number];
    readonly graph_fuzzy_similarity: (a: any) => [number, number, number];
    readonly graph_kge_predict: (a: any) => [number, number, number];
    readonly graph_kge_score: (a: any) => [number, number, number];
    readonly graph_shortest_path: (a: any) => [number, number, number];
    readonly graph_spreading_activation: (a: any) => [number, number, number];
    readonly xform_dft: (a: any) => [number, number, number];
    readonly xform_idft: (a: any) => [number, number, number];
    readonly xform_laplace_numeric: (a: any) => [number, number, number];
    readonly xform_laplace_table: (a: any) => [number, number, number];
    readonly xform_z_geometric: (a: any) => [number, number, number];
    readonly xform_z_transform: (a: any) => [number, number, number];
    readonly xform_z_unit_step: (a: any) => [number, number, number];
    readonly __wbg_webengine_free: (a: number, b: number) => void;
    readonly create_canvas: (a: number, b: number) => [number, number, number];
    readonly init_panic_hook: () => void;
    readonly webengine_last_parsed: (a: number) => any;
    readonly webengine_load_json_scene: (a: number, b: number, c: number) => [number, number, number];
    readonly webengine_load_q42: (a: number, b: number, c: number) => [number, number, number];
    readonly webengine_mount_qapp: (a: number, b: number, c: number) => [number, number];
    readonly webengine_new: () => [number, number, number];
    readonly webengine_render_to_canvas: (a: number) => [number, number];
    readonly __wbg_federatednodemanager_free: (a: number, b: number) => void;
    readonly __wbg_get_wasmoffloadintent_opcode: (a: number) => number;
    readonly __wbg_get_wasmoffloadintent_payload_size: (a: number) => number;
    readonly __wbg_get_wasmoffloadintent_priority: (a: number) => number;
    readonly __wbg_set_wasmoffloadintent_opcode: (a: number, b: number) => void;
    readonly __wbg_set_wasmoffloadintent_payload_size: (a: number, b: number) => void;
    readonly __wbg_set_wasmoffloadintent_priority: (a: number, b: number) => void;
    readonly __wbg_wasmoffloadintent_free: (a: number, b: number) => void;
    readonly __wbg_wasmq42session_free: (a: number, b: number) => void;
    readonly __wbg_wasmsimulationworld_free: (a: number, b: number) => void;
    readonly cas_differentiate_wasm: (a: any) => [number, number, number];
    readonly cas_evaluate_wasm: (a: any) => [number, number, number];
    readonly cas_expand_wasm: (a: any) => [number, number, number];
    readonly cas_factor_wasm: (a: any) => [number, number, number];
    readonly cas_simplify_wasm: (a: any) => [number, number, number];
    readonly cas_solve_quadratic_wasm: (a: any) => [number, number, number];
    readonly compileGgufToP64: (a: any, b: number) => [number, number, number];
    readonly compile_query_to_json: (a: number, b: number) => [number, number];
    readonly compile_shacl_turtle_wasm: (a: number, b: number) => [number, number, number];
    readonly device_storage_policy_wasm: () => [number, number, number];
    readonly enforce_rights_ontology: (a: bigint) => number;
    readonly estimate_browser_storage: () => any;
    readonly execute_ntriples_query: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly federatednodemanager_discover_capabilities: (a: number) => number;
    readonly federatednodemanager_new: () => number;
    readonly federatednodemanager_offload_intent: (a: number, b: number) => [number, number, number, number];
    readonly forward_chain_wasm: (a: any) => [number, number, number];
    readonly getBrowserExecutionReceipt: () => [number, number, number];
    readonly getEngineVersion: () => [number, number];
    readonly getResidentTokenizerVocab: () => number;
    readonly getWasmBackend: () => [number, number];
    readonly getWebgpuInitStatus: () => [number, number];
    readonly get_shacl_capability_manifest_wasm: () => any;
    readonly inferWasmAsync: (a: number, b: number, c: any) => any;
    readonly inferWasmAsyncMeasured: (a: number, b: number, c: number, d: any) => any;
    readonly inferWasmStreaming: (a: number, b: number, c: any) => any;
    readonly inferWasmStreamingWithChatGraph: (a: number, b: number, c: number, d: number, e: number, f: number, g: any) => any;
    readonly inferWasmStreamingWithContext: (a: number, b: number, c: number, d: number, e: any) => any;
    readonly inferWasmWithContext: (a: number, b: number, c: number, d: number) => any;
    readonly infer_wasm: (a: number, b: number) => any;
    readonly initializeCpuWasmEngine: (a: any) => any;
    readonly initializeCpuWasmEngineWithContext: (a: any, b: number) => any;
    readonly initialize_webgpu_engine: (a: any) => any;
    readonly intercept_computational_opcode: (a: number, b: number) => number;
    readonly intercept_pharmacogenomics_intent: (a: number, b: number) => number;
    readonly isWasmEngineReady: () => number;
    readonly isWebgpuEngineReady: () => number;
    readonly is_opfs_block_cached: (a: number) => any;
    readonly jsonld_context_digest_wasm: () => [number, number, number];
    readonly list_hmc_bundle_entries_wasm: (a: number, b: number) => [number, number, number];
    readonly p64FormatVersion: () => number;
    readonly pack_quins_into_superblock: (a: bigint, b: bigint, c: number, d: number) => [number, number, number];
    readonly package_exposure_manifest_wasm: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly parse_cbor_ld_wasm: (a: number, b: number) => any;
    readonly parse_csv_wasm: (a: any) => [number, number, number];
    readonly parse_json_mapping_wasm: (a: any) => [number, number, number];
    readonly parse_json_wasm: (a: number, b: number) => any;
    readonly parse_jsonld_wasm: (a: number, b: number) => [number, number, number];
    readonly parse_n3logic_wasm: (a: number, b: number) => any;
    readonly parse_rdf_document_wasm: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly parse_turtle_wasm: (a: number, b: number) => any;
    readonly parse_yaml_ld_q42_wasm: (a: number, b: number, c: number, d: bigint, e: number, f: bigint) => [number, number, number];
    readonly plan_device_storage_wasm: (a: any) => [number, number, number];
    readonly rdfc10_graph_hash_wasm: (a: any) => [number, number, number];
    readonly read_hmc_bundle_entry_wasm: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly read_opfs_block: (a: number) => any;
    readonly releaseWebgpuEngine: () => any;
    readonly sample_packed_quins_wasm: (a: number, b: number, c: number) => [number, number, number, number];
    readonly serialize_csv_wasm: (a: any) => [number, number, number];
    readonly serialize_float64_array: (a: number, b: number) => any;
    readonly serialize_float_array: (a: number, b: number) => any;
    readonly serialize_json_wasm: (a: any) => [number, number, number];
    readonly serialize_rdf_wasm: (a: any) => [number, number, number];
    readonly solid_negotiate_accept_wasm: (a: number, b: number) => [number, number, number];
    readonly validate_shacl_constraint_wasm: (a: any) => [number, number, number];
    readonly validate_shacl_graph_wasm: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly validate_shacl_json_wasm: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly verifyFirstLayerQuant: () => any;
    readonly verify_backup_manifest_wasm: (a: any, b: number, c: number) => [number, number, number];
    readonly verify_superblock_ecc: (a: number, b: number) => [number, number];
    readonly wasmoffloadintent_new: (a: number, b: number, c: number) => number;
    readonly wasmoffloadintent_with_string_payload: (a: number, b: number, c: number, d: number) => number;
    readonly wasmq42session_active_quin_count: (a: number) => number;
    readonly wasmq42session_advance_tick: (a: number, b: bigint) => void;
    readonly wasmq42session_commit_transaction: (a: number, b: bigint, c: bigint) => [number, number, number];
    readonly wasmq42session_current_tick: (a: number) => bigint;
    readonly wasmq42session_export_active_quins_bytes: (a: number) => [number, number];
    readonly wasmq42session_export_journal_bytes: (a: number) => [number, number];
    readonly wasmq42session_load_from_bytes: (a: number, b: number, c: number, d: number) => [number, number, number];
    readonly wasmq42session_new: (a: bigint, b: number, c: number) => [number, number, number];
    readonly wasmq42session_query_has_quin: (a: number, b: bigint, c: bigint, d: bigint) => number;
    readonly wasmq42session_query_object_for_predicate: (a: number, b: bigint, c: bigint) => [number, bigint];
    readonly wasmq42session_rewind_to_tick: (a: number, b: number, c: number, d: bigint) => [number, number];
    readonly wasmq42session_rollback_staged: (a: number) => void;
    readonly wasmq42session_stage_add_quin: (a: number, b: bigint, c: bigint, d: bigint, e: bigint, f: bigint) => void;
    readonly wasmq42session_stage_remove_quin: (a: number, b: bigint, c: bigint, d: bigint, e: bigint, f: bigint) => void;
    readonly wasmsimulationworld_compute_group_formation: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => [number, number];
    readonly wasmsimulationworld_compute_state_hash: (a: number) => bigint;
    readonly wasmsimulationworld_current_tick: (a: number) => bigint;
    readonly wasmsimulationworld_find_path_grid: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => [number, number];
    readonly wasmsimulationworld_get_agent_pos_x_mm: (a: number, b: bigint) => [number, bigint];
    readonly wasmsimulationworld_get_agent_pos_y_mm: (a: number, b: bigint) => [number, bigint];
    readonly wasmsimulationworld_new: (a: bigint) => number;
    readonly wasmsimulationworld_query_agents_in_bounds: (a: number, b: bigint, c: bigint, d: bigint, e: bigint) => [number, number];
    readonly wasmsimulationworld_register_agent: (a: number, b: bigint, c: bigint, d: bigint) => number;
    readonly wasmsimulationworld_step_tick: (a: number) => number;
    readonly wasmsimulationworld_submit_command: (a: number, b: bigint, c: number, d: bigint, e: number, f: bigint, g: bigint, h: bigint) => [number, number];
    readonly webizen_poll_agreements: () => [number, number];
    readonly webizen_propose_agreement: (a: any, b: number, c: number, d: number, e: number, f: number) => bigint;
    readonly webizen_sign_agreement: (a: bigint, b: number, c: number) => void;
    readonly write_opfs_block: (a: number, b: number, c: number) => any;
    readonly compileGgufToQ42: (a: any, b: number) => [number, number, number];
    readonly prune_and_validate_mesh: (a: bigint) => number;
    readonly q42FormatVersion: () => number;
    readonly __wbg_qualiaportal_free: (a: number, b: number) => void;
    readonly check_drug_interactions_wasm: (a: any) => [number, number, number];
    readonly compute_framingham_risk_wasm: (a: any) => [number, number, number];
    readonly crypto_aead_decrypt: (a: any) => [number, number, number];
    readonly crypto_aead_encrypt: (a: any) => [number, number, number];
    readonly crypto_blake3: (a: any) => [number, number, number];
    readonly crypto_hkdf_sha256: (a: any) => [number, number, number];
    readonly crypto_sha256: (a: any) => [number, number, number];
    readonly crypto_sha3_256: (a: any) => [number, number, number];
    readonly crypto_sha512: (a: any) => [number, number, number];
    readonly design_encode_wasm: (a: number, b: number) => [number, number, number];
    readonly export_tensor_buffer_wasm: (a: number, b: number) => [number, number, number];
    readonly export_tensor_slice_wasm: (a: number) => [number, number, number];
    readonly geosparql_operation_wasm: (a: number, b: number) => [number, number, number];
    readonly portal_abort_webgpu: () => void;
    readonly portal_init_webgl2: (a: any) => [number, number, number];
    readonly portal_init_webgpu: (a: any) => any;
    readonly portal_webgpu_canvas_claimed: () => number;
    readonly qualiaportal_acoustic_enabled: (a: number) => number;
    readonly qualiaportal_acoustic_sab_byte_length: (a: number) => number;
    readonly qualiaportal_acoustic_sidecar_pinned: (a: number) => number;
    readonly qualiaportal_acoustic_uniform_bytes: (a: number) => [number, number, number];
    readonly qualiaportal_acoustic_uniform_float_count: (a: number) => number;
    readonly qualiaportal_acoustic_uniform_floats: (a: number) => [number, number, number];
    readonly qualiaportal_ambient_intensity: (a: number) => number;
    readonly qualiaportal_animate_artefact: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
    readonly qualiaportal_artefact_refused: (a: number) => number;
    readonly qualiaportal_bake_cqt_sidecar_demo: (a: number, b: number) => [number, number, number];
    readonly qualiaportal_bake_stft_sidecar_demo: (a: number, b: number) => [number, number, number];
    readonly qualiaportal_body_render_receipt: (a: number) => [number, number, number];
    readonly qualiaportal_budget_collapses_3d: (a: number, b: number) => number;
    readonly qualiaportal_camera_pitch: (a: number) => number;
    readonly qualiaportal_camera_target_x: (a: number) => number;
    readonly qualiaportal_camera_target_y: (a: number) => number;
    readonly qualiaportal_camera_target_z: (a: number) => number;
    readonly qualiaportal_camera_yaw: (a: number) => number;
    readonly qualiaportal_camera_zoom: (a: number) => number;
    readonly qualiaportal_collapse_node_q: (a: number, b: number) => [number, number];
    readonly qualiaportal_control_pending: (a: number) => number;
    readonly qualiaportal_create_acoustic_sab: (a: number) => [number, number, number];
    readonly qualiaportal_demo_artefact_refusal: (a: number) => void;
    readonly qualiaportal_drain_control_commands: (a: number, b: number) => number;
    readonly qualiaportal_drain_sonic_tokens: (a: number, b: number) => [number, number, number];
    readonly qualiaportal_encode_geometry: (a: number, b: number, c: number) => [number, number, number];
    readonly qualiaportal_epistemic_q: (a: number) => number;
    readonly qualiaportal_last_parsed: (a: number) => any;
    readonly qualiaportal_load_10d: (a: number, b: number, c: number) => [number, number, number];
    readonly qualiaportal_load_10d_colored: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number, number];
    readonly qualiaportal_load_body_from_qualia_bundle: (a: number, b: number, c: number) => [number, number, number];
    readonly qualiaportal_load_body_from_qualia_bundle_mixed: (a: number, b: number, c: number, d: any, e: any) => [number, number, number];
    readonly qualiaportal_load_body_organs_colored: (a: number, b: any) => [number, number, number];
    readonly qualiaportal_load_json_scene: (a: number, b: number, c: number) => [number, number, number];
    readonly qualiaportal_load_q42: (a: number, b: number, c: number) => [number, number, number];
    readonly qualiaportal_mount_qapp: (a: number, b: number, c: number) => [number, number];
    readonly qualiaportal_navigate_to_node: (a: number, b: number) => [number, number];
    readonly qualiaportal_new: (a: any) => [number, number, number];
    readonly qualiaportal_observe_node_at: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
    readonly qualiaportal_operational_mode: (a: number) => number;
    readonly qualiaportal_pack_manifest: (a: number, b: number, c: number) => [number, number, number];
    readonly qualiaportal_poll_selected_node: (a: number) => number;
    readonly qualiaportal_project_resident_plane2d: (a: number, b: number) => [number, number];
    readonly qualiaportal_publish_acoustic_sab: (a: number, b: any) => [number, number];
    readonly qualiaportal_push_control_command: (a: number, b: bigint) => number;
    readonly qualiaportal_push_sonic_token_raw: (a: number, b: bigint) => number;
    readonly qualiaportal_resize: (a: number, b: any, c: number, d: number) => [number, number];
    readonly qualiaportal_sample_telemetry: (a: number) => [number, number, number];
    readonly qualiaportal_select_node_at: (a: number, b: number, c: number, d: number, e: number) => [number, number];
    readonly qualiaportal_set_acoustic_enabled: (a: number, b: number) => void;
    readonly qualiaportal_set_ambient_enabled: (a: number, b: number) => void;
    readonly qualiaportal_set_body_fit_json: (a: number, b: number, c: number) => void;
    readonly qualiaportal_set_camera: (a: number, b: number, c: number, d: number) => [number, number];
    readonly qualiaportal_set_camera_pan: (a: number, b: number, c: number, d: number) => [number, number];
    readonly qualiaportal_set_camera_target: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
    readonly qualiaportal_set_clear_color: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly qualiaportal_set_display_mode: (a: number, b: number, c: number) => [number, number];
    readonly qualiaportal_set_lighting: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
    readonly qualiaportal_set_preserve_authored_frame: (a: number, b: number) => void;
    readonly qualiaportal_set_sky_preset: (a: number, b: number) => void;
    readonly qualiaportal_set_standpoint: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
    readonly qualiaportal_set_telemetry: (a: number, b: number, c: number) => [number, number];
    readonly qualiaportal_set_temporal_slice: (a: number, b: number, c: number) => void;
    readonly qualiaportal_sonic_token_pending: (a: number) => number;
    readonly qualiaportal_spatial_encode: (a: number, b: number, c: number) => [number, number, number];
    readonly qualiaportal_standpoint_class: (a: number) => number;
    readonly qualiaportal_stop_artefact_animation: (a: number) => void;
    readonly qualiaportal_sun_intensity: (a: number) => number;
    readonly qualiaportal_t_slice: (a: number) => number;
    readonly qualiaportal_t_window: (a: number) => number;
    readonly qualiaportal_tick: (a: number, b: any, c: number) => [number, number];
    readonly qualiaportal_tier: (a: number) => number;
    readonly qualiaportal_upload_mesh_asset: (a: number, b: number, c: number, d: number, e: number) => [number, number, number];
    readonly qualiaportal_upload_tensor_buffer: (a: number, b: number, c: number) => [number, number];
    readonly qualiaportal_write_part_vertices: (a: number, b: number, c: number, d: number) => void;
    readonly sample_browser_telemetry_wasm: () => [number, number, number];
    readonly spatial_encode_wasm: (a: number, b: number) => [number, number, number];
    readonly validate_fhir_observation_wasm: (a: any) => [number, number, number];
    readonly wasm_convex_hull_2d: (a: number, b: number) => [number, number, number];
    readonly wasm_delaunay_triangulation_2d: (a: number, b: number) => [number, number, number];
    readonly qualiaportal_selected_node_index: (a: number) => number;
    readonly __wbg_wasmhealthstore_free: (a: number, b: number) => void;
    readonly evaluate_n3_rules: (a: number, b: number) => [number, number];
    readonly heart_rate_turtle_from_csv: (a: number, b: number) => [number, number, number, number];
    readonly parse_heart_rate_csv_json: (a: number, b: number) => [number, number, number];
    readonly parse_sleep_csv_json: (a: number, b: number) => [number, number, number];
    readonly parse_steps_csv_json: (a: number, b: number) => [number, number, number];
    readonly parse_weight_csv_json: (a: number, b: number) => [number, number, number];
    readonly sleep_turtle_from_csv: (a: number, b: number) => [number, number, number, number];
    readonly steps_turtle_from_csv: (a: number, b: number) => [number, number, number, number];
    readonly validate_health_quin: (a: number, b: number, c: bigint, d: bigint, e: bigint, f: bigint, g: bigint) => [number, number];
    readonly validate_health_turtle: (a: number, b: number) => [number, number];
    readonly vault_biometrics_to_turtle: (a: number, b: number) => [number, number, number, number];
    readonly vault_diet_to_turtle: (a: number, b: number) => [number, number, number, number];
    readonly vault_meds_to_turtle: (a: number, b: number) => [number, number, number, number];
    readonly wasmhealthstore_load_turtle: (a: number, b: number, c: number) => [number, number];
    readonly wasmhealthstore_new: () => [number, number, number];
    readonly wasmhealthstore_query: (a: number, b: number, c: number) => [number, number, number, number];
    readonly weight_turtle_from_csv: (a: number, b: number) => [number, number, number, number];
    readonly __wbg_qualiastore_free: (a: number, b: number) => void;
    readonly qualiastore_clear: (a: number) => void;
    readonly qualiastore_insert_from_cbor_ld: (a: number, b: number, c: number) => number;
    readonly qualiastore_insert_quin: (a: number, b: bigint, c: bigint, d: bigint, e: bigint, f: bigint) => number;
    readonly qualiastore_len: (a: number) => number;
    readonly qualiastore_new: () => number;
    readonly qualiastore_query_context: (a: number, b: bigint) => any;
    readonly qualiastore_query_predicate: (a: number, b: bigint) => any;
    readonly qualiastore_query_subject: (a: number, b: bigint) => any;
    readonly wasm_bindgen_2b06ec36d7f4b29___convert__closures_____invoke___wasm_bindgen_2b06ec36d7f4b29___JsValue__core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_2b06ec36d7f4b29___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_2b06ec36d7f4b29___convert__closures_____invoke___wasm_bindgen_2b06ec36d7f4b29___sys__JsOption_wgpu_40212802320f0c30___backend__webgpu__webgpu_sys__gen_GpuError__GpuError___core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_2b06ec36d7f4b29___JsError___true_: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_2b06ec36d7f4b29___convert__closures_____invoke___wasm_bindgen_2b06ec36d7f4b29___sys__JsOption_wgpu_40212802320f0c30___backend__webgpu__webgpu_sys__gen_GpuError__GpuError___core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_2b06ec36d7f4b29___JsError___true__2: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_2b06ec36d7f4b29___convert__closures_____invoke___wasm_bindgen_2b06ec36d7f4b29___sys__JsOption_wgpu_40212802320f0c30___backend__webgpu__webgpu_sys__gen_GpuError__GpuError___core_9b3796e30d99ddb7___result__Result_____wasm_bindgen_2b06ec36d7f4b29___JsError___true__3: (a: number, b: number, c: any) => [number, number];
    readonly wasm_bindgen_2b06ec36d7f4b29___convert__closures_____invoke___js_sys_4665151f90cca40___Function_fn_wasm_bindgen_2b06ec36d7f4b29___JsValue_____wasm_bindgen_2b06ec36d7f4b29___sys__Undefined___js_sys_4665151f90cca40___Function_fn_wasm_bindgen_2b06ec36d7f4b29___JsValue_____wasm_bindgen_2b06ec36d7f4b29___sys__Undefined_______true_: (a: number, b: number, c: any, d: any) => void;
    readonly wasm_bindgen_2b06ec36d7f4b29___convert__closures_____invoke_______true_: (a: number, b: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_destroy_closure: (a: number, b: number) => void;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
