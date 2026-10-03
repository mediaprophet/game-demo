# QualiaDB Upstream Gates: Engineering Work Orders

Status: **implementation instructions**, 2026-10-03. These work orders turn
the [RTS blueprint](17-rts-aaa-uplift-blueprint.md) and [asset catalog](18-asset-production-catalog.md)
into a sequence of QualiaDB changes. They govern the **full `wasm-full`
Qualia package**, not the WebCivics profile. They do not change the licence
of QualiaDB or game content. Existing QG-01–QG-08 are defined in the
[format register](14-qualiadb-format-and-tooling-upstream-tasks.md); QG-09–QG-18
are defined here in executable detail.
The platform-wide, game-independent completion contract for the QualiaDB
development agent is in
`qualiaDB/docs/work-in-progress/qualia-capability-demonstration-program.md`.

## Operating rule for every gate

1. **Reproduce first.** In `C:/github/qualiaDB`, inspect the named entry
   points, tests, feature flags and the exact `wasm-full` exposure. Record the
   tested Git revision. An API's existence is not proof it works at game scale.
2. **Write a failing fixture** using the smallest original Kestrel Flats case.
   Include expected success, malformed input, budget exhaustion, and version
   mismatch. Where behaviour spans native and browser, test both.
3. **Extend QualiaDB's public interface**, spec, conformance fixtures and
   browser binding together. Keep game content in `game-demo`; keep general
   geometry, formats, runtime, navigation, rendering, scripting and storage in
   `qualiaDB`. Preserve Qualia's bounded construction and zero-heap hot-path
   rules in `AGENTS.md`.
4. **Integrate the game against that interface.** Replace the temporary
   browser/shell behaviour. Never leave two authoritative implementations.
   Pin the Qualia revision and format/API versions in the game.
5. **Close only with receipts:** native tests, `wasm32` build, browser fixture
   on the declared baseline, performance/memory measurement, and a game
   playthrough. Update the [living capability ledger](15-living-qualiadb-capability-ledger.md)
   with the revision, result, and remaining limits.

If a test shows a new missing feature, create QG-19 onward, link the blocked
game task, and fix upstream. Do not reduce the feature silently or introduce
a second renderer, asset compiler, pathfinder, save store, scripting engine,
or simulation in the game repository.

## Dependency order

```text
QG-01 canonical HMC ──┐
QG-02 asset authoring ├─> QG-14 game pack ─> finished sector
QG-03 animation ──────┘       │
QG-17 terrain/tooling ────────┘

QG-15 mutable Q42 ─> QG-09 fixed ticks ─> QG-11 navigation ─> RTS agents
        │                    │
        └─> QG-16 Vibe host   └─> QG-10 controls

QG-05 save contract + QG-12 large scene + QG-13 animation playback
       + QG-18 input/audio/accessibility ─> finished browser slice
```

Work can proceed concurrently when independent, but no game feature is
marked integrated until all its incoming gates pass. QG-06 P64 and QG-08
on-demand geography remain conditional; do not add them to the critical path
for a playable offline RTS. QG-07 becomes required for data-shaped terrain.

## A. Asset and content gates

### QG-01 — Choose and version the canonical HMC

**Inspect:** `qualia-core-db/src/bundle/{format,writer,reader}.rs`, the
semantic-library `.hmc` implementation, HCF/HMC standards prose, and current
native tests. The core writer concatenates intact aligned files and verifies
per-entry SHA-256; another `.hmc` implementation uses ZIP. Decide which
producer/reader is canonical for game packs and assign an explicit version.

**Implement:** one public producer/reader contract with a typed manifest
entry for key, kind, digest, licence/provenance reference, dependencies and
format version; or a documented versioned conversion if both variants remain.
Reject ambiguous variants clearly. Expose the selected reader in `wasm-full`
without copying or reinterpreting `.10d` entry bytes. Align specifications
with implemented integrity checks; update old `.d10` wording to `.10d` where
it describes the canonical game format.

**Fixture:** build a mixed pack with two `.10d` assets, a Q42 manifest and an
HCF description. Read it natively and in browser, verify all digests and
rights, extract a byte-identical `.10d`, then alter one byte and assert a
specific rejection. An incompatible legacy variant must not open by accident.

### QG-02 — Make Qualia the complete asset authoring path

**Inspect:** `render/scene_primitives.rs`, `render/compile_10d.rs`,
`container_10d/`, and `specialized_libs/computational_geometry/`.
The current game already uses bounded primitive assembly and sealed `.10d`
output, but has no general production compiler or HMC manifest.

**Implement:** a versioned source recipe/import interface with units, pivot,
footprint, material slots, LOD metadata, semantic pick anchors, source digest
and rights. It must validate topology, finite bounds, index ranges, units and
budget before compilation. Make the compiler callable in a reproducible
asset-build workflow. Keep public output caller-buffered and cold scratch
bounded by `GeometryWorkspace` or an equivalent Qualia budget.

**Fixture:** compile the existing workshop and one distinct worker prop twice
from source and compare `.10d` bytes/digests. Malformed topology, NaNs,
missing licence, wrong units and exhausted arena fail with useful diagnostics.
Load both in QualiaPortal; picking resolves the intended Q42 IDs.

### QG-03 and QG-13 — `.10d` animation format and runtime playback

**Inspect:** `container_10d/section.rs`, `render/compile_10d.rs`,
`render/scene_graph/`, `render/portal/`, and `vibe/src/animation/`. Current
section types do not establish a stored skeletal or articulated clip format;
existing Vibe animation functions do not prove `.10d` playback.

**Specify before coding:** versioned rig, joint hierarchy, bind pose, named
clips, timebase, interpolation, bounds, optional events, and the link from
the `.10d` asset to a Q42 entity and simulation tick. Decide whether clips
are sections or linked sealed `.10d` assets; record compatibility behaviour
for static readers. Never repurpose a reserved section without updating the
format spec and conformance reader.

**Implement:** compiler, parser, validation, scene binding and GPU/CPU
playback as needed. Simulation chooses a named state at an authoritative
tick; presentation interpolates. Reduced-motion mode uses a readable static
or low-motion state without altering simulation.

**Fixture:** one original worker plays idle, walk and build; one workshop
plays construction and operating states. Pack, reload, seek and replay them
at named ticks. Corrupt joint index, clip time or section CRC fails closed.
Measure animated crowd memory/frame time, not just one isolated rig.

### QG-14 — Build an independently distributable game pack

**Depends on:** QG-01, QG-02, and QG-05; animated entries depend on QG-03.

**Implement:** Qualia-owned pack builder, dependency resolver, manifest
validation and browser loader. A pack contains immutable `.10d`, Q42/HCF
content, rights and build metadata. Save files reference the pack version and
entry digests but remain separate. Support incremental rebuild of a changed
source without silently changing unrelated asset identity. Fail visibly on
missing, mismatched or incompatible entries.

**Fixture:** move the current `rc:asset/workshop` recipe out of runtime scene
generation into a versioned Kestrel Flats HMC. Load it offline through full
Qualia WASM, select it, change its Q42 power state, save/reload, and prove its
mesh digest and semantic ID remain stable. Repeat for the full first sector.

### QG-17 — Terrain, placement and source-to-scene tooling

**Inspect:** `domains/geospatial/terrain_pipeline.rs`, terrain/DEM adapters,
computational geometry, `render/scene_primitives.rs`, and spatial index code.

**Implement:** a bounded, deterministic source-to-terrain build path for an
original heightfield first, with explicit units/coordinate frame, roads,
buildable footprint mask, collision and navigation surface, utility anchors,
LOD/tiles, seams and visual debug. Attach Q42 spatial identities and HMC
manifest entries. Real public-data inputs require QG-07 provenance/licence
and transformation gates before being presented as such.

**Fixture:** compile one original sector twice to identical outputs. Place
the workshop on valid ground; reject overlap, slope and route blockage with
specific reasons. Cross a tile seam without a crack or navigation break.
Render/select in browser within the chosen memory budget.

## B. Authoritative RTS world gates

### QG-05 and QG-15 — Mutable Q42 world and durable save boundary

**Inspect:** `q42/volume/`, `q42/q42_reader.rs`, `poet_host/invoke/graph/`,
OPFS surfaces, and the game shell's current N3 text/action tape. Do not
mistake a read-only Q42 volume or empty Vibe SPARQL result for a mutable
authoritative session.
The proposed upstream design is in
`qualiaDB/docs/work-in-progress/q42-mutable-journal-game-proposal.md`.
It recommends sealed `.q42` checkpoints plus a provisional `.q42j`
transaction journal; `T48` is not an accepted format name.

**Implement:** a browser-capable mutable graph transaction API with stable
entity IDs, queries, validation, event receipts, checkpoint, reload,
migration and failed-write recovery. Define exactly what goes into an
immutable content pack versus a mutable player save. Include pack digest,
schema/rule version, seed, tick and command cursor in the save header.
Transactions either commit all validated effects or none. Preserve
provenance and role/permission scopes.

**Fixture:** load a seeded Q42 world, issue 1,000 mixed commands, checkpoint,
force an interrupted write, recover the last valid checkpoint, reload, replay
and compare graph/event digests. Reject a save tied to a tampered or
incompatible pack with a player-readable recovery path. Migrate the current
game shell so its N3 text is no longer authoritative.

### QG-09 — Deterministic fixed-tick multi-agent simulation

**Depends on:** QG-15 for authoritative state; may prototype its pure kernel
with a fixed fixture while QG-15 is implemented.

**Implement:** a public Qualia command/tick API with declared tick rate,
seeded randomness, stable command ordering, bounded work per tick, explicit
pause/speed controls, typed accepted/rejected receipts, and replay. Separate
simulation from frame/render time. Model task assignment, work progress,
resource transfers, production, maintenance and event scheduling as
validated state transitions; presentation reads projections only. Use
fixed-point/integer quantities where float variation would break replay.

**Fixture:** 10 people, two facilities, one power network, a resource haul,
a build queue and an outage for a 20-minute scripted trace. Run native and
WASM from the same seed; compare canonical final state and receipt digests.
Rejected commands leave state unchanged. Tick budget and allocations are
measured under the Qualia hot-path rules.

### QG-11 — Navigation and group movement

QualiaDB commit `0300525b` now contains a bounded deterministic
`simulation/navigation.rs` grid. The game's canal and rule-gated bridge are
the next cross-tile fixture: a route from Kestrel Flats to Saltwind Reach
must be unavailable before repair and available afterward. Integrate this
through the public simulation/scene authority when worker orders are added;
keep one source of truth for bridge passability.

**Inspect:** existing Qualia spatial index, computational geometry and any
path-query APIs before adding a new module. Define a public navigation
surface with terrain, obstacles, reservations and dynamic build changes.

**Implement:** deterministic pathfinding with bounded queue/scratch,
explicit unreachable/error results, group order assignment, formation or
collision policy, route reservation/cancellation, and rebuilding only the
affected navigation region when a structure changes. Movement commands
carry destination, allowed route class, role/access and tick; permissions
are validated before path commitment.

**Fixture:** 100 agents move across two terrain tiles while a bridge closes,
a building is placed and a route reopens. No agent teleports, loops forever
or crosses an inaccessible node. Replay paths and arrivals exactly on the
same seed and commands. Record tick time, arena high-water and route latency.

### QG-04 and QG-16 — Game-scoped VibeScript host

**Inspect:** `vibe/src/bind/host.rs`, `vibe/src/capability_schema.rs`,
`qualia-core-db/src/poet_host/`, and the live capability catalog. The current
game now proves pure Vibe cells over a game-owned snapshot projection,
SHACL-gated catalogue action submission, and one authored sphere/cylinder
`.10d` asset. It does not prove a live Q42 host or reusable typed bindings.

**Implement:** a discoverable game capability profile for read-only world
projection/query, rule preview, typed command proposal, diagnostics and
scenario authoring. Every call has declared effect, gas, scope and receipt.
The scene profile should expose typed Qualia computational geometry recipes,
stable semantic asset ids, source and licence provenance, validation budgets,
and HMC packaging; the current game's bounded record adapter is the migration
fixture, not the long-term generic authoring API.
Preview returns a candidate and explanation; it cannot mutate Q42. Command
submission goes through QG-09 validation and reducer. Provide browser REPL
bindings so game logic does not grow a private JavaScript bridge.

**Fixture:** a Vibe cell inspects workshop power, proposes a build stage and
gets a rule receipt; a denied direct-storage/graph-mutation call fails.
Repeat against a live browser Q42 fixture and after save/reload.

## C. Presentation and browser gates

### QG-10 — RTS camera, selection, orders and minimap

**Inspect:** `render/portal/mod.rs`, `render/gpu/`, scene graph and Webizen UI
surfaces. The portal already has orbit camera and single semantic GPU pick;
QualiaDB commit `0300525b` also exposes camera pan and target placement, now
used by the two-territory game scene. The game still needs world-space mapping
and selection sets.

**Implement:** public camera pan/zoom/rotation/bookmarks; cursor-to-world hit;
box selection; additive selection; semantic selection sets and control groups;
command target preview; world overlays and minimap projection. Expose
stable entity IDs instead of relying on transient mesh order. Keep the
selection kernel in Qualia; browser events remain thin input delivery.

**Fixture:** select 100 mixed entities at two resolutions and zoom levels,
issue a 5-person order, select a partially occluded structure, and clear on
miss. Test scaling, resize, camera movement, keyboard path and pick latency
on both GPU and supported lower-spec paths.

### QG-12 — Large-scene rendering and measured budgets

**Inspect:** `render/gpu/`, `render/scene_graph/`, `render/portal/`,
`webizen-render/src/scene.rs` and material/texture APIs. Preserve the full
Qualia renderer and its semantic picking. The 91-organ Kestrel Flats browser
scene demonstrates geometry and per-organ colour, while the full Portal path
has no game-verified material, texture, light, shadow or sky authoring
contract yet. Browser inspection of the earlier full Portal path showed a black clear
and pale, ghostlike surfaces despite opaque authored RGBA. The current
`32ef0175` source / rebuilt game package reports a more severe browser regression:
the canvas remains entirely black despite a successful WebGPU scene receipt.
Inspect the HDR
mesh target's additive blend (`render/gpu/bloom.rs`), tone mapping and depth
composition with a solid-colour occlusion fixture; determine the root cause
before tuning game colours around it. Verify what exists before implementing
a missing surface. Commit `0300525b` now exposes public daylight, sunset and
night sky presets plus configurable sun lighting; the game consumes the sky
API. The duplicate WASM export was fixed in `32ef0175`; the game now builds
and consumes the sky and camera APIs, but visual acceptance is blocked.

**Immediate browser repro:** serve `game-demo/web/`, open `game.html`, and
observe `QualiaPortal tier 2 · 2 territories · 115 .10d meshes` while the
960 × 600 canvas stays black. `?selftest` passes the scene, actions and replay.
For isolation, call `init_webgpu(canvas)`, construct `GamePortal`, pass only
`scene_build(...).organs[0]` to `load_scene`, then tick. The receipt reports
`organs_loaded:1`, `total_triangles:12`, `vertex_count:8`,
`renderer:"webgpu"`, `uploaded:true`; the canvas is still black. Camera zoom
4.0, legacy origin camera, and changing sky preset after initialization do
not restore visible pixels. No JS tick exception or browser warning was
reported. Skipping WebGPU yields Qualia's tier-1 fallback field, confirming
the browser canvas can display output, but that path has no game scene.
The [captured tier-2 viewport](../images/qualia-webgpu-black-viewport.png)
shows the 115-mesh receipt and black canvas on 2026-10-03.
Reproduce this first with a Qualia-owned sealed opaque cube and assert both
a nonblack pixel/readback and correct occlusion, not just upload/frame counts.

**Implement only measured gaps:** instancing, culling, LOD, terrain tiles,
material/texture upload, lighting, effects and residency/streaming under
bounded GPU/WASM memory. Preserve stable Q42 identity across LOD and tile
changes. Fallback presentation must be Qualia-owned and action-equivalent.

**Fixture:** a representative sector with 100 agents, 50 structures, 500
props and terrain at close/strategic zoom. Record p50/p95 frame time,
GPU/WASM high-water, load time and pick latency against the declared browser
baseline. A missing texture or over-budget asset fails visibly and safely.
Include a small visual fixture with one textured building, vegetation,
daylight/shadow, sky or atmosphere, and a night light. Its `.10d`/HMC source
and rights must survive a native/browser round-trip; selection IDs must still
resolve after LOD swaps. Close this visual contract before calling the game
sector finished art.
Add opaque near/far cubes with known linear RGB values to the fixture: the
near cube must fully occlude the far cube and match the expected display
colour in both HDR/bloom and ordinary paths. This is a generic Portal
conformance test, not a game-only shader tweak.

### QG-18 — Integrate existing Qualia audio with browser input and accessibility

**Inspect and reuse:** `crates/qualia-audio/` (production, music, sound
generation, effects, media store and event APIs),
`qualia-core-db/src/audio/`, QualiaPortal's acoustic/token surfaces,
`webizen-render/src/audio_contract.rs`, Webizen UI/input controls and game
keyboard handling. Qualia already has a substantial audio library; this
gate is the game-facing integration and browser proof, not a request to
invent another audio engine. Decide the narrow browser substrate duties.

**Implement:** versioned input actions and remapping, keyboard-equivalent
selection/commands, focus/announcement state for semantic UI, caption/event
metadata, volume buses, reduced-motion state and audio loading through
existing Qualia-owned production/content interfaces. Add only any missing
bindings or capabilities to Qualia. DOM/Web Audio may be host substrates,
but the game must not create a second semantic command or audio-state engine.

**Fixture:** complete the opening RTS task without a mouse or sound;
selection, blocked order, completion and outage are perceivable through
text/caption and accessible focus. Reload preserves settings. Audio events
are triggered once per authoritative receipt and do not duplicate on replay
or tab resume.

## Conditional gates retained from the original register

- **QG-06 P64:** optional graph-scoped NPC inference. First prove that the
  full authored RTS journey works offline without model weights. If enabled,
  use a separately versioned `.p64`, bounded context and typed proposal gate.
- **QG-07 public-data terrain:** required only when a real data-shaped place
  pack is produced. Preserve source, licence, coordinate transform, privacy,
  uncertainty and repeatable build receipts.
- **QG-08 on-demand geography:** deferred. A future decision must add
  Qualia-owned acquisition/cache/tile lifecycle and offline replay before
  browser network geography affects authoritative play.

## First implementation sequence

1. **QG-01 + QG-02:** choose canonical HMC and compile the already-authored
   Kestrel Flats workshop into a separate source-controlled pack. This proves
   the asset boundary before creating hundreds of finished assets.
2. **QG-15 + QG-09:** replace the shell's temporary N3 session with mutable
   Q42 transactions and fixed ticks. Make one worker deliver one part and
   build one stage; save and replay it.
3. **QG-17 + QG-11:** compile one terrain sector and route that worker to the
   workshop, including a blocked route and a changed construction footprint.
4. **QG-10 + QG-12:** add Qualia-owned RTS selection/camera and verify a
   stress scene at the declared browser budget.
5. **QG-03/13 + QG-14:** add one animated worker and a versioned sector HMC;
   switch the game from runtime blockout compilation to pack loading.
6. **QG-16 + QG-18:** finish scenario authoring, keyboard/audio/captions and
   accessible feedback. Then build more assets using the validated pipeline.

For each step, update the game only after the relevant Qualia implementation
and tests land. Keep the current working blockouts available as a regression
scene until the finished pack has passed the same pick, save, replay, browser
and provenance checks.
