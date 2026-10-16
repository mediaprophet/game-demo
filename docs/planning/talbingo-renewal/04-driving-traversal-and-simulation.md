# 04 — Driving, traversal and simulation

Status: proposed implementation design, supported by source inspection on 17 October 2026. These are game mechanics, not real driving, towing, mechanical repair or campsite safety instructions. New numerical values below are initial tuning targets to validate in playtests, not measured performance or upstream capabilities.

The gameplay loop is **plan → travel → park → explore → choose/setup → optional activity → pack/return → debrief**. Build one complete version at The Pines, then apply the same contracts to all six sites. All six remain available without a progression grind. Profile variations come after the coherent default slice; they share content and outcomes. See [product and gameplay](01-product-and-gameplay.md).

## What the current implementation actually does

| Source and symbol | Source-verified behavior | Renewal implication |
| --- | --- | --- |
| `web/talbingo.html`: `step` | Passes `rig_id:0`, `rig_roadworthy:true`, `cooling_efficiency:1`, zero terrain heights, `crossing_open:true`, `period:'day'` | Existing policy complexity does not mean Talbingo has working rig choice, slope loading, closure or time simulation. |
| `talbingo-drive.vibe`: `drive_step` | Vehicle motion, shoulder rejection, nearest hazard, thermal response through `PhysicsWorkbench.compute`, traffic queue and trailer formulas | Reuse authored behavior only after its scene/context inputs and authoritative state are correct. Do not copy this into JavaScript. |
| `talbingo_scenes.rs`: `road_z`; Vibe `road_center_z` | Similar hand-authored curves are duplicated in Rust and Vibe | The alignment smoke test samples 30 positions; one shared versioned route source should replace the duplicated formulas. |
| `talbingo-drive.vibe`: `camp_step` | Axis-aligned camp bounds and a few site-specific water thresholds | No general obstacle sweep or terrain-grounded character controller is demonstrated. Diagonal input moves faster than a single axis. |
| `lib.rs`: `talbingo_vehicle_pose`, `talbingo_walker_pose` | Bounds-checks X/Z and applies root transforms with zero vertical displacement | Meshes are authored using terrain elevation, but root movement is not terrain-following across the scene. Walker pose takes no facing direction. |
| `talbingo_scenes.rs`: `author_walker` | Single-joint binding and idle clip | It is not a complete locomotion rig with walk, turn and interaction states. |
| `drive_contacts.rs` and `drive_contact_integration.rs` | Main-game terrain sampling and Qualia spring integration | This is a useful integration candidate, but samples `terrain_height_world` from the main terrain. It must accept validated scene context before serving Talbingo. |
| `talbingo.html`: arrival tests | Outbound enters camp at `x >= 16.9`; inbound completes at `x <= -5.4` **or any** `route-end` warning | A threshold alone does not prove arrival, parking, correct destination or direction. The inbound endpoint condition can treat the wrong route end as completion. |
| `talbingo.html`: hazard construction | Hazard X is fixed at 8, but its Z uses road height-in-plan at the **vehicle's current X** | The hazard's position can move laterally as the vehicle progresses. Encounters need stable world anchors and accepted lifecycle state. |
| `check-talbingo.mjs` | All six routes are sampled/traversed with rig 0, clear crossing, no hazards and zero terrain; one browser round trip is played | These checks establish a limited baseline, not towing, collision, save/replay or input-quality coverage. |

## Design principles that constrain implementation

1. **Readable, forgiving control.** A player can learn braking, steering, parking and interaction from feedback. Difficulty comes from decisions and varied terrain, not unexplained control changes.
2. **One world definition.** Rendered terrain, driveable surface, collision, route progress, landmarks and audio surfaces resolve to the same scene IDs and geometry references.
3. **Accepted motion drives presentation.** The renderer, wheel animation, HUD and footsteps follow authoritative accepted movement. Visual interpolation never authorizes movement through an obstacle.
4. **Deterministic consequences.** Vehicle condition, encounters, time and activities survive saving and reproduce on replay. Optional assistance changes are recorded as policy inputs.
5. **QualiaDB owns reusable simulation.** Generic collision, integration, path queries, terrain contact and navigation are upstream capabilities. This repository authors scene facts, rig/handling parameters and bounded Vibe policies under the [development contract](../13-qualiadb-only-development-contract.md).

## World scale, route geometry and geographic truth

Replace ambiguous `game units` with a documented physical scene frame. Target metres for local position, seconds for simulation time, radians for angles, metres/second for speed and Celsius for temperature. Display km/h only as `m/s × 3.6` after physical scale is established. The current display multiplies speed by 18; do not preserve that conversion as real telemetry.

For every scene manifest specify:

- Stable scene ID, local origin, axis orientation, vertical datum or authored-height label, coordinate reference, bounds and `metresPerUnit` during legacy migration.
- Separate source geographic geometry from any authored or compressed play-space geometry. Record the transform/provenance and label illustrative space to the player.
- Directed route edges with start/end anchors, centreline geometry reference, lane/corridor width, surface segments, speed profile, shoulder/recovery areas, passing/turning areas and closures.
- Semantic polygons/volumes for parking, walking, water, marsh, barriers, bridges, lookouts and activity interaction. A bridge's walkable surface must override water beneath it using height/layer queries.
- Rig constraints by supported profile: clearances, turn space, permissible routes and trailer access. A visible road is not sufficient evidence that every rig fits.

Use route arc length and segment identity to measure progress; X alone cannot support bends, loops, reverse travel or branches. Crossing a gate produces an event only in the correct route/direction, inside the arrival volume and under a parking threshold. Backing up decreases current progress but cannot erase an already accepted observation. Cumulative travelled distance is a separate field.

An initial authored route may be shorter than its real-world approach, but the UI must distinguish **playable route length** from **reference real-world distance**. Do not interpolate latitude/longitude between endpoints and label it live GPS. Real route coordinates require a sourced geometry and validated coordinate transform through QualiaDB. See [world content and provenance](05-world-content-and-provenance.md).

## Proposed simulation tick pipeline

This is the required behavior of the verified upstream/Vibe integration, not a new game-owned engine. Preserve 20 Hz during migration, with bounded upstream substeps if the collision/vehicle owner requires them.

1. Accept normalized controls and discrete requests for tick N; use saved authoritative state rather than browser-supplied health/position.
2. Resolve scene/route/rig context, surfaces, closures, environment facts and encounter state from the pinned world.
3. Integrate proposed vehicle or actor motion using the selected QualiaDB solver and authored parameters.
4. Query swept collision/contact against the same authored scene geometry; resolve permitted motion and ground contact.
5. Update wheel/body/trailer pose, condition, encounter phases and route gates from the **accepted** path.
6. Validate invariants and commit one state revision/receipt, including derived warnings, cue events and activity availability.
7. Project previous/current accepted poses to rendering; interpolate at display rate without changing the simulation.

Rollback on invalid numeric output, missing required collision data or an exhausted mandatory query budget: retain previous accepted pose and condition, stop issuing motion inputs, and surface a recoverable fault. Do not replace missing terrain with zero height during active play. Loading a scene without valid drive/walk surfaces must fail admission before controls transfer.

## Vehicle feel and control contract

### Inputs

Use analog throttle/brake in `[0,1]`, steer in `[-1,1]`, an explicit gear/direction state and separate parking/handbrake intent. Keyboard and touch map into those channels using approved bounded ramps. Gamepad triggers retain analog values rather than becoming boolean key presses. Define a configurable dead zone with visual calibration feedback and preserve independent device states.

The current S/down behavior immediately requests reverse. Target behavior: brake while moving forward, reach a low-speed threshold, then request reverse through a deliberate held or discrete gear action. The input guide must match the selected option. Brake must decelerate both forward and reverse motion; steering sign and wheel animation must remain coherent in reverse. Opposing throttle/brake input has a documented deterministic result.

Keep control smoothing in the upstream input/vehicle surface or bounded authored Vibe policy. The browser can choose bindings and render prompts; it must not integrate a second steering model.

### Handling and tuning

Use a parameter table per rig with mass/load class, wheelbase, axle/track geometry, steering limit/rate, drive force/acceleration curve, braking curve, drag/rolling resistance, surface response and suspension travel. Verify units and field ranges. Avoid adjusting many constants until a known route/contact test shows which effect is wrong.

Initial review targets:

- Steering returns predictably to centre; speed affects turning capability gradually.
- Braking produces visible stopping distance and never snaps from full speed to full reverse.
- Gravel/shoulder surfaces change feedback and handling without trapping the rig at an invisible wall.
- Small terrain height changes produce bounded body/wheel response; wheel contact does not sink through terrain or float conspicuously.
- Thermal/cargo effects affect required decisions on a longer trip, but do not overwhelm the introductory route.
- Camera mode and frame rate cannot change the accepted stopping/turning trajectory.

Maintain one recorded handling course: straight acceleration/braking, steady turn, S bend, sealed-to-gravel transition, modest slope, uneven shoulder, parking bay, reverse bay and tow reversal. Record input tape, final pose and important distances. Review feel in browser at 30/60/high-refresh rendering; compare semantic output by fixed tick.

### Trailer and rig families

The existing Vibe profiles cover four numeric IDs and some trailer behavior. ARC-01 must establish the meaning and scene/animation mapping of each rig before exposing selection. Use stable profile IDs and refer to the existing asset/workshop catalog; do not assign new user-facing names based on numeric guesses.

Trailer requirements:

- Hitch anchor, tow-ball height, wheelbase/axles, track and body collision extents come from the rig definition shared with the visual asset.
- Trailer sweep uses its own accepted previous/next pose; the towing vehicle fitting through a gap does not prove trailer clearance.
- Articulation warning uses hysteresis to avoid flicker; excessive articulation prevents further harmful reverse motion and permits forward recovery.
- Reverse behavior, inner-wheel tracking and turning clearance are visibly distinct from an untowed rig.
- Wheel phase reflects signed distance; visual rotation alone cannot move a trailer.
- Parking, detach/attach if supported later, save/reload and recovery preserve identity, cargo, condition and ownership.

Do not enable a trailer profile until its route access, collision, contact, animation and save fixtures pass. Keep the feature visibly tracked as blocked on the required capability rather than silently presenting a car-shaped surrogate.

## Terrain contact, collision and recovery

### Collision layers and queries

Proposed semantic layers: `vehicle_solid`, `walk_solid`, `ground`, `water`, `soft_ground`, `interaction`, `wildlife`, `route_gate`, `camera_occluder`. These are schema concepts to map to the selected public API, not assumed engine flags. Each instance needs a stable semantic ID and geometry digest. Declare intentionally non-solid vegetation/decoration so collision choices are reviewable.

Required queries include ground height/normal and material at a contact, swept rig/actor volume, stable overlap/arrival volume, visibility/interaction distance, and camera occlusion. Use coarse terrain/obstacle collision representations generated by QualiaDB from authored assets; collision need not use every visual leaf triangle. Store the relation to the visual asset and test its intended clearance.

Collision resolution should slide an actor along a wall where supported, prevent penetration/tunnelling at maximum speed and avoid a high-speed jump across a narrow barrier. Water/steep-edge areas require matching visible cues and collision boundaries. Never infer traversability from object colour or a hard-coded site-number threshold.

The local upstream `render/physics` module contains AABBs, bounds admission, kinematic joints and spring/thermal response. Those source files do **not** establish a complete vehicle/character swept-collision system. Search broader QualiaDB spatial/physics owners and make a minimal proof; if unavailable, implement the generic query/integration upstream before terrain-blocking features ship.

### Grounding and pose

Generalize `drive_contacts.rs` around a validated scene/terrain reference, not `site` switches or direct main-terrain calls. Sample current terrain at rig-defined contact locations using QualiaDB transforms and spatial queries. Feed those samples to the verified integrator; author spring/response parameters in Vibe. Use accepted position and contact state in the vehicle root, wheel transforms and trailer transforms.

Replace Talbingo root translation at Y=0 with the accepted terrain-following pose through the public actor API. The initial mesh authoring origin and inverse bind must agree with the scene frame; simply adding elevation to a mesh already authored at world height can double its offset. Add an explicit origin/contact fixture before changing art scale.

Walking needs ground height, normal, maximum walkable slope, step height, actor radius/height and surface identity. Start with accessible walkable paths and low obstacles; climbing, swimming and free jumping are not implicit requirements. If added later, each requires authored routes, animation and collision acceptance.

### Recovery

Each route/site has reviewed recovery anchors with valid clearance for supported rigs and actors. A recovery command is authorized in the session, checks a destination anchor and records any intended gameplay cost/time. Preserve damage, cargo and completed choices; recovery cannot be used to duplicate supplies. Offer an accessible “return to vehicle” action only through the same governed availability rules.

When stuck, show the cause and a clear recovery action. Restore to the last valid grounded/parked anchor after confirmed geometry failure. Do not silently teleport on every collision, and do not reset the whole site to free the player.

## Walking and interaction

- Use a normalized movement vector so diagonal speed equals straight speed. Support camera-relative movement in chase/third-person mode and an explicit consistent alternative if needed; do not invert controls during a camera transition.
- Turn the actor toward movement or the selected interactable using accepted facing state. Drive a small animation state set: idle, walk, turn, inspect/use and enter/exit vehicle. Animation availability is gated by the [art/animation plan](06-art-audio-and-rendering.md).
- Find nearby interactables through QualiaDB semantic queries. Resolve candidates using stable, deterministic priority and expose cycling when several overlap. Avoid a single hidden nearest point that is hard to locate.
- Interaction requires current scene identity, distance, reachability/visibility where relevant, activity prerequisites and correct phase. A rendered tooltip is not authority.
- Focus, controller confirm and E/Enter activate the **selected visible choice**. Delete the current shortcut that queries `button[data-correct="true"]`; keyboard/gamepad must not automatically choose the known correct answer.
- Inspection reveals useful evidence; it does not force a binary quiz. Contextual choice, placement or activity completion commits a typed action described in [gameplay](01-product-and-gameplay.md).
- Preserve non-motor alternatives: keyboard list selection for small objects, accessible parking/return assistance and readable route prompts. They use the same accepted outcomes.

## Wildlife, environment and condition

### Wildlife encounters

Use stable encounter IDs and a small authored state machine: `dormant → approaching → crossing/present → clearing → complete`. Transitions derive from simulation ticks and validated world facts. Save encounter state and replay seed so loading does not respawn a cleared encounter or erase a stop consequence.

An encounter occupies an authored anchor/path; do not recompute its location from the player's current road centre. Communicate presence using sight, optional sound and a non-colour-only warning. Decouple warning distance from actual collision/stop volume, and derive any stop assist from the selected assistance policy. Stopping should let the player watch the animal clear before continuing; immediately setting `eventCleared=true` on a contact event is insufficient.

The default experience encourages observation and calm travel. Avoid rewards for approaching or chasing wildlife. If a contact occurs, use a non-graphic recovery/feedback event consistent with product tone. A wildlife model/animation can remain a authored scenario approximation, but species/location/season claims need provenance from the content plan.

### Time, weather, surfaces and closures

Start with one authored daytime environment for the vertical slice, clearly labelled. Then add governed time/season/weather variations only with matching lighting, visibility, surface, audio, route access and replay state. Do not claim current park conditions or feed uncontrolled live weather into deterministic play.

Use an authored environment profile keyed to the scenario; the current Vibe `ambient_for_period` returns the same temperatures for all sites and is not sourced climate. Remove unearned realism claims. Distinguish purely visual rain/fog from semantic surface or visibility effects, and record the latter.

Closures/access checks run at planning and entry to the affected route segment. Provide a valid turnaround/wait/alternative decision when a simulated closure is encountered. A calendar label alone does not implement seasonal access. Changing a source's real-world advisory must create a new reviewed content version, not mutate an active saved route unexpectedly.

### Vehicle condition and other modes

Use existing workshop-owned roadworthiness/cooling facts after the shared session integration passes. Engine temperature, damage, fuel/energy if adopted, equipment and cargo carry between Talbingo and workshop/management views. The current `begin()` call to `sim.setup(0)` resets conditions and must be retired.

Expose a compact diagnostic hierarchy: normal, attention, action required, immobilized. A gauge warning should name an understandable in-game action and can be supplemented by the workshop inspection loop. Numerical simulation constants are game tuning, not repair advice.

Add consequence systems sequentially. First preserve current thermal state correctly. Then demonstrate one meaningful cooling-related route/workshop loop. Additional fuel, tyre, electrical, cargo and wear models require product decisions, public capabilities, UI feedback and balancing; do not add invisible resource decay simply because fields are available.

## Implementation work packets

Estimates are game integration effort after upstream readiness, excluding asset production and upstream development. Each packet requires an owner and a failing fixture before implementation begins.

### SIM-01 — Unify scene units, route and navigation context (3–5 days)

Dependencies: ARC-01; content/provenance decisions for the first site.

1. Inventory coordinates, bounds, origins and rig asset dimensions in Talbingo scene, terrain, pose and Vibe files.
2. Agree the versioned scene frame and route/surface schema through QualiaDB; write one authored The Pines route with entry, parking, turnaround and recovery anchors.
3. Replace duplicate `road_z`/`road_center_z` authority with one compiled route definition consumed by rendering and policy. Retain adapter tests during migration.
4. Use arc-length progress and arrival volumes. Remove fabricated GPS interpolation and arbitrary km/h conversion.
5. Expand the manifest to each of six sites after the first passes, with distinct route/scene IDs and reviewed extents.

Done when rendered/queried route geometry agrees across segments and both directed legs, and only the correct low-speed arrival can complete a leg. Wrong-end reverse arrival must fail.

### SIM-02 — Prove shared terrain, contact and collision (5–8 days)

Dependencies: ARC-01/03, SIM-01, upstream spatial/collision proof.

1. Create a minimal slope, bridge-over-water, wall, narrow gap and uneven-ground fixture through QualiaDB asset tooling.
2. Generalize terrain/contact context in `drive_contacts.rs`; never pass Talbingo through the main terrain implicitly.
3. Bind required ground/sweep/overlap queries; register scene collision layers and stable semantic IDs.
4. Send accepted grounded pose through actor/rig presentation; verify bind origins to prevent double elevation.
5. Add authored recovery anchors and governed recovery commands; handle absent/corrupt collision data before control activation.

Done when walking and each enabled rig remain grounded, cannot tunnel through a barrier, can cross the bridge, and recover without condition/progress reset. A missing public collision capability blocks this task upstream.

### SIM-03 — Tune driving, analog controls, rigs and towing (5–8 days)

Dependencies: ARC-03, SIM-01/02; matching rig visuals from chapter 06.

1. Replace boolean-only gamepad mapping with normalized controls and define source priority/dead zones.
2. Implement/author explicit forward/brake/reverse behavior, speed-sensitive steering, assistance policy and coherent handbrake behavior through approved capabilities.
3. Put rig parameters in validated authored profiles linked to physical and visual geometry.
4. Integrate wheel contact/animation and trailer sweep/articulation. Add warning hysteresis and forward recovery from reverse limits.
5. Tune one rig on the recorded handling course, then each remaining profile independently. Expose only profiles with complete behavior/visual/save proof.

Done when each enabled rig completes both legs and parking/reversing tasks at stable behavior across render rates. Record objective trajectory checks plus human playtest notes; a formula-only unit test does not prove vehicle feel.

### SIM-04 — Deliver grounded exploration and contextual interaction (4–7 days)

Dependencies: ARC-03/04, SIM-02; activity and animation contracts.

1. Replace rectangular water tests with scene queries and normalize movement.
2. Add facing and animation state through supported actor APIs; retain accessible movement assistance.
3. Build interactable candidate selection, visible focus and next/previous cycling using semantic IDs.
4. Route inspect/use/choice through governed commands; remove automatic correct-choice keyboard/gamepad shortcuts.
5. Add parked vehicle entry/exit, valid spawn clearance and contextual return/recovery.

Done when every required landmark is reachable through valid ground, blocked shortcuts cannot trigger interactions, and keyboard/touch/gamepad can select either allowed response without knowing correctness metadata.

### SIM-05 — Author encounters and environment consequences (4–7 days)

Dependencies: ARC-03/05, SIM-01/03/04; content review and audiovisual cues.

1. Build one seeded, stateful wildlife encounter with stable anchors, clear/stop behavior and consistent cues.
2. Record accepted encounter phase/seed and assistance choice; test save during approach, wait and clearing.
3. Add per-site authored environment/surface profiles; keep factual and illustrative labels correct.
4. Add route closure/turnaround behavior from approved content; then add chosen time/weather variants.
5. Tune interruption frequency so optional observation enriches travel without repetitive forced stops.

Done when encounters behave identically on replay, clear once, cannot be farmed by resetting a leg, and time/closure changes produce understandable valid alternatives.

### SIM-06 — Connect condition and prove the full simulation (4–6 days)

Dependencies: ARC-03/05, SIM-03/05 and workshop integration proof.

1. Replace Talbingo's forced roadworthy/cooling values with accepted shared rig facts.
2. Carry the same rig condition through departure, arrival, camping, return and workshop entry.
3. Complete one diagnostic/repair/verification loop through existing governed workshop actions; verify a repair receipt, not an animation, changes condition.
4. Run the acceptance matrix below across all six sites and each supported rig/profile combination; prioritize pairwise device coverage but exercise every route/rig rule.
5. Record deterministic replay and long-session resource receipts alongside handling playtest findings. Close upstream dependencies only on the tested profile.

Done when the game supports a continuous, saved and replayable journey with meaningful condition and no resets/duplication across modes.

## Simulation acceptance matrix

| Test | Required result |
| --- | --- |
| Straight/diagonal walk for equal ticks | Same distance within declared solver precision; no diagonal speed advantage |
| Same input tape at 30, 60 and high display Hz | Same accepted checkpoints; only visual interpolation differs |
| Keyboard + gamepad + touch overlap, disconnect/blur | Defined priority, zero stuck controls, no accidental repeated confirm |
| Brake forward, stop, reverse, brake reverse | Smooth intended direction behavior; no sign error or instant gear jump |
| Max-speed barrier/shoulder contact | No tunnelling; correct visible obstruction and recoverable accepted pose |
| Bridge over water | Surface-layer query permits bridge crossing and prevents entering water alongside |
| Terrain slope/uneven surface | Body/wheel contact follows actual terrain; no main-world terrain sampling |
| Narrow gap with and without trailer | Vehicle/trailer clearance independently respected |
| Reverse articulation limit then pull forward | Bounded articulation, legible warning and successful recovery |
| Wrong endpoint or outside arrival volume | No camp/trip completion, even if a legacy route-end condition fires |
| Proper parking and exit | Valid low-speed arrival; unobstructed actor spawn; same parked rig |
| Near/far/occluded interactable; stale choice | Only currently valid action accepted; no DOM-only unlock |
| Wildlife warning → stop → clear; save mid-encounter | Stable location and single completion; restored event phase/seed |
| Closure discovered after departure | Valid turnaround/recovery choice; no impossible trapped route |
| Save/load rig on slope with trailer | Pose/contact/condition restored under the same policy; no impulse spike |
| Travel → camp → workshop → travel | Shared damage, parts and inventory; no free repair or duplicate reward |
| Recovery or repeated mode entry | One recorded effect; condition/progress invariant except declared cost/time |
| Complete each site outbound and return | Distinct route/scene contract, activity and debrief; no dependence on array order |

Use existing `scripts/check-talbingo.mjs` as a baseline to extend, not as evidence these cases already pass. Integrate deterministic fixtures through the supported WASM test harness and upstream unit/conformance tests as specified in [testing and release](08-testing-and-release.md). Physics ownership, source-data claims and acceptance failures must remain visible in the [delivery backlog](09-delivery-backlog.md).
