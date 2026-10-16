# 00 — Current build audit

Audit date: 17 October 2026. Method: read local source, build scripts and prior plans; inspect committed desktop/mobile QA images. No fresh browser playthrough, cargo suite or performance benchmark was run for this documentation task. Existing screenshots and tests are evidence of available review material, not a fresh pass of the current build.

## Baseline identity

| Item | Observed value |
|---|---|
| Game checkout HEAD | `b2bfe7cdc5e75e7423c855ded420fb0bb24a2d3e` |
| `qualia.ref` | `7bfc84d4122bbd8549134ab9eb589f7e5cd2cb73` |
| Sibling QualiaDB HEAD | Same revision as `qualia.ref` when inspected |
| Initial working trees | Clean game and QualiaDB working trees when inspected |
| Browser entry | `web/talbingo.html` |
| Existing compiled WASM size | 9,668,474 bytes on disk; not a download-size or peak-memory measurement |
| Core runtime dependency | Sibling `../qualiaDB`, shell feature `wasm-full`, plus Vibe/Vibe-WASM |
| Six sites | The Pines, Humes Crossing, Jounama Creek, Yarrangobilly Village, O'Hares, Three Mile Dam |

The game Cargo workspace uses a path dependency, so `qualia.ref` alone does not enforce a runtime pin. The build script and Pages workflow check the upstream checkout. Dirty upstream changes, generated artifacts and runtime versions must also be captured by a release receipt.

## Keep these foundations

- Six actual scene recipes with differing terrain, vegetation, water and landmark arrangements; they are not merely six labels on one scene.
- `GamePortal`/Qualia `.10d` loading, textured assets and controlled actor roots already used by Talbingo.
- A checked Vibe module for drive response and bounded local walking. Main-game systems provide useful contact, workshop, journal and animation integration entry points.
- Outbound drive, local walk/inspect/choice and inbound drive already joined in a small loop.
- Keyboard, touch and gamepad input; chase, drone and orbit camera modes; basic synthetic audio.
- A Talbingo browser check that covers road agreement, route simulation, six scene captures and one end-to-end round trip.
- Existing source/provenance, pack, release and graphics plans to extend rather than duplicate.

## Findings and implementation implications

Priority: P0 = blocks reliable foundations or misrepresents evidence; P1 = central quality gap; P2 = depth/polish after foundations. These are proposed implementation priorities, not a claim that every source observation has been runtime-reproduced.

| ID / priority | Source evidence | Consequence | Planned owner |
|---|---|---|---|
| A01 / P0 | `web/talbingo.html`: `phase`, `acted`, `inspected`, `eventCleared`, arrival checks and choice handlers | Authoritative progress is browser state; Talbingo does not use campaign save/replay | ARC-03, ARC-05 |
| A02 / P0 | `SITES`, `display()`, `step()` show hard-coded distances/elevations/geology and interpolate GPS between endpoints | Scientific-looking precision lacks source records; simulated path is not mapped road geometry | DATA tasks; immediate claim correction |
| A03 / P0 | `talbingo_terrain.rs::elevation` uses mathematical formulas while comments claim surveyed/authentic terrain | Documentation and UI evidence can mislead implementers and players | DATA tasks, ARC-01 |
| A04 / P1 | `display()` compares a fixed `correct` index; E/Enter/gamepad A search `[data-correct="true"]` | Choice becomes answer selection; input can bypass deliberation entirely | PLAY tasks, UX tasks |
| A05 / P0 | `loadSite()` changes `site`/phase before building; selection remains usable during travel; reset discards local booleans | No staged transition, durable checkpoint or explicit recovery from failed scene load | ARC-04, REL-02 |
| A06 / P1 | `begin()` invokes `sim.setup(0)` on each journey; drive input uses `rig_id:0`, zero terrain heights, `period:'day'`, `crossing_open:true` | Vehicle variety, thermal state continuity and terrain-derived behavior are not integrated | SIM-01..03, SIM-06 |
| A07 / P1 | `lib.rs::talbingo_vehicle_pose` / `talbingo_walker_pose` and `camp_step` | Root movement and bounded walking exist; terrain contact, obstacle-aware traversal and useful gait/heading need explicit integration | SIM-02, SIM-04, ART tasks |
| A08 / P1 | `camp_step` adds x/z independently, clamps bounds and uses a few water strips | Diagonal speed and scene-specific blocking need tests; rocks, trees and camp props are not complete collision geometry | SIM-04 |
| A09 / P1 | Hazard object is synthesized at x=8; browser `eventCleared` removes it after one event | Wildlife is mostly a one-off trigger, not a visible persistent actor with lifecycle and behavior | SIM-05, PLAY tasks |
| A10 / P0 | `frame()` catches errors, sets phase error and returns before the next animation request | UI reset does not itself restart a stopped scheduler; recovery must be designed | REL-02 |
| A11 / P1 | `blur` clears `keys` only; `pollGamepad` mixes keys and keyHeld; keyboard guard excludes only the select | Sticky/mixed input, lost focus and dialog conflicts need dedicated input lifecycle handling | UX tasks, REL-03 |
| A12 / P1 | `HolosonicEngine` constructs local oscillators/noise/gains directly; mute zeros several gains but omits `floraGain` | Audio is not an upstream acoustic integration; master mute and audio lifecycle need correction/proof | ART audio tasks, REL-03 |
| A13 / P1 | Committed [overview](../../qa/talbingo-site-0.png) and [mobile](../../qa/talbingo-mobile.png) images | Pale world/weak focal hierarchy; instrument-heavy UI; mobile controls overlap bottom readouts | ART tasks, UX tasks; reproduce before tuning |
| A14 / P1 | `loadSite()` calls `talbingo_scene` synchronously and loads full scene bytes | Scene construction/loading work needs measurement, cancellation and bounded resource ownership | PERF-01..03 |
| A15 / P0 | `check-talbingo.mjs` logs console/page errors but does not directly fail on them; mobile check only asserts control visibility | A logged error or visible controls can coexist with a broken release | QA-01..03 |
| A16 / P0 | Pages workflow builds/exports but does not invoke Talbingo browser check, main WASM smoke or release verifier | Publication has weaker gates than the available local tooling | QA-05 |
| A17 / P1 | `check-talbingo.mjs` requires a user-specific default Playwright path; main `lib.rs` is wasm32-only | Setup is not portable; native `cargo test` alone cannot prove shell behavior | QA-01 |
| A18 / P1 | `GameSession`, `session_journal`, `drive_session` belong to the wider game; ADR 0002 explicitly describes a temporary implementation | Existing main-game persistence is a reuse candidate, not evidence Talbingo already has the required canonical Q42 session | ARC-01, ARC-03..05 |

## Source map for an intermediate implementer

| Area | Existing files / symbols | First useful investigation |
|---|---|---|
| Page and controls | `web/talbingo.html`: `SITES`, `display`, `loadSite`, `begin`, `step`, `pollGamepad`, `frame` | Trace one start → arrive → inspect → choice → return sequence |
| Simulation rules | `web/fixtures/talbingo-drive.vibe`: `drive_setup`, `drive_step`, `camp_step`, road functions | List input/output fields and identify unused supplied capability paths |
| WASM boundary | `crates/rolling-commons-shell/src/lib.rs`: Talbingo exports, `GamePortal`, `GameDriveSimulation`, `GameSession` | Identify which interface owns semantics and which only presents state |
| Scene construction | `talbingo_scenes.rs`, `talbingo_terrain.rs`, `talbingo_vegetation.rs` in the same Rust source folder | Trace geometry recipes → compile → scene load → actor roots |
| Contact and vehicle reuse | `drive_contacts.rs`, `drive_contact_integration.rs`, `vehicle_animation.rs`, `drive_session.rs` | Check assumptions tied to main-game terrain before reuse |
| Persistent world reuse | `session_journal.rs`, `drive_session.rs`, `campaign_simulation.rs` | Read save schema, validation, replay coverage and version handling |
| Existing checks | `scripts/check-talbingo.mjs`, `scripts/test-game-wasm.mjs`, `scripts/verify-game-release.mjs` | Distinguish Talbingo, shared campaign and release artifact coverage |
| Packaging and delivery | `scripts/build-game.ps1`, `scripts/ci.ps1`, `.github/workflows/game-pages.yml`, `qualia.ref` | Record generated artifacts, toolchain versions and pin checks |
| Data contracts | `docs/schemas/community-place-pack.v1.schema.json`, `scripts/validate-community-place-pack.mjs` | Check exact existing fields before proposing a schema migration |

## Evidence limitations to resolve at M0

1. Run a fresh browser baseline at the audited revisions and compare with committed images; do not silently replace historical captures.
2. Measure loading, frame pacing and resource lifetime on specified hardware. No FPS, GPU memory, replay determinism or accessibility conformance is established by this audit.
3. Reconcile all six location anchors: current HTML coordinates differ from the candidate anchors in plan 31. This establishes disagreement, not which values are correct. Verify identities before using either set.
4. Inspect public upstream APIs and tests before marking collision, pack streaming, acoustic rendering or canonical session support available in the browser profile.
5. Re-run shared campaign tests after any change to common Rust modules. A successful Talbingo demo does not prove the main campaign still works.

The complete renewal should address every audit row. The [delivery backlog](09-delivery-backlog.md) provides the implementation order; detailed workstream documents describe the actual contracts and acceptance cases.
