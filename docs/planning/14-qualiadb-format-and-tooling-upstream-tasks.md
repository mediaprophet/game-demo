# Maslows Challenge: QualiaDB Format and Tooling Upstream Tasks

## Purpose and status

This is the **upstream dependency register** for game needs involving `.10d`,
HCF/HMC, VibeScript, Q42, and P64. Items are proposed QualiaDB subtasks, not
claims that a capability is absent or ready until an integration test proves
it. The [QualiaDB-only contract](13-qualiadb-only-development-contract.md)
applies: complete a missing capability in QualiaDB, pin it, then use it here.
Current status and newly discovered blockers belong in the
[living capability ledger](15-living-qualiadb-capability-ledger.md).
The corresponding work packets are also recorded upstream at
`qualiaDB/docs/plans/rolling-commons-format-runtime-integration.md` as a local
working plan. QualiaDB intentionally Git-ignores `docs/plans/`; this document
is the tracked cross-project record until upstream tasks are incorporated into
an approved tracked workflow.

The inspection below is of the local `C:/github/qualiaDB` tree. It establishes
code entry points and documented divergences; it is **not** a clean build or
browser capability certification. Record the tested commit in the living
capability ledger before closing any task. Keep this register open throughout
implementation and add QG tasks when new work reveals further QualiaDB gaps.

## Roles of the artifacts

| Artifact | Role in the intended game pipeline | Boundary to preserve |
|---|---|---|
| `.q42` | Semantic entities, scene identity, relations, rules, permissions, provenance, project and event state | References dense assets by stable identity and digest; does not absorb mesh or model-weight payloads |
| `.10d` | Dense spatial/visual geometry and applicable 10D sections | Canonical extension is `.10d`; animation representation and browser playback need an explicit tested contract |
| HCF | Hypermedia content/document representation for authored material | Distinguish document content from the archive carrying multiple files |
| `.hmc` | HyperMedia Container that bundles intact content/assets for distribution | Verify the **implemented** bundle format; do not assume draft Bao streaming or an interchangeable ZIP implementation |
| VibeScript/REPL | Capability-bounded authoring, preview, diagnostics, and scenario behaviour | Scripts propose/evaluate through published host capabilities; never directly edit Q42 bytes or bypass game validation |
| `.p64` | Model weights and inference metadata | Separate from Q42 truth/provenance; include only if a bounded local-inference feature is enabled and verified |

The current VibeScript core spec explicitly identifies `.10d` as canonical and
`.d10` as a non-normative alias. The HCF draft still uses `.d10` in examples;
the game must not make that spelling into a second asset format. QualiaDB's
`container_10d` code implements mesh, Tensor10D nodes, topology, spatial index,
and provenance sections, but its current section enum does **not** define an
animation section. Animation APIs exist elsewhere in Vibe/Render; the file-to-
runtime animation contract therefore needs design and proof.

The live core `.hmc` bundle concatenates intact, aligned files with an index,
whole-file CRC-32C, and per-entry SHA-256. QualiaDB's own implementation
divergence log says the HCF draft's Bao/BLAKE3 claims do not match that code.
There is also a `qualia-semantic-library` `.hmc` ZIP implementation. Which
producer and reader are canonical for a game content pack must be decided and
tested before shipping; an extension alone does not establish interchange.

## Upstream subtasks

| ID | Priority / dependency | QualiaDB work and evidence | Acceptance for Maslows Challenge |
|---|---|---|---|
| QG-01 | Gate for all packs | Decide the canonical game-pack `.hmc` producer/reader and version. Reconcile core `bundle/` with the semantic-library ZIP variant and update stale HCF/Bao and `.d10` prose or implement a versioned migration. | One `.hmc` built by the chosen producer opens in native and selected browser profile; its manifest, digests, rights, and entry kinds are verified; incompatible variants fail clearly. |
| QG-02 | Gate for first 3D scene | Expose QualiaDB asset authoring from geometry recipe/mesh input through validation, `.10d` compilation, Q42 manifest, and HMC packaging. | Two distinct original props build reproducibly; malformed topology/units/digest fail; browser loading and semantic picking reach the matching Q42 IDs. |
| QG-03 | Gate for animated character or object | Make `.10d` fit for game animation with a versioned rig/motion representation or linked `.10d` sections, bound to Q42 semantic objects, Vibe `Animation.*`, Webizen rendering, and event time. Extend the format where required; existing mesh sections do not yet prove skeletal animation. | A small rigged or articulated original asset plays two named actions in browser, survives pack/reload, supports reduced motion, and replays to the same state at recorded times. |
| QG-04 | Gate for authored scenario iteration | Expose a game-scoped VibeScript/Poet REPL host profile with discoverable **live** capabilities for graph query, rule evaluation, preview, action proposal, and diagnostics. Preserve gas/effect scopes and no direct state mutation. | Edit and run one scenario cell against a fixture, inspect its proposal and rule receipt, reject an unauthorised call, and repeat in the selected browser/authoring host without a private JS bridge. |
| QG-05 | Gate for offline saves | Define Q42 ↔ HCF/HMC pack references and mutable-save separation: immutable content identity, asset digests, rule/format versions, local event state, migration, and failed-write recovery. | Load a pinned pack, save, reload, replay, and reject a tampered or incompatible asset without silently reinterpreting the world. |
| QG-06 | Conditional on local NPC inference | Connect an approved `.p64` model to QualiaDB's graph-scoped inference and Vibe/game proposal gateway without packaging model weights into Q42. Decide whether P64 is a separate optional download or HMC entry. | One bounded local call shows model/version, allowed context, latency/memory, and validated proposal; disabling or missing P64 leaves the authored journey complete. |
| QG-07 | Conditional on data-shaped terrain | Join QualiaDB source snapshots, geospatial transforms, terrain/mesh generation, `.10d`, Q42 spatial identity, and HMC packaging. | A licensed or fictional heightfield yields a versioned offline pack with visible source/transform evidence, stable coordinates, scene picking, and repeatable generation. |
| QG-08 | Later, only if on-demand place streaming is chosen | Design QualiaDB-owned range fetch, source policy, cache, geometry repair, tile seams, budgeted WASM/GPU lifecycle, and deterministic snapshot/replay for HTTP geography. | Same saved snapshot replays offline; bounded traversal releases old tiles; missing or malformed source fails visibly. No direct game-side HTTP/mesh stack. |

## Work order and completion rule

The RTS uplift adds QG-09 through QG-18 for deterministic multi-agent ticks,
RTS controls, pathfinding, large-scene rendering, animation, game packs,
mutable Q42 state, Vibe authoring, terrain tooling, and browser input/audio.
Their requirements and acceptance fixtures are specified in the
[RTS uplift blueprint](17-rts-aaa-uplift-blueprint.md#8-required-qualiadb-upgrade-work).
The [upstream gate work orders](19-qualiadb-upstream-gate-work-orders.md)
give the concrete Qualia entry points, implementation sequence, failure
fixtures, and browser completion receipts for QG-01–QG-18.
Treat them as continuous upstream dependencies alongside QG-01–QG-08;
implementation evidence and status belong in the living ledger.

Start with **QG-01, QG-02, QG-04, and QG-05** as Phase 0/1 integration
contracts. QG-03 is required before promising animated 3D characters, even if
static props are working. QG-06 is optional for v0.1 because authored dialogue
must remain complete without a model. QG-07 precedes a public-data-shaped
terrain pack. QG-08 is deferred by D-041.

The numbered packets are a starting backlog, **not an exhaustive one-time
audit**. A newly discovered block creates another QG subtask and leaves its
dependent game work visibly blocked. After the upstream capability is changed,
its specification and tests pass, and a pinned game integration succeeds,
resume that game work. Re-scope a feature only through an explicit product
decision; a missing ecosystem capability is not itself grounds to replace or
silently remove the feature.

For each subtask, record the QualiaDB owner/crate, issue or plan link, tested
commit, format/API version, native and WASM tests, performance budget, and game
integration receipt in the capability ledger. “Exists in a document” and
“compiles somewhere in the monorepo” are not completion evidence.

Relevant local starting points: `crates/qualia-core-db/src/container_10d/`,
`crates/qualia-core-db/src/bundle/`,
`crates/qualia-semantic-library/src/container/`, `crates/vibe/`,
`crates/qualia-core-db/src/poet_host/`, `crates/webizen-render/`,
`docs/manuals/standards/vibescript-core.md`, and
`docs/manuals/standards/q42-implementation-divergence.md` in QualiaDB.
