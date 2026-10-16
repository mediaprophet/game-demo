# 08 — Testing and release

Status: implementation instructions. Commands below were inspected in the repository, not executed as acceptance tests for this documentation-only delivery. Proposed scripts are clearly labelled. Test a small behavior with a meaningful invariant; do not add tests that merely mirror internal function structure.

## 1. Establish a reproducible baseline

Run from the `game-demo` repository in PowerShell. Read the upstream contribution instructions before changing QualiaDB. The local build requires the sibling checkout, Rust's WASM target, Node, wasm-pack and the matching wasm-bindgen CLI/cache. The current build script expects wasm-bindgen `0.2.125` and uses an offline install when missing. Record tool versions; do not automatically update them to make a failing build pass.

```powershell
git status --short
git rev-parse HEAD
Get-Content qualia.ref
git -C ../qualiaDB rev-parse HEAD
git -C ../qualiaDB status --short
node --version
cargo --version
rustup target list --installed
wasm-pack --version
```

If the upstream checkout does not match `qualia.ref`, record the discrepancy. Do not reset another person's working tree. Build in an agreed clean checkout or finish the upstream change and intentionally update the pin with evidence.

Existing build/check commands:

```powershell
cargo fmt -p rolling-commons-shell -- --check
cargo check --workspace --target wasm32-unknown-unknown --locked
cargo test --workspace --target wasm32-unknown-unknown --no-run --locked
powershell -NoProfile -File scripts/build-game.ps1
node scripts/validate-community-place-pack.mjs web/fixtures/places/fictional-community-v1.json
node scripts/test-game-wasm.mjs
node scripts/verify-game-release.mjs
```

Run the build only when the binaries need rebuilding. It updates generated artifacts; record/review that diff separately from source. `cargo test ... --no-run` compiles tests and **does not execute them**. The shell crate has `#![cfg(target_arch = "wasm32")]`; native cargo tests alone cannot exercise its browser code. Execute applicable upstream native tests plus the WASM/browser path for each capability change.

For a manual baseline, `node scripts/serve.mjs` serves on `http://127.0.0.1:8089`; open `/talbingo.html` (the root defaults to the main game). Stop the owned server after inspection. For the existing automated Talbingo check, set `PLAYWRIGHT_CORE` to an installed package directory and, if needed, `BROWSER_CHANNEL` to a supported installed channel:

```powershell
# Replace the placeholder with your actual package directory.
$env:PLAYWRIGHT_CORE = '<absolute-path-to-playwright-core>'
$env:BROWSER_CHANNEL = 'chrome'
node scripts/check-talbingo.mjs
```

The placeholder is not executable setup. Today the script uses request interception at `https://talbingo.invalid`, so it does not require the dev server, and overwrites `docs/qa/talbingo-*.png`. Preserve existing evidence first or implement QA-01's output-directory option before capturing. It launches with special WebGPU flags, so a pass does not establish behavior in a normally launched browser. Add an ordinary-browser production smoke path.

## 2. What the current checks prove

| Existing check | Actual coverage | Important gap |
|---|---|---|
| `check-talbingo.mjs` | 30 road samples, six outbound + six inbound simulations with rig 0, six landmark reaches, six scene captures, one UI round trip at The Pines, mobile controls visibility | No durable saves/replay, full per-site UI loop, obstacle collision, realistic contact, controller disconnect, mobile usability or performance gate |
| `test-game-wasm.mjs` | Shared campaign rules/replay, main drive module and asset/runtime checks | Loads `drive-simulation.vibe`; not proof of Talbingo session/pack correctness |
| `verify-game-release.mjs` | WASM digest and three campaign art packs, 40 authored scene states, dependencies and main entry markers | Not a Talbingo content/release manifest verifier |
| Place validator | Existing place-pack contract and referenced content checks | Not a review of geographic truth or every proposed Talbingo schema |
| `ci.ps1 -Step test` | WASM-target test compilation | No runtime execution |
| Pages workflow | Pin, compilation, export and static deployment | Does not currently invoke all runtime/browser acceptance checks |

Log/page errors currently print in the Talbingo browser check. Update it to collect and fail on unexpected errors. Any known warning allowlist must be exact, justified and time-limited. Selectors must use stable action/test IDs and accessible labels, not English status prefixes or a `data-correct` shortcut that encodes the expected answer.

## 3. Test layers and ownership

1. **Content validation:** schema, IDs, graph references, route topology, source records, budgets and pack hashes. Runs without a GPU.
2. **Upstream capability tests:** canonical command/replay, physics/contact, bounds, loader cancellation, persistence/version migration and audio/render contracts. Use the owning Qualia crate/profile.
3. **WASM integration:** actual built artifacts, Talbingo rules/fixture, shared session, state transitions and import/export. Must assert outcomes, not only export existence.
4. **Browser integration:** presentation, input, context lifecycle, renderer load and real save/reload. Pair synthetic deterministic fixtures with the packaged scene.
5. **Visual/audio/accessibility review:** fixed viewpoints, actual listening, keyboard and screen-reader passes, physical touch device. Automated screenshots cannot replace these.
6. **Playtest:** first-time player comprehension, pacing and decision quality. Observe without coaching and capture the actual points of confusion.

Keep tests near the owning modules where possible. Proposed new scripts: `scripts/check-talbingo-session.mjs`, `scripts/check-talbingo-accessibility.mjs`, `scripts/profile-talbingo.mjs`, `scripts/verify-talbingo-release.mjs`. These do not exist yet. Add them through the existing toolchain; assess new tooling through the project contract before introducing dependencies.

## 4. Deterministic gameplay matrix

Each case has fixed seed, pack/rules hashes, rig, scenario clock, inputs and expected state receipt. Assert blocked commands leave world state unchanged except for permitted diagnostic receipts. Use input tapes for simulation tests; use public UI controls for end-to-end interaction tests.

| Case | Action and assertion | Required coverage |
|---|---|---|
| T01 New expedition | Select site/rig, prepare, start; exactly one journey committed | Every site; all supported rig profiles |
| T02 Outbound/return | Reach correct directed endpoint, park, return; one arrival/departure event each | 12 directions; initial smoke all six with base rig; full release 6 × 2 × supported rigs |
| T03 Road agreement | Sample center, tangent, height, width and contact along route and at seams | All routes; dense/adaptive samples, not only five x values |
| T04 Traversal | Walk around tree/rock/rig, try water edge/steep slope, compare diagonal vs axial speed | Every collision category; each site spawn→required anchor→exit reachable |
| T05 Intentional decision | Inspect; choose each allowed response using keyboard/touch/controller; inspect consequence | All authored outcomes; E/Enter never selects a hidden answer |
| T06 Repeat commands | Send duplicate activity/arrival command IDs; reject or return original receipt | No duplicated items, awards, time or expenditure |
| T07 Mid-drive resume | Drive, save, close context, load; compare position, rig state, hazard state and route | Every site/direction at least once; representative all rigs |
| T08 Mid-activity resume | Save partial setup/tool use, reload and complete | Each activity template and pending decision |
| T09 Replay | Replay seed + accepted input/command tape; compare canonical state hash | Each outcome and mode boundary; render FPS changes must not alter result |
| T10 Invalid transition | Change site mid-load, cancel, retry, missing destination pack | Previous valid state survives; stale request cannot commit |
| T11 Condition continuity | Damage/heat/load rig, enter camp/workshop, return to drive | No reset or duplicated repair; blocked parts operation spends nothing |
| T12 Time/environment | Pause, hide tab, change allowed scenario setting, reload | No wall-time catch-up; explicit setting command and deterministic consequence |
| T13 Recovery | Fault storage, renderer, script and pack independently | Explain recovery; last good durable checkpoint remains readable |
| T14 Isolation | Play Talbingo, open fictional campaign, save/reload both | No cross-scenario inventory, route, origin or save overwrite |
| T15 Geographic pack | Wrong CRS/order, bad seam, one-way return, missing DEM, stale access source | Fail/flag honestly; never substitute a schematic drive as mapped |

Avoid a combinatorial explosion on every commit. PR smoke = all six scenes and base-rig routes plus touched feature cases. Full release = complete site/direction/rig traversal and every activity outcome. Use pairwise environment/quality/input combinations, plus known high-risk combinations (mobile towing, save during transition, denied storage). Publish the selected matrix so exclusions are visible.

## 5. Presentation and human acceptance

Maintain proposed `web/fixtures/graphics/talbingo-review-cameras.json`: site/scene IDs, camera pose, time, weather, seed, quality tier, viewport and content digest. Capture overview, arrival, driver view, camp approach, landmark interaction and dusk/light test. Each site needs at least overview/arrival/interaction; fully sample the first slice and risky art changes. Store images by revision/run, never as an unlabeled latest screenshot.

Review world scale, grounded feet/wheels, silhouette, material identity, terrain/water joins, clipping, distracting aliasing, far-field transitions, camera collision and mobile HUD overlap. Compare screenshots only on a fixed renderer/browser configuration with tolerances. A human signs off changes that are intentional; do not blindly update image baselines after a diff.

Listen on headphones and ordinary speakers at moderate levels. Verify master mute, separate ambience/effects control, no clicks at transitions, no looping stale motor sound at camp, balanced warnings and equivalent visual cues. Rendering a waveform is not audio QA.

Use chapter 02's accessibility specification for actual keyboard navigation, visible focus, dialog escape/return, text resizing, touch reach, color independence and screen-reader status. Target WCAG 2.2 AA for applicable web UI and document assessed scope/remaining game-specific limitations rather than making an untested conformance claim. The relevant normative reference is [W3C WCAG 2.2](https://www.w3.org/TR/WCAG22/); accessibility participation also requires hands-on playtesting beyond those UI checks.

Suggested first usability round: 5–8 people unfamiliar with the build, including keyboard-only and touch play. Observe whether they can start within 90 seconds, find the first interaction without hints, explain a camp tradeoff and resume after a reload. Targets: at least 4 of the first 5 complete a round trip unaided; no participant loses progress; record small-sample limitations. Ask what confused them and what consequence they noticed, not just whether they liked it. Fix the biggest failure and repeat with new participants.

## 6. Evidence and release artifact

Proposed QA path: `docs/qa/talbingo-renewal/<run-id>/`. Each run gets an `evidence.md` summary and machine-readable receipts from supported tools. Keep bulky traces outside Git when necessary with a stable reference/hash and retention policy. Proposed run receipt fields:

```text
runId, localDateAndZone, gameCommit, dirtyTreeDigest,
qualiaCommit, qualiaRef, wasmSha256, packHashes, rulesHash,
toolVersions, deviceAndBrowser, rendererTier, settings,
testCases[], commandAndExitStatus[], timings[], memoryComponents[],
screenshots[], knownFailures[], reviewer, releaseDecision
```

No field is evidence merely because it appears in this example. Fill it from actual execution. Record blocked tests as blocked with a reason; never convert unavailable GPU measurements to zero or a skipped test into a pass.

The proposed Talbingo release manifest binds entry modules, WASM, site/route packs, rules, schema/save versions, content digests and the exact Qualia revision. Add verification of all dependencies and unsupported schema versions. Existing main-game manifests stay valid and separately checked.

## 7. CI and rollout

Extend `.github/workflows/game-pages.yml` after tests become portable:

1. Verify game/upstream/toolchain inputs and content schema.
2. Compile/check/build the pinned WASM and packs.
3. Run shared WASM tests, Talbingo session tests and both release verifiers.
4. Run GPU-independent browser smoke; run the GPU matrix on a proven GPU-capable runner/device job. A software-adapter pass must be labelled as such.
5. Preserve failure traces/screenshots; fail publication for mandatory failures or missing required release evidence.
6. Upload one immutable tested artifact; deployment uses that same artifact, not a second build.
7. Perform a normal-browser deployed smoke and record the artifact revision displayed by the app.

Keep PR validation separate from publication. The existing workflow deploys on main pushes; implementation must account for that before merging. This plan authorizes no deployment by itself.

Save migration rollout: keep old schemas readable for the defined compatibility window (proposed current + previous release); migrate to a new slot/checkpoint, verify replay/invariants, then mark it active. Keep the original until successful load is confirmed. If a rollback build cannot read the new schema, offer the preserved older checkpoint with an explicit progress date; do not pretend rollback retains newer actions. For source-backed packs, include original pack references or supported migration rules for changed geometry and spawn points.

Release gates: no open save-loss/authority/false-source-claim defects; all six complete loops; tested accessibility and controls; measured tier budgets; successful failure/recovery cases; correct source/rights labels; main-campaign regression pass. Roll back the deployed artifact if first-run load, persistence or route completion fails; preserve diagnostic evidence and user saves.

## 8. Work packages

| ID | Concrete work | Dependencies | Done when |
|---|---|---|---|
| QA-01 | Make Talbingo harness paths/channel/output configurable, capture versions, fail unexpected errors; document environment setup | ARC-01 | Runs from a clean documented machine without one user's home path |
| QA-02 | Add authoritative session/command/save/replay fixtures and rejection cases | ARC-03/05, PLAY slice | T01, T05–09, T11–14 receipts pass |
| QA-03 | Expand six-site UI/input/accessibility and visual review cases with stable selectors | UX work, DATA scenes, SIM traversal | Complete loops and actual controls, not only visibility assertions |
| QA-04 | Add performance/fault/leak matrix and physical-device review | PERF/REL/SEC work | Budgets and recovery table have measured receipts |
| QA-05 | Add Talbingo release manifest verification, portable CI jobs and deployment gate | ARC-06, QA-01..04 | Only the exact tested artifact can publish |
| QA-06 | Run staged playtests and release rehearsal including migration and rollback | Complete slice, then six sites | Unaided completion target met; rollback preserves compatible checkpoints |

Attach the receipt path to each completed task in chapter 09. A checked box without evidence is still pending verification.
