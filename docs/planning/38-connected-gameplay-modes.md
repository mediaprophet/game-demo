# Connected gameplay modes and scene presentation

User direction, 12 October 2026. This extends plan 35; it does not replace its
graphics, campaign, native-engine or acceptance requirements. Status: implementation
required. Existing activity panels are entry points, not completed versions of
the systems described below.

## Shared simulation, different play

The player moves between four substantial gameplay systems. Each needs its own
camera, controls, HUD, objects, puzzles and stories. They share Qualia-owned actor,
rig, inventory, condition, place, campaign, clock and event data. Mode changes do
not copy the world into independent economies or advance time by accident.

| Mode | Player experience | Concrete systems |
|---|---|---|
| Driving | A driving game with first-person and elevated options | Steering, throttle/brakes, terrain/contact, towing, traffic and overtaking, wildlife, road surfaces, visibility, ambient/grade/load thermal response, oil/coolant gauges and actual mirror views |
| Workshop | Mechanical investigation and assembly puzzles inspired by Street Rod II | Inspect symptoms, measure components, isolate faults, remove/install dependent parts, compare compatible upgrades, use tools, source spares and test the completed rig |
| Community/adventure | Human-scale exploration, stories and practical puzzles, inspired by Fable/Zelda | Walk around lived-in places, interact with residents and objects, follow branching situations, gather/use tools and resources, perform jobs, unlock routes and shared facilities through validated outcomes |
| Management | Production and planning informed by RTS, Civilization and SimCity | Connect producers, consumers, storage and routes; assign labour, track capacity and throughput, respond to shortages and transport interruptions, plan staged site upgrades |

The references describe experience and interaction, not licences to copy assets,
characters, maps, stories or proprietary rules from those games.

## Mode boundaries

Qualia owns deterministic simulation, graph validation, spatial queries, rendering,
animation, packaging and persistence. VibeScript owns authored bounded behaviours
through those APIs. The browser host routes input and displays receipts; it does
not implement a parallel vehicle simulator, puzzle solver or supply-chain reducer.

Mode entry records a stable place/site and actor, requested scene pack, camera,
available controls, required inventory and campaign preconditions. Loading succeeds
before controls transfer. Cancellation or failure retains the previous valid
world. Mode exit commits only accepted actions; repeated entry does not duplicate
resources, reset damage or manufacture parts. Autosave and replay cross mode boundaries.

## Workshop requirements

Use inspectable assemblies rather than a button labelled Repair. Model mount
points, parts, fasteners, tools, compatibility, order dependencies, wear and faults.
The rig family determines available assemblies: car, 4WD, campervan, caravan,
camper trailer and rooftop tent retain distinct layouts and constraints.

Start with a cooling-system fault and a touring electrical/solar upgrade. Give
the player evidence, several plausible causes, bounded tool operations and a
verification step. A puzzle must be solvable from information available in the
scene. Wrong choices have clear, reversible feedback where appropriate. Hints
reveal reasoning progressively. Motor condition changes must carry an accepted
Qualia receipt; an animation alone never repairs the rig.

Represent the mechanics as fictional gameplay, not real vehicle repair advice.
Allow keyboard selection and touch assembly focus without requiring pixel-perfect
dragging. Parts can be manipulated visually while semantic snap/compatibility
decisions remain in Qualia. Save partially completed disassembly with stable part
IDs; verify lossless reload and deterministic replay.

## Stories, work and production

Author stories as stateful situations with discovery, prerequisites, choices,
object interactions, consequences and alternative resolutions. Link fruit picking,
housekeeping, repair, food preparation and resident conversations to common
inventory, schedules and validated outcomes. Do not reduce every job to clicking
one campaign action. Accessible alternatives preserve the same outcomes.

Supply chains track actual inputs/outputs, rates, storage, labour, route capacity
and demand. Tools repaired at the workshop can support production; delivered
produce can support a kitchen; damaged transport or wildlife delays can affect
deliveries. Map planning is a view of these flows, not a second simulated world.
Pause/speed controls use an explicit Qualia clock contract and preserve the
existing campaign day and tide semantics during migration.

## World-first responsive presentation

The game scene fills the viewport. Panels overlay it; documents, text cards and
diagnostics must not allocate a small residual rendering box. Keep a large
unobstructed area for driving, exploration and assembly work.

Desktop/laptop: compact mode/navigation strip, concise player/rig status, small
context inspector and optional edge docks. Open inventories, quests and production
inspectors on demand. Permit collapse and return focus to the scene. Driving
gauges occupy the bottom/edges; workshop parts/tools frame a large assembly view.

Mobile: full-screen scene, safe-area-aware touch controls, large interaction
targets, compact status and one collapsible bottom sheet at a time. Keep essential
warnings visible with the sheet closed. Adapt information density instead of
shrinking the desktop interface. Support portrait and landscape; do not force
desktop minimum canvas dimensions beyond the actual visible viewport.

Dialogs for save/reset decisions may temporarily cover the scene. Ordinary
object interactions should use a compact contextual panel. Hidden-HUD capture,
reduced motion, keyboard navigation and readable focus states remain required.

## Acceptance

The browser host now has a full-viewport scene, an optional tabbed information
dock, and native HUD menus bounded to a desktop side overlay or a mobile bottom
sheet. Long HUD menus page inside that overlay; character setup no longer opens
a full-screen text panel. Mobile landscape moves mode selection to a narrow
right rail and leaves room for touch movement. These are source changes awaiting
the deferred build and device/browser validation; they do not establish visual
acceptance or completion of the gameplay modes.

- Drive a route, encounter a hazard and preserve resulting rig condition on entry
  to the workshop. Mirrors show scene evidence rather than a textual queue alone.
- Complete the cooling fault and electrical upgrade with inspectable parts,
  compatible tool choices, saved intermediate state and replayable outcomes.
- Complete a branching community story using exploration, conversation and an
  object puzzle, with at least two validated resolutions.
- Run a producer/storage/transport/consumer chain; observe an interruption and
  resolve it through another mode without duplicated goods or untracked money.
- Render and exercise each mode on desktop and mobile. Record actual viewports,
  screenshots, interaction paths and performance. Text-panel presence and source
  compilation alone do not prove a finished mode.
