# Community Grounds graphics overhaul: implementation tracker

Source of truth: [35 — implementation handoff](35-community-grounds-graphics-overhaul.md). User accepted the Community Grounds illustration at docs/reference/communitygrounds.jpg as the primary appearance target. The target is an extensible simulation grounded in suitable civics.au / dev.civics.au project data: Australia atlas and trip planning → town/area → linked distinct Community Grounds sites → human-scale scene, with personal/interpersonal stories and place-based work/activity modes over shared state.

## Baseline

- Game checkout at start: 9c52d96db08673d7f579f14f83ac18ba7b4bdc57 (main).
- QualiaDB checkout at start: 9463dc6231a0ce10e04ea2837c039e9ec3018fb0.
- Updated qualia.ref from abbreviated 9463dc62 to the full SHA required by both build scripts.
- Baseline manifests had 340 assets across 10 scene states. Storybook, Earthlight and Community Grounds now each list 585 assets across 40 scene states and declare WASM SHA-256 `3aa7252c64dd309ce2e9de3fb89f8ecc49bb8dd99f965e203afccefbfaa84027`.
- The ten-camera read-only showcase route and generic Qualia OSM feature-to-mesh API are implemented; the game WASM bridge emits a provenance-bearing `.10d` from supplied Overpass JSON and supports optional tile clipping. One offline release Cargo build, WASM-bindgen generation, three-profile asset export and Qualia HMC structural validation completed. The WASM browser bridge normalizes the scene into a shared orbit frame and applies the same transform to separately uploaded water geometry. The illustrated Australian loader covers initial entry and style-pack switches, with staged progress and reduced-motion support. The source page now restores responsive campaign panels and provides Community, Trip Map, Drive, Explore and Activities navigation; Trip Map shows the Noosa–Melbourne corridor and inland South Australian border leg as an explicit schematic pending licensed route geometry. Direct Drive adds a DOM rig selector, oil/coolant gauges, ambient/grade/surface/load readings and the simulated rear-mirror queue state. Economy, Balanced and High detail controls adjust existing renderer features. The local browser preview is open at `http://127.0.0.1:8766/game.html`; rebuilt-page visual, gameplay and performance acceptance remains pending.
- No Actions result or rendered capture has been claimed in this tracker.
- QualiaDB was clean at the observed commit; its current engine pin is available locally.

## Work packages

| ID | Depends on | Status | Evidence / next exact action |
|---|---|---|---|
| G00 | — | in-progress | Full Qualia pin, reference copy, art bible, capability matrix, evidence index and a new civics project data-bridge inventory exist. The generated asset register covers 542 stable IDs. A provisional v1 Observatory package envelope and integrity-validator scaffold now await alignment with the producer export; no artifact-level project-data export or licence agreement is yet integrated, and baseline captures remain open. |
| G01 | G00 | in-progress | Ten named cameras and a validated, read-only 30-second showcase route load from versioned `web/fixtures/graphics/review-cameras.json`; `?graphics-view=` forces the seed roster, Community Grounds style, rig, light and ecology profile in memory and disables save/load. Direct Drive has first-person and local aerial route-follow cameras. A responsive Community/Trip/Drive/Explore/Activities shell now frames the Qualia scene; Trip Map presents the Australian corridor as a clearly labelled schematic. Real ABS/road pack loading and rendered run receipts remain open. |
| G02 | G00–G01 | in-progress | Game HMC loads use a shared normalized orbit frame, per-asset material sections and explicit water geometry; AO/shadow/probe/water/temporal bindings reach Qualia. Offline release WASM and three web bundles are regenerated; browser renderer receipts remain open. |
| G03 | G02 | in-progress | Game recipes emit stable PBR material families, roles and authored bounds; vehicle, fauna and profile-selected flora source modules are separate. Three regenerated 585-asset/40-state HMC packs passed Qualia validation of bundle digests, `.10d` CRC/sections, finite mesh/index data and material-to-submesh ranges. Texture file dependencies and full texture HMC round-trip remain open. |
| G04 | G01–G03 | in-progress | Four touring rigs, eleven fauna/roadside encounter groups and seven Australian flora model groups, including full-height genus-level eucalypt and Acacia silhouettes are present in generated scenes. 4WD, sedan, campervan and caravan shells use rounded Qualia parametric bodies; the roof-top tent has a gabled profile. Residents have tapered Qualia-authored limbs, softened faces, varied palettes and six authored task gestures. Kestrel Flats includes a timber-framed three-channel hydroponic display and open workshop interior. Curved Qualia spline paths, waterways and sculpted banks replace several square edges. Semantic picks now connect the workshop repair bench, hydroponics, touring camp and orchard props to their site/activity panels. Visual acceptance remains open pending package rebuild and rendered review. |
| G05 | G04 | in-progress | Added profile-selectable bottlebrush, banksia, grass-tree, saltbush and kangaroo-paw models through the Qualia mesh/10D path. Qualia compiles parsed OSM features to local ENU meshes with optional DEM draping and now clips point/line/simple-area geometry to optional rectangular render-tile bounds; the game exposes whole-feature and tile-bounded provenance-bearing `.10d` compilers for supplied Overpass snapshots. A versioned normalized place/route schema, synthetic graph fixture, structural validator and Places HUD navigation now cover the existing fictional territories and distinct site targets. Source-derived Australia→town→separate-sites data, route/terrain tile loading, public data adapters, connected playable geography, landmarks and place-specific flora/fauna remain open. |
| G06 | G03–G04 | in-progress | Curved water geometry/banks use Qualia spline recipes, and campaign state/quality now reaches Qualia's animated water controls. Foliage materials carry a MAT4 wind flag consumed by the engine mesh shader's frame-time deformation. Character clips, moving vehicle geometry, pump animation, shadow agreement and rendered/browser acceptance remain open. |
| G07 | G04–G06 | in-progress | Walk and Direct Drive are playable controls; the drive fixture has rig-dependent handling, profile-selected encounters, dawn/day/dusk/night timing, separate road and engine alerts, simulated oil/coolant meters, load readings, and a following-traffic mirror with bend reveal. A checked VibeScript module now owns movement, hazard distance/stops, thermal load/damage and mirror state; the WASM compatibility action adapter evaluates a VibeScript cell before the QualiaDB SHACL gate. Focused WASM checks for dusk stops, mirror occlusion and hot-climb damage are authored but not yet run. The HUD and V key toggle a local aerial route-follow view with first person; the current route is fictional. The versioned review route now includes a local aerial-drive pose; a read-only showcase tours ten views. An illustrated Australian loader now appears on initial online entry and style-pack switches; it announces stages, streams real pack-byte progress where available, handles fatal startup errors with retry, and honors reduced motion. A live Activities panel now hosts versioned interpersonal, harvest, workshop repair, shared-meal and housekeeping practice modules over the shared GameSession action/event tape. Dedicated task animations, broader hospitality, source-linked localities, Australian route navigation, rendered acceptance and geographic transition loading remain open.
| G08 | G05–G07 | in-progress | Economy, Balanced and High detail settings now cap the ambient, shadow, AO and temporal feature paths and persist locally. LOD/instance wiring, measured profiles, geographic streaming performance, memory recovery and quality receipts remain open. |
| G09 | G00–G08 | in-progress | The release-verification script passes for the compiled WASM digest, all three 585-asset/40-state packs and expected map/drive/quality controls. Node WASM gameplay checks pass for gated actions, deterministic replay, 469 compiled scene assets, the communications landmark and high-tier capability. Full browser render/screenshot review, end-to-end save/replay and measured performance receipts remain open; the local headless Edge runner exited before exposing its debug page in this workspace. No deployment or Actions result is claimed. |

## Files created

### 12 October active implementation continuation

Starting inspected game SHA: `b939cbac09d0d370f69fd22d8a1d314103986556`.
Qualia SHA: `8b90829327825f524cd2f941c4d016fa24668448`; new engine source remains
uncommitted and has not been pinned as a new revision.

Implemented in source, awaiting build and validation:

- Mature gum family replaces every old solid oval crown/spray assembly while
  preserving its 36 stable IDs. Authored tapered limbs and pendulous leaves
  compile through Qualia. Corrected canopy/fork material classification.
- Convex coachwork loft is a new focused Qualia authoring module with bounded
  inputs and caller-buffered output. SUV, sedan, caravan and campervan source
  consumes it; window frames, wheel rims and glazing placement are refined.
- Adult resident proportions use Qualia transforms, and four terrain bases use
  graded DEM aprons. Static work poses still need tools/hand-contact refinement
  and the required native animation pipeline.
- The page scene now fills the viewport. Campaign docks collapse; normal play
  uses a compact native HUD. Mobile has a bottom sheet, safe-area navigation,
  touch movement and actual viewport canvas dimensions. Self-test defaults to
  Community Grounds, with explicit `style=storybook|earthlight|community` options.
- Plan 38 adds the user's distinct driving, mechanical puzzle, community/adventure
  and production-management systems and their shared-state acceptance contract.

No Cargo build, WASM export, browser self-test or test suite has been run for these
edits. The user explicitly requires completion of implementation before testing.
Current browser assets therefore do not establish these source changes. Remaining
G00–G09 gates and plan 38 gameplay requirements remain open; nothing is accepted.

Formatting attempt: `rustfmt --edition 2021` against the two new focused source
modules exited 1 because Windows path canonicalization returned access denied
for the workspace ancestors. This is not a compilation or test result. No green
formatting, build, test or rendered acceptance is claimed for this revision.

Next work: finish hero workshop/vehicle interiors and contact poses, connect native
animation and contextual mode panels, implement workshop assemblies/puzzle rules
in VibeScript over shared Qualia state, then continue connected geography and
production systems. Complete implementation before the requested build/test pass.

- docs/planning/35-community-grounds-graphics-overhaul.md
- docs/planning/37-civics-project-data-bridge.md
- docs/schemas/civics-observatory-game-pack.v1.schema.json
- docs/schemas/community-scenario-module.v1.schema.json
- docs/schemas/community-place-pack.v1.schema.json
- scripts/validate-civics-observatory-pack.mjs
- scripts/validate-community-place-pack.mjs
- docs/reference/communitygrounds.jpg
- docs/reference/README.md
- docs/art/asset-replacement-register.csv
- docs/art/community-grounds-art-bible.md
- docs/art/graphics-capability-matrix.md
- docs/qa/graphics-overhaul/README.md
- web/fixtures/graphics/review-cameras.json
- web/fixtures/scenarios/community-grounds-v1.json
- web/fixtures/places/fictional-community-v1.json
- crates/rolling-commons-shell/src/flora.rs
- docs/art/australian-ecology-data-contract.md
- assets/ecology/profiles/se-corridor-reference.json
- crates/rolling-commons-shell/src/pack_validation.rs
- crates/rolling-commons-shell/src/vehicles.rs
- crates/rolling-commons-shell/src/fauna.rs
- scripts/import-ala-ecology.mjs

## Implementation log

### Implementation checkpoint — 2026-10-11

- Preserved the supplied concept image and made Community Grounds the appearance target; created the replacement register, art bible, engine capability matrix and QA evidence index.
- Connected material-aware body loading, water-pass routing and stateful Qualia controls in the game/upstream engine. Temporal receipts now reflect actual producer views and successful execution state.
- Added four independent touring-rig recipes, four rig variants in exports, configurable rig selection, and a Direct Drive mode with throttle/reverse/brake/steer, rig-dependent handling, a bounded local road and a visible road-hazard HUD.
- Added original procedural silhouettes for goats, horses, rabbits, kangaroos, wallabies, wombats, deer, snakes, goannas, kookaburras, and crows at a non-graphic roadkill scene. The broad showcase profile selects this full encounter set; model groups and placements are documented as illustrative, with generic families left unresolved until a place profile selects local taxa.
- Added dawn/day/dusk/night drive periods. Twilight and night extend warning distance, dusk/dawn lighting changes at runtime, and the HUD labels the combination as a showcase rather than a real local occurrence.
- Added color-coded, labeled simulated oil and water/coolant temperature gauges plus live ambient, signed grade, road-surface, engine-load and engine-health indicators. Values respond to ambient temperature, throttle, terrain grade, road surface, speed and rig load; persistent critical heat degrades the simulated engine. A Qualia HUD mirror widget paints a rear-road view, following vehicle line and trailer obstruction, which clears when a bend exposes the queue.
- Separated road-hazard and engine-temperature alerts so nearby wildlife cannot hide an overheat warning. Both gauges show temperature trend and risk color; engine guidance responds to critical/elevated heat, rising/cooling trend and failure state. The temperature curve can reach critical under sustained hot, high-load conditions. The demo road centerline now matches its visible bends closely enough to drive mirror reveal independently of steering.
- Added a versioned camera graphics review fixture with six Community Grounds views; it now also includes a local aerial-drive view. Fixture mode pins the opening seed world and presentation defaults in memory, disables save/load and skips character persistence; normal campaign mode keeps its existing preferences.
- Added a bounded `set_flora_selection` Qualia WASM bridge and five original, profile-selectable Australian plant groups: bottlebrush, banksia, grass tree, saltbush and kangaroo paw. The first broad corridor profile selects them only as an illustrative mix, and the package catalog records `flora.rs` as their source. The three styles now each contain 578 assets across 40 states, and the replacement register covers 536 stable IDs.
- Added the first broad Noosa–Melbourne/inland-to-SA-border profile and an offline ALA CSV aggregation/report tool that keeps coordinates out of the output, preserves DOI/query/licence context, and refuses to imply occurrence equals road hazard.
- Rechecked the current Qualia geospatial source before continuing the place-pack work. Its consent-gated Overpass adapter now reconstructs OSM points, ways and relation multipolygons; its Terrarium adapter fetches and mosaics elevation into provenance-bearing `.10d`. Corrected the Australia atlas plan: the remaining gap is a repeatable offline OSM import/pack workflow plus the game-facing tile, terrain, streaming and route integration, not basic way/relation or DEM acquisition support.
- Added optional ENU rectangle bounds to Qualia's OSM vector frame and Sutherland–Hodgman clipping for marker, road/line and simple-area triangles, preserving interpolated elevation at clip edges. Added `compile_osm_feature_10d_in_tile` to the game WASM surface while retaining the existing unclipped bridge. The game release-target offline cargo check and the subsequent release build passed.
- Expanded the implementation handoff and Australia atlas plan to reflect the user's clarified hierarchy, civics.au / dev.civics.au project-data underpinning, and extensible mode architecture for first-person/aerial driving, interpersonal choice stories, and agriculture, hospitality, workshop/vehicle repair and food tasks. These are scope specifications; no Australian place pack or those scenario modules are claimed as implemented.
- Inspected the relevant local workspaces read-only and added `docs/planning/37-civics-project-data-bridge.md`. The game has a fictional campaign seed, illustrative ecology profile and ALA aggregation tool; the checked `www.civics.au` source has concept articles rather than the new public evidence exports; `observatory.civics.au` has a tracked README only; and the Humanitarian Equity Framework contains RDF models with a custom reserved-rights licence. The public dev.civics.au pages describe area-level planning evidence, site and energy/water models, the rig budget pathway, a reviewed food evidence graph and synthetic databox demos. No missing source data was fabricated or bundled.
- Added a provisional v1 Observatory game-pack manifest schema and an offline CLI scaffold that checks envelope metadata, relative artifact paths, uniqueness, containment and SHA-256 integrity while leaving RDF/Q42, Qualia Logic and VibeScript payloads opaque. Updated the source bridge and overhaul plan to match the user's clarification that Observatory is the likely registry and dev.civics.au provides related project information. The package contract awaits alignment with the still-in-progress producer; no source pack is claimed integrated.
- Built the release WASM offline in 14m11s, regenerated `web/pkg` with the cached `wasm-pack --mode no-install` CLI, and exported all three HMC style packs. Qualia HMC validation passed for each 578-asset/40-state pack; the stable-ID register still contains 536 entries. Updated WASM SHA-256: `5036235ed73c1d6f581ee95c93e312eb8f91443784e09a19337bbe217bc8582f`.
- Added an original SVG/CSS Australian loader illustration with moving gum foliage, sun, road and touring rig, plus a live region, staged status, measurable pack download progress, fallback copy, fatal retry and reduced-motion treatment. Startup now waits for pack verification or the valid built-in scene fallback before dismissing the overlay; style changes show the same loader and use normal browser caching for reusable static packs.
- Added a switchable aerial route-follow camera to the local driving slice, alongside first person. The HUD names the active view and offers a button; `V` toggles between them. This is a presentation view on the fictional demo road, not Australia-scale route navigation; route packs and geographic scope remain open.
- Added a versioned interactive scenario-module schema and five fictional fixtures for resident conversations, fruit harvest, workshop repair, a shared meal and a housekeeping practice shift. The new Activities panel follows each module's node from accepted action IDs, routes choices through the existing Qualia `GameSession` gates and event tape, and reads progress back from replayable actions. Three housekeeping priority choices and shift completion add scoped facts to the same world without introducing a second currency. Modules identify their fictional/synthetic provenance; there is no real employer, venue or public food recommendation implied.
- Improved all touring rig silhouettes with rounded Qualia `Pod` geometry for the 4WD cabins, campervan shell and caravan shell; the roof-top tent now has a gabled roof mesh. Stable asset IDs are retained, so existing rig selection and campaign references remain intact.
- Added a normalized game-side place/route pack schema and structural validator after the provisional Observatory intake envelope. The current fictional fixture populates a Places HUD tab that navigates the existing campaign territories and distinct site scenes. The package supports nested places, source-artifact geometry references, route surfaces/grades/elevation, rig constraints and hashed render tiles while leaving producer RDF/Q42 and enhancement payloads opaque; it contains no Australian GIS or route claim.
- Added a static human model pass using Qualia computational geometry: tapered limb segments replace block arms/legs, rounded hands and footwear replace mitten blocks, facial details are softened, and six residents receive distinct skin/hair palettes plus planting, repair, harvest, teaching, watering or carrying gestures. Stable scene-pick IDs remain for the original resident parts; six neck/nose detail assets were added.
- Updated README and Australia route/graphics plans to distinguish the fictional local driving fixture from a future source-derived Australian road graph. No exact corridor GIS boundary or ecological snapshot has been imported.
- No tests or browser self-test were run, following the user's standing instruction. The direct WASM-target `cargo check`, offline release `cargo build`, pinned `wasm-pack --mode no-install` and asset exporter passed. Qualia HMC validation passed for all three regenerated 578-asset, 40-state packs. The register generator records 536 stable asset IDs across styles, scene states, material families, roles, source paths and review tasks. Current WASM SHA-256 is `5036235ed73c1d6f581ee95c93e312eb8f91443784e09a19337bbe217bc8582f`. Actual render captures and visual acceptance remain open.
- Static inline HTML JavaScript parsing, exporter syntax, three manifest counts/WASM digests and both repositories' `git diff --check` passed. The first inline parser invocation used a malformed check expression; it was corrected and the parser then passed. The standalone `rustfmt --check` invocation is blocked by Windows sandbox access to the drive root (`Access is denied`); changed Rust compiled cleanly. No browser run, screenshots or visual acceptance were performed.
- Latest local aerial-drive review fixture, Observatory scaffold and scenario UI changes passed inline-script parsing without execution, `node --check` for the validator, and JSON parsing for the manifest/scenario schemas and camera/scenario fixtures. Targeted `git diff --check` passed for `web/game.html`. No test or browser run was performed.
- Expanded the review route to ten read-only camera and HUD states, including the fictional place graph, resident choice module and workshop repair module. Visual fixtures now block campaign edits and restore the prior HUD/selection after the showcase. No rendered captures or acceptance are claimed.
- Latest static checks passed: node --check for both validators; JSON parsing for the Observatory, scenario and place schemas plus current fixtures; non-executing parsing of both inline module scripts; tracked-file git diff --check; and targeted trailing-whitespace review. No tests or browser run were performed, following the user instruction.
- Replaced one low soil bed with a timber-framed hydroponic display in the Kestrel garden recipe: three separated Qualia water surfaces appear when water is online, three crop heads render when garden activity is active, and the muted lined hardware remains in the dry/offline scene. Rebuilt the release WASM with the locally installed, pinned wasm-bindgen 0.2.125 tool and regenerated all `.10d`, manifests, HMC packs and the review register. Each style now has 585 assets across 40 states; the register has 542 stable IDs and all three HMC readers accepted their exports. The scene crate's release-target `cargo check` and targeted `git diff --check` passed. Following the user's preview request, opened the rebuilt page in the system browser and confirmed host HTTP 200 for the game HTML and WASM; no rendered screenshot or visual acceptance is recorded. No test suite was run.
- Latest startup repair: the generated WASM package lacked the `GameDriveSimulation` export referenced by the page, and `qualia.ref` pinned an older QualiaDB commit. In addition, `scripts/build-game.ps1` failed when resolving `node` under PowerShell environments without expanded NVM symlinks, and `scripts/verify-game-release.mjs` was checking the outdated 585-asset count. Added resilient local Node discovery to `build-game.ps1` and aligned `verify-game-release.mjs` with the 630-asset catalog. Completed a clean rebuild via `scripts/build-game.ps1`: the release WASM (SHA-256 `3ca0c643bb536daca40855c3a0d114d01eeb87f4e9911640a24492bb7ea93df7`), all three 630-asset/40-state HMC style packs, asset manifests, and page cache keys (`game.html`, `spike.html`) are synchronized and verified. `scripts/test-game-wasm.mjs`, `scripts/verify-game-release.mjs`, script syntax checks, and `git diff --check` all pass cleanly. Next exact action: reload the local preview and assess the rendered water, vegetation, vehicle silhouettes and object picking. Actual localities still await the first Observatory publisher contract; keep corridor ecology illustrative and occurrence records separate from road-hazard probabilities. G00–G09 and the overhaul remain incomplete.
Latest continuation (12 October, source only): replaced the workshop shell with
the focused `workshop_structure.rs` family and extracted equipment into
`workshop.rs`. The shed has human-scale open bays, framed corrugated construction,
roof trusses, real side-window aperture and solar cells aligned to their roof
plane. Replaced camp capsule recipes with `camp_home.rs`, using native Qualia
closed-loft coachwork and an engine-transformed orientation, plus preserved
camp fixtures and canvas-room upgrade states. Removed duplicate legacy workshop
trim geometry, preserved its semantic IDs at the new heights and moved the access
spur outside the wider shed. Updated material metallic values, hero colour
preservation and review cameras. The preceding turn converted ordinary menus
to compact native HUD docks and added tabbed desktop/mobile information panels.

No Cargo/build, test, browser or rendered capture was performed in these two
continuations, following the user's instruction to complete implementation before
testing. Generated artifacts still represent the preceding build. These source
changes do not satisfy G04 acceptance; vehicle apertures/interiors, animation,
material/texture round-trip, renderer feature proofs and the connected gameplay
modes remain outstanding. Next: finish native material/texture/animation bindings
and their authored assets, continuing the source implementation before regenerating
and testing the full release. Keep G00–G09 and the full goal open.

Material continuation (12 October, source only): added Qualia surface map/PNG/UV
authoring and encoded texture hydration modules; preserved UV01 in scene merge;
added caller-buffered UV tangent generation and delegated GPU upload to it.
Added the game material library, PNG digest sharing, MAT4 texture dependencies,
UV01 export, textured dynamic/HMC hydration and resident/requested receipts.
Updated native pack validation, asset export, release verification and authored
tests for missing dependencies and round trips. Detailed seam-aware UVs and
animated skin/clip export/playback remain unfinished. No build, Cargo, test or
browser command was run; checked-in generated packages are unchanged. Next:
connect authored normal/skin/clip streams and playback, finish hero UV topology
and the remaining graphics/gameplay packages before the deferred full validation.
Full G00–G09 acceptance and the goal remain open.

Surface-frame continuation (12 October, source only): replaced single-plane game
UV generation with Qualia face-oriented charts, explicit seams and source vertex
reuse inside smooth charts. Compiled assets now carry authored normal sidecars;
scene accumulation preserves those normals and the portal supplies them to the
GPU. Hard sheet/box normals and smooth curved normals are game authoring choices;
chart construction and frame generation remain native Qualia capabilities.
Extended pack/read-back validation and authored tests for vertical projection,
atomic buffer failure, chart reuse and normal preservation through merging/orbit
normalisation. No Cargo/build, tests, formatter or browser command ran. Generated
packages remain unchanged. Next: finish organic/hero mapping and rig/clip content
with native animation playback, then continue the remaining G05–G09 and connected
gameplay requirements before the deferred full validation. The full goal stays open.

Viewport presentation continuation (12 October, source only): updated
`web/game.html` so desktop navigation floats over the world, tools have an
explicit close control, and supported browsers can enter document fullscreen.
Closed tools no longer reserve camera framing space. Opening tools returns to
the compact play HUD; contextual interactions close the tools to prevent
overlapping docks. Mobile keeps bottom navigation and bounded bottom sheets,
hides movement controls during contextual interactions, and removes movement
hint text that competed with touch buttons. Canvas/HUD dimensions now follow
the actual scene surface using ResizeObserver and visual viewport events for
mobile browser chrome, orientation and fullscreen changes. No Cargo/build,
tests, formatter or browser command ran; there is no rendered acceptance
evidence for these changes. Continue native rig/clip implementation and the
remaining game scope before deferred validation. The full goal remains open.

Native animation continuation (12 October, source only): added Qualia
`render::skeletal` with caller-buffered VibeScript motor clip sampling,
transition blending, iterative parent hierarchy composition, inverse bind
palettes, attachment transforms and four-influence position/normal skinning.
Limits are 128 joints, 4096 keys and 262144 vertices; malformed data and short
outputs fail before modifying output. Loop tracks must declare matching end
poses. Added authored tests for articulated rotation, bind/root placement,
loop and transition sampling, atomic rejection, and measured zero allocation.
Extended native surface chart authoring to emit original vertex lineage;
game `PreparedSurface` retains it for propagating skin influences through UV
seams. No Cargo/build, test, formatter or browser execution occurred. This
kernel is not yet serialized in `.10d` or connected to portal playback, so
residents are still static. Next: implement the canonical rig/skin/clip codec,
renderer lifecycle and resident authoring, then the remaining overhaul scope
before deferred verification. The full goal remains open.

Animation format continuation (12 October, source only): implemented Qualia
SKL1 section tag 16, little-endian rig/skin/clip records, stable identities,
caller-buffered encoding/decoding, validated zero-copy views and sealed
container attachment. Validation checks inverse binds against rest hierarchy,
complete clip/track/key partitioning, loop seams, skin weights and actual mesh
vertex counts. Added authored round-trip-to-skinning, malformed data, capacity,
container integrity and zero-allocation fixtures. Game HMC validation now
validates optional rigs, `.10d` verification reports joint/clip counts, and
multi-asset portal admission refuses malformed optional animation sections.
Format documented in engine `docs/manuals/standards/10d-skeletal-animation.md`.
No Cargo/build, test, formatter or browser command ran. No generated packages
changed; resident assets still do not carry rigs and there is no portal player
yet. Next: implement bounded resident playback buffers and scene lifecycle,
author the residents and activity clips, then complete the remaining full
overhaul requirements before the deferred verification. Goal remains active.

Resident playback continuation (12 October, source only): added native
`render::skeletal::SkeletalPlayer` with bounded cold admission and preallocated
frame buffers. Playback samples from immutable bind geometry, supports clip
fades, deterministic seek, root transforms, attachment lookup, reduced-motion
still poses and uniform orbit output transforms. It regenerates UV-aligned
tangents alongside posed normals. Added a GPU frame-span upload method for
positions/normals/tangents that updates draw and shadow bounds. Extracted portal
CPU replay mesh into `cpu_mesh.rs` and retained full-scene vertex lineage for
correct fallback span updates after decimation/reordering. Authored fixtures
cover deformation drift, attachments, reduced motion, invalid commands, fades,
allocation-free frame advance and fallback mapping. No Cargo/build, tests,
formatter or browser command ran. These components are not yet connected to
scene admission/tick, and game residents still lack authored SKL1 content.
Next: wire resident player ownership, transforms, CPU/WebGL/GPU update paths,
scene replacement/recovery and clip controls, then author resident rigs/clips
and finish the remaining overhaul before deferred validation.

Scene playback continuation (12 October, source only): connected SKL1 admission
to native players and attached their full-scene vertex spans to the portal.
The native tick now advances players and uploads full GPU surface frames;
CPU/WebGL and compact GPU paths update through retained fallback vertex lineage.
Orbit normalization and nonuniform body fit propagate to playback; native
normals/tangents regenerate after fitting. Scene replacement releases actor
buffers, resets receipts and retains reduced-motion preference; recovered GPU
replay is explicitly marked compact. Added aggregate resident memory admission,
group clip prevalidation, deterministic seek and native playback receipts.
Game bindings delegate clip identity hashing/control to Qualia; the browser
only forwards the user's reduced-motion preference. Extracted scene mesh tests
to keep the admission implementation below 500 lines. Added an authored
admission/orbit-motion fixture. No Cargo/build, tests, formatter or browser
command ran. No runtime/visual evidence exists; generated packs still contain
static residents. Next: author and attach resident rigs, skin fields and seven
activity clips, then wire accepted activity receipts and complete the remaining
overhaul/gameplay scope before deferred validation. Goal remains active.

Resident content continuation (12 October, source only): added game-authored
18-joint rigs for Ada, Bo, Cam, Dev, Eve and Finn, sharing bind positions with
their existing geometry. Each of the 60 stable resident parts now receives an
SKL1 section during compilation, with native segment-based influences and
native seam-weight remapping. Authored eight clips: idle, walk, repair, plant,
harvest, carry, talk and rest; interpolation/deformation stays in Qualia.
Export identity includes the new resident source and animation bindings.
Authored checks verify every resident part retains 18 joints, eight clips and
aligned UV topology, plus native rigid/hinge influence and lineage fixtures.
No Cargo/build, test, formatter or browser command ran; existing WASM/HMC
artifacts remain unchanged and there is no rendered animation evidence.
These are first articulated assets, not accepted AAA character art. Accepted
action routing, timed return to idle, tools, navigation/roster poses and save/
scene interruption behavior still require implementation, along with the
remaining full graphics/gameplay scope before deferred validation.

Activity presentation continuation (12 October, source only): added a native
bounded visual cue timeline with event-token idempotence, timed return clips,
pending off-scene cues and validated JSON snapshots. Scene replacement now
restores cue phase rather than discarding an ongoing activity; the timeline
continues while its actor is absent. Newer activities interrupt that actor,
and completed cue slots can be reused as actors change. VibeScript policy in
`web/fixtures/resident-activities.vibe` routes supported campaign actions to
repair, planting, harvest, carrying, conversation and rest. `GameSession`
checks its own current accepted event/sequence before invoking presentation;
the browser supplies no actor/clip policy and animation adds no campaign
effects. Save/load includes a separate native visual snapshot linked by a
world CRC fingerprint, rejecting stale visual files without changing campaign
RDF or replay. New campaigns reset presentation. Authored policy, snapshot,
idempotence, completion, invalid-state and zero-allocation fixtures. No
Cargo/build, tests, formatter or browser command ran; WASM/HMC artifacts remain
unchanged and visual acceptance remains unproven. Next: resident tool props,
navigation/roster integration and interruption refinements, then the remaining
hero assets, landscape/water, connected modes and full deferred verification.

Resident tool continuation (12 October, source only): added sixteen original
mesh parts forming Ada's trowel, Bo's spanner, Cam's harvest basket, Dev's
clipboard, Eve's hollow watering can and Finn's produce crate. Rounded stems,
handles, lofted blade, open slatted boxes and fruit use native geometry recipes;
each part is rigidly skinned to its resident's left/right hand through SKL1.
Authored material palettes are preserved for tools. Surface normal authoring
now unwraps placement transforms before choosing flat/smooth shading, retaining
hard clipboard and crate panels. Export identity includes `resident_props.rs`;
authored read-back checks cover the sixteen attachments and a crate fixture
checks single-wrist weights and distance preservation during carrying motion.
No Cargo/build, tests, formatter or browser command ran; generated artifacts
are unchanged. Tool scale/contact, two-hand constraints, pickups, collision
and visual quality remain unaccepted. Next: integrate resident identities and
navigation/roster behavior, refine attachment contacts and interaction, then
continue the full graphics/gameplay scope before deferred validation.

Viewport presentation continuation (12 October, source only): the game canvas
owns the full viewport; information remains in optional overlays. Removed the
old inspector display conflict and moved inspector/journal into a Journal tab
inside the closable dock. Mobile mode buttons and header controls retain touch
targets, with bottom sheets in portrait and a side rail in landscape. Native
HUD panel placement now derives available space from the actual host controls,
limits desktop panel width to 34% of the scene, and fits paired action rows to
the panel instead of clipping the second button. No Cargo, build, tests or
browser validation ran. This presentation change does not complete the graphics
overhaul; generated engine/game artifacts and visual acceptance remain pending.

Connected workshop continuation (12 October, source only): added the authored
`rig-workshop.vibe` cooling-fault and protected solar assembly policies, plus
`rig_workshop.rs` native session bindings. Operations check evidence, tool use,
mount compatibility, assembly order and finite scenario-kit stock. Accepted
operations save intermediate assembly snapshots and a bounded 256-operation
tape per rig in the shared RDF world. Reload reads and deterministically replays
the tape before trusting a snapshot; stale revisions and missing/duplicate
events fail closed. Campaign action events, day/tide and currency remain owned
by their existing reducer. Connected replay verifies each workshop tape before
comparing campaign replay. The host adds a Workshop entry and a paged native HUD
control surface. Native/VibeScript drive conditions prevent driving an open,
isolated or incomplete cooling assembly; fitted radiator efficiency affects
the authored thermal model. Authored fixtures cover cooling dependencies,
incompatible mount choices, kit consumption, saved disassembly, stale revisions,
tampered tape and electrical verification. No builds, Cargo, tests or browser
validation ran; served/generated WASM remains unchanged. The workshop still
needs an inspectable assembly scene with movable parts/fasteners, tool inventory,
reversible electrical disassembly, supply sourcing, progressive hints and
animation/contact integration. Driving damage/temperature persistence and full
cross-mode chronology are also unfinished. Continue those systems and the
remaining graphics/gameplay requirements before the user's deferred validation.

Workshop assembly continuation (12 October, source only): extracted the shared
asset compiler into `asset_compile.rs` so focused scenes use the same Qualia
geometry/material/UV/provenance pipeline as the community. Added a native
session `workshop_scene` binding and `workshop_assembly.rs` service-bay source:
rig-dependent coachwork, chassis, bonnet/support, wheels, engine/cover ribs,
finned radiator and mounts, hose segments, reservoir, controller/heatsink,
protection cartridge, cell-panel/frame, bench, socket, gauge and catch tray.
Accepted assembly state moves removed radiator/mounts/hoses onto the bench,
opens the bonnet and places installed electrical components on the rig.
Native semantic part picks focus corresponding operations; keyboard/touch
panel alternatives remain. Workshop entry loads its own full-viewport scene;
exit restores community loading, and camera position is preserved across
assembly changes. The workshop bench activity enters this mode. Export source
identity includes both new modules. Authored geometry fixtures check finite
meshes, unique semantic IDs and stable identities across disassembly. No
build, Cargo, tests, formatter or browser validation ran. Component geometry,
contact, shading, part visibility/occlusion and interaction are unaccepted;
this is still an authored assembly scene rather than finished AAA assets.
Continuous manipulations/fastener animation, tool selection, progressive hints,
electrical disassembly, spares sourcing and connected drive damage persist.

Persistent touring condition continuation (12 October, source only): added
`drive_session.rs` with per-rig native driving state in bounded RDF checkpoints.
Tick inputs retain browser controls/scene facts while saved motion, thermal
condition, health, counters and distance override browser-supplied state. The
session obtains readiness/cooling from accepted workshop state. Entry/reload
verifies policy identity, bounds and latest-tick deterministic reproduction;
accepted native mutations invalidate entry verification. Explicit VibeScript
parking stops motion without repairing damage, changing position/distance or
advancing time. Drive entry, rig changes, mode exit, save/load and new campaigns
now use the native state. Save captures RDF and its visual fingerprint together
before asynchronous writes. Idle thermal load no longer treats a stationary
rig as continuously hauling uphill. Added authored native persistence,
overheating, isolation-between-rigs, parking and corrupt-checkpoint fixtures.
No Cargo, builds, formatter, tests or browser validation ran; generated artifacts
remain unchanged. Checkpoints retain only the latest tick and cannot prove the
complete driving history. Replay UI states that limit; full cross-mode history,
policy migration, performance/resource acceptance, engine overhaul gameplay,
assembly animation and all remaining graphics/gameplay requirements still need
implementation and the deferred validation.

Vehicle cabin continuation (12 October, source only): added `vehicle_cabins.rs`
to replace closed cab lofts and dwelling volumes behind the glazing. Original
rig/body/cabin/glazing identities are retained. Car and 4WD models now separate
bonnet/trunk volumes, thin door walls, floor, A/B/C pillars, sloped windscreens,
roof caps and glazing. Campervan/caravan shells use wall strips around open
window belts and low floor caps. Interiors add independent rounded seats,
headrests, dashboard, right-hand steering wheel, gear lever, cabinets/berth and
mattress. Window seals follow each pane's orientation through native Qualia
transforms; the existing transparent, double-sided, non-shadow-casting glass
material remains. Added the general authored `Oriented` placement recipe and
preserved flat panel shading under it. Export identity includes the new cabin
source. Authored fixtures use Qualia ray/triangle queries to check clear side
apertures, separate material roles and unique semantic part IDs. No Cargo,
builds, formatter, tests or browser validation ran. Vehicle appearance and
transparency remain unaccepted. Wheel arches, refined coachwork joins, doors,
lighting details, suspension/wheel/towing animation, actual mirrors and full
AAA-quality art still need completion, alongside the remaining programme.

Vehicle guard/exterior continuation (12 October, source only): added the
directory-backed native `computational_geometry::wheel_panel` cold builder.
It accepts bounded guard/axle parameters and caller-owned output buffers;
193 four-point sections at most pass through Qualia's closed loft kernel.
Its allocation classification is ColdBounded. It is not declared compiled,
tested or accepted in the capability maturity registry. Added the game
`WheelClearancePanel` recipe binding and `vehicle_exterior.rs`. Thin native
guards now replace opaque strips across wheel openings in doors and dwelling
sides. Bonnet/rear central volumes narrow behind broad separate curved caps;
floor caps narrow away from tyre sidewalls. Added rounded arch trim, mud flaps,
separate headlamps/indicators/tail lamps, mirror arms/housings/glass and door
handles. Authored fixtures check finite guard geometry, parameter/buffer
rejection without output mutation and upper-tyre clearance using native ray
queries. Export identity includes the exterior source. No Cargo, builds,
formatter, tests or browser validation ran. Continuity, arch clearance across
all angles, silhouettes, material/lighting quality and performance remain
unaccepted. Physical mirror geometry is not a live rear-view renderer. Full
mirror views, wheel/suspension/towing animation, detailed coachwork and the
rest of the graphics/gameplay programme remain outstanding.

Native touring animation continuation (12 October, source only): added
`vehicle_animation.rs` SKL1 authoring for touring body parts and wheel/rim
hinges, retaining stable part identities and using the shared seam-aware rig
attachment pipeline. Corrected duplicate caravan wheel IDs with their own
component namespace and added missing camper-trailer wheels. Native skeletal
players now retain bounded joint-control deltas in separate pose scratch, so
clip fades do not apply a control rotation twice. Portal group/root and joint
controls feed the existing native deformation/upload path. VibeScript records
signed wheel phase, steering and radius; native accepted driving state applies
vehicle translation/yaw/slope and wheel steering/rotation, with trailer tyre
radius compensation. The host forwards native receipts on ticks/reloads;
resident activity reset skips independent touring idle clips. Scene admission
now bounds animated mesh parts at 256 while retaining the aggregate 42 MiB
player budget. Authored binding fixtures check body/wheel weighting, shared
root IDs, motion-joint coverage and unique part IDs. No builds, Cargo, tests,
formatter or browser validation ran. Native geometry movement is source work,
not observed animation. Pose/contact, driver camera fit, lights/mirrors,
suspension and articulated trailer dynamics, resource/performance gates and
the remaining full graphics/gameplay scope remain unfinished.

Trailer articulation continuation (12 October, source only): towing rigs now
have a stable hitch joint. Caravan/camper-trailer body parts skin to it; their
wheel hinges parent to the hitch while tow-vehicle wheels parent to the root.
Coupling/drawbar placement follows the authored hinge positions. VibeScript
persists trailer heading, signed trailer wheel phase, articulation and warning
state; its single-track model includes lateral velocity at the rear hitch.
Reverse angle protection stops further backing while allowing forward recovery.
Vehicle heading no longer rotates in place from steering input; reverse steering
changes heading in the opposite direction. Native presentation applies relative
hitch yaw and independently accumulated trailer wheel rotation. Read-back
expectations now cover ten joints for 4WD/caravan and eight for car/trailer.
Authored fixtures check fixed hitch position, independent tow-body pose,
wheel parenting, saved heading and reverse angle recovery. No Cargo, builds,
formatter, tests or browser validation ran. This kinematic source is unverified;
terrain/contact at trailer axles, swept collision/off-tracking, drawbar limits,
suspension, mass/braking response, camera/mirror rendering and performance
acceptance still need completion, with the rest of the graphics/gameplay goal.

Native contact/suspension continuation (12 October, source only): added
`drive_contacts.rs` to sample the authored terrain at accepted vehicle and
trailer wheel locations through native motor transforms. Forward/rear grade
samples follow saved heading and override browser terrain values. The authored
VibeScript contact controller supplies damped body height, pitch, roll and
bounded independent wheel travel; trailer pitch/roll respond to terrain and the
attached hitch. Native rendering composes body orientation and uses its inverse
to set independent trailer orientation, then applies wheel suspension offsets
alongside steering/spin. First-use parking initializes contact without advancing
time or changing damage/travel. Checkpoints identify the terrain source as well
as the driving policy. Authored fixtures cover flat-ground tyre placement,
parking initialization, ignored browser grade fields and bounded wheel travel.
No builds, Cargo, tests, formatter or browser validation ran. This samples the
authored height field, not the rendered bridge deck or arbitrary streamed
collision geometry. Contact accuracy, motion quality, swept collisions, shock
travel/steep terrain behavior, camera/mirrors and resource/performance acceptance
remain unverified; the full graphics/gameplay overhaul is still active.

Native water appearance continuation (12 October, source only): corrected WGSL
fragment depth to use rasterised Z directly and Fresnel to use camera view angle.
Removed the depth-gradient foam cue. Added caller-buffered native boundary
distance authoring with component-aware topology, independent depth scales,
finite/manifold validation and zero-heap scratch processing. GPU water vertices
carry those attributes; cold upload accounts for caller geometry, shoreline
scratch and GPU packing before the 42 MiB budget gate. Shore attenuation follows
curved outlines, foam is confined to banks, and per-surface scale avoids treating
small channels as deep lakes or painting coarse triangle surfaces with foam.
Water now uses restrained shallow/deep tint, analytic sky Fresnel reflection,
sun highlights and fine ripples. Added native metre-based appearance controls;
game waves use 10–25 mm amplitudes, spatial frequency follows world scale and
animation speed is independent of spatial frequency. Authored fixtures cover
boundary/interior values, disconnected overlapping surfaces, rejection without
output mutation, zero allocation and physical-unit conversion. No builds,
Cargo, formatter, tests or browser validation ran. Shader compilation, actual
appearance/depth/temporal correctness, resource/time budgets and mobile/fallback
acceptance remain unverified. Reflection is analytic sky, not scene reflection;
bathymetry/refraction, shadows on water, tile seam topology, spatial acceleration
for large shore meshes and the rest of the full graphics/gameplay scope remain.

Native shoreline acceleration and computational integration continuation
(12 October, source only): water authoring now reuses QualiaDB's existing
Morton BVH builder, native bounding-box queries and exact point-to-segment
distance kernel. The adapter preserves per-component shores, uses bounded
caller arrays and accounts for the native builder's centroid scratch before
the 42 MiB admission gate. Queries use a fixed traversal stack. Water upload
records boundary-edge count, segment evaluations and admitted workspace bytes.
Authored fixtures compare curved-bank results with the reference kernel,
require reduced segment work, check deterministic output and overlapping
components, and reject short scratch without modifying output. These fixtures
have not been executed.

Read the supplied computational/scientific demo pages from their local engine
copies after the web reader could not fetch the public URLs. Inspected the
canonical solver registry, specialized library cfgs, Vibe math dispatch and
game WASM features. Plan 39 records the computational reuse map, current WASM
feature availability and source-first integration procedure. Existing Qualia
CAS, calculus/ODE, units, algebra, optimization, statistics and graph kernels
must be inspected before introducing numerical algorithms. The current Vibe
thermal/contact approximation has not yet been replaced by the native ODE
kernel or calibrated. No Cargo, builds, formatter, tests or browser validation
ran; the full graphics/gameplay goal remains incomplete.

Native suspension computation continuation (12 October, source only): added
Qualia `render/physics/response/` as a bounded adapter over the existing fixed
state `RungeKutta4Static` solver. VibeScript selects ground target, normalized
spring/damping parameters and travel/speed limits; native code substeps body
height/velocity response at up to 240 Hz over an admitted maximum 50 ms tick.
The solver implementation is reused rather than copied into the game or browser.
The native contact bridge augments geometry facts before Vibe pose evaluation;
initial/teleport contact still follows the authored reset policy. Removed the
unexplained 20 mm body offset so flat-ground wheel placement targets the surface.
Travel projection stops outward velocity at hard limits. Source identity now
includes the native bridge, response adapter and canonical solver; checkpoints
with changed numerical identity are refused without rewriting the world.

Authored fixtures cover analytic critical damping, interval subdivision,
zero-time preservation, hard-limit velocity, invalid inputs, zero allocations
and refusal of a changed saved solver fingerprint. None were executed. Existing
checkpoint migration, thermal integration/calibration, bridge/streamed collision
contact, axle load/mass response, camera/mirrors and visual/performance acceptance
remain incomplete. No Cargo, builds, formatter, tests or browser validation ran;
the full graphics/gameplay objective remains active.

Native thermal/Vibe host continuation (12 October, source only): the driving
policy now invokes `PhysicsWorkbench.compute` with `thermal_response` instead
of integrating oil/coolant temperatures in its own exponential formula.
Qualia's response adapter delegates to the existing fixed-state RK4 solver
with bounded substeps and finite/rate/interval validation. The Vibe policy
continues to own grade/load, ambient temperature, cooling-efficiency targets,
thresholds, accumulated overheat and irreversible engine damage. Driving setup,
standalone step and persisted tick/replay now use the real Qualia Poet host
rather than the local Vibe host that cannot invoke engine computations.
Vibe declares the required workbench capability; its drive tick is effect-class
because capability invocation is classified that way, while the selected native
thermal operation is deterministic and performs no graph or clock operation.

Authored analytic cooling, zero-time, invalid-rate, zero-allocation and actual
Vibe-to-native invocation fixtures remain unexecuted. Contact source identity
includes the thermal kernel through the shared response hash. Old numerical
identities refuse resume rather than silently reinterpreting checkpoint state;
versioned migration and calibration of physical parameters still remain.
No Cargo, builds, formatter, tests or browser validation ran. This is source
integration, not evidence of verified temperatures or visible game improvement.

Native cabin camera continuation (12 October, source only): added a generic
Qualia rigid-body camera mount adapter over the existing motor/orbit camera
APIs. First-person driving now has authored right-hand driver eye positions
matching the car, 4WD and campervan cabin geometry. Vehicle rendering and camera
attachment share native accepted body translation/yaw/pitch/roll composition.
The browser transports normalized camera parameters to the portal and HUD;
relative cabin look persists during driving rather than being overwritten by
the next steering tick. Entering the native driving view initializes contact
by parking without advancing thermal/travel state and presents the touring rig.
Native camera conversion uses the admitted 0.35 orbit distance, preserving the
mounted eye instead of requesting 0.30 and letting native clamping move it.

Authored placement fixtures check exact eye reconstruction, scene normalization,
body rotation and invalid scale. They remain unexecuted. The existing camera
ABI keeps world-up rather than camera bank roll and has fixed FOV/near-plane
settings; cabin visibility, near clipping, mobile comfort, mirrors and actual
visual quality still require completion and rendered acceptance. No Cargo,
builds, formatter, tests or browser validation ran. The full overhaul remains
active and incomplete.

Native close-view lens continuation (12 October, source only): camera state
now carries an admitted near plane rather than requiring every view to use the
fixed 0.05 scene-unit default. Native portal/GPU/HUD controls preserve that lens
when camera position changes. The game converts a 35 mm walking/driver near
plane through the current world scale; wider map/aerial modes retain the default.
The native projection matrix is shared by GPU uniforms, current/previous camera
motion matrices, CPU picking, HUD markers, Canvas geometry and the WebGL2 mesh
fallback. Temporal linear-depth producers receive the same near plane; changing
it invalidates GPU history. Portal lens behavior lives in an extracted module
under the tracked portal decomposition rather than expanding its monolith.

Authored fixtures check a nearby dashboard point against the clip volume,
GPU-uniform/shared-matrix identity, preservation of default projection and
physical-unit conversion/refusal. No fixtures ran. FOV and world-up roll remain
unchanged; visible cabin quality, backend parity, depth precision, AO/temporal
acceptance, mirrors and the remaining full game overhaul still require work.
No Cargo, builds, formatter, tests or browser validation ran.

Australian macropod asset/animation continuation (12 October, source only):
replaced kangaroo/wallaby recipes with extracted original anatomical forms,
tapered two-section limbs/tail, shaped ears and inner ears, facial eyes/nose,
and higher-resolution curved surfaces through Qualia's authoring primitives.
Existing encounter mesh IDs remain; additional detail parts are explicit new
IDs. The model placement transform supplies native bind coordinates and scale.
Each model part carries a shared 18-joint SKL1 rig with idle, alert and hop clips,
native segment-authored skin weights and the existing UV-seam weight remapping.
Qualia's player remains responsible for sampling, deformation and backend upload.
Accepted driving warnings select alert clips for the matching macropod encounter
and return others to idle. Cosmetic idle/alert does not relocate encounter roots;
the hop clip is authored but not used as a substitute for accepted navigation.

Authored fixtures check preservation of original IDs, shared joint identity,
SKL1 validation/readback and native pose/skinning deformation with fixed feet
and encounter roots during idle. None ran. Species-specific gait/contact,
wildlife navigation and animated goats/horses/rabbits/wombats/deer/reptiles/birds,
fur/eye shading acceptance, resource budgets and rendered quality remain
unfinished. No Cargo, builds, formatter, tests or browser validation ran; the
full graphics/gameplay goal remains active.

Sleep-break handoff (12 October, source only): extracted species-specific goat,
horse, deer and wombat models into `quadruped_models.rs`. Original coat/body,
head/muzzle/nose, horns/antlers/mane, tail, ears and leg IDs are preserved; added
eyes and feet are new detail IDs. Native transformed recipes now expose bind
origins. `quadruped_animation.rs` supplies shared 19-joint skeletal sections
with independent hip/knee/ankle, neck/head, ear and tail articulation, and idle,
alert, graze and walk clips. The shared asset compiler attaches these bindings
through the existing SKL1/UV-seam pipeline. Accepted driving warning presentation
now selects idle/alert for these four species as well as macropods. Walking and
grazing clips are authored data, not verified navigation or grounded interaction.

User requested wrapping up for a sleep break; the goal is being paused, not
completed. All current changes remain in the working trees. Generated WASM/HMC
were not regenerated. No Cargo, builds, formatter, tests or browser validation
ran. The preview therefore is not evidence of these source changes. On resume,
inspect current state first, then continue the full plan 35 scope. Remaining
work includes additional fauna animation, grounded navigation/contact, shared
animation resource admission, physical calibration/checkpoint migration, real
mirrors/traffic, water reflection/refraction, environment and asset quality,
connected modes/story/supply-chain implementation and full rendered/performance
acceptance. Keep the user's implementation-before-testing command in force.
