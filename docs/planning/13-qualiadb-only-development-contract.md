# Rolling Commons: QualiaDB-Only Development Contract

## Binding rule

Rolling Commons is a **QualiaDB ecosystem application**. Game development must
use QualiaDB/Webizen capabilities for the game runtime, semantic state, rules,
simulation, rendering, spatial processing, asset generation and packaging,
authoring, inference, and persistence. The game repository may contain scenario
content, configuration, presentation composition, and thin calls to **public,
versioned QualiaDB interfaces**. It must not grow a parallel game engine or
private substitute for a missing QualiaDB capability.

If an essential function is absent, incomplete, or unavailable in the selected
browser/WASM profile, implement or expose it in the **QualiaDB repository first**,
test it there, then consume the pinned capability in this game. A missing
capability is a tracked ecosystem dependency and blocks the dependent game
feature. A simplified presentation or authored scenario is acceptable only when
it still uses QualiaDB's own supported APIs; it must not conceal a replacement
renderer, geometry pipeline, rules engine, database, or inference gateway.

This rule applies **throughout implementation**, not only during initial
architecture review. New requirements and obstacles discovered in asset
creation, playtesting, integration, performance work, or release testing reopen
the QualiaDB capability assessment. A previously verified API can become
partial when a new use case or browser profile exposes a gap.

The browser, operating system, GPU APIs, and a build tool are host substrates.
They do not become alternate owners of game semantics. Existing dependencies
inside QualiaDB remain governed by that repository; new third-party libraries
or services for a game need must be assessed and integrated **within QualiaDB**
before the game may call them. This rule applies to development tooling as well
as shipped play code.

## Boundaries by workflow

| Need | QualiaDB-owned path | Game-side responsibility |
|---|---|---|
| World state and actions | Q42 graph, logic/validation, governed command interface, deterministic simulation and event receipts | Define scenario facts, actions, and content through published schemas |
| 3D creation | QualiaDB geometry/mesh tooling, validation, `.10d` compilation, provenance, and Webizen renderer | Author original parameterised asset descriptions and inspect results |
| Place and terrain | QualiaDB geospatial adapters, coordinate transforms, terrain/mesh pipeline, spatial queries, pack compiler | Select licensed sources and review fictional scenario transforms |
| Player presentation | Webizen UI/render surfaces and published projection/input contract | Compose accessible game screens and original visual language on those surfaces |
| NPCs and LLMs | QualiaDB local inference, graph-scoped retrieval, intent/policy gate, structured proposal handling | Supply NPC roles, dialogue content, and allowed scenario intents |
| Authored behaviour | VibeScript/QualiaDB capability host | Write bounded scenario scripts against approved game capabilities |
| Saves and offline use | QualiaDB browser storage, volume/version, replay, and recovery surfaces | Expose player-facing save, reset, and recovery controls |

The format handoffs are explicit: Q42 holds semantic state and references;
`.10d` holds dense geometry; HCF carries authored hypermedia content; HMC is
the distribution container; VibeScript is the authoring/behaviour language;
P64 supplies optional inference weights. The
[upstream task register](14-qualiadb-format-and-tooling-upstream-tasks.md)
tracks format reconciliation and integration proofs, including animation.

A coding agent may generate source, parameter sets, tests, and procedural asset
recipes. Its outputs enter the QualiaDB asset pipeline and are validated before
play. Blender, Maya, OpenSCAD, separate Python geometry stacks, Unity, Godot,
Unreal, Three.js, Babylon.js, separate model APIs, and custom game-side
serialization/rendering are **not planned development dependencies**. Standard
exchange formats may be supported **through QualiaDB import/export** when
needed; a format does not imply an external tool is required.

## What the supplied 3D/LLM note contributes

The note distinguishes **development-time** LLM work from **runtime** NPC
behaviour. That distinction is adopted. A coding agent can construct geometric
assets as structured data or algorithms without a visual design package, but
those assets still require mesh-quality checks, semantic identity, provenance,
`.10d` compilation, rendering, and visual review in the QualiaDB workflow.
Complex characters and rigs need a specific QualiaDB capability assessment;
the ability to write mesh coordinates does not establish an animation pipeline.

At runtime, the renderer/input layer supplies a **bounded spatial and semantic
projection** to the QualiaDB agent gateway. The LLM may return dialogue or a
typed candidate action. QualiaDB rules and the deterministic reducer decide
whether anything changes. The LLM is neither the authoritative state machine
nor a route to direct animation, graph mutation, or raw browser/GPU control.
No runtime LLM is required for core play.

The note's HTTP geography → WASM mesh idea is a **capability candidate**, not a
decision to stream uncontrolled map data during play. The v0.1 path remains
licensed, reviewed, versioned snapshots compiled by QualiaDB into offline
packs. A later on-demand path would need QualiaDB-owned acquisition, licence and
privacy gates, caching, coordinate conversion, topology repair, tile seams,
memory limits, deterministic replay, and offline behaviour. Claims such as
“milliseconds,” direct zero-copy GPU transfer, or universal browser support are
performance hypotheses to measure, not planning assumptions.

## Capability evidence from the local QualiaDB tree

The following are **entry points to investigate**, not proof that the full game
path works. The cited paths are in the sibling `qualiaDB` repository.

| Capability candidate | Local evidence | Required game proof |
|---|---|---|
| Geometry and mesh export | `crates/qualia-core-db/src/specialized_libs/computational_geometry/` and `specialized_libs/computer_vision/spatial/` contain geometry and MeshIR exporters | Create, validate, package, and render an original prop through one supported QualiaDB interface |
| Terrain from elevation | `crates/qualia-core-db/src/domains/geospatial/terrain_pipeline.rs` builds terrain meshes and a `.10d` tile | Compile a licensed or fictional heightfield into a replayable, semantically pickable browser scene |
| Web rendering | `crates/webizen-render/` has scene and renderer code; `crates/webizen-web/src/render_stub.rs` is a minimal canvas path | Prove `.10d` load, scene display, picking, input, and fallback in the selected browser profile |
| Browser graph/runtime | `crates/webizen-lite-wasm/` exposes a deliberately limited ontology profile | Identify and pin the actual game-capable WASM profile; do not assume the lite profile supplies rendering or inference |
| Local inference | QualiaDB README documents native and browser inference profiles | Prove the selected browser build, scoped retrieval, policy gate, memory, latency, and no-model path |

`README.md` and design documents in QualiaDB describe a moving pre-release
system. Every row needs a clean build and narrow integration test against a
pinned revision before it can be marked **verified for this game**.

## Continuous gap-to-ecosystem workflow

At the start of a feature and whenever work encounters a block:

1. State the player need and the smallest public QualiaDB interface it needs.
   Reproduce the block with a narrow fixture; do not infer absence solely from
   missing documentation.
2. Search the QualiaDB implementation and tests; record the owning crate,
   target profile, version, and runnable proof. Mark the capability **verified**,
   **partial**, **missing**, or **unverified** in the living ledger.
3. If partial/missing, mark the dependent game task **blocked on QualiaDB**.
   Add an upstream subtask with the API/format change, target platforms,
   budgets, privacy/licence obligations, failure behaviour, and acceptance tests.
4. Implement or expose the capability in QualiaDB under that repository's
   contribution rules. Update its specification, conformance tests, and host
   bindings together where the change crosses `.10d`, HMC/HCF, Q42, Vibe, or P64.
5. Pin the updated QualiaDB revision in the game, run upstream and end-to-end
   browser/game fixtures, record the receipts, and **resume the blocked task**.
6. Repeat for the next block. Each implementation change reviews the relevant
   ledger entries; the ledger is not closed after Phase 0.

Unrelated game tasks can continue while an upstream fix is underway. Do not
quietly recast an unmet feature as optional, design around the missing
capability, duplicate it in game code, or introduce a second stack to keep a
release date. A deliberate product-scope change requires an explicit decision
record; it is separate from an engineering workaround.

Phase 0 opens a **living capability ledger** for graph/rules, commands/replay,
Webizen UI, `.10d` authoring/loading/animation, HCF/HMC packaging, rendering/picking,
geography/terrain, VibeScript REPL/host, P64 inference, offline storage, and
save recovery. Each entry needs an
owner in QualiaDB, a pinned commit, a minimal test, a status, and a game
dependency. Every later phase updates the ledger as new needs or blocks emerge;
open upstream dependencies remain visible in release gates.
The working ledger is [maintained here](15-living-qualiadb-capability-ledger.md).
