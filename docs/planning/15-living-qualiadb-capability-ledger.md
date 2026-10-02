# Rolling Commons: Living QualiaDB Capability Ledger

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
on **2026-10-02**; no clean browser integration was run for these rows.

| Need / QG task | Current evidence | Status | Next QualiaDB proof | Game work dependent on it |
|---|---|---|---|---|
| Browser authoritative profile `wasm-webcivics` | `cargo check` wasm32 clean at `6356bb5a`; **browser spike 2026-10-02** (Chrome headless, real-time): module init ~30 ms warm / ~950 ms cold, turtle parse ✓, SHACL conform + violation report ✓ (`focus_node`/`message`/`result_path`/`severity`), OPFS save+reload ✓ ~288 ms; bundle 2.29 MiB, JS heap ~3.5 MiB. Caveats: `OpfsVfs` path is a bare filename (no directories); SHACL `ShapeSpec` JSON is camelCase; N3 token path needs `a`/prefixed-token lexical form (`RDF_TYPE_KEYS` = `rdf:type`,`a`); `serde_wasm_bindgen` returns `Map` for structs | **Game integrated** (spike-level) | Nested-path OPFS variant or explicit flat-namespace contract; production SHACL fixtures | Authoritative simulation shell, rules, saves (Phase 0/1) |
| WebGPU portal profile (`portal` feature) | `cargo check` wasm32 clean at `6356bb5a` | Upstream verified (compile); render unverified | `webizen-render` scene + `.10d` pick in target browser (D-019) | Storybook-3D presentation (lead profile) |
| `webizen-render` on wasm32 | wasm32 `cargo check` interrupted by host disk-full 2026-10-02; retry pending | Unverified | Clean wasm32 check, canvas render, semantic pick, frame/memory budget | 3D scene |
| Canonical HMC game pack / QG-01 | Core transparent `bundle/` and semantic-library ZIP `.hmc` both exist; HCF draft integrity prose diverges from core code. | Partial; version unpinned | Select/version producer and reader; native + browser round-trip of mixed entries | Pack distribution and loading |
| `.10d` asset creation to browser pick / QG-02 | Geometry, container, renderer, and Q42 manifest surfaces exist separately. | Unverified end to end | Deterministic prop build, validation, HMC load, Q42 identity, browser pick | First 3D scene |
| Animated `.10d` asset / QG-03 | Vibe animation APIs exist; current `.10d` section enum has no explicit animation section. | Partial; representation undecided | Versioned rig/motion contract and browser replay of two actions | Animated characters and objects |
| Vibe game REPL/host / QG-04 | Vibe REPL language and Poet exposure work exist; game-scoped host not proven. | Unverified for game | Live catalog, proposal/rule receipt, denied call, browser or authoring-host fixture | Iterative scenario authoring |
| Q42/HCF/HMC save boundary / QG-05 | Separate graph, content, and bundle formats exist. | Unverified for game | Pack/save version checks, replay, tamper and failed-write recovery | Offline save/reload |
| Optional P64 local NPC path / QG-06 | P64 is a separate model-weight format; QualiaDB inference profiles exist. | Unverified for game | Scoped local proposal with model/version and memory evidence | Optional local NPC inference; authored path remains complete |
| Terrain → `.10d`/Q42/HMC / QG-07 | QualiaDB terrain and `.10d` tile compilation code exists. | Unverified end to end | Source-linked offline terrain pack and browser spatial selection | Data-shaped terrain pack |
| On-demand geography / QG-08 | Concept considered; no game-specific path certified. | Deferred by D-041 | Governed source/cache/tile/replay browser proof if feature is selected | Future on-demand place streaming |

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
