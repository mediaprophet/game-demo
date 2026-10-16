# Talbingo renewal: experience and accessibility

Status: proposed implementation specification, 17 October 2026. This is the interaction handoff for [the gameplay plan](01-product-and-gameplay.md). Read [architecture/state ownership](03-architecture-state-and-platform.md) before implementation. All behaviours below require verified public Qualia/Webizen interfaces; unavailable capabilities are upstream work, not permission to add a parallel UI/game runtime.

## What needs to change in the current page

`web/talbingo.html` combines layout, audio, input, camera, semantic game state and error handling in one module. The side panel leads with geology/datum details and technical badges; the player must work through that information to reach the game. The mobile layout places the scene above a long panel, while `.play-touch` shows movement/brake/zoom but leaves inspection, decisions and return elsewhere. The scene is technically visible but the interaction loop is fragmented.

Concrete problems to address:

| Current code/surface | Player impact | Required replacement |
|---|---|---|
| `display()` recreates choice buttons; Enter/E and gamepad A select `[data-correct="true"]` | Input device decides the answer; focus is unstable | Stable option IDs and focus; confirm only the highlighted/focused option |
| Global `keydown` excludes only the site `<select>` | Typing/navigation can trigger movement, camera, audio or immediate reset | Context-scoped action routing and explicit input ownership |
| `keys`, `keyHeld`, `pollGamepad()` and `blur` handling | Keyboard/gamepad/touch contend; `keyHeld` survives blur; held inputs can stick | One reconciled input adapter that releases every source on focus/device loss |
| `attachCampCamera()` always attaches drag/wheel; chase camera rewrites yaw/zoom | Free-look and chase behaviour compete; little indication of active camera | Explicit camera mode, free-look/recenter contract and separate menu/scene focus |
| Sidebar says scroll or `+ / −` to zoom; key handler has no matching zoom branches | Displayed help differs from behaviour | Help generated from actual bound actions |
| Clickable `#cam-mode-pill` is a `<span>` | Camera control lacks normal keyboard button semantics | Labelled button with its current mode and shortcut |
| `begin()` scrolls smoothly on narrow screens | Unexpected motion and viewport change | Predictable focus/scene layout; obey reduced-motion preference |
| Site selection calls `loadSite()` and `reset()` immediately | Accidental state loss or transition during travel | Staged destination selection and explicit embark/abandon/resume controls |
| Errors set text; a frame error stops `requestAnimationFrame` | Page offers no useful recovery path | Recoverable error state with actions tied to the last valid canonical checkpoint |
| Audio unlocks on first key/pointer action and has one toggle | Unrequested sound and no useful mix control | Explicit sound choice and master/category settings through Qualia audio |
| Canvas has one label and status text; little scene semantics exposed | Nonvisual users cannot discover targets or understand changes | A semantic, reachable-object projection with equivalent intent controls |

These are source observations, not results of a new browser/device audit. [The audit](00-current-build-audit.md) and [testing plan](08-testing-and-release.md) specify the evidence to collect before claiming an issue is fixed.

## Target layout and information hierarchy

The scene fills the available viewport. During play, the player should normally see only the current destination/mode, their next chosen goal, a compact status cluster and the immediate interaction prompt. Journal, inventory, source information, controls and settings open on demand. A diagnostic panel must not compete with the world.

```text
Desktop / landscape tablet
┌────────────────────────────────────────────────────────┐
│ Menu · Talbingo / current site         Journal  Settings│
│                                                        │
│ Next chosen goal                      Optional panel   │
│                                        (collapsible)   │
│                 WORLD / PLAYER                         │
│                                                        │
│ Context action / interaction target                    │
│ Journey status / essential warning     Movement/camera  │
└────────────────────────────────────────────────────────┘

Portrait phone
┌────────────────────────┐
│ Menu   Site   Journal  │
│ Goal (one short line)  │
│                        │
│         WORLD          │
│                        │
│ Context action         │
│ Move          Look     │
│ Status · Camera · Pause│
│ Optional bottom sheet  │
└────────────────────────┘
```

This is layout intent, not pixel artwork. Use Webizen's supported UI/semantic projection contract. Ordinary browser controls may only serve as the allowed host/accessibility projection; they must not implement another semantic owner.

| Surface | Primary content | Interaction policy |
|---|---|---|
| Hub | Continue expedition, new trip, six destination cards, source/coverage status | Previewing a site does not move the player or discard a save. |
| Driving HUD | Route name, remaining route measure, speed with valid units, one priority warning, pause and camera | No long descriptions or journal overlays obstruct steering. Large panels pause under the canonical clock policy. |
| Camp HUD | Current target, chosen objective, interact, inventory/journal | World remains visible; each prompt names the object and action. |
| Context inspector | What was observed, available actions, cost/reason, source status when relevant | Opens near an edge; stable focus; can close without acting. |
| Choice panel | Two or more authored options with costs/benefits and uncertainty | No answer revealed by DOM flag; no default irreversible choice; focused confirm activates exactly one option. |
| Journal | Goals, findings, receipt-based consequences, sources, unfinished work | Filter by current expedition/site; supports full text and keyboard reading. |
| Pause/menu | Resume, help/settings, save state, return/quit options | State whether simulation is paused; restoring focus never resumes throttle. |
| Error/recovery | Plain-language cause, preserved progress, safe next actions, optional diagnostics | Focus moves to the error heading; retries cannot create two sessions or loops. |

At 1280×720 desktop and 844×390 phone landscape, the closed-panel HUD must leave at least 65% of the scene area unobstructed. At 390×844 portrait, use at least 55% as an initial product target because touch controls consume more area. Measure actual overlapping HUD rectangles in screenshots; these are proposed design gates, not current performance claims. Open reading panels may exceed those limits, with an obvious close/resume action.

## A guided first session without a form wall

1. Show a truthful welcome: “Explore six Talbingo game destinations. This pack uses [illustrative/source-reviewed] places and authored situations.” Offer **Continue**, **Start a trip** and **Settings** as appropriate. Put detailed provenance one action away.
2. Offer three optional quick preferences: input/help level, sound on/off, and calm/reduced-motion presentation. Detect system preferences as initial suggestions, show the effective values, and allow changes at any time. No diagnosis, identity or LifeChapter disclosure is necessary.
3. At the hub, recommend the tested introductory site but keep all six discoverable. Each card shows a short character description, estimated *game session* duration, coverage/readiness, and any scenario constraint.
4. Ask the player to choose one or two trip goals from authored options; use a starter party/rig/kit by default. Deeper customisation is optional.
5. Teach movement and braking in a bounded practice space or the initial safe route segment. Only show the next relevant instruction. Detect the action receipt to advance the tutorial; a timer alone is insufficient.
6. Teach parking/exit, an observation and a meaningful choice. Explain consequences from the accepted outcome, then let the player play.
7. End with a concise debrief and **Continue exploring**, **Plan another trip**, or **Save and leave**. A player can skip/reopen teaching steps independently of their expedition progress.

Tutorial steps must carry IDs and completion state in the canonical tutorial/settings owner when persisted. Skipping a tutorial never auto-completes an expedition task or grants items. Returning players go directly to their checkpoint; they are not forced through welcome again.

## Interaction and focus contract

Every action follows one user-visible sequence: **available → focused → requested → accepted/rejected → explained**. A pending action disables only conflicting actions and displays its target. The UI receives a revisioned projection, sends an intent with stable entity/action IDs, then updates from a receipt. It does not optimistically declare a camp complete, decrement inventory, advance a story or unlock return.

For object interaction, show one primary target with its name and action (“Inspect shared table”), plus a next-target control when multiple objects overlap. Targets must be semantically discoverable and spatially eligible according to Qualia; presentation-only distance checks such as `distanceToLandmark() < 2.2` cannot be the authority. If an object becomes unreachable or occupied while a panel is open, keep the panel readable and disable the now-invalid action with the current reason.

When a panel opens, focus its heading or first logical interactive element, then return focus to the launching control or scene anchor on close. Preserve a focused option across projection updates using stable IDs. Do not replace an entire control tree every simulation tick. If the focused item disappears, move focus to the nearest valid sibling and announce why. Escape closes the top panel before it opens pause; it never silently abandons an expedition.

## Input actions, contexts and defaults

These are proposed defaults, to test and allow remapping. Store bindings through the verified Qualia settings surface; if durable settings support is missing, mark that capability blocked. Displayed help is generated from the effective binding map. Avoid separate hard-coded keyboard, touch and gamepad rule paths.

| Action | Scene keyboard/mouse default | Touch control | Gamepad default |
|---|---|---|---|
| Walk / steering | WASD or arrow keys in scene context | Large directional pad or configurable stick | Left stick, adjustable dead zone |
| Throttle | W / Up | Separate throttle button | Right trigger |
| Brake / reverse | S / Down with visible state and deliberate reverse transition; separate handbrake | Brake button; reverse mode explicit | Left trigger; reverse confirmation/state per handling contract |
| Handbrake | Space in driving context | Labelled brake/hold control where enabled | Mapped face/shoulder button shown in help |
| Interact / confirm | E for scene interaction; Enter/Space for focused semantic control | Context button or selected panel action | Primary confirm button |
| Cancel / pause | Escape, context stack as above | Always reachable back/pause | Menu/back according to active context |
| Camera look | Pointer drag in scene; remappable keyboard camera keys | Dedicated look region or separate controls | Right stick |
| Camera preset | C in scene context | Labelled camera button | A mapped shoulder/face action distinct from confirm |
| Zoom | Wheel over scene or bound +/− | Labelled +/− or pinch as optional enhancement | Optional remappable action pair |
| Recenter | Dedicated bound action and visible button | Recenter button | Dedicated bound action |
| Journal / inventory | Remappable shortcuts; no effect in text fields | Menu buttons | Menu navigation |

Input contexts are **scene-driving**, **scene-walking**, **camera**, **panel/menu**, **text-entry**, **transition**, and **paused/error**. Only one context owns a conflicting action. A menu can support normal Tab/Shift+Tab/Enter without turning the rig; text input never moves the actor. Global shortcuts are limited to deliberate, nonconflicting accessibility/help actions. Reset is a menu command, never a single globally active `R` keystroke.

Implementation rules:

- Collect input per source and pointer ID, then reconcile into one input-intent frame. Do not let `pollGamepad()` delete touch/keyboard state. Document the policy for conflicting axes; a reasonable default is last active intentional source per axis until released, with no summed throttle.
- Treat confirm/toggle as edge-triggered actions; key repeat must not submit multiple commands. Preserve analogue values when the supported simulation accepts them.
- On window blur, hidden document, controller disconnect, pointer cancel/lost capture, modal opening, page freeze or fatal error, release every held input and request the supported safe pause/stop policy. Clear `keyHeld` equivalents as well as the pressed set.
- On return, require an explicit resume where the scene was paused; never infer a still-held key from stale state. Do not advance accumulated hidden-tab time through the simulation.
- Support simultaneous steering and throttle on touch with independent pointer capture. One pointer dragging the camera cannot release the other's throttle.
- Detect connected controllers, show the active mapping, tolerate hot-plug, and let the player choose the active pad. Do not assume the first two navigator slots identify the intended controller.
- Binding changes report conflicts, offer restore-defaults and permit separate movement/camera sensitivity and axis inversion. Remapping must leave a reachable menu/undo path.

Vehicle control semantics and actual assistance belong to [04](04-driving-traversal-and-simulation.md). UI code cannot implement its own steering correction or autopilot.

## Camera and movement comfort

Offer a small set of predictable presets: chase, optional verified first-person, overhead/navigation and camp orbit. Do not expose a preset as available before the corresponding renderer/actor/camera integration is verified. Retain each preset's settings separately so cycling back does not discard the player's preferences.

Chase camera follows the vehicle under an upstream camera contract. Dragging enters a visible temporary free-look state; manual orientation is not overwritten on the next frame. Recenter is explicit, with an optional gradual return after a configurable delay. Camp inspection can frame the object only after the player activates it, and the camera returns to the prior pose or a safe actor view when inspection closes. Collisions/occlusion queries and camera sweeps belong upstream; do not add game-side geometry tests.

Provide reduced motion: disable bob, shake, speed-dependent field-of-view changes, rapid zoom, decorative pulses and forced smooth scrolling; use short fades or immediate cuts for nonessential transitions. Clamp sensitivity and allow inversion. Avoid involuntary camera spins on scene load, return travel and reset. Fixed/overhead presentation should be usable for the complete loop where the supported capabilities allow it. If a required accessible camera mode is missing, track it as a release-blocking capability for that target profile.

Test comfort with manual camera input during steering, camera changes while stationary/moving, entering/exiting a vehicle, landmark inspection, reversing, opening panels and loading the next scene. No single setting can guarantee comfort for every person; ship independent controls and a quick way to pause/change them.

## Responsive and touch behaviour

Use the visual viewport and device safe-area insets; do not force desktop minimum canvas heights on a small phone. Reflow the UI instead of shrinking text. Portrait and landscape both work. A single bottom sheet or edge panel is open at a time, with clear snap/close behaviour and a persistent essential warning strip.

Use a 44×44 CSS-pixel minimum product target for ordinary interactive controls and prefer 48×48 for movement/confirm controls, with separation that prevents accidental neighbours. These are deliberate product targets, not a claim that every such target alone establishes standards compliance. The current 42-pixel mobile minimum must be reconsidered in context of actual padding, target height and spacing.

The scene's touch area must distinguish movement/look/interaction. Restrict `touch-action` only where the gesture is actually consumed; the journal and settings remain scrollable and zoomable. Avoid required multi-finger gestures, long-press precision, drag-only actions or orientation lock. Pinch/drag enhancements always have buttons or semantic alternatives. One-handed presets may reposition controls and support toggle rather than hold where safe under the simulation contract.

At 320×568, 390×844, 844×390, 768×1024, 1280×720 and 1440×900 test: menus, every activity choice, long source labels, translated-length placeholder strings, on-screen keyboard, browser chrome resizing, device rotation during a pause and two simultaneous touch inputs. No essential button may end up under the browser toolbar/notch or require horizontal page scrolling. Zoomed text may create a scrollable reading panel; movement controls must not cover the focused item.

## Accessible information and equivalent play

Use [WCAG 2.2](https://www.w3.org/TR/WCAG22/) as a reference for the web shell's keyboard, focus, reflow, contrast and target behaviour. The full interactive game needs additional manual testing; automated checks and these targets do not establish conformance. The following are product requirements and verification tasks.

| Need | Product requirement | Verification |
|---|---|---|
| Keyboard operation | All menus/actions usable without pointer; visible focus; no trap; remapping; camera alternative | Finish the full first-site loop using keyboard only; test Tab order at every modal boundary. |
| Low vision / text | Scalable text, readable labels, optional high-contrast HUD, no important information encoded by colour alone | Test 200% and 400% browser zoom on reading surfaces; text/background contrast target at least 4.5:1 for ordinary text, 3:1 for large text; inspect controls/focus against their adjacent colours. |
| Screen reader | Headings, named buttons, meaningful status announcements, a structured scene/object/route projection | Use an actual screen reader with keyboard to plan, inspect, choose, save, recover and debrief; test a supported nonvisual traversal path before claiming full playable access. |
| Hearing / muted audio | Captions for relevant speech/environment cues; directional/location text where needed; equivalent hazard feedback | Complete all activities and react to journey cues with sound off. Ambient subtitles can be independently disabled. |
| Motor control | Remapping, sensitivity/dead-zone controls, toggle/hold options, large targets, no required timed precision | Complete equivalent activities with touch, single-step semantic controls or the verified assistance path. |
| Motion sensitivity | Independent reduced-motion/comfort settings and stable camera presets | Inspect camera transitions, scrolling, highlights and scene loading; no essential event depends on a motion effect. |
| Cognitive load | One current goal, consistent verbs, optional step breakdown, rereadable clues, hint ladder, pause | New players can identify the next available action and explain a consequence without memorising transient text. |
| Speech / communication | Authored dialogue choices and text; no microphone or voice requirement | Every conversation/contribution path completes without speech input. |

The semantic scene view lists reachable/known objects with name, relative location/distance in valid game units, status and available intent. It also explains known obstructions and route progress. It must come from the same bounded Qualia projection as rendering. Selecting an object opens its details; it does not teleport, inspect from an invalid range or auto-complete the task.

A nonvisual or reduced-motor traversal path may offer named reachable waypoints, stepwise motion, guided traversal or assisted driving only after the corresponding Qualia command/collision/clock contract is proven. It must preserve route access, encounters, action eligibility and resource/time effects. For a missing capability, document the supported scope honestly and block release claims that depend on it; do not substitute browser position changes.

Announcements should be event-based and prioritised: receipt/rejection, arrival, changed objective, new warning and recovery status. Do not announce coordinates, distance or speed on every 20 Hz simulation step. Offer an on-demand status readout and a configurable periodic navigation summary. A repeated hazard warning is announced once per material state transition, not every frame. Urgent action changes must not be buried beneath decorative discoveries.

## Sound and sensory settings

Replace player-facing “Holosonic wavefield” jargon with “Sound”. Offer master, ambience, vehicle, effects and dialogue controls only for categories implemented in the Qualia audio capability. Mute must include every category, including vegetation ambience; disabling sound cannot change simulation outcomes. Explicit first-session sound selection avoids equating any keypress with consent to playback.

Provide visual equivalents for warnings, success/rejection and useful spatial sound. Captions name fictional speakers and important sound sources without claiming unverified wildlife identity. Allow low-intensity audio and fewer ambient cues. Settings expose effective output when playback is unavailable; the loop remains playable. See [art/audio/rendering](06-art-audio-and-rendering.md) for the production and upstream audio work.

## Loading, transitions, pause and recovery

Use the canonical lifecycle and staged scene commit in [03](03-architecture-state-and-platform.md); the labels below are presentation states, not a second state machine. Existing scene, route and save data remain authoritative until the replacement is verified and committed.

| Situation | What the player sees | Allowed action / invariant |
|---|---|---|
| Initial bootstrap | Named phase: checking runtime, opening pack, preparing scene; real progress where measurable | Retry or leave. Do not draw a fabricated percentage or hang behind “Loading…” indefinitely. |
| Pack download/prepare | Pack name, known byte progress, optional size, cancel if supported | Cancellation retains the prior valid expedition and verified pack. |
| Scene transition | Destination, short progress explanation, essential status | Controls transfer only after commit; input is neutralised. Rapid repeats cannot create two arrival receipts. |
| Shader/renderer setup | A brief preparation message, supported fallback status | Fallback is supplied by Qualia; lowering quality does not alter semantic state. |
| Pause / hidden tab | Clear “Paused” state and last saved checkpoint status | Freeze according to canonical clock; resume with neutral input, not a frame backlog. |
| Save pending / saved | “Saving…” then a durable receipt time/checkpoint label | Never claim saved until the durable commit receipt exists. Gameplay retry does not overwrite the last valid save with a partial snapshot. |
| Save unavailable | “Progress has not been saved” with reason and safe action | Explain storage/permission/budget limit; retry or supported export; do not imply persistence works. |
| Recoverable scene failure | Plain-language failure; previous valid location/checkpoint retained | Retry preparation, use supported renderer tier, or return to hub via a valid transition. |
| Runtime/simulation failure | Game paused, last committed checkpoint identified | Resume only after validated reinitialisation, or reload the checkpoint; no partial world is saved as success. |
| Corrupt/incompatible save | Version and recovery choices; optional diagnostics | Preserve original bytes/version; select a supported prior checkpoint/migration; new expedition requires explicit choice. |
| Offline / missing pack | Which installed destinations work, which assets are missing | Existing verified offline content works under its published capability profile; never simulate a successful download. |

Define an operation timeout and cancellation contract in the platform work; do not simply retry forever. A progress explanation changes when the operation changes, not on an arbitrary animation cycle. If no progress estimate is available, say what is happening. A recoverable failure must expose an action within the tested time bound in [07](07-performance-reliability-and-security.md).

For quit, new expedition, resetting a site or abandoning an active plan, show what will be retained/lost and the last durable checkpoint. Use a two-step confirmation only where actual player progress would be discarded. Changing camera, hints, sound or panel visibility never needs a confirmation. Cancelling a dialog leaves world state and focus intact.

Diagnostics belong behind “Technical details”: error code, game/Qualia versions, pack digest and operation ID. Avoid dumping stack traces in the main play HUD or leaking private LifeChapter/party text into reports. Render diagnostic strings as inert text. Copy/export reports only through supported, deliberately invoked paths.

## Truthful geographic and scenario presentation

Move datum, geology and licence details into a well-labelled source panel. Keep a concise coverage indicator beside destination/route selection because it affects player understanding. Technical engine claims such as “CORE: ZERO-HEAP” do not belong in the ordinary player navigation.

For the illustrative pack use “Game position” and “Illustrative route”. Do not display interpolated latitude/longitude as a live GPS track, hard-coded kilometres as measured route distance, or unsupported geology as a retrieved fact. Where source-backed coordinates exist, show their source/date/uncertainty and format hemispheres consistently (for example a signed latitude or an unsigned southern latitude, not a negative number plus “S”).

Separate **source fact**, **authored simulation assumption**, **uncertain observation** and **current external information link**. The source panel provides the originating organisation/link, snapshot date, licence, transformations and pack version supplied by the content contract. No live-access claim derives from a cached alert. Do not load external trackers or maps merely by opening a destination card; outbound links are explicit user actions.

## Implementation work orders

### UX-01 — define presentation states and redesign navigation

**Depends on:** ARC-01/ARC-02; PLAY-01; shared scenarioPackId/sceneId/routeEdgeId contract.

1. Inventory all current DOM elements and assign each to hub, driving, camp, journal, settings, source panel or diagnostics.
2. Write a screen/state inventory with entry, exit, focus target, simulation-clock policy and permitted intents.
3. Create low-detail layouts for desktop, portrait and landscape using supported Webizen surfaces; keep technical/source details discoverable.
4. Replace the site dropdown's immediate load/reset with destination preview and explicit journey controls.
5. Review wording for what the player needs next; keep displayed geography/status tied to provenance fields.

**Done when:** every current and planned control has one home; the player can preview all six sites without mutation; the HUD meets the unobstructed-area targets; every screen has a keyboard close/back route.

### UX-02 — onboard and guide the first expedition

**Depends on:** UX-01, PLAY-02, ARC-03 authoritative session.

1. Implement continue/new-trip flow, quick preferences and a starter party/rig/kit projection.
2. Author brief tutorial steps keyed to real accepted actions, with skip, replay and help.
3. Generate current controls/help from bindings; retain the next goal after a panel is closed.
4. Add one hint ladder and a rereadable evidence record before introducing more activities.

**Done when:** fresh and resumed sessions show the appropriate entry; skipping help grants no resources; the first meaningful decision is reachable without a character questionnaire; every tutorial instruction matches an actual control.

### UX-03 — consolidate keyboard, touch and controller input

**Depends on:** ARC-02, ARC-03; SIM-03/SIM-04 input contracts; UX-01 contexts.

1. Replace scattered event handlers with the approved thin input adapter; keep per-source held state and edge actions separate.
2. Implement context ownership, device arbitration, mapping display, remapping and input-loss neutralisation.
3. Remove automatic correct-answer selection and global reset behaviour.
4. Add integration fixtures for menu focus, text entry, key repeat, blur, hidden tab, pointer cancel, two-touch control, controller disconnect/reconnect and conflicting sources.

**Done when:** no UI interaction moves the actor unexpectedly; all devices select the same named intent; no stale throttle resumes; held/toggle actions emit the intended count; the loop works on keyboard, touch and gamepad.

### UX-04 — provide stable focus and semantic interaction

**Depends on:** UX-03; ARC-03; SIM-04 semantic picking/reachability; PLAY-04.

1. Bind targets and options by stable semantic IDs instead of replacing controls by array index.
2. Implement target cycling, context panel, pending/rejected/accepted feedback and explicit consequence previews.
3. Add focus entry/return/disappearing-target behaviour, native control semantics and accessible names.
4. Ensure stale target revisions and inaccessible actions return reasoned rejection without local state changes.

**Done when:** focus survives ordinary projection updates; Enter/confirm invokes the focused option; overlapping targets are selectable; invalid range/state is explained; no action requires pixel-perfect pointing.

### UX-05 — make camera behaviour predictable

**Depends on:** verified Qualia camera/occlusion support in ARC-06; SIM-03/SIM-04; UX-03.

1. Define preset entry/exit and persistence, free-look ownership, recenter and inspection framing.
2. Add independent sensitivity/inversion and keyboard/touch controls generated from bindings.
3. Implement reduced-motion behaviour through supported camera/effect controls.
4. Review transitions at forward/reverse travel, walking, inspection, pause and scene load.

**Done when:** manual camera input is not overwritten unexpectedly; no initial camera is inside terrain/actors; complete play is possible with the supported low-motion preset; actual preset names match the UI.

### UX-06 — build responsive and motor-accessible controls

**Depends on:** UX-01, UX-03, UX-04; supported assistance controls in [04](04-driving-traversal-and-simulation.md).

1. Implement viewport/safe-area layout, single-sheet navigation and touch zones.
2. Add large targets, one-handed layout where supported, toggle/hold preferences and non-drag alternatives.
3. Exercise the six viewport sizes, rotation, virtual keyboard, browser chrome and simultaneous touch input.
4. Keep essential warnings and pause reachable when sheets are closed and when text is enlarged.

**Done when:** no essential control is obscured or requires horizontal scrolling; target dimensions meet product targets; portrait/landscape complete the same expedition; the activity set has a viable low-precision path.

### UX-07 — implement and verify sensory/nonvisual access

**Depends on:** UX-04 through UX-06; ARC-06 semantic UI/audio; SIM-04 accessible traversal; audio tasks in [06](06-art-audio-and-rendering.md).

1. Expose the bounded semantic scene/object/route projection with headings and event announcements.
2. Add effective text/contrast/motion settings, captions, sound categories and muted-play equivalence.
3. Implement supported nonvisual/low-motor route intents without bypassing canonical constraints.
4. Perform manual keyboard/screen-reader, zoom, muted-audio and reduced-motion journeys with participants/tools recorded in the QA evidence.

**Done when:** all critical UI tasks can be completed without sight or sound; full nonvisual gameplay is claimed only after traversal and all site activities pass; no warning exists solely as a colour or sound; announcements remain readable during travel.

### UX-08 — explain goals, progress and consequences

**Depends on:** PLAY-03 and PLAY-08; UX-04; ARC-03/ARC-05 projections and persistence.

1. Build the expedition journal and concise debrief from objective/discovery/receipt records.
2. Show why a consequence happened, which goals are incomplete, available alternative resolutions and the next suggested action.
3. Display source status on place/observation facts and allow rereading skipped dialogue facts.
4. Keep progression, preferences and accessibility separate; no single overall character-worth score.

**Done when:** journal and world agree after reload/replay; players can identify one choice consequence; unfinished/declined goals do not appear completed; the current task can be found in one action.

### UX-09 — make loading, saves and failures recoverable

**Depends on:** ARC-04 staged transitions, ARC-05 saves/replay, ARC-06 platform; reliability gates in [07](07-performance-reliability-and-security.md).

1. Implement the presentation matrix for bootstrap, progress, cancellation, pause, save, missing packs and errors.
2. Bind “saved” to durable receipts; expose retry/recovery options from supported upstream operations.
3. Add guarded destructive-progress confirmations and return focus on cancel.
4. Test interrupted transitions, rapid destination requests, missing/tampered assets, storage exhaustion, corrupted/version-mismatched save, render-context loss and runtime failure.
5. Prove retry leaves exactly one active session/render loop/input subscription and retains the previous valid checkpoint.

**Done when:** each injected failure has a clear useful action and accurate retained-progress text; no success is announced before commit; cancelling/retrying cannot duplicate arrival or discard valid state; recovery does not require developer tools.

### UX-10 — validate the complete experience with people

**Depends on:** UX-01 through UX-09; PLAY-03 initially and all six sites before regional release.

1. Run task-based sessions: start/continue, choose goals, drive, inspect, choose a plan, pause/change controls, save/reload, return and recover from a prepared failure.
2. Include keyboard-only, touch, gamepad, enlarged text, reduced motion, muted audio and at least one actual screen-reader workflow; involve users of assistive technology where possible rather than inferring access from a checklist.
3. Record task completion, help needed, focus/input failures, comprehension and discomfort reports. Avoid collecting sensitive player context.
4. Fix blocking problems, repeat the affected paths, and preserve evidence with exact viewport/device/browser/renderer/pack versions.

**Done when:** all input paths complete the first expedition and each site activity has an equivalent accessible route; no blocking focus/control/recovery issue remains for a supported profile; the product comprehension gate in PLAY-09 passes. Publish supported limitations truthfully instead of claiming universal accessibility.

## Review checklist before marking UX complete

- A player always knows their current mode, chosen goal, next available action and save state without reading diagnostics.
- Every shortcut matches the help text and active context; confirmation never auto-selects an answer.
- All state-changing actions use accepted Qualia receipts and remain consistent across keyboard, pointer, touch, controller and semantic alternatives.
- Focus, pause, camera comfort, text scaling, captions, sound-off play and screen-reader UI are verified manually as well as by automated checks.
- Illustrative and sourced geography are visibly distinct; no fabricated GPS/route-distance/geology precision remains.
- Loading and recovery preserve the previous valid world and explain genuine options.
- Evidence in [testing/release](08-testing-and-release.md) includes actual player/device interactions; screenshots alone do not prove usability or accessibility.
