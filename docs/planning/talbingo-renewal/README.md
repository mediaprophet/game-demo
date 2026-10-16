# Talbingo renewal: implementation handbook

Status: proposed implementation plan, 17 October 2026 (Australia/Sydney). This delivery changes documentation only. No improvement described here should be reported as implemented until its acceptance evidence exists.

## Intended result

Turn the existing six-site Talbingo prototype into a coherent camping expedition game: enjoyable driving, grounded walking, distinctive places, practical activities, meaningful choices, readable presentation, convincing sound and animation, and reliable continuation after closing the browser. Keep the QualiaDB/Webizen platform and preserve the separate fictional main campaign.

The radical improvement is a complete player experience, not a larger list of rendering effects. First ship one excellent round trip at The Pines with save/reload and accessible controls. Expand the proven systems across all six sites. In a separately gated geographic milestone, replace illustrative geometry with reviewed source-backed site and road packs. Both milestones are part of this roadmap; the illustrative release must remain labelled honestly.

## Reading and implementation order

| File | Purpose | Read when |
|---|---|---|
| [00 — Current build audit](00-current-build-audit.md) | Evidence, strengths, defects, uncertainty and existing entry points | Before making estimates or changes |
| [01 — Product and gameplay](01-product-and-gameplay.md) | Expedition loop, six-site activities, progression, decisions and content | Before implementing any new feature |
| [02 — Experience and accessibility](02-experience-and-accessibility.md) | Screens, controls, mobile layout, camera, accessibility and recovery | With every player-facing change |
| [03 — Architecture, state and platform](03-architecture-state-and-platform.md) | Ownership, modules, commands, transitions, saves and upstream interfaces | Before moving authoritative state |
| [04 — Driving, traversal and simulation](04-driving-traversal-and-simulation.md) | Road scale, contact, collision, vehicles, walking and environment | Before expanding world size or physics |
| [05 — World content and provenance](05-world-content-and-provenance.md) | Content schemas, geography, ecology, source review and six-site packs | Before publishing place claims or adding assets |
| [06 — Art, audio and rendering](06-art-audio-and-rendering.md) | Art direction, materials, terrain, lighting, animation, audio and review | Alongside the playable slice |
| [07 — Performance, reliability and security](07-performance-reliability-and-security.md) | Budgets, measurement, loading, lifetime management and failure handling | From the first foundation change |
| [08 — Testing and release](08-testing-and-release.md) | Runnable baseline, test matrix, evidence, CI, release and rollback | Before claiming a milestone complete |
| [09 — Delivery backlog](09-delivery-backlog.md) | Ordered work packages, dependencies, estimates and first tasks | To select and track implementation work |

Paths in this handbook are relative to `game-demo` unless explicitly prefixed with `qualiaDB/`. New filenames, records and interface names are proposals; inspect their owning APIs before writing code. A code-shaped example specifies a contract, not a drop-in implementation.

## Ground rules and defaults

1. **One authoritative world.** Qualia owns state, validation, simulation, spatial processing, asset compilation, rendering and persistence. Browser code gathers input and presents results. Follow [the development contract](../13-qualiadb-only-development-contract.md).
2. **Evidence before capability claims.** Existing source, a compiled WASM export, a successful GPU frame and a replay test prove different things. Record each separately. Missing generic capabilities are implemented and tested upstream, then pinned here.
3. **All six sites remain in scope.** The Pines is the first implementation slice, not a reason to leave the other five as reskins. Each site has a distinct composition, interaction and route identity.
4. **Honest geography.** The current game is illustrative. UI coordinates, terrain, species and geology require independent provenance review before scientific or geographic claims. An external map link does not validate local geometry.
5. **No automatic scope expansion into services.** The base game works without accounts, multiplayer, runtime AI, live map acquisition or online telemetry. Those are later proposals with separate capability and privacy reviews.
6. **Accessible participation.** Assistance changes controls and timing, not story worth or access to the ending. Player choices are intentional; the input handler never selects a hidden correct answer.
7. **Small reviewable increments.** Each implementation task includes a visible outcome, failure path, tests, evidence and migration/rollback implications. Preserve previous valid saves and the fictional campaign.

Working defaults: single-player browser-first; keyboard/mouse, touch and gamepad; offline continuation after an explicitly installed pack is verified; English source text with localization keys; original stylized Australian landscape; no score or ranking of real people. A warm, readable presentation replaces the current instrument-heavy interface. These are planning recommendations, not a claim that the user has accepted every individual design detail.

## Relationship to earlier plans

This handbook provides the concrete Talbingo work breakdown beneath [plan 31](../31-talbingo-campgrounds-and-drivable-site-maps.md), [connected gameplay modes](../38-connected-gameplay-modes.md), and the wider [master graphics/simulation direction](../42-definitive-aaa-simulation-master-plan.md). It does not cancel requirements in those plans or change engine policy.

For current implementation status, use source and reproducible evidence in the audit. Historical documents contain ambitions and completion assertions that are not proof of the complete Talbingo browser path. In particular, formula-based terrain is not surveyed terrain, standalone Web Audio is not proof of an upstream acoustic engine, and the 42 MiB semantic arena is not the entire application's memory budget. Resolve discrepancies in the capability ledger before implementation proceeds.

## What completion means

- A newcomer can prepare, drive, park, explore, make a meaningful camp decision, do an optional activity, pack up and return without developer help.
- All six destinations have distinct, completed content; every outbound and return path is tested.
- The same rig, inventory, decisions, clock and place survive mode changes, save/reload and replay.
- Walking, wheels, interaction reach and water/obstacle bounds agree with rendered geometry.
- Mobile and keyboard-only play can reach the same outcomes; failures explain how to recover.
- Supported device tiers meet agreed performance targets with measured evidence, and unsupported configurations get a usable explanation.
- Geographic release additionally passes source, directed-route, terrain seam, rights and attribution gates. Illustrative release alone does not fulfil that later milestone.

Start with the audit and milestone M0 in the [delivery backlog](09-delivery-backlog.md). Do not start by rebuilding all six scenes or adding a new renderer.
