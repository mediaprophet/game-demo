# 03 — Architecture, authoritative state and platform

Status: implementation proposal, based on source inspection on 17 October 2026. No build, browser run or upstream conformance proof was performed for this chapter. Start with the [audit](00-current-build-audit.md) and the binding [QualiaDB-only contract](../13-qualiadb-only-development-contract.md). The existing [capability ledger](../15-living-qualiadb-capability-ledger.md) remains the place to record upstream evidence; this chapter does not declare its historical entries current.

## Outcome

Talbingo becomes a scenario within the shared game runtime. A player can plan a trip, drive, park, walk, complete activities, return, save, reload and replay without resetting vehicle condition or losing site progress. One accepted state produces the scene, HUD, audio cues and accessible interface. A failed scene load or storage write leaves a recoverable session.

Keep the existing Talbingo URL as a scenario entry point. The implementation can initially retain its page while sharing the session adapter with the main game. Merging all markup into `game.html` is not a prerequisite and would make the first migration unnecessarily large.

## Source-verified starting point

| Existing source | What is present | Limit relevant to the renewal |
| --- | --- | --- |
| `web/talbingo.html`: `SITES`, `loadSite`, `reset`, `begin`, `step`, `display` | Six authored scenes, a 50 ms fixed simulation step, keyboard/touch/gamepad input and a rendered round-trip loop | `phase`, `drive`, `walk`, `acted`, `inspected` and `eventCleared` are browser variables. Choices, arrival and completion mutate them directly. No Talbingo save path is wired. |
| `web/fixtures/talbingo-drive.vibe`: `drive_setup`, `drive_step`, `camp_step` | Authored vehicle, thermal, towing and bounded walking policy | The browser supplies both input and much of the state. Driving alone does not authorize campaign transitions. |
| `crates/rolling-commons-shell/src/lib.rs`: `GameDriveSimulation`, `GamePortal`, `talbingo_*` | Vibe evaluation and thin renderer bindings | A successful simulation or pose call is not a durable session transaction. |
| `crates/rolling-commons-shell/src/lib.rs`: `GameSession`, `session_replay` | Existing campaign action validation, N3 world text and accepted action tape | This is the current main-game slice, not proof of a public, durable Q42 session suitable for all Talbingo state. Do not enlarge private text-reducer infrastructure to implement the renewal. |
| `crates/rolling-commons-shell/src/drive_session.rs` | Main-game rig checkpoints, workshop condition checks, `drive_tick`, `drive_resume`, `drive_park`, `present_drive_rig` | The file explicitly describes latest-tick checkpoints, not a full driving event tape. It samples main-game terrain and is not directly compatible with arbitrary Talbingo routes. |
| `crates/rolling-commons-shell/src/session_journal.rs`: `GameSessionJournal` | In-memory Q42 journal for waypoints and observations | `record_waypoint` ignores Y and speed; there is no complete Talbingo load/replay/durability contract here. It is not a ready save solution. |
| `lib.rs`: `world_save`, `world_load` | QualiaDB `OpfsVfs` byte writes/reads | Comments specify bare filenames. Multi-resource atomic commit, torn-write recovery, migration and quota UX are not established by these functions. |
| `qualia.ref`, workspace Cargo files | Requested QualiaDB revision and sibling path dependencies with `wasm-full` | A file containing a revision does not prove the sibling checkout or generated `web/pkg` matches it. Record all three. |

Source extraction must not conceal existing technical debt. In particular, the browser-owned semantic loop, runtime asset compilation and local audio engine predate the target boundary; moving them into separate JavaScript files is only an interim organization step.

## Ownership and module boundaries

| Responsibility | Owner in the target | Game-side material |
| --- | --- | --- |
| Session, clock, state transactions, command validation, deterministic journal, migration | Public, versioned QualiaDB runtime | Scenario schema, initial facts and action policies |
| Movement integration, contact, collision, spatial queries, navigation | QualiaDB with approved Vibe capability access | Rig parameters, route restrictions, terrain/obstacle references |
| Authored consequences, activities, seasonal access rules | Vibe/QualiaDB governed policies | Talbingo rules and content |
| Geometry, terrain, animation, material compilation, scene loading/rendering | QualiaDB asset pipeline and portal | Original recipes and scenario manifests |
| Audio routing and semantic cue playback | QualiaDB audio/runtime surface | Cue catalog, mix presets and captions |
| Input sampling, DOM mounting, focus, presentation-only preferences | Thin browser/Webizen host | Bindings, layout and copy; no accepted-world mutation |
| Save storage, export/import validation and recovery | QualiaDB storage/session interfaces | Player-facing progress and recovery controls |

Proposed game files, introduced incrementally:

```text
web/talbingo.html                       # entry markup only after extraction
web/talbingo/main.js                    # bootstrap and lifetime ownership
web/talbingo/input.js                   # normalized intents; no game rules
web/talbingo/view.js                    # render accepted projection to DOM
web/talbingo/camera.js                  # presentation target requests
web/talbingo/platform.js                # fetch, capability and lifecycle adapter
web/talbingo/talbingo.css               # scoped presentation styles
web/fixtures/talbingo/scenario.*         # versioned authored scenario data
web/fixtures/talbingo/activities.*       # governed actions and dialogue
web/fixtures/talbingo-drive.vibe         # migrate policy without duplicate physics
crates/rolling-commons-shell/src/
  talbingo_runtime.rs                   # proposed thin bindings/scenario composition
  talbingo_content.rs                   # proposed authored data/manifest bindings
```

The extensions and exported function names of new scenario/runtime files are selected only after ARC-01 validates the public format. These are suggested organizational destinations, not instructions to invent a serializer. Avoid a `state.js` module that becomes a second authority.

## Proposed logical contracts

The following tables are requirements for the upstream-backed interface. They are **not existing API names, executable declarations or a new on-disk format**. Map them onto supported QualiaDB schemas; if a necessary concept cannot be expressed, version the upstream interface first.

### Session identities and versions

| Field | Meaning and invariant |
| --- | --- |
| `sessionId` | Stable play-session identity; retries cannot create a duplicate session. |
| `scenarioPackId`, `scenarioPackVersion`, `scenarioPackDigest` | Identify the reviewed Talbingo content independently of the fictional campaign and renderer resources. |
| `schemaVersion`, `runtimeVersion`, `policyDigest` | Define semantic interpretation. A save with mismatched versions is migrated explicitly or refused. |
| `sceneId`, `siteId` | Stable identifiers, never array positions or visible translated names. The existing six numeric indices become legacy adapter inputs only. |
| `routeEdgeId`, `routeDirection`, `routeProgress` | Directed route and physical progress, independent of what camera is shown. |
| `actorId`, `rigId` | Existing shared identities; vehicle condition and inventory are never copied into a Talbingo-only economy. |
| `tick`, `stateRevision`, `lastCommittedSequence` | Integers controlled by the session; logical time, mutation revision and event ordering have separate meanings. |
| `seed`, `randomStreamVersions` | Reproduce authored randomized events; renderer noise has no semantic authority. |

Hashes crossing JavaScript must use canonical encoded bytes or strings. Do not pass 64-bit identifiers through floating-point `Number`; `GameSessionJournal` currently accepts some such identifiers as `f64`, so precision needs an explicit upstream review.

### Authoritative and derived state

Authoritative state must cover:

- Current site, directed route, leg, route/arrival checkpoint and scenario activity state.
- Actor position/orientation/locomotion and occupied vehicle; vehicle pose, speed, towing state, wheel/contact state needed by deterministic integration.
- Per-rig condition, workshop assemblies, cargo and shared inventory references.
- Discovered landmarks, observations, accepted activity choices and consequences by stable ID.
- Simulation time/season, encounter state, random counters, route closures and access decisions.
- Last stable recovery anchor, queued governed commands and transition/save revision where required by the upstream protocol.

Derived projections may contain progress labels, enabled actions, nearest interactable, active warning, gauges and cue events. They must carry the source `stateRevision`. The view may cache them, but cannot make choices legal, declare arrival, clear hazards or grant inventory.

Presentation-only state includes open panels, camera preference, volume, text size and last focused control. Store it in a separate versioned preferences namespace supported by QualiaDB. Restoring preferences cannot change the world hash. An accessibility option that changes driving assistance **does** affect semantics; record its policy setting in session/replay instead.

### Commands and receipts

Each submitted command carries `commandId`, `sessionId`, `expectedRevision`, `targetTick`, `actorId`, a typed `kind`, and a bounded payload. Suggested kinds are `choose_destination`, `start_leg`, `drive_controls`, `park`, `exit_vehicle`, `walk_controls`, `inspect`, `choose_response`, `enter_vehicle`, `recover_vehicle` and `request_mode`.

The browser sends intent, not `newHealth`, `arrived=true` or an accepted pose. For example, choosing a response submits the response ID currently selected by the player; there is no browser `correct` field. Spatial/availability checks happen at acceptance time, including for keyboard/gamepad activation.

An accepted receipt contains `commandId`, `sequence`, `tick`, `previousRevision`, `stateRevision`, accepted events, a projection and its semantic digest. A refusal contains a stable reason code, current revision, player-facing message key and any supported recovery action. Refusals do not advance inventory, route progress or decision history. Diagnostics may log them separately.

Initial refusal vocabulary: `STALE_REVISION`, `WRONG_MODE`, `OUT_OF_RANGE`, `ROUTE_CLOSED`, `RIG_UNSUITABLE`, `RIG_NOT_ROADWORTHY`, `SCENE_NOT_READY`, `INVALID_PAYLOAD`, `RESOURCE_LIMIT`, `VERSION_MISMATCH`. Confirm names in ARC-01; localized prose must not become a machine protocol.

Retries use the same command ID and return the original result without applying the action again. Inputs targeting an already processed tick are rejected or assigned according to a documented upstream policy; do not silently rewind. Cap pending commands and payload sizes in the public interface.

## Mode and scene transitions

Separate the gameplay mode (`hub`, `driving`, `camp`, `workshop`, `trip_complete`) from host lifecycle (`booting`, `loading`, `active`, `paused`, `recovering`, `failed`). An error message must not become an accidental gameplay state.

| Transition | Accepted preconditions | Committed result |
| --- | --- | --- |
| Hub → driving | Destination/route selected; access and rig checks pass; route scene ready | Directed leg and departure checkpoint; existing rig condition preserved |
| Driving → camp | Vehicle inside valid arrival/parking volume, below parking threshold; local scene and spawn ready | Parked rig, visited site, actor exit position; no objective auto-completion |
| Camp → driving | Actor at vehicle or approved accessible return action; trip policy permits departure; route ready | Resume same rig and condition on selected return edge |
| Camp/driving → workshop | Rig at supported workshop/recovery location; parked; workshop scene ready | Shared rig and part state become the workshop subject |
| Return → complete | Valid hub arrival and stop accepted | One trip-completion event and durable progress |
| Any active mode → recovery | Explicit player request or unrecoverable scene problem | Last stable accepted anchor; no unearned progress or duplicate rewards |

Generic transition sequencing belongs upstream:

1. Validate the requested transition at revision R; retain the previous scene/session.
2. Prepare the target pack using a unique transition token and validate its manifest, assets, collision and spawn anchors. Do not advance simulation during blocking load.
3. If another request supersedes it, cancel/discard its resources. A late promise may not change the selected site or active input target.
4. Revalidate revision R and required resources, then atomically commit the mode change with a receipt.
5. Bind presentation to the new revision, transfer focus, clear held inputs and release old scene resources through the supported portal lifecycle.
6. On preparation failure, keep the prior valid state and scene, explain the failure, and offer retry/cancel. On post-commit renderer failure, reconstruct presentation from the committed state; do not replay the world mutation.

Investigate whether the selected portal supports two staged scenes within budget. If it does not, upstream must provide transactional replacement or a proven unload/reload recovery protocol. Do not solve this by maintaining a second game renderer. See [performance and reliability](07-performance-reliability-and-security.md).

## Clock, input and browser lifetime

Retain the existing 20 Hz (`0.05 s`) simulation interval during migration. A higher tick rate requires a new measured policy version and replay fixture; it is not a cosmetic change. Render interpolation uses accepted previous/current poses and never writes them back to the session.

Upstream fixed-tick scheduling owns semantic catch-up. The host measures elapsed time and requests bounded progress; it does not run a new privately authored time model. Establish a maximum catch-up budget, and pause with visible feedback if the budget is exceeded. Browser throttling must not cause hours of thermal damage or route movement on return.

`input.js` keeps independent keyboard, pointer and gamepad channels and produces one normalized control intent per tick. Define priority for competing sources. Normalize diagonal walking, dead zones and analog ranges through the accepted input/control contract. Clear all channels, keyboard latches and button-edge history on blur, visibility loss, controller disconnect, scene change and pause. The existing `keys.clear()` alone does not clear `keyHeld` or all gamepad edge state.

Bootstrap owns exactly one animation callback, resize observer/listener set, input binding set and renderer instance. `dispose` cancels callbacks, removes listeners, releases pointer lock/capture and frees resources via supported interfaces. Retry must start one valid loop; today a caught frame error returns before scheduling the next frame, while reset does not restart it.

## Save, load, migration and replay

### Required save behavior

- Autosave after accepted trip start, parked arrival, activity completion, mode change and trip completion; use a bounded checkpoint cadence during long drives. Coalesce autosaves and display saving/saved/failed states.
- Capture a consistent accepted tick, scene/route identity, shared rig state, activity state and command/journal position. Save does not repair, refuel, complete an activity or move the player.
- A save request during an in-flight transition waits for a stable revision or saves the prior stable revision with an explicit receipt. Never write a mixture of old world and new scene identity.
- Upstream storage writes the new snapshot/journal generation, validates it and changes the committed-generation pointer only after durable success. The exact mechanism is an upstream decision, not game-authored file choreography.
- Retain at least the prior good generation according to a measured retention budget. Storage quota denial must preserve the in-memory session and prior save; offer supported export and cleanup UX.
- Unavailable persistence produces a clearly labelled temporary session. Do not silently claim autosave works. Core play may proceed if the player chooses that supported mode.

### Load validation order

1. Enforce input byte/entry limits before expensive parsing.
2. Validate container, integrity and schema/runtime/content dependencies using QualiaDB readers.
3. Resolve exact pack/policy versions offline where available. A different pack with the same visible name is not interchangeable.
4. Migrate through registered, deterministic migrations on a copy; preserve the original.
5. Check actor/rig references, route range, finite numeric bounds, event uniqueness and activity prerequisites.
6. Prepare the required scene and recover to a supported anchor if an accepted migration requires it; show the migration result.
7. Replace the active session only after all checks pass. A malformed import leaves the current game untouched.

The old standalone Talbingo page has no durable Talbingo save to migrate. Existing campaign saves still need compatibility fixtures. Do not manufacture visited-site history from screenshots or browser labels. Cross-content migration must map stable IDs and explicitly handle retired activities/routes.

### Replay proof

Distinguish decision replay from trajectory replay in both tools and UI. Record the accepted discrete commands plus normalized control segments/tick counts, required external fact snapshots and random seeds. A current/previous drive checkpoint proves only the most recent step, not the trip history.

A replay starts from an exact seed snapshot and pinned pack/policy/runtime, re-executes accepted commands in order and compares semantic digests at checkpoints. The first mismatch report includes tick, sequence, field/group and relevant versions. Rendering and audio need not be pixel/sample identical; resulting accepted state and events must match under the declared determinism contract. Cross-browser numeric behavior must be tested before promising cross-platform replay. If quantization is needed, implement/version it upstream.

Do not record every render frame or unbounded JSON state copies. Let upstream own canonical compact journal encoding, bounded checkpoint intervals and pruning/export policy. Benchmark one hour of input before choosing limits. Save/replay must include pause, load, encounter clearance, rig changes and recovery commands.

## Upstream discovery work, not established APIs

Paths below are relative to the sibling `qualiaDB` repository. Their existence is source evidence only.

| Required proof | Candidate owner / source | Minimum fixture before game integration |
| --- | --- | --- |
| Durable governed session, transactions and ordered replay | `qualia-core-db/src/q42/journal.rs`; `webizen-lite-wasm/src/session.rs`; relevant full-profile host | Accept/refuse/retry one action, persist, reload, replay and recover an interrupted commit in `wasm-full`. The lite session explicitly describes memory-only tools and is not a drop-in runtime. |
| Portal lifetime and replacement/recovery | `qualia-core-db/src/render/portal/{scene_mesh,scene_animation,recovery,capabilities}.rs`, `portal_wasm.rs` | Load A, fail loading B, retain/reconstruct A; repeat load/unload without resource growth. |
| Browser persistence/volume integration | `qualia-core-db/src/q42/volume/opfs_source.rs`, public storage surfaces used by `world_save` | Versioned snapshot+journal restore and failed-write recovery in supported browsers; exact APIs chosen through implementation search. |
| Authored policies and capability host | `crates/vibe`, `qualia-core-db` capability host used by `GameDriveSimulation` | Bounded context query and proposal; no direct browser mutation; denied capability leaves state unchanged. |
| Fixed tick and deterministic state | `webizen-runtime/src/{clock,kernel,snapshot}.rs` | Prove selected WASM profile integration, pause/catch-up semantics and replay equivalence before adoption. |

Add/extend QG work orders in the established ledger rather than inventing an untracked replacement engine. A missing proof blocks the dependent feature, while CSS extraction, content review and fixture design can continue.

## Implementation work packets

Effort ranges are engineering planning estimates, excluding upstream feature development and review; replace them after discovery. Dependencies refer to IDs within this plan.

### ARC-01 — Establish supported contracts and fixtures (2–4 days)

1. Record game commit, actual QualiaDB checkout, `qualia.ref`, toolchain and generated WASM digest. Mark mismatches explicitly.
2. Map each logical contract above to a public upstream owner. Add the minimum failing/passing fixture and the selected build profile to the existing capability ledger.
3. Write schema examples for a site visit, outbound drive, refused inspection and saved rig condition; validate them with the selected upstream schema tools.
4. Create upstream work orders for unproven durability, transactional scene change, full trajectory replay and spatial contact. Link downstream tasks and exact acceptance fixtures.

Done when every required capability has a tested status and owner, not when all gaps are solved. Depends on the baseline audit. Blocks ARC-03 onward and SIM-01/02 contract adoption.

### ARC-02 — Extract the host with behavior preserved (2–4 days)

1. Move scoped styles and module code into the proposed `web/talbingo/` files without changing game rules in the same commit.
2. Give bootstrap one lifecycle owner and remove repeated event/animation registration hazards.
3. Isolate current browser semantic mutations behind a clearly labelled legacy adapter so the migration has one replacement seam. Do not add new game mechanics there.
4. Preserve the Talbingo URL and asset-relative paths in local/Pages builds; verify the existing six-scene smoke path.

Done when the current loop still works and host modules have explicit responsibilities. Requires recorded baseline; independent of upstream fixes. This is organizational work, not completion of the authoritative-state requirement.

### ARC-03 — Bind one authoritative Talbingo session (5–8 days after upstream readiness)

1. Implement scenario facts/actions and stable IDs through the verified QualiaDB interface.
2. Replace `acted`, `inspected`, `eventCleared`, arrival and completion assignments with commands/receipts.
3. Replace `begin()` resets with resume/start-leg commands that preserve shared rig condition.
4. Connect the derived UI projection, remove `data-correct` handling and deny direct pose/state authority from the browser.
5. Integrate the main-game entry/return path with shared actor, rig and inventory identities; Talbingo visits do not overwrite the fictional campaign namespace.

Done when duplicate/illegal commands leave state unchanged and a full loop crosses the command boundary. Depends on ARC-01/02 and the upstream governed-session proof.

### ARC-04 — Implement cancellable scene/mode lifetime (4–6 days)

1. Bind the upstream transition protocol and track transition tokens in the host.
2. Stage/validate target scenes and spawn anchors; transfer controls only after commit.
3. Add input clearing, pause/visibility behavior, restartable render failure and supported device recovery.
4. Exercise rapid site selection, cancel during load, missing scene, bad manifest and repeated retry.

Done when the final requested site wins, no late load overwrites it, and failed transitions preserve accepted state. Depends on ARC-03 and portal recovery proof.

### ARC-05 — Add saves, migration and full replay (5–8 days)

1. Bind the verified upstream snapshot/journal storage path and player controls.
2. Define checkpoint cadence, generation retention and migration mappings; register schemas/version compatibility.
3. Implement import/export/recovery UX without custom parsing or storage formats.
4. Record trajectory inputs and replay checkpoints through upstream journal APIs.
5. Test interruption before/after commit, quota denial, corrupt import, old content and mismatched policy.

Done when a mid-drive save resumes identically and a full round trip plus activity/workshop events replays to the same semantic digest. Depends on ARC-03/04 and the upstream durability/replay proof.

### ARC-06 — Enforce platform and release boundaries (2–4 days)

1. Have CI verify the QualiaDB pin, WASM ABI/toolchain and packaged content/policy digests.
2. Use one capability receipt at startup to choose supported render/input/storage behavior; surface actionable failure states.
3. Remove the legacy semantic adapter once equivalent governed paths pass; prevent unused fallback code from remaining a second authority.
4. Document bootstrap, disposal, commands, save versions and debugging procedures for an intermediate maintainer.

Done when fresh offline installation, compatible update and rejected incompatible update have recorded browser receipts. Depends on ARC-04/05 and [release tests](08-testing-and-release.md).

## Required acceptance matrix

| Fixture | Observable pass |
| --- | --- |
| Same response submitted twice | One consequence/event; retry returns original receipt |
| Inspection from outside range or wrong scene | Refused; objective and rewards unchanged |
| Outbound → camp → return | Same rig ID/condition; accepted parking/spawn; no teleport-based completion |
| Site A loading, then B selected | Only B may commit; A resources released or never installed |
| Scene failure and retry | Prior world/scene retained or restored; exactly one running loop afterward |
| Blur with throttle held; reconnect gamepad | No stuck throttle/movement and no unsolicited action edge |
| Save during a transition | One stable revision restored, never mixed scene/world |
| Truncated save / wrong policy / storage denial | Current play and prior save survive; clear recovery choice |
| Full recorded trip replay | Same final semantic digest and ordered accepted events; first divergence reported precisely |
| Change camera/volume/text size | Semantic world digest unchanged |
| Load a saved trip in each supported profile | Same accepted state; explicitly documented visual capability differences |

See [simulation](04-driving-traversal-and-simulation.md), [testing](08-testing-and-release.md) and the [delivery backlog](09-delivery-backlog.md) for integration sequencing.
