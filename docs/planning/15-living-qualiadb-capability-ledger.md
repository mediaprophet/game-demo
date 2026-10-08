# Maslows Challenge: Living QualiaDB Capability Ledger

## How to use this ledger

This ledger is updated **during every implementation phase**. It is not a
one-time Phase 0 checklist. Before starting a game feature, identify its
QualiaDB interface and test it against the selected build. When a new block
appears, add a row and a QG upstream subtask immediately, mark the dependent
game work **blocked on QualiaDB**, and keep that work linked here until the
upstream fix and game integration both pass. Other independent work may proceed.

Statuses: **unverified** (candidate only), **partial** (specific gap evidenced),
**blocked on QualiaDB** (game implementation stopped by that gap), **upstream in
progress**, **upstream verified**, and **game integrated**. A status change needs
a QualiaDB commit/format version, an upstream test, and—before **game
integrated**—a browser/game receipt. Do not erase failed attempts; summarize
them in the row's linked task or test record. A product-scope change is a
separate decision, never a substitute for closing a capability gap.

The [development contract](13-qualiadb-only-development-contract.md) gives the
full loop. [Upstream tasks](14-qualiadb-format-and-tooling-upstream-tasks.md)
define the current work packets. This table starts from repository inspection
on **2026-10-02**. QualiaDB base revision: **`726f95d7`** (branch
`0.0.40.5` / upstream work branch `rolling-commons/phase0`). This is the **base revision**;
local QualiaDB scene geometry changes are not yet a reproducible pin. The
current two-territory integration uses clean QualiaDB commit `32ef0175` with
camera target, sky presets and a repaired WASM HMC bridge export. Its Cargo
dependency is still a sibling path, so each tested checkout must be recorded.
The release/browser receipt for this revision is tracked in
`20-current-game-build-and-qualiadb-needs.md`.
The
historical browser receipts below are from a Chrome headless (swiftshader)
run against a superseded portal+webcivics+vibe bundle: 10/10 spike checks passed (init 395 ms, OPFS
round-trip 187 ms, `.10d` compile 6 ms, pick index 0, vibe eval + denied
capability). The current game dependency is `wasm-full`, which passes
`cargo check --workspace --target wasm32-unknown-unknown` and a release
`wasm-pack` build with the new QualiaDB primitive assembler. Browser proof:
QualiaPortal tier 2, 57 governed `.10d` meshes accepted, solar, shelter,
water and garden action checks and replay passed; OPFS save/reload restored
the paid water repair and planted garden; VibeScript expression evaluation
returned `3`. GPU semantic
picking selected the market and workshop nodes. The final release package
also verified caller-buffered assembly and clearing selection on a miss.

| Need / QG task | Current evidence | Status | Next QualiaDB proof | Game work dependent on it |
|---|---|---|---|---|
| Full Qualia WASM engine (`wasm-full`) | Game dependency selects full engine; wasm32 check passes. Browser shows tier-2 GPU rendering of 57 provenance-bearing `.10d` meshes, market/workshop semantic picks, solar/shelter/water/garden action and replay checks, OPFS restore, and VibeScript evaluation. Final browser check includes caller-buffered assembly and clearing a missed pick. | Game integrated for current scene and action slice | HMC, animation, Q42 graph session and game-scoped Vibe capabilities | All gameplay and presentation |
| Historical `wasm-webcivics` spike | Earlier browser measurements on a separate profile remain evidence for its APIs only. This profile is no longer a game dependency. | Superseded | None for the game | None |
| WebGPU portal profile (`portal` feature) | `cargo check` wasm32 clean at `27d1644a`; **browser spike 2026-10-02** (Chrome headless + swiftshader): `QualiaPortal` canvas init ✓, `load_10d` ✓, `tick` ✓, tensor upload + semantic pick ✓ (CPU path, index 0); combined portal+webcivics+vibe bundle 6.1 MiB. GPU tier-2 (WebGPU device) unconfirmed headless — portal ran tier 1 | **Game integrated** (spike-level, tier-1 path) | WebGPU device present in a real browser session → tier-2 render + GPU pick; frame-time measurement (spike 7) | Storybook-3D presentation (lead profile) |
| `webizen-render` on wasm32 | Superseded: the `portal` feature's internal `PortalGpu` is the selected render path; a standalone `webizen-render` build is not on the game critical path | Deferred (not required) | Revisit only if portal-internal GPU path proves insufficient | 3D scene |
| Canonical HMC game pack / QG-01 | Core transparent `bundle/` and semantic-library ZIP `.hmc` both exist; HCF draft integrity prose diverges from core code. | Partial; version unpinned | Select/version producer and reader; native + browser round-trip of mixed entries | Pack distribution and loading |
| `.10d` asset creation to browser pick / QG-02 | Current town uses QualiaDB primitive and parametric geometry with deterministic source receipts and provenance sidecars; 57 sealed meshes pass the portal gate at GPU tier 2. QualiaDB GPU readback keeps the async map receiver across frames and uses top-left texture coordinates; browser clicks selected market and workshop semantic nodes and a miss clears selection. The primitive assembler writes into caller buffers under a GeometryWorkspace budget. | Game integrated for first scene | HMC packaging, Q42 manifest↔semantic-id binding, and animated `.10d` contract | First 3D scene |
| Animated `.10d` asset / QG-03 | Vibe animation APIs exist; current `.10d` section enum has no explicit animation section. | Partial; representation undecided | Versioned rig/motion contract and browser replay of two actions | Animated characters and objects |
| Vibe game host / QG-04 | Full-profile browser now has a VibeScript workbench; `= 1 + 2` returns `3` in the game. Earlier denied-capability spike still applies, but no game-scoped world/query capability is bound yet. | Partial; REPL integrated, game binding blocked on QualiaDB | Live game-scoped capability catalog + proposal/rule receipt fixture | Iterative scenario authoring and less JavaScript |
| Q42/HCF/HMC save boundary / QG-05 | Separate graph, content, and bundle formats exist. **Browser slice 2026-10-02:** world doc + `ev:` event lines round-trip through `OpfsVfs` (flat filename); replay re-validates the tape and reproduces the world byte-for-byte | Partial — doc-level save/replay proven; Q42-native save format and version checks unproven | Pack/save version checks, tamper and failed-write recovery | Offline save/reload |
| Mutable Q42 graph session and Vibe host / new QG gap | `GraphDatabase.sparql` in the Vibe `LocalHost` returns an empty list. The current shell uses N3 text and declared add/remove triples as a temporary slice. Concurrent uncommitted QualiaDB work includes a `.q42` journal and WASM session candidate, but the inspected bridge uses `Cursor<Vec<u8>>`; browser durability, versioned replay and live Vibe query remain unproven. | Blocked on QualiaDB for Q42 migration | Verify a public durable mutable WASM graph session, deterministic action/replay receipts, Q42 save/reload and Vibe capability; then migrate shell | Authoritative state, NPC queries, aggregate views, durable saves |
| Optional P64 local NPC path / QG-06 | P64 is a separate model-weight format; QualiaDB inference profiles exist. | Unverified for game | Scoped local proposal with model/version and memory evidence | Optional local NPC inference; authored path remains complete |
| Terrain → `.10d`/Q42/HMC / QG-07 | QualiaDB terrain and `.10d` tile compilation code exists. | Unverified end to end | Source-linked offline terrain pack and browser spatial selection | Data-shaped terrain pack |
| On-demand geography / QG-08 | Concept considered; no game-specific path certified. | Deferred by D-041 | Governed source/cache/tile/replay browser proof if feature is selected | Future on-demand place streaming |
| Australia OSM atlas, local geographic tiles and travel graph / QG-20 | Current build has three authored local areas in one scene; no Australian atlas, OSM snapshot/import, regional tile pack, or inter-region route simulation. Place switching is a character spawn choice and does not model journey costs or arrivals in a distinct geographic destination. | Partial; OSM-derived packs blocked on QualiaDB | Generic OSM/PBF or documented-intermediate import, CRS/topology/spatial query and route graph APIs, bounded tile/HMC loading, provenance/licence surface, then an offline Australian regional-pack browser receipt. See [world atlas plan](29-australia-world-atlas-and-osm-pack.md). | Australia-wide destinations, realistic regional geography, large mapped facilities, and consequential travel |
| RTS simulation and commands / QG-09, QG-15 | Current slice has discrete rule-gated actions and an N3 text session. A generic fixed-tick candidate exists in concurrent uncommitted QualiaDB work; continuous multi-agent command, browser and Q42 replay proofs remain open. | Blocked on QualiaDB for RTS slice | Native/WASM fixed-tick, command, save, and replay fixture | Workforce, construction, economy, campaign |
| RTS camera, selection, navigation / QG-10, QG-11 | Current scene picks one semantic object; group selection, orders, and pathfinding are not yet game integrated. | Unverified | QualiaPortal selection and 100-agent route fixtures | Core RTS control loop |
| Large map and finished content / QG-12–QG-14, QG-17 | Fifty-seven static runtime-generated blockout meshes render through full Qualia WASM; no governed HMC sector or animated person. | Partial | Instancing/LOD, animated asset, terrain, and HMC browser fixtures | Finished RTS sector |
| Vibe scenario host and browser input/audio / QG-16, QG-18 | Expression evaluation works. Qualia has `qualia-audio`, core audio DSP/acoustic surfaces and Portal sonic-token APIs; their RTS game-event, browser output, caption and keyboard integration is unproven. | Partial | Scenario proposal and keyboard/Qualia-audio browser fixtures | Authored missions and polished interaction |
| Game-owned blockout asset catalog / QG-02, QG-14 | Kestrel Flats now declares 57 persistent original source recipes plus conditional solar/garden variants in a separate game content module. The shell uses Qualia bounded primitive assembly and public computational geometry authoring (cylinder, sphere, torus, transform, CAD revolution), then seals `.10d` with source provenance. Release `wasm-pack` build passes; browser shows QualiaPortal GPU tier 2 with 57 loaded meshes, and scene, action, replay, and VibeScript self-checks pass. These remain blockouts, not finished pack assets. | Game integrated for detailed blockout catalog; HMC/finished asset gate open | Source-to-HMC validation, animated assets, materials/lighting and offline pack load | First RTS sector art production |

## New-block entry template

When implementation finds a block, append a row above with:

1. the player-facing need, affected game task, and a minimal reproduction;
2. the closest QualiaDB crate/API and tested revision, or “not found” with
   search notes;
3. an upstream QG task, owner, format/host contract, and acceptance fixture;
4. the current status and the next proof needed; and
5. the QualiaDB commit, upstream test, and game integration receipt when fixed.

Review open rows at each feature handoff and release gate. A row remains open
when only a draft, one platform, or an isolated unit test works; it closes as
**game integrated** only when the required player path runs through QualiaDB.
