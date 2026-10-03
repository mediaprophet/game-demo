# Maslows Challenge: RTS and AAA-Quality Uplift Blueprint

Status: **production direction and implementation instructions**, 2026-10-03.
This document responds to the project owner's request for an Age of Empires-like
game. It defines the desired *RTS interaction and production quality*, not a
copy of that game's setting, units, factions, art, sounds, rules, or interface.
The current Kestrel Flats browser slice is the technical starting point, not
evidence that the target quality has already been reached.

## 1. Product target

Build a readable, beautiful, large-scale **real-time community strategy game**.
The player surveys a living territory, directs several people and teams,
establishes and grows a network of settlements, gathers and moves material,
constructs and upgrades infrastructure, researches practices, negotiates
access and cooperation, and adapts to weather, wear, supply pressure, and
neighbouring communities. The camera, selection, command feedback, economic
loops, construction, technology progression, map exploration, and spectacle
should have the immediacy and depth players expect from a major RTS.

Maslows Challenge retains its own premise: prosperity means useful shared
capacity and practical conditions for people to live with peace. Its scenarios
may include conflict, contested resources, and hard trade-offs. The existing
product vision does not authorise combat as a default progression mechanic;
the exact place of military conflict is a product decision (see section 13).
Cooperation must be fun because it changes what the player can build and do.

**Player fantasy:** turn a fragile, scattered set of people, places, and
resources into a resilient, interconnected commons whose systems visibly work.

**Quality bar:** "AAA-level" is a target for feel, clarity, art, audio,
performance, accessibility, reliability, and content finish. It is not a
claim about current scope or a promise that a small first release has the
content volume of a commercial AAA game. The production method is successive
vertical slices with measured exit gates, then scalable content production.

## 2. What changes from the current slice

| Current evidence | Target experience | First proof |
|---|---|---|
| Six Qualia-rendered static `.10d` town meshes and semantic picking | A composed terrain map with landmarks, roads, districts, animated people, vegetation, water, weather, and legible building states | One visually complete map sector, viewed at strategic and close zoom |
| Single inspected object, button-driven actions | RTS camera, drag selection, multi-select, context commands, queue, build placement, rally points, minimap, command panel | Select five workers, assign two jobs, build one structure, inspect every result |
| Short scripted action chain | Continuous deterministic world ticks with workers, production, logistics, construction, upkeep, and events | A 20-minute unscripted session replays to the same state |
| Runtime generated mesh recipe in the shell | Separately authored game content distributed in a governed Qualia HMC pack | Replace one runtime mesh with a pack asset and validate its Q42 identity, digest, licence, and pick |
| N3 text session and action tape | Mutable Q42 world plus command/event checkpoints | Save, reload, migrate, and replay a busy world without divergence |
| Bounded Vibe expression workbench | Vibe-authored scenarios, behaviour, UI composition where supported, and in-game tooling | Edit a scenario rule, preview it, reject an unauthorised call, and run it in browser |

Do not discard the current QualiaPortal proof. Refactor and extend it until the
game can meet these targets through the **full Qualia WASM package**. Do not
route game features through the WebCivics profile or change Qualia's licence.

## 3. Core RTS loop and scenario structure

The first campaign chapter, **Kestrel Flats**, has a clear strategic arc:

1. **Survey.** Reveal nearby terrain, usable buildings, material sources,
   routes, hazards, and people who can collaborate. Unknown areas are genuinely
   unknown; the map distinguishes surveyed facts from estimates.
2. **Stabilise.** Secure shelter, water, energy, repair capacity, and safe
   access through several viable paths. Show the effect in the world.
3. **Build.** Assign teams to salvage, collect, carry, craft, construct, repair,
   teach, and maintain. Place facilities spatially; their position affects
   route length, capacity, resilience, and access.
4. **Connect.** Link the workshop, trading post, showground, garden, energy
   site, and remote partners through roads, schedules, agreements, and utility
   networks. Interdependent systems create strategic choices.
5. **Advance.** Unlock capability through research, learning, designs,
   partnerships, and successful projects. Progress is expressed as new actions
   and infrastructure, not an abstract people score.
6. **Respond.** Handle supply interruptions, weather, equipment faults,
   competing plans, and disputes. There must be recoverable responses and
   player-readable reasons for blocks.
7. **Expand.** Establish or support a second ground with a different topology,
   then decide which services to centralise, share, or duplicate.

The opening 10 minutes should let a new player pan, zoom, select, command,
place, watch construction progress, and see one useful system switch on. The
first 30-45 minutes should offer a genuine strategic choice between at least
two valid routes to workshop power and viable shelter. A longer scenario should
support expansion to multiple sites and simultaneous work queues.

### Simulation rules

- Use deterministic fixed ticks. Define tick duration, command ordering,
  seeds, integer/fixed-point quantities, pausing, speed settings, and replay
  before adding complex agents. Presentation interpolates between ticks.
- Model people as agents with location, task, capabilities, consent/role,
  schedule, carrying capacity, needs, and current intent. A person is never an
  inventory item or a disposable unit.
- Buildings have footprint, stage, health/condition, inputs, outputs, capacity,
  permissions, maintenance, connections, and occupancy. Visual state derives
  from authoritative Q42 state.
- Resources have units, stock, quality/condition where useful, provenance,
  storage constraints, transfer/transport cost, and replenishment/depletion.
  Start with a few legible resources; add detail only where choices improve.
- Tasks are issued as typed commands; schema, role/permission, SHACL,
  N3/deontic checks, and the deterministic reducer decide outcomes. Every
  rejection identifies a practical next step and spends no resources.
- Queues can be inspected, reordered, cancelled, and resumed. Cancellation
  returns or accounts for committed inputs according to a visible rule.
- Spatial placement matters: terrain grade, routes, utility reach, access,
  hazards, adjacency, and working radius affect output and costs. Avoid
  invisible bonuses with no scene explanation.
- Difficulty comes from authored situations and transparent constraints,
  not identity attributes or hidden penalties. Multiple starts and viable
  routes remain available.

## 4. RTS interaction contract

### Camera and map

- Isometric or high oblique strategic camera with smooth pan, edge pan option,
  zoom, rotation, reset, keyboard navigation, and camera bookmarks.
- Continuous zoom must preserve selection and command context. Close zoom
  shows craft and character detail; far zoom shifts to clear strategic symbols
  without losing actionable state.
- Terrain supports elevation, passability, roads/paths, waterways, ownership
  or stewardship boundaries, work zones, and fog/survey state.
- Minimap shows camera box, selected units, alerts, projects, routes, and
  known territory; it must remain useful in colour-blind and low-sensory modes.

### Selection and commands

- Click a person, structure, prop, route, or project to inspect its semantic
  entity. Drag-select groups; Shift adds/removes; double-click selects a type;
  control groups save selections. Always show selection and hover feedback.
- Context click issues a valid move, work, transport, repair, inspect, or
  interaction order; ambiguous targets open a concise choice menu. A preview
  shows likely route, cost, permission, and blockers before commitment.
- Build mode offers footprint ghost, snapping where relevant, orientation,
  collision/terrain/utility indicators, and explicit reason when invalid.
- Support task queues, rally points, repeat production, priority, and a
  pause/speed control. The user can undo only when the event model permits it;
  otherwise provide cancel or a compensating action.
- The command panel shows actionable verbs first and a separate **Why?**
  evidence view for conditions and provenance. Alerts navigate to the cause.
- A full keyboard path and remappable keys must reach every essential action.
  Touch/gamepad support needs its own tested command grammar, not emulated
  mouse events alone.

### Player information

- Persistent top bar: available resources, energy/water, workers, time/speed,
  active alerts. Detail panels show rates, bottlenecks, and forecast with
  source/uncertainty when applicable.
- Clear world overlays: power, water, logistics flow, project progress,
  accessibility, hazard exposure, survey, and governance/access boundaries.
  No overlay may present a fictional outcome as real-world advice.
- A campaign journal explains objectives, decisions, alternate routes,
  consequences, unresolved barriers, and replayable event history.
- Optional developer evidence mode exposes Q42 IDs, rule receipts, asset
  digests, and performance counters without cluttering normal play.

## 5. Original game systems to build

| System | Player-facing decisions | Minimum vertical-slice proof |
|---|---|---|
| Workforce and tasking | Assign specialists or general teams; balance travel, skill, training, rest, and cooperation | Five agents work concurrently; queues and blockers are visible |
| Logistics | Choose depots, haul routes, stock priorities, and remote fulfilment | A part moves through listing, reservation, delivery, inspection, storage, and build use |
| Construction and repair | Place, fund, supply, approve, build, commission, and maintain | A workshop power project moves through visible stages and changes production |
| Utilities | Route power/water/communications; manage capacity and load priority | Energy outage and restoration visibly affect at least two facilities |
| Knowledge and progression | Research designs, train people, share blueprints, unlock methods | One upgrade unlocks a new viable plan, with provenance and prerequisites |
| Governance and diplomacy | Propose, endorse, negotiate access, review a rejection, join a network | A contributor helps without gaining treasury or site authority |
| Territory and environment | Survey, choose sites, account for terrain and weather | Two locations have distinct strategic advantages and constraints |
| Events and recovery | Respond to fault, shortage, route closure, or dispute | No scripted event forces a restart; at least two recoverable choices exist |
| Campaign and skirmish | Authored goals plus a repeatable seeded scenario | A deterministic seeded map/session can be completed through two routes |

Do not add all systems at once. Each must first prove one enjoyable decision,
one visible world consequence, deterministic replay, and an explanation path.

## 6. World, art, animation, sound, and narrative

Create an **original art bible** before broad asset production. Specify
architecture, materials, silhouettes, terrain palette, vegetation, weather,
day/night lighting, iconography, character readability, camera scale, and
building-stage language. The first finished sector must show a distinctive
Kestrel Flats rather than anonymous primitive boxes. Keep useful colour and
shape coding at strategic zoom; close-up richness must not hide task state.
The [asset production catalog](18-asset-production-catalog.md) lists the
families, first source batch, replacement queue, and per-asset completion
contract.

The asset inventory is separate from the engine: terrain tiles; roads and
connections; 12-20 modular building families with construction/damage/upgrade
states; people with role silhouettes and actions; vehicles; 30-50 props;
vegetation; effects; UI icons; portraits; ambient sound; music; voiced or
subtitled barks; and cinematic moments. Those are **planning ranges**, not
approved v0.1 counts. Prove one complete family and one complete character
before multiplying content. Record per-asset owner, source, licence, revision,
semantic ID, LODs, animation clips, visual QA, and pack digest.

Target animation coverage for the slice: idle, walk/carry, work, build,
repair, interact, and celebration/relief; structures need construction,
operating, fault, and repair states. Animation is a **QualiaDB `.10d`/renderer
upstream dependency** until the QG-03 contract passes. Reduced-motion mode
must preserve state readability. Effects should help players read power flow,
water flow, construction, and weather without relying on colour alone.

Sound should communicate intent and result: selection, accepted order,
blocked order, completion, outage, repair, weather, and territory ambience.
Mixing must prioritise speech and alerts, support subtitles and separate
volume controls, and avoid exhausting repeated barks. Music follows tension
and progress without asserting a moral score. All audio and visuals are
original or rights-cleared game assets, packaged through the governed Qualia
content path.

Narrative appears in mission setup, character relationships, short event
choices, and world consequences. The player should be able to understand the
strategy without reading long text panels. Optional P64-backed NPC dialogue
may enrich authored content only after the bounded Qualia inference path is
verified; it never decides authoritative state.

## 7. QualiaDB ownership and data pipeline

The game repository owns authored scenario data, asset recipes, dialogue,
balance, missions, visual direction, and thin presentation composition.
**QualiaDB owns** runtime simulation primitives, Q42 state and queries,
validation/rules, geometry, `.10d` authoring/loading/animation, HMC packaging,
rendering/picking, navigation/spatial processing, persistence, Vibe host, and
optional P64 inference. New missing capabilities go upstream first. This is
an iterative rule throughout production.

The desired release path is:

`original source/recipe -> Qualia validation and compilation -> .10d asset`
`+ Q42 identity/provenance -> versioned HMC content pack -> full Qualia WASM`
`-> QualiaPortal scene and semantic picking -> Q42 commands/events/save`.

Q42 references assets by stable ID and digest; it does not embed meshes.
The pack remains immutable and separately licensable from the Qualia engine.
Player saves contain mutable state/events and content/version references, not
private copies of the engine. The existing shell's hard-coded geometry is a
temporary proof and should be replaced by game-owned source assets compiled
and loaded through Qualia's HMC path. Keep provenance and the actual licence
of every asset explicit; do not relabel engine or game content as WebCivics.

Use VibeScript for authored scenarios, agent/task policies where appropriate,
data binding, and developer iteration when the public Qualia host supports
them. Browser JavaScript should be a thin substrate for bootstrapping, input,
DOM accessibility, and calls to Qualia WASM. As the host gains capabilities,
move game rules and orchestration into Q42/Qualia/Vibe rather than expanding
JS. Never make Vibe a bypass around permissions or the reducer.

## 8. Required QualiaDB upgrade work

The following are **upstream work packets to verify or implement**, not
assertions that every capability is absent. Search implementation and tests,
build a minimal fixture, then update the living capability ledger. Existing
QG-01 through QG-08 remain in force.

| ID | Required Qualia surface | Acceptance evidence before game integration |
|---|---|---|
| QG-09 | Deterministic fixed-tick multi-agent simulation, typed command queues, event receipts, pause/speed/replay | Same seed and 20-minute command trace produce identical graph/event digests in native and WASM |
| QG-10 | RTS camera, selection set, drag/multi-select, group commands, pointer hit tests, overlays, minimap projection through QualiaPortal/Webizen | Browser fixture selects 100 mixed entities accurately at multiple zooms and input scales |
| QG-11 | Navigation meshes/grids, terrain-aware pathfinding, obstacle updates, group movement, route reservation and cancellation | 100 agents traverse changing routes within measured tick and memory budgets; replay is stable |
| QG-12 | Large-scene rendering: scene graph/instancing, culling, LOD, texture/material pipeline, lighting, effects and scene streaming | Representative map meets the declared browser frame/memory/load budgets with semantic picking intact |
| QG-13 | `.10d` skeletal/articulated animation and Vibe/scene/event-time binding, continuing QG-03 | Original person and facility play named actions, save/reload/replay correctly, including reduced motion |
| QG-14 | Canonical HMC game asset pipeline, incremental build, manifest, dependency/digest/licence validation, continuing QG-01/02/05 | One separately authored sector pack loads offline and rejects tampered or incompatible entries |
| QG-15 | Mutable Q42 WASM session, spatial/aggregate queries, transaction validation, checkpoints and save migration | Busy world saves, reloads, migrates, and replays with identical result; game shell no longer stores authoritative N3 text |
| QG-16 | Game-scoped VibeScript host for world projections, command proposals, scenario cells, diagnostics, authoring preview | A scenario can be edited and previewed without private JS rules; forbidden mutation fails with a receipt |
| QG-17 | Terrain and world tooling: authoring/import, topology, roads, placement, nav/utility graph derivation and visual debug | One original sector builds reproducibly from source to `.10d`/Q42/HMC and supports placement/path tests |
| QG-18 | Integrate Qualia's existing audio libraries and Portal acoustic surfaces with game events; fill measured browser input/UI/accessibility gaps | One complete RTS interaction path works with keyboard and assistive technology; Qualia audio controls and captions persist |

For each packet: name the owning Qualia crate, public API/format version,
native and WASM fixtures, target baseline, bounded memory/CPU/GPU budgets,
failure behaviour, tested commit, and game integration receipt. If an API
exists but does not handle the game's scale or platform, it is still a gap.
Do not substitute a second renderer, pathfinder, scripting system, asset
format, or game-side simulation. The Qualia AGENTS.md constraints on bounded
construction and zero-heap hot paths apply to upstream implementation.

## 9. Production order and playable gates

**Gate A — RTS control prototype.** Preserve the current town, then add
Qualia-owned camera/selection/group command primitives. Five selectable
people, one valid placement, one blocked placement, one production queue,
and a minimap must work with mouse and keyboard. Use simple art only while
proving the controls. Record input latency and semantic-pick accuracy.

**Gate B — simulation vertical slice.** Migrate authoritative state to a
mutable Q42 session. Add fixed ticks, workers, movement, transport, build
stages, power, events, and replay. A player can choose two strategies to
restore workshop power in an unscripted 20-minute session. Save/reload and
explanations pass. Resolve QG-09/11/15 before calling it complete.

**Gate C — one finished sector.** Ship a separately authored HMC pack with
terrain, a landmark, several readable building families, one animated person,
effects, UI icons, ambience, music, and subtitles. Render through full Qualia
WASM. Visual review uses strategic and close zoom screenshots and actual play.
Resolve QG-01/02/03/12/13/14/17 before calling it complete.

**Gate D — full RTS slice.** Add multiple work teams, queues, rally points,
research progression, logistics bottlenecks, weather/fault response, diplomacy,
two linked ground nodes, campaign guidance, skirmish seed, overlays, options,
and accessibility. A new player can finish the scenario without reading a
technical guide, and an experienced RTS player has meaningful optimisation.

**Gate E — content production and release.** Expand only after the finished
sector's asset pipeline, build time, quality review, budgets, and playtest
metrics are stable. Produce more territories, factions/communities, campaigns,
music, cinematics, localisation, and difficulty profiles. Verify licence,
provenance, save migration, offline play, performance, accessibility, and
content review at pack and release level.

Each gate should produce a playable browser build, a short recorded gameplay
capture, test receipts, a measured budget report, a known-gap list, and the
next Qualia upstream work. Do not call a gate complete because only its UI
mockup or a unit test works.

## 10. Engineering and quality budgets

Choose a declared browser/device baseline at Gate A and record actual
numbers. Initial **targets to validate, not claims**: 60 fps on the main
desktop baseline and a stable 30 fps low-spec Qualia profile; no unbounded
per-tick allocation; command feedback visible within one frame; deterministic
world ticks under their assigned frame budget; 100 active agents in the
vertical slice; staged pack loading with progress; no lost or corrupted save
after interrupted writes. Revise budgets with measured results and a written
trade-off before increasing agent count or map scale.

Test at four levels: Qualia native unit/format fixtures, WASM integration
fixtures, browser playthroughs, and human playtests. Track frame time (p50,
p95), tick time, memory high-water, GPU memory/asset count, load time,
pick/command latency, save/replay digest, and crashes. Test ordinary,
low-spec, offline, storage-full, failed-pack, and reduced-motion paths.
Automate deterministic fixtures and asset validation; human review remains
necessary for art, usability, audio, narrative, and accessibility.

## 11. Immediate executable backlog

1. Record this RTS direction in the decision register; update the release
   matrix so the present button-driven slice is described as a technical
   precursor. Preserve existing progress and provenance.
2. Capture the current build at strategic and close zoom. Audit camera,
   selection, map, asset, animation, UI, sound, simulation, and persistence
   gaps against sections 3-8; put each missing engine function into the
   Qualia capability ledger with its QG dependency.
3. Make one original **sector specification**: map size, terrain heights,
   routes, six named sites, buildable areas, 10 starting agents, 8 initial
   structures, 6 resources, two workshop-power strategies, and one event.
   These are design targets for a fixture, subject to playtest tuning.
4. Prototype RTS camera and multi-selection in QualiaPortal, then bind
   pointer/keyboard input in the game shell. Keep selection and command state
   in Qualia; the browser layer sends input and renders UI.
5. Implement Q42 tick/command and navigation proofs in Qualia; migrate one
   worker assignment and one construction task from text/button actions.
6. Build one complete asset family through Qualia: recipe/source, `.10d`, Q42
   semantic manifest, HMC pack, browser render, selection, stage variants,
   provenance, and rights. That becomes the template for content production.
7. Playtest the control prototype before expanding map/content. Log where
   players mis-select, misread a cost, lose an agent, or fail to understand a
   blocked action. Fix the cause in the owning Qualia/game layer.

## 12. Definition of done for a feature

A feature is done only when a player can use it in the browser; the full
Qualia WASM build owns its engine behaviour; its Q42 facts and command receipts
are inspectable; it survives save/reload and deterministic replay; its assets
are separately tracked with provenance/licence; keyboard/accessibility and
reduced-motion behaviour are considered; it meets measured performance
budgets; and its tests and docs identify the pinned Qualia revision. If a
Qualia capability is missing, the dependent game feature stays open until
that capability is upgraded and integrated.

## 13. Decisions that need explicit product definition

The RTS direction is requested. These details still need a short design
decision and prototype, while independent Qualia work proceeds:

| Decision | Why it matters | Prototype or evidence |
|---|---|---|
| Combat and opponents | Age of Empires includes warfare; current Maslows Challenge vision explicitly excludes violence as entertaining player power | Compare one non-combat pressure scenario with a separate, clearly specified conflict proposal; decide before unit/combat design |
| Time model | Continuous RTS, pausable real time, and turn-like time blocks imply different simulation and accessibility needs | 20-minute fixed-tick prototype with pause/speed and replay |
| Scale | Agent, map, structure, and network counts determine engine budgets and asset pipeline | QG-11/12 stress scenes measured on declared browsers |
| Visual identity | Storybook 3D is the existing direction; the strategic camera needs strong readability at scale | One finished sector art bible plus two zoom-level reviews |
| Release scope and staffing | AAA polish needs art, audio, UX, QA, accessibility, writing, and technical production capacity | Costed asset inventory after one complete building/character pipeline |
| Online modes | Multiplayer would change authority, synchronisation, moderation, and save rules | Keep outside first RTS vertical slice until its own Qualia protocol gate |

Record accepted outcomes in the decision register and revise dependent specs.
Do not let unresolved decisions become a reason to replace Qualia technology.
