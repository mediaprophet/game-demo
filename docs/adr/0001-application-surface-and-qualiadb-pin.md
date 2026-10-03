# ADR 0001: Full QualiaDB WASM engine for Maslows Challenge

Status: Accepted, supersedes the earlier `portal` + `wasm-webcivics` spike choice.
Date: 2026-10-02

## Decision

The game depends on `qualia-core-db` with `wasm-full` and on Qualia's
`vibe-wasm` host. It uses `QualiaPortal` for 3D rendering and semantic picking,
Qualia computational geometry for scene authoring, sealed `.10d` assets,
Qualia graph and logic capabilities, and Qualia OPFS storage. The engine and
its licence remain QualiaDB's. WebCivics is a separate capability profile and
is not a game dependency or destination for missing game features.

The tested QualiaDB checkout for the two-territory build is tag
`v0.0.40.11` (commit `19e2abe448bf6f5d9e87067ffe51ef889910bb6d` on branch
`0.0.40.6`). That commit is the upstream QG-12 black-viewport fix. The
previous observation was `32ef0175` (WASM bridge duplicate-export fix only).
The game still uses a sibling path dependency at `../qualiaDB` (historically
`C:/github/qualiaDB`); that path can move, so the revision string alone is an
observation, not an immutable Cargo pin. Record a clean checkout and browser
conformance with each build. Canvas soft-rise is allowed only after visual
confirm: a scene receipt is not paint. A later release should bind the
dependency to an immutable revision.

## Implementation rule

When the game needs a capability that Qualia lacks, add that capability to
QualiaDB, verify it there, then expose it through a thin game shell. Do not
implement a parallel renderer, geometry engine, graph store, language runtime,
or asset format in the game. JavaScript handles browser input and display.

The game can run without loading an LLM. `wasm-full` remains the build profile
so the full engine is available for future game systems.

## Verified and outstanding

The `wasm32-unknown-unknown` workspace check and release `wasm-pack` build
pass with `wasm-full` and QualiaDB primitive assembly. In the local browser,
`QualiaPortal` reaches GPU tier 2, ingests six provenance-bearing `.10d`
meshes, and the scripted ten-action playthrough plus replay pass. OPFS
save/reload restores the powered workshop and seven scene meshes. The
in-game VibeScript workbench evaluates an expression. GPU semantic picking
selects the market and workshop nodes; a missed click clears selection.
The latest caller-buffered scene assembly and pick-state reset passed the
release package build and browser check.

The current session still stores its world as N3 text in the game shell.
Moving authoritative mutable graph state, replay, and Q42 persistence into
QualiaDB is the next upstream capability task; ADR 0002 documents the
temporary implementation and its limit.
