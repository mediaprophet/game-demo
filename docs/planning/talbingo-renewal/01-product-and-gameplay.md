# Talbingo renewal: product and gameplay

Status: proposed implementation specification, 17 October 2026. Read [the index](README.md), [the audit](00-current-build-audit.md), and the binding [QualiaDB-only contract](../13-qualiadb-only-development-contract.md) first. This document describes work to build, not capabilities already delivered. Task IDs are stable; keep them in implementation issues and evidence records.

## The intended change

Turn the current drive–inspect–answer–return demonstration into a small expedition game: choose what the travelling party wants from a trip, prepare, enjoy a journey, make a camp that fits those preferences, discover a place, help with an optional situation, and return with a story and reusable knowledge. Six destinations should support six recognisably different evenings of play. Driving skill, observational skill, planning and cooperation should each be useful without forcing every player to excel at all four.

The existing strengths are worth preserving: six selectable destinations, local scenes compiled through Qualia, a rendered camper and walker, outbound/return travel, wildlife cues, and browser controls. The main product problem is that almost everything leads to the same binary quiz. In `web/talbingo.html`, `SITES[*].correct` specifies the answer, `display()` accepts it by setting browser-local `acted`, and Enter/E and gamepad A select `button[data-correct="true"]`. `return` stays disabled until that answer is accepted. A new activity must change what players notice, decide and do; adding more answer cards to this structure is insufficient.

Target session lengths below are design hypotheses to validate with playtests, not measured current results:

| Session | Proposed experience | Target duration |
|---|---|---|
| First visit | Brief planning, assisted or direct drive, a camp decision, one discovery, return and debrief | 15–25 minutes |
| Return visit | A different preference or scenario changes the useful route, pitch, activity or resolution | 20–35 minutes |
| Regional expedition | Plan two or three sites with a shared party, equipment and scenario clock | 45–75 minutes, resumable anywhere a checkpoint is valid |
| Short session | Resume at a committed camp checkpoint and finish one activity | 5–10 minutes |

## Product rules

1. **The player chooses the goal.** A quiet visit, learning about place, a sociable evening, an exploration challenge and a practical contribution are all valid. Do not secretly turn every expedition into money or completion optimisation.
2. **Choices have inspectable trade-offs.** At least two solutions to each main site situation must succeed. The player can inspect why a resource, time or social consequence happened. An obviously dangerous option is not required to make a choice interesting.
3. **Leaving is valid.** A player can decline an activity, turn back, choose another day or return after arrival. A camp quiz must never imprison the vehicle. Unfinished objectives remain visibly unfinished and can be resumed under their stated conditions.
4. **Useful complexity is revealed through play.** Start with one rig, a small equipment set, two preferences and one situation. Add inventories, weather scenarios, party schedules and linked projects only after the first loop is understandable.
5. **Progression expands agency.** Completing an expedition reveals optional variations, equipment uses and shared projects. It does not assign moral worth to the player, penalise a chosen identity, or force grinding to see a named site.
6. **Content and presentation are distinct from authority.** Q42/Qualia owns action validation, state, resources, time, spatial eligibility, outcomes, saves and replay. VibeScript expresses bounded authored behaviour through supported capabilities. Webizen displays projections and sends intent. No browser-local quest reducer, inventory store or save format is permitted.
7. **Real places require evidence.** These proposals are fictional game situations attached to separately reviewed place packs. They do not establish actual pitches, structures, businesses, access rights, species distributions, ranger services or safe real-world behaviour.

## One coherent expedition loop

| Stage | Player action | Canonical result required | Presentation and failure behaviour |
|---|---|---|---|
| Plan | Choose destination, scenario date, party preferences and route; inspect pack status | Expedition references pinned scenario/pack, route edge, party, rig and deterministic scenario seed | State illustrative or source-backed coverage and show applicable authored constraints. Unavailable mapped routes stay visible with a reason. |
| Prepare | Pick or borrow bounded equipment; review rig and schedule | Reserve/transfer identified items through the existing inventory owner; confirm expedition | Explain missing prerequisites and alternatives. Do not charge or duplicate an item until an accepted receipt. |
| Travel | Drive or use a supported accessibility assistance profile | Route progress, contacts and encounter results belong to Qualia simulation | The same route, hazards, clock and arrival rules apply. Input/camera help is adjustable. |
| Park | Stop in an approved authored parking anchor; exit when conditions allow | One arrival receipt, parked rig pose, walker entry and accessible interaction targets | Show a clear arrival zone and available exit. Never place a walker inside the vehicle or an obstacle. |
| Explore | Inspect two or more meaningful features, optional conversation, field journal | Discoveries refer to stable entity IDs and evidence; observations update only allowed fields | An observation reveals information rather than unlocking an arbitrary correct button. Accessible target navigation follows the same reachability rules. |
| Choose and set up | Compare two viable pitches/approaches, then perform a small practical sequence | Validate prerequisites, reserve equipment, commit placement and consequences atomically | Preview costs and limitations. Reposition/pack up is supported where scenario rules allow. |
| Activity | Complete, collaborate, defer or decline a site-specific situation | Objective progress and consequences, including alternate resolution IDs | Never require a runtime LLM or a reflex minigame as the sole resolution. |
| Pack and return | Recover eligible equipment, check remaining tasks, choose a directed route | Persist camp changes, item ownership, party, route and checkpoint | Returning is allowed after safe exit prerequisites, even with optional work incomplete. |
| Debrief | Review what changed; choose a next goal or stop | Journal and progression reference accepted receipts | Explain observed consequences, not a generic gold star or one overall human-worth score. |

Arrival and setup are separate. Merely crossing a world-coordinate threshold must not complete arrival, parking, camp setup and discovery at once. Similarly, entering a new scene must not replenish supplies, repair damage or reset the scenario clock. See [architecture/state](03-architecture-state-and-platform.md) and [driving/traversal](04-driving-traversal-and-simulation.md).

## Bounded state model to agree before authoring

The following are proposed content concepts, not names of existing public APIs. Map them to the verified Qualia schemas first; if they cannot be represented, open an upstream capability task. Do not implement these as a JavaScript substitute.

| Concept | Minimum fields/references | Invariant |
|---|---|---|
| Expedition | Stable ID, content version, scenario seed/date, selected goal IDs, party ID, rig ID, active leg/site, status | Reloading reconstructs the same expedition; switching input devices changes none of these facts. |
| Site situation | Stable ID, site/entity anchors, objective graph, available resolution IDs, repeat policy, provenance classification | Scene labels and geometry refer to the same semantic entities. |
| Discovery | Entity ID, observation ID, evidence version, visibility and journal status | Repeated inspection does not duplicate rewards or resources. |
| Camp plan | Validated pitch anchor, arrangement ID, equipment references, prerequisites, effects, reversible actions | No overlapping exclusive occupancy or second copy of a unique item. |
| Resource | Item/capacity ID, owner, quantity or condition, unit, reservation state | Consumption/transfers are explicit; no generic number whose meaning changes between screens. |
| Consequence | Source action/receipt, affected fields, reason codes, timing, uncertainty where relevant | Displayable explanation agrees with canonical state. |
| Objective | Status, prerequisites, optional/required scope, accepted evidence, alternate resolutions | A declined optional objective cannot block exit or corrupt the journal. |

Begin with three limited planning dimensions: **available time**, **equipment/carrying capacity**, and **camp fit against the player's declared preferences**. Represent environmental restrictions as explicit constraints, not a spendable “nature damage” currency. Add money only when an existing campaign transaction genuinely requires it. Do not add hunger, health, fatigue, temperature or wellbeing bars merely to fill a HUD. Any later needs model must obey [wellbeing boundaries](../09-wellbeing-and-health-boundaries.md).

Illustrative authored choice, to be translated into the approved schema:

```yaml
# DESIGN EXAMPLE ONLY. Not an existing Qualia API or publishable pack.
id: talbingo.pines.camp-plan.v1
classification: authored-simulation
requires: [arrival-accepted, rig-parked, clearing-observed]
options:
  - id: compact-near-rig
    label: Keep the compact camp near the rig
    effects: [shorter-setup, fewer-carry-trips, less-shared-seating]
    resolveWith: [pitch-eligible, equipment-available, arrangement-confirmed]
  - id: shared-clearing
    label: Use the shared clearing arrangement
    effects: [longer-setup, shared-seating, more-carry-trips]
    resolveWith: [pitch-eligible, equipment-available, arrangement-confirmed]
  - id: day-visit
    label: Make this a day visit
    effects: [no-overnight-setup, shorter-visit]
    resolveWith: [day-visit-confirmed]
repeatPolicy: one-active-camp-plan-per-expedition-site
```

An option card displays what is known, what remains unknown and what can be inspected. It does not show a secret answer flag. Preview is advisory; commit revalidates the current world revision. An expired reservation or changed constraint returns an explainable rejection with no partial effects.

## Six destinations with different authored play

Everything in the following briefs is a **proposed authored scenario**. Geometry, routes, ecological labels, cultural interpretation and actual facilities must pass [content/provenance review](05-world-content-and-provenance.md). The current timber crossing, hut and ridge platform are prototype motifs, not evidence that those structures belong at the real named locations. If source review rejects one, preserve the interaction intent using a clearly fictional prop in the illustrative pack or redesign the sourced pack; never silently promote it into fact.

Each delivered site needs: a unique approach/arrival composition, at least three inspectable entities, a main activity with two viable resolutions plus a decline/defer exit, one optional discovery, two visible aftermath states, and a replay variation. Reusing interaction components is encouraged; copying the same objective text with different scenery is not.

### The Pines — make a camp that fits this party

**Design role:** welcoming first expedition; learn observation, planning and a visible consequence without pressure. Proposed entities: arrival board, two eligible pitch anchors, shared table area, equipment station and a shoreline observation point. Their existence and positions are authored until validated.

**Sequence:** arrive and park → inspect the two pitch summaries → choose a compact near-rig arrangement or a more communal arrangement → carry/place two identified equipment pieces using semantic snap anchors → invite a fictional travelling companion to choose a seat, or retain a private arrangement → record the preference trade-off → pack and return or remain at a checkpoint.

**Viable outcomes:** compact camp minimises setup/carry time; shared camp creates more useful gathering space; day visit avoids overnight setup. Neither requires unsafe water-edge camping as the “wrong” foil. Costs use scenario time and actual equipment reservations. Arrangement preview must show which preference it serves. An incomplete setup can be packed away.

**Optional discovery:** observe an authored wildlife moment from an approved interaction point and record what was visible; no chasing, feeding or guaranteed sighting. **Replay:** new party preference or a published scenario condition makes the previous plan less convenient, not forbidden. **Required evidence:** both arrangements and the day visit complete from a fresh expedition and survive reload before/after placement; the debrief names the selected trade-off.

### Humes Crossing — coordinate shared space

**Design role:** cooperation and scheduling. Proposed entities: arrival/parking anchors, shared equipment staging zone, notice board and two fictional visitor parties with explicit schedules. Do not rely on the existing timber-deck motif as a factual crossing.

**Sequence:** observe space occupancy and visitor requests → inspect a simple two-slot schedule → choose staggered use of the shared area or move the player's activity to another eligible anchor → place/remove the party's kit without blocking access → resolve a consent-based conversation → see both parties complete their plans.

**Viable outcomes:** wait and share equipment to save carrying capacity, or set up separately to save time; a quiet day visit remains possible. An authored conflict is a solvable resource constraint, not an NPC who must be persuaded by one magic sentence. A party can decline sharing; the alternative remains available.

**Optional discovery:** compare two observations in the field journal. **Replay:** change arrival order, declared preference or a nonessential item request from the deterministic seed. **Required evidence:** shared reservations cannot overlap; no item duplicates; both resolutions and refusal path preserve departure. UI and world show the same occupied/available anchors.

### Jounama Creek — observe and choose a route

**Design role:** local traversal and evidence-based planning. Proposed entities: trail entry, two observation points, a reviewed authored boundary, two route segments and an alternative activity anchor.

**Sequence:** inspect a route board and a nearby surface clue → choose a longer accessible observation loop or a shorter permitted route with an equipment constraint → traverse with collision-aware movement → record two observations → decide whether to finish the optional loop or return within the declared time budget.

**Viable outcomes:** longer route with no special equipment; shorter route using an explicitly supported item/party capability; stationary observation activity for players who choose an equivalent accessible route. This is a fictional route puzzle, not a lesson in crossing actual water. Never require entering a creek or leaving a reviewed route.

**Optional discovery:** connect an observed rock/plant detail to a sourced journal entry only when that entity's provenance supports it; otherwise label the description authored. **Replay:** a published scenario condition changes one route's eligibility, with the alternative still viable. **Required evidence:** reachability matches both navigation and interaction; inaccessible entities cannot be remotely activated by an inspector; hints explain the missing condition rather than the exact answer immediately.

### Yarrangobilly Village — follow evidence and contribute

**Design role:** a short environmental story and a small cooperative task. Proposed entities: interpretation point, two contemporary fictional story clues, visitor meeting place and a removable field display. Do not invent historical events or Indigenous cultural knowledge. Heritage content needs reviewed sources and appropriate authority.

**Sequence:** inspect an attributed place note → follow two clues in either order → speak to a clearly fictional visitor → choose to help assemble an approved temporary display or contribute an observation from the player's journal → review how the contribution is attributed → leave the area as the scenario specifies.

**Viable outcomes:** practical assembly uses a tool reservation and time; a documented contribution uses already collected evidence; decline permits the visitor's alternative plan. Do not imply permission to repair or alter a real heritage structure. The current hut can only be retained with correct provenance/classification.

**Optional discovery:** a second fictional perspective changes the wording of the journal summary without changing historical facts. **Replay:** different request/evidence availability, with both story paths understandable. **Required evidence:** observations can be collected in either order; attribution survives save/reload; no unsupported factual narrative becomes the canonical place description.

### O'Hares — prepare, carry and adapt

**Design role:** longer-journey planning and a bounded equipment puzzle. Proposed entities: parking anchor, loadout inspection point, two eligible activity positions and a fictional equipment fault. Source review decides the actual landscape and whether any lookout motif is appropriate; the current ridge platform is not a location fact.

**Sequence:** arrive after a journey whose condition persists → inspect the equipment needed for the chosen activity → identify a missing/incompatible component from visible clues → choose a compact activity using available kit or borrow a compatible item from a consenting fictional party → carry/setup within the declared capacity → test the result and return the borrowed item.

**Viable outcomes:** adapt the activity to existing equipment, borrow with an explicit return obligation, or defer. The puzzle is about game components and compatibility, not real vehicle repair advice. One short assembly can later connect to the existing workshop system; do not build a separate repair simulator here.

**Optional discovery:** a panoramic observation from a verified accessible anchor or equivalent seated/near-rig activity. **Replay:** another compatible kit combination and different party preference. **Required evidence:** borrowing and return are canonical transfers; leaving with a borrowed item follows a visible scenario rule; failed assembly validation consumes nothing.

### Three Mile Dam — change the plan when conditions change

**Design role:** exposed-place atmosphere, preparation and dignified decisions to defer. Proposed entities: scenario access board, sheltered activity anchor, permitted exposed observation anchor, equipment cache and departure anchor. Real seasonal/access facts are dated source records; the game uses an explicit scenario date and separately labelled authored weather.

**Sequence:** review the scenario's known conditions at the hub → pack relevant kit → inspect the arrival observation → choose a prepared shorter activity at one eligible anchor, a sheltered alternative, or return/defer → use the appropriate setup sequence → record what changed the decision.

**Viable outcomes:** the exposed activity requires declared prerequisites, the sheltered alternative requires different time/kit, and deferral preserves resources and marks an intentionally postponed goal. Do not turn a closure into a driving challenge. If a mapped route is unavailable, retain a source-labelled atlas state and another playable destination; do not teleport through a gate.

**Optional discovery:** a season-specific authored observation journal, with species/source uncertainty visible. **Replay:** a different documented scenario seed/date changes conditions and therefore useful preparations. **Required evidence:** dated real-world notes and simulated conditions cannot be mistaken for each other; all three choices produce understandable aftermath and permit a return or valid next destination.

## Common activity grammar

Build five reusable content patterns against Qualia capabilities: **observe → compare**, **reserve → place → verify**, **talk → agree → fulfil**, **diagnose from evidence → choose compatible part → test**, and **plan route → traverse → record**. A site combines two or three; it should not require new engine code for every object.

Every activity record must include stable IDs, version, site/entity references, prerequisites, steps, alternate resolutions, costs/effects, cancellation rules, repeat policy, hint ladder, accessible equivalent, provenance classification and automated fixtures. Optional NPC dialogue is authored first. A runtime model can later propose dialogue only through the approved gateway; it is never necessary to solve, verify or remember a task.

Use a three-stage hint ladder: describe what is missing → point to evidence/location → describe the next valid operation. Hints never spend scarce resources or lower rewards. A resolved clue stays in the journal. Conversations can be reopened; skipping dialogue retains the facts needed to act.

## Progression and return visits

All six destinations remain visible at the outset. “Available”, “requires reviewed pack”, “scenario route unavailable” and “optional challenge not started” are different states. First-visit recommendations are optional. Content coverage and platform readiness must never masquerade as player skill locks.

Use a regional journal with four sections: places visited, discoveries with their source status, equipment/techniques learned, and contributions with their receipts. Show individual completed and unfinished goals. Do not collapse these into a single score. A return visit should change at least one choice through an authored scenario variation, not merely repeat all three inspections for a larger counter.

Proposed progression tiers:

| Tier | Added decisions | Required earlier evidence |
|---|---|---|
| First expedition | One site, one rig, two goals, a small starter kit | Verified loop, accessibility and save/recovery gates |
| Regional explorer | Optional alternate activities and two-site itinerary | Both paths at the first site work; route graph and transitions verified |
| Contributor | A bounded shared project connects observations/equipment across two sites | Inventory, reservations, contributions and replay are canonical |
| Expedition author | Select a published scenario date/seed and equipment constraint | Every variation has valid routes, reachable goals and authored exits |

The initial shared project should be small: an in-game regional visitor field journal assembled from contributions at two sites, with explicit attribution and no implication of an official park publication. Larger trading, production and workshop links belong to the existing connected-mode programme after the six-site game works. Multiplayer, procedural quest generation, live maps, live weather, dynamic economy and unrestricted building are separate scope decisions, not hidden prerequisites for this renewal.

## Play profiles and difficulty

Offer **Relaxed**, **Expedition**, and **Fieldwork** as proposed named presets, with individual settings editable. Relaxed reduces time pressure and enables assistance; Expedition uses published time/equipment constraints; Fieldwork asks for more observations and planning. None gates content by disability or identity. Motor assistance and sensory preferences are independent from scenario difficulty.

The same action validation and conservation rules apply in every profile. An assistance mode may provide a Qualia-owned guided route or semantic target selection, but cannot bypass a blocked route, fabricate discovery, or mutate position in browser code. Accessibility alternatives should earn equivalent narrative completion. Scenario time advances only under the published clock contract; opening a help/settings panel does not quietly consume time.

## Implementation work orders

File paths below are locations to inspect or proposed content destinations. Create new folders only after [architecture ownership](03-architecture-state-and-platform.md) is agreed. Each work order must record the game revision, Qualia revision, pack digest, fixture and result. A missing capability blocks the dependent task until its upstream proof exists.

### PLAY-01 — establish the expedition content contract

**Depends on:** architecture capability assessment; provenance classifications in [05](05-world-content-and-provenance.md).

1. Inventory `SITES`, `phase`, `acted`, `inspected`, `eventCleared`, `reset()`, `begin()` and return gating in `web/talbingo.html`; distinguish authored facts, presentation state and authoritative state.
2. Agree stable site IDs shared by place packs, scene entities, activities and save records; stop using array position as persistent identity.
3. Map the expedition/objective/resource concepts above to existing public Qualia schemas. Add upstream schema/capability work for missing concepts before game integration.
4. Author one minimal valid activity and malformed fixtures: missing entity, cyclic prerequisite, no viable exit, duplicate resolution ID, and unsupported effect.
5. Record the validated content contract and an author checklist alongside the versioned scenario sources; proposed directory: `web/fixtures/talbingo/activities/`.

**Done when:** content validation rejects all malformed fixtures; every gameplay field has one owner; a new activity can be added without editing a browser rule function; published capability names/versions and proof commands are recorded.

### PLAY-02 — implement the expedition flow and first arrival

**Depends on:** PLAY-01; canonical transition/command/save gates in [03](03-architecture-state-and-platform.md); traversal arrival gates in [04](04-driving-traversal-and-simulation.md); UX-01 and UX-02.

1. Build hub goal/route preparation from the canonical projection.
2. Replace local phase changes with accepted transition receipts and pending/rejected feedback.
3. Separate route arrival, parking, exit vehicle, explore and return; use stable semantic anchors.
4. Allow cancel-before-commit, early return and incomplete-objective departure. Keep a durable summary of unfinished work.
5. Resume after each stage from the existing canonical checkpoint mechanism, including mid-activity once supported.

**Done when:** an expedition can complete, turn back, decline its activity and reload at each valid checkpoint; double-click arrival/departure cannot duplicate effects; changing a selector cannot discard active state.

### PLAY-03 — deliver The Pines as the complete gameplay slice

**Depends on:** PLAY-02, PLAY-04, UX-03 through UX-07; The Pines pack and semantic anchors accepted under [05](05-world-content-and-provenance.md).

1. Author its two pitch arrangements and day-visit path with explicit costs, preference effects and reversal rules.
2. Build two meaningful observations, equipment placement and one optional discovery using approved interaction capabilities.
3. Connect a consequence preview and receipt-based explanation to the in-world aftermath.
4. Add a revised party preference scenario to demonstrate replay value.
5. Walk both primary resolutions and the decline/return path with keyboard, touch and gamepad; save/reload halfway through each setup.

**Done when:** a first-time player can state their goal, recognise two valid plans and explain one consequence; all paths reach debrief; no `correct` flag or device shortcut decides the activity; no unaccepted action changes camp state.

### PLAY-04 — reusable activity, inventory and hint patterns

**Depends on:** PLAY-01; supported Qualia inventory/reservation/spatial interaction contracts.

1. Implement the five activity patterns as bounded content/Vibe behaviours, not bespoke browser reducers.
2. Author explanatory rejection messages from structured reason codes; include missing item, unreachable target, occupied anchor, incompatible part and expired world revision.
3. Add cancellation, repeated interaction, hint progression and accessible selection to every pattern.
4. Prove conservation and idempotency through native/WASM fixtures plus one browser interaction for each pattern.

**Done when:** adding a content instance requires IDs, anchors, conditions and authored text; repeated actions cannot mint items; all five patterns expose at least one alternative/cancel path and a persisted intermediate state.

### PLAY-05 — author Humes Crossing and Jounama Creek

**Depends on:** PLAY-03 validated in playtesting, PLAY-04; reviewed packs/traversal for the two sites.

1. Author the shared-space schedule and the route-observation situation from their briefs.
2. Create at least three meaningful entities per site, two viable resolutions, optional discovery and visible aftermath.
3. Add deterministic variations that change constraints rather than only text.
4. Test order independence where allowed, refusal/defer paths, unreachable targets, resource conservation and reload.

**Done when:** players can distinguish cooperation at Humes Crossing from evidence/traversal at Jounama without reading the site title; each resolution is validated and all exits work.

### PLAY-06 — author Yarrangobilly Village and O'Hares

**Depends on:** PLAY-03, PLAY-04; reviewed narrative/provenance and the two site packs.

1. Author the fictional contribution story and equipment-adaptation puzzle.
2. Separate factual interpretation from fictional dialogue at the content field and UI levels.
3. Bind borrowed/returned items to canonical ownership and explicit obligations.
4. Test both clue orders, dialogue skipping/reopening, compatible alternatives and departure while unfinished.

**Done when:** both main resolutions per site complete; no story invents authoritative heritage facts; ownership and attribution survive replay; decline is a valid outcome.

### PLAY-07 — author Three Mile Dam and conditions-based planning

**Depends on:** PLAY-03, PLAY-04; explicit scenario clock/date and access constraints; reviewed pack.

1. Distinguish dated source access notes from the authored scenario conditions in data and text.
2. Author prepared, sheltered and defer/return paths with visible prerequisites.
3. Build two deterministic scenario variants and validate all routes/objectives before publication.
4. Test an unavailable mapped route without substituting illustrative geometry; keep coverage status distinct from an in-game locked objective.

**Done when:** all three paths are understandable and recoverable; changing real-world wall clock does not silently alter a saved scenario; no route allows passing a canonical blocked gate.

### PLAY-08 — regional journal, progression and multi-site expedition

**Depends on:** PLAY-05 through PLAY-07, canonical multi-leg routes/save/replay, UX-08.

1. Add a receipt-backed journal with source status, completed and unfinished goals and revisit notes.
2. Offer a two-site itinerary whose party, equipment, condition and time carry across site transitions.
3. Author the small regional field-journal contribution project with at least two completion paths.
4. Ensure repeat visits cannot duplicate one-time rewards or erase prior contributions; expose replay/new-expedition separately from resume.

**Done when:** two-site play reloads after each leg with identical canonical state; useful new choices appear on revisit; all six sites remain discoverable without grind; contribution attribution is complete.

### PLAY-09 — calibrate profiles, pacing and consequence explanations

**Depends on:** PLAY-03 for initial calibration; repeat on PLAY-08; UX-06 and UX-07.

1. Write each profile's explicit differences in time, equipment constraints, hints and assistance.
2. Run structured sessions with new players, including players using assistive controls. Ask what they intended, what options existed and why the outcome happened.
3. Measure time to first meaningful action, help requests, unfinished-session reasons and ability to explain trade-offs. Use observer notes or opt-in aggregate events only.
4. Adjust content sequencing before adding more UI instruction. Preserve alternative outcomes and conservation invariants.

**Done when:** in an initial moderated cohort of at least eight first-time players, at least six reach a meaningful camp decision unaided and can explain one consequence; every participant can find pause/exit; all reported blocking accessibility failures are resolved or the affected release is withheld. These are formative release gates, not statistically representative claims.

### PLAY-10 — connect to the broader game after the regional loop works

**Depends on:** PLAY-08 and release evidence in [08](08-testing-and-release.md); existing campaign/connected-mode contracts.

1. Choose one modest bridge: a borrowed tool, a workshop-compatible component puzzle, or the regional field journal. Record its exact canonical IDs and ownership.
2. Use the shared campaign party/rig/inventory/clock rather than copying Talbingo values into another state store.
3. Test entering/leaving Talbingo repeatedly, imported campaign constraints, missing pack versions and save migration.
4. Add other campaign systems only through separate reviewed work orders with their own capabilities and acceptance.

**Done when:** a complete Talbingo expedition changes exactly the intended campaign facts, survives round-trip mode transitions and replay, and does not reset damage, duplicate resources or overwrite the player's optional private context.

## Product acceptance for the whole renewal

- Six independent playable destinations, each meeting its scene, activity, alternative-resolution, discovery and aftermath contract.
- At least twelve distinct main resolution paths across the six sites, plus valid decline/defer/return behaviour everywhere.
- One complete first-time loop, one multi-site expedition and one worthwhile revisit verified by human playthrough.
- All authoritative outcomes, item movements, activity progress and scenario time come from Qualia receipts; reload and replay preserve them.
- Input assistance, hints and sensory settings preserve access to equivalent content/outcomes; the game requires no personal disclosure or runtime model.
- Content status, scenario assumptions and real-place source dates are visible where they affect a decision.
- Updated tests prove meaningful alternatives and consequences. Replace the current “incorrect choice rejected / correct choice accepted” expectations in `scripts/check-talbingo.mjs`; retaining those assertions would preserve the behaviour this plan is replacing.

Cross-cutting release, performance, security and evidence requirements remain in [07](07-performance-reliability-and-security.md), [08](08-testing-and-release.md) and [the delivery backlog](09-delivery-backlog.md).
