# 06 — Art direction, rendering, animation, and audio

Status: implementation plan, 17 October 2026. This document changes no game or engine code. It specifies an illustrated, believable outdoor world delivered through QualiaDB. Read [world/content](05-world-content-and-provenance.md), [traversal](04-driving-traversal-and-simulation.md), [performance](07-performance-reliability-and-security.md), and the binding [QualiaDB-only contract](../13-qualiadb-only-development-contract.md).

## Intended result

The scene should feel like a place worth exploring: readable ground, varied natural silhouettes, convincing shorelines, grounded people and vehicles, useful light, and quiet sound that responds to location and action. The visual direction is restrained illustrated realism, continuing the useful principles in [the Community Grounds art bible](../../art/community-grounds-art-bible.md). It does not require photorealism, a particular studio's style, or every expensive graphics effect.

Make composition, scale, contact, materials, and camera framing correct before increasing detail. The lowest supported quality tier must retain landmarks, traversable boundaries, clues, and readable actors. Quality changes may reduce detail and cosmetic motion; they cannot remove an obstacle or alter authoritative movement.

## Inspected baseline and gaps

Evidence here comes from source at game revision `b2bfe7cdc5e75e7423c855ded420fb0bb24a2d3e`, with historic repository screenshots reviewed for visual symptoms. Those screenshots are not a fresh run or proof of the current browser/backend combination.

| Area | Existing source or artifact | Improvement implication |
|---|---|---|
| Overall image | `docs/qa/talbingo-site-0.png` shows pale/low-contrast scene values, sparse repeated planting, a visible hard terrain edge, and a broad rectangular water patch. `talbingo-walk.png` frames mostly empty ground with important objects near/off the edge. | First establish repeatable cameras, palette, light, and exposure. Do not assume a particular shader bug explains the image without controlled comparisons. |
| Terrain/road | `talbingo_terrain.rs` samples a 48 × 36 authored field; `talbingo_scenes.rs` drapes a 65-point road ribbon over the formula. Common camp spurs remain boxes. | Preserve the supported QualiaDB compilation path; author continuous banks/verges and replace repeated rectilinear composition. Verify render/query agreement. |
| Water | Talbingo reservoir/creek/inlet assets use `push` → `asset` → `render_role: "opaque"`. `lib.rs` routes only `render_role=water` or two specific legacy IDs into the native water path. | The current Talbingo water asset names do not activate the existing native water pass. Correct content roles and prove the pass in this scene before promising reflection/foam quality. |
| Vegetation | Swept trunks, limb clusters, botanical hulls, and ellipsoid leaf clusters already use QualiaDB geometry. Seven main tree positions repeat across sites. `VegetationPatch` centres use `y=0.02`, unlike terrain-grounded trees. | Ground every plant through shared surface queries; build silhouette/density variation with explicit exclusions and budgets. More individual leaves are not the first priority. |
| Materials | `material_library.rs` creates 128 × 128 colour/normal/ORM families through Qualia surface authoring and ingestion. `material_family_for` infers many roles from asset IDs. | Preserve digest-bound shared maps; author critical roles explicitly and review colour space/roughness/metal response. Current Talbingo names can fall through to an unsuitable family. |
| Lighting | `web/talbingo.html` sets sky preset 1, lighting `(0.45, 0.85, 0.35, 1.25, 0.38)`, exposure `0.25`, shadows off, screen-space AO on. | These settings are present, not an approved lighting grade. Review neutral calibration and admitted shadows on each backend. |
| Walker | `author_walker` creates one joint, rigid unit weights, and a one-second clip with identical keys. `talbingo_walker_pose` translates that root. | It is a rigid moving figure, despite “articulated” comments. A walk cycle, turning, feet contact, and interaction poses still need deliberate integration. |
| Vehicle/fauna | Shared vehicle/fauna recipes and animation modules exist; Talbingo uses a root pose for movement. | Existing modules are starting points. Validate actual wheel, suspension, skeleton, and clip behavior in Talbingo rather than claiming them from module names. |
| Audio | `web/talbingo.html::HolosonicEngine` builds oscillators/filtered noise directly with `AudioContext`, uses random noise, auto-initializes on interaction, and catches errors silently. Its mute path omits `floraGain`. No panner/HRTF graph appears in that class despite spatial wavefield copy. | Move synthesis/mixing/playback ownership to QualiaDB, prove browser output, fix complete mute and lifecycle behavior, and use honest feature names. |

Existing general graphics plans contain useful tests and also older capability claims. Use [the capability matrix](../../art/graphics-capability-matrix.md) and living ledger as starting points; do not copy their status into Talbingo without a pinned browser fixture.

## Ownership and capability gates

The game owns art parameters, original recipes, semantic asset references, camera composition, accepted-state-to-presentation mapping, and creative review. QualiaDB owns geometry processing, material/texture compilation, rendering, rig evaluation, clip blending, spatial queries, audio DSP/mixing/output, package formats, and resource management.

| Needed path | Existing evidence to inspect | Status for this plan | Required proof before use |
|---|---|---|---|
| Parametric geometry → `.10d` | Game `asset_catalog.rs`, `asset_compile.rs`; Qualia public authoring functions | Used by current source | One build and browser receipt for the actual proposed recipe, with bounds and digest |
| Materials/textures | `material_library.rs`; `.10d` material records and texture ingestion | Used by current source | Matched neutral test scene on admitted backends; correct role/colour/UV/alpha |
| Water | Game `lib.rs` load split, `water_presentation.rs`, Qualia renderer water path | Available wrapper; Talbingo role integration missing | Talbingo creek/reservoir rendered through `water` role; clipping, depth and low tier verified |
| Rig/clip playback | `resident_animation.rs`, `fauna_animation.rs`, `vehicle_animation.rs`, Qualia skeletal sections | Ingredients exist; desired Talbingo behavior unverified | Serialized multi-joint rig, clip transition, attachment, contact, and save/load presentation fixture |
| Shadows/AO/wind/quality | Existing `GamePortal` wrappers; Qualia renderer capability admission | Wrapped; visual/backend result unreviewed | Toggle comparisons, truthful capability receipt, fallback with same gameplay readability |
| Audio | Qualia `crates/qualia-core-db/src/audio/`, `render/portal/mod.rs` sonic-token methods, `crates/qualia-audio/`, `crates/webizen-render/src/audio_contract.rs` | Upstream ingredients; Talbingo browser event-to-output path unverified | User-gesture unlock, finite bounded audio events, spatial/stereo fallback, full mute, teardown and captions |
| Offline art package | Game HMC export/load/validation paths | Existing general path; Talbingo packaging work required | No runtime re-authoring in release; atomic load/unload; corrupted dependency rejected |

All new interface names in the following tasks are proposals. When a capability is missing, create the upstream work order, prove it in QualiaDB, pin its revision, and only then add a thin game bridge. Do not write a renderer, rig solver, shader stack, audio engine, or geometry tool in this repository.

## Art direction specification

### Scale and composition

Verify the scene frame against chapter 04 before resizing assets. Current code normalizes the scene for rendering; a number that looks like metres in a recipe is not automatically the player's accepted physical unit. Adopt source-world metres only after route, terrain, actor, camera, audio, and collision transforms agree.

Calibrate a standing adult around 1.7 m, a familiar table, and the actual vehicle dimensions as reference objects; final values are art/game design targets, not measurements of real campground objects. Keep walkable headroom, door/step scale, vehicle clearances, and tent footprints consistent.

Use three readable depth layers: a clear foreground approach, a midground activity/landmark, and a quiet distant landscape. Give each site one dominant silhouette and one secondary accent. Keep required clues in visible, reachable space. Distant ridges should hide the play area's end through composition and supported landscape geometry, not hide a hole with opaque fog.

Define five repeatable camera frames per representative site: arrival, driving bend, parked vehicle, landmark approach, and camp outcome. Frame the subject in the central portion of the available viewport after HUD/safe-area offsets. Camera collision and ground occlusion are upstream spatial/presentation responsibilities, coordinated with chapter 04.

### Palette, value, and surfaces

Starting palette: muted grey-green foliage, warmer dry grass, differentiated brown/grey ground, cool water, neutral bark, and restrained orange/ochre camp accents. Treat these as an authored colour script, not ecology evidence. The UI chapter owns the interface palette.

Use broad value separation first. In a grayscale capture, the vehicle, main path, interaction landmark, and water boundary should remain identifiable. Avoid a uniform green or cyan tint across soil, skin, clothing, wood, metal, and foliage. Review sunlit, shaded, and overcast-looking authored states without changing semantic surface identity.

Explicit material specification per asset family:

| Family | Authoring requirement | Review failure to catch |
|---|---|---|
| Ground/road/gravel | Separate broad colour variation from fine normal texture; consistent world-scale UVs; supported roughness. | Repeating noisy tiles, shiny dirt, z-fighting verge/road |
| Bark/timber | Directional grain follows trunk/board; varied large-scale bark forms; restrained relief. | Uniform stripes, metal highlights, discontinuity at branches |
| Foliage/grass | Engine-supported opacity policy, normals, wind weights and shadow behavior; preserve silhouette. | Shimmer, opaque balloon crown, synchronized movement, excessive overdraw |
| Cloth | Folds in silhouette where visible; matte roughness and limited seam detail. | Tent shaped only as a sharp roof prism, plasticky highlight |
| Skin/hair | Separate from cloth where supported; readable head/hands without tiny geometry. | Skin inherits canvas texture or foliage colour grading |
| Painted metal/rubber/glass | Explicit component roles; suitable alpha/depth ordering; wheels and glazing readable. | All components classified by ambiguous ID, opaque “glass” claimed as transmission |
| Water | Explicit native water render role, stable shoreline geometry and supported per-tier quality. | Solid coloured block, hovering bank, hard rectangular edge |

New surface families and effects must fit published QualiaDB material records. Do not create a private material format or infer taxonomic/geological facts from a texture name.

## Provisional production budgets

These are conservative **initial stop-and-profile thresholds**, not measurements, guarantees, or final release limits. Chapter 07 owns the device matrix and final budgets. Count GPU-visible triangles/material batches and decoded resources, not only source file sizes; shadows and transparency can multiply cost.

| Allocation per active local scene | Low tier starting target | Standard tier starting target |
|---|---:|---:|
| Visible triangles, all scene assets combined | 150,000 | 350,000 |
| Visible material batches/draw groups | 150 | 250 |
| Estimated resident texture GPU bytes | 32 MiB | 64 MiB |
| Initial compressed site-specific art payload, excluding shared engine/library | 12 MiB | 24 MiB |
| Active audio voices including UI/ambience | 8 | 16 |
| Particle count | 0–64 | 0–256 |
| Shared texture base size | 128–256 px; exceptional hero up to 512 px | 256–512 px; exceptional hero up to 1,024 px |

Do not treat all maxima as simultaneously affordable. Start well below them, measure the representative worst view, and allocate the remaining budget where it improves readability. Prefer three useful plant silhouettes over more leaf tessellation. Static landmarks and nearby actor silhouettes take priority over small ground detail.

Use engine-owned LOD/instancing when its browser path is proved. Otherwise author a deliberately bounded scene using current supported APIs and record the missing capability; do not invent a game-side LOD engine. An optional high-quality effect must fail to an explicitly designed lower presentation, with the same interaction and collision state.

## Implementation work packages

### ART-01 — Establish a visual benchmark and camera composition

**Priority:** P0/P1. **Depends on:** DATA-01; shared scale/camera contract. **Files:** existing `web/talbingo.html`, `camera_presentation.rs`, `talbingo_scenes.rs`; proposed `docs/art/talbingo-art-bible.md`, `web/fixtures/talbingo-review-cameras.v1.json`, `docs/qa/talbingo-renewal/`.

1. Capture the existing build with exact engine/game revision, browser, backend, viewport, device scale, and quality settings. Mark old screenshots historical.
2. Create the five camera presets above for The Pines and a representative creek scene. Use supported camera controls and fix viewport-relative framing.
3. Create a neutral calibration scene containing ground, grey/white references, vehicle, tree, person, and water. Keep exposure fixed while comparing material and light changes.
4. Record the intended palette/value hierarchy and a small original silhouette reference sheet. Use existing original art and allowed reference material; no external DCC or substitute geometry pipeline.
5. Approve one full arrival-to-camp sequence before making all six scenes denser.

**Acceptance:** screenshots can be reproduced from fixture values; walk camera keeps actor/landmark in frame; no mandatory object is hidden by HUD; grayscale and low-tier views retain the path and objective. Record what passed and what still needs review instead of labelling all art “AAA.”

### ART-02 — Rebuild terrain, road edges, and believable contact

**Priority:** P1. **Depends on:** DATA-02/03; chapter 04 shared surface/contact gate. **Files:** `talbingo_terrain.rs`, `talbingo_scenes.rs`, `asset_catalog.rs`; proposed per-site terrain recipes. Potential upstream owners: QualiaDB geospatial terrain and computational geometry modules.

1. Correct unsupported survey/geology comments. Store authored height/landform parameters in the scene content layer.
2. Make the rendered ground and accepted contact queries consume the same versioned height/surface data. Never maintain a separate approximate JavaScript height function.
3. Author distinct major forms per site: shore terrace, inlet swale, creek banks, river flat, steep backdrop/terrace, and open dam margin. Preserve reachable gameplay space.
4. Replace rectangular camp-spur blocks with supported continuous ribbons/surfaces connected to the main route. Give roads shoulders, graded verges, and logical ends.
5. Shape the horizon/apron and camera limits together. Test every playable camera direction for visible cut edges, gaps, cliffs created by skirts, and inverted normals.
6. Ground rocks, table legs, gates, tents, plants, vehicles, and actors using surface contact. For uneven surfaces, use supported multi-point grounding/orientation or author suitable level pads. Do not simply sink all props to hide intersection.

**Acceptance:** no visible floating/sunken required prop in review frames; route and terrain remain continuous at joins; every required actor contact agrees with authoritative surfaces within the tolerance defined in chapter 04. Raising one terrain sample invalidates/rebuilds dependent derived geometry deterministically.

### ART-03 — Build varied, grounded vegetation with bounded cost

**Priority:** P1. **Depends on:** ART-01/02; DATA-05 for species labels. **Files:** `talbingo_vegetation.rs`, `mature_gums.rs`, `flora.rs`, `asset_catalog.rs`, scene/ecology profiles.

1. Make three canopy silhouettes, three trunk/branch variants, two shrub groups, and two grass/tussock groups as an initial generic illustrative kit. Reuse existing public sweep/limb/botanical APIs.
2. Replace the seven-position repeated layout with authored clusters and negative space per scene. Seed only secondary variation; landmarks and route clearance are deliberate.
3. Define exclusion areas for roads, paths, pitch footprints, water, interactions, vehicle turning space, and camera sightlines. Placement queries come from QualiaDB.
4. Ground `VegetationPatch` centres and blade bases through the same terrain query; remove the fixed `y=0.02` assumption. Test sloped ground explicitly.
5. Separate near silhouette detail from distant mass. Avoid combining opaque canopy hulls with so much leaf detail that both costs remain at every distance.
6. Verify supported wind weights keep roots/trunks grounded, vary phase by stable instance ID, and produce matching shadow behavior. Reduced motion limits cosmetic sway without changing collision.

**Acceptance:** trees and shrubs differ visibly without random clutter; no plant blocks required paths or hides a clue; slope-grounding fixture passes; low-tier foliage remains legible and stable while moving the camera; no unreviewed species assertion is introduced by the art names.

### ART-04 — Integrate actual water surfaces and shorelines

**Priority:** P1. **Depends on:** ART-02; DATA-03; native water capability proof. **Files:** `talbingo_scenes.rs`, `asset_catalog.rs`, `lib.rs` water loading, `water_presentation.rs`; engine water owner if missing behavior is discovered.

1. Author an explicit water recipe/helper using the existing supported `render_role: "water"`; assign it deliberately instead of relying on names. Verify load receipts count Talbingo water surfaces.
2. Replace water boxes with curved/ring/polygon surfaces supported by QualiaDB and banks that meet terrain. Existing `Waterway`/`WaterwayBanks` are candidate recipe building blocks, not a guarantee for every basin shape.
3. Calibrate the shared world-to-render conversion. `set_water_surface_metres` is an existing helper to investigate for scale-aware appearance.
4. Tune colour, roughness/response, depth edge, waves, and any supported foam under fixed lighting. Do not promise refraction, reflection, or shoreline foam on a backend until observed there.
5. Keep collision/wading/placement exclusion tied to semantic water boundaries from the world contract. Visible waves are cosmetic and cannot change camp eligibility frame by frame.
6. Provide a stable low-tier opaque or simplified **engine-supported** water presentation with the same clear boundary if the native higher-tier effect is unavailable.

**Acceptance:** six scenes load explicit water roles where intended; shorelines follow authored geometry with no rectangular slab sides; objects intersecting the shoreline expose no depth-sorting artifacts in review views; reduced motion and low quality retain a readable water edge. Tests inspect receipts plus visual captures; a role string alone is insufficient.

### ART-05 — Calibrate materials, lighting, atmosphere, and quality

**Priority:** P1. **Depends on:** ART-01/02/04; chapter 07 quality admission. **Files:** `material_library.rs`, `asset_catalog.rs`, `lib.rs` presentation wrappers, `web/talbingo.html` initialization; proposed art/lighting profiles.

1. Give critical Talbingo assets explicit material families and roles; replace unsuitable ID-based fallthrough for water, stone, soil, clothing, timber, and poles. Preserve the stable asset ID.
2. Verify base colour uses the supported sRGB path while normals/ORM use linear interpretation. Check whether generated ORM multipliers and material scalar values interact as intended before changing them.
3. Render the neutral calibration scene with AO off/on, shadows off/on when admitted, exposure fixed, and each intended sky/light preset. Find the cause of pale output through isolated comparisons, not arbitrary saturation increases.
4. Establish neutral readable midday first, then a warm authored late-day variant. Separate lighting presets from actual time/season/weather facts.
5. Keep distance atmosphere subtle enough that foreground actors and cliffs remain distinct. If fog is unsupported or broken, fix/expose it upstream; do not overlay a CSS colour wash on the canvas.
6. Verify material alpha/cutouts and wind in shadow/depth passes. Add higher-cost effects only after low-tier quality and performance targets pass.

**Acceptance:** cloth/skin/wood/soil/glass/metal remain distinguishable in lit and shaded views; no distracting shimmer or clipping; no unsupported quality badge; screenshots show a readable improvement under identical camera settings. Document the actual backend capabilities and any remaining effects gap.

### ART-06 — Replace placeholder camp props and finish the vehicle

**Priority:** P1. **Depends on:** ART-01/02/05; stable interaction identities from DATA-02. **Files:** `talbingo_scenes.rs`, `camp_home.rs`, `vehicles.rs`, `vehicle_exterior.rs`, `vehicle_cabins.rs`, shared prop recipes.

1. Create a compact reusable camp kit: tent with readable entrance/flysheet, table/bench with supports, packed gear, sign/wayfinding post, ground marker, and approved site-specific landmarks.
2. Give objects plausible construction, thickness, contact points, and silhouette in walk and overview frames. Fine bolts, labels, and seams are lower priority than shape and orientation.
3. Vary arrangement and condition within the illustrative briefs. Avoid identical tents arranged in the same three positions across all scenes.
4. Improve vehicle wheel/arch contact, glazing, body proportions, lights, and cargo silhouette using the existing QualiaDB recipe path. Coordinate wheelbase and collision dimensions with chapter 04.
5. Bind inspect regions and semantic picks to stable entity IDs, not mesh part names or screen coordinates. Cosmetic mesh replacement must preserve interaction state.
6. Review fire/cooking imagery against the declared fictional scenario state. Do not show perpetually burning embers as an implied real-world permission or facility guarantee; use unlit equipment unless the scenario explicitly enables it.

**Acceptance:** required props read without text labels at normal play distance; interactions remain reachable after the mesh upgrade; vehicle body/wheels match accepted contact and steering; all six layouts use the kit differently. Original artwork rights are recorded in DATA-06.

### ART-07 — Deliver a grounded player rig and readable animation

**Priority:** P1. **Depends on:** chapter 04 movement/contact contract; QualiaDB clip/rig capability proof; ART-01/02/05. **Files:** `talbingo_scenes.rs::author_walker`, `resident_animation.rs`, `animation_bridge.rs`, `lib.rs::talbingo_walker_pose`; proposed original character recipes/clips.

1. Replace the one-root rigid figure with a bounded multi-joint character authored through supported skeletal APIs. Start with pelvis/spine/head and paired upper/lower limbs; exact rig count is an authoring choice after engine limits are known.
2. Author idle, walk, turn, short inspect, reach/place, and seated/rest clips only where the gameplay requires them. Begin with one original adult character and reusable clothing variants.
3. Use in-place locomotion driven by accepted simulation velocity. The authoritative position comes from chapter 04; a clip must not move the player independently. Request a public engine blend/transition API if absent.
4. Match walk cadence to accepted speed, turn toward intended facing, and preserve feet contact. Foot placement/IK, if required, belongs in QualiaDB; a supported authored pose/transition is an interim milestone, not completion of the full contact requirement.
5. Attach gear to supported joints without deforming rigid props. Keep all skin weights finite, normalized, and within the rig joint bounds.
6. Derive animation state from accepted session state on reload. Pausing, tab hiding, and reduced-motion settings must not advance simulation or accumulate a large animation jump.

**Acceptance:** no sliding at ordinary walking speed, detached accessories, inverted limbs, or persistent feet penetration in the review course; inspect/reach begins and ends in a valid state; reduced motion retains legible interaction feedback. Pack round-trip preserves skeleton, clip identity, and stable actor/entity mapping.

### ART-08 — Animate fauna and add restrained environmental effects

**Priority:** P1 after the player; optional extra effects P2. **Depends on:** DATA-05, ART-03/07, scenario encounter contract. **Files:** `fauna.rs`, `fauna_animation.rs`, `quadruped_animation.rs`, `macropod_models.rs`, `quadruped_models.rs`, proposed encounter/art profiles; QualiaDB effect owner for unsupported effects.

1. Validate existing animal recipes and skeletons against a small catalog of required illustrative encounter silhouettes. Do not build an unbounded wildlife roster.
2. Provide idle/observe/move/leave states using supported clips and authoritative encounter events. Align movement, heading, ground contact, and visible warning state.
3. Add a modest non-synchronized ambient layer: grass/tree sway, tiny permitted dust on the accepted loose surface, and subtle water response. Use fixed pools and engine-owned effects; no per-frame object creation or game-side particle system.
4. Tie dust/footstep/wheel visuals to accepted contact/surface events, not a second proximity calculation. Weather-looking effects remain scenario presentation unless a supported simulation owns them.
5. Respect reduced motion, photosensitivity settings, and visibility. No flash or particle cloud may obscure a required obstacle or carry the only hazard cue.

**Acceptance:** an encounter can be completed with secondary effects disabled and audio muted; animal pose and collision do not disagree; effects remain bounded after ten site switches; published labels accurately reflect illustrative versus reviewed ecology.

### ART-09 — Replace the prototype audio graph with a supported sound system

**Priority:** P0 for mute/honesty defects; P1 for complete replacement. **Depends on:** QualiaDB audio browser gate, architecture event/session contract, DATA-06 rights/credits, chapter 02 accessibility. **Files:** existing `web/talbingo.html::HolosonicEngine`; proposed thin `talbingo_audio.rs` bridge and declarative audio profile; upstream candidates under QualiaDB `src/audio/`, `render/portal/`, `qualia-audio`, and `webizen-render`.

1. Record the current behavior and deficits: direct game-owned DSP graph, unspecific random noise, no spatial panner in the class, incomplete mute, swallowed errors, and no explicit disposal path. Correct the user-facing label to “Sound”/“Ambient audio” unless a specific spatial capability is proved.
2. Create an upstream proof using one supported event, one loop, a listener transform, and a stop/dispose operation. Verify the chosen browser/WASM profile and isolated/non-isolated deployment requirements rather than assuming a sonic token produces audible output.
3. Define a **proposed** bounded event projection: `eventId`, simulation tick, semantic emitter ID, sound key, accepted surface/velocity parameters, optional local position, gain category, and caption key. It carries presentation data only and cannot mutate gameplay.
4. Required mix groups: master, ambience, vehicle, actions, and UI. Provide complete mute, persisted user preferences through the approved settings path, and separate action/ambience controls. A muted master must silence every source including flora noise and chimes.
5. Required sound set: quiet wind/vegetation bed; local water bed; engine/load loop; accepted surface tyre/footstep variations; inspect/place/confirm cues. Optional music remains subtle and skippable. Exact wildlife calls require appropriate source/species/rights review; generic ambience must not claim field-recording accuracy.
6. Spatial behavior: a verified engine listener follows the selected camera/player convention; local emitters attenuate smoothly; moving away from water reduces it. A tested stereo/mono fallback is acceptable when the engine reports no HRTF. Do not describe it as wavefield synthesis without proof.
7. Audio starts only after an explicit enabled user gesture and visible setting. Handle browser suspension, denied/unsupported output, tab hiding, route transition, reset, and disposal. Failed output produces a readable settings state while gameplay and captions continue.
8. Build original procedural sound through QualiaDB DSP or ingest licensed recordings through its supported pipeline. Store source/seed/generation parameters and credits. Do not add a game-owned Web Audio graph, audio codec, third-party generator, or external sound library pipeline as a workaround.
9. Replay accepted cue events deterministically and deduplicate by event ID. Cosmetic mixer timing may differ by device; do not require bit-identical PCM or feed audio callback timing back into game state. On resume/load, start current loops and avoid replaying every historic notification.
10. Set voice priorities so critical action/hazard feedback survives ambient culling. Verify all warnings also have visual/text equivalents. Use a limiter and avoid clicks/clipping through the supported engine mix path.

**Acceptance:** master mute produces silence for loops and one-shots; no duplicate loop after ten switches/resets; lifecycle teardown releases sources; spatial direction/distance is audible where supported and honestly labelled otherwise; caps are enforced; captions and game completion work without audio. Browser failure is visible, not swallowed. An upstream missing audio path blocks completion of audio replacement rather than authorizing a second engine.

### ART-10 — Finish packaging, visual/audio QA, and regression review

**Priority:** P1. **Depends on:** ART-01 through ART-09, DATA-07, chapter 07 budgets, chapter 08 release gates. **Files:** existing `scripts/check-talbingo.mjs`, asset export/review scripts; proposed review-camera fixture and `docs/qa/talbingo-renewal/art-review.md`.

1. Export assets through the supported offline package path and verify semantic identity/provenance/rig/material roles survive. Keep original source recipes separate from generated output.
2. Capture six overviews and six landmark views at low and standard quality, plus driving/character/water/material close-ups from fixed fixtures. Include wide desktop, small laptop, portrait touch, and landscape touch viewports.
3. Exercise the worst visible foliage/water/actor view, full driving course, ten scene changes, reset, pause/resume, device loss/recovery where supported, and muted/suspended audio.
4. Use image comparison only for stable coarse regions and missing-asset/large-layout regressions. Renderer noise, backend differences, and animation phase require tolerances and human review; do not enforce exact pixels across unlike GPUs.
5. Listen with speakers, headphones, mono, reduced volume, and master mute. Review harshness, loops, boundary transitions, warning clarity, and subtitle timing. No audio-only completion condition.
6. Record the art acceptance rubric and measured resource costs next to every capture. If cost exceeds chapter 07 targets, first remove unnecessary detail/overdraw or fix an engine issue; do not lower visibility of required gameplay cues.

**Acceptance:** no unexplained visual regressions; all six identities remain clear; no visible terrain cuts in allowed cameras; ground contact, material role, water, and animation review pass; sound lifecycle/mute/caption tests pass; measured budgets and browser/backend receipts are attached to the release checklist. An exported pack alone is not visual or audible acceptance.

## Acceptance rubric and implementation order

Score each criterion `0 = broken`, `1 = usable with issues`, or `2 = accepted` for each scene and quality tier: composition, scale/contact, terrain boundaries, material separation, water/shoreline, vegetation, actor/animation, interaction readability, audio behavior, and accessibility. A required criterion at 0 blocks release. A 1 needs an explicit tracked defect and release decision; averages must not hide a broken scene.

Implement in this order: baseline/cameras → shared frame/ground contact → one terrain/shoreline/vegetation slice → neutral materials/lighting → props/vehicle/player → three differentiated sites → six-site completion → audio and encounter polish → measured packaging/review. Audio's upstream proof can run in parallel early. Avoid polishing ten animal species while basic movement framing or water-role binding is still wrong.

The result is accepted when the player can read, navigate, and enjoy the six places consistently across supported devices, and the evidence demonstrates that the actual Talbingo path uses the capabilities the UI claims.
