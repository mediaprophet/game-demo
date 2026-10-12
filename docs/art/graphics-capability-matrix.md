# Game graphics capability matrix

## 12 October material/texture continuation — source only

The multi-asset portal now preserves `UV01` through `SceneMeshAccum` and submits
the merged stream to `PortalGpu`. The GPU path generates UV-aligned tangent
frames through caller-buffered `render::mesh_tangent_frames`, replacing arbitrary
normal-derived tangents when authored UVs are present. Legacy streams retain
their existing fallback. New `portal::texture_upload` hydrates encoded image
dependencies with SHA-256 validation, bounded codec output, semantic mip filters
and reuse of already-resident digest/colour-space/semantic bindings.

`render::surface_authoring` owns bounded deterministic tiled colour/normal/ORM
generation, standard PNG encoding into caller memory and planar UV projection.
The game `material_library.rs` supplies authored family parameters and shares
content-addressed images. Its MAT4/UV01 assets and PNG dependencies travel through
both dynamic scene compilation and exported HMC. Native pack validation checks
referenced images and UV counts; load receipts distinguish requested from
resident texture bindings. Export/release verifiers have been updated accordingly.

Ownership review: codec/map authoring, tangent kernels and portal hydration are
separate focused modules. Existing portal/GPU upload modules contain only the
delegating calls; further authoring or texture lifecycle behavior belongs in those
focused modules, not in the oversized portal facade.

No build, test or browser execution was performed for this continuation. Tests
were authored for PNG normal-pixel round-trip, atomic admission failures, UV merge
preservation, mirrored tangent orientation and missing HMC texture dependencies.
That interim planar mapping has now been replaced for compiled game assets by
the caller-buffered `render::mesh_surface_frames::box_project_mesh_into` chart
builder. Dominant-face projection supplies noncollapsed coordinates on side,
top and end surfaces; chart seams split vertices, while smooth charts reuse
source vertices within their chart. Triangle order/material ranges are retained.
Authoring supplies smooth source normals for curved models and flat normals for
sheet/box parts. The `.10d` normal sidecar is now retained through scene merge
and uploaded with UV01; nonuniform anatomical fits recompute their geometric
normal frame before upload. Read-back receipts and pack validation inspect
normal/UV stream counts. Authored tests additionally cover vertical face UVs,
seam buffer admission, chart vertex reuse and merge/orbit normal preservation.

Hand-designed texture atlases, cylindrical organic mappings and character
skin/clip playback remain open. These entries are source evidence,
not graphics acceptance or proof that the checked-in WASM executes the changes.

## 12 October coachwork authoring addition — awaiting validation

Qualia `specialized_libs::computational_geometry::authoring::loft_convex_sections_into`
is implemented in focused `authoring_loft.rs` and re-exported by `authoring.rs`.
It accepts bounded convex XY rings at ordered Z stations and writes closed loft
geometry into caller-owned vertex/index buffers, returning counts and bounds.
The game `ParametricRecipe::LoftedBody` consumes it for touring coachwork; shared
Qualia primitives and transforms still own other model generation.

Targets: native and game `wasm-full`. Status: source implemented, uncommitted,
unbuilt and unverified. Authored tests cover deterministic geometry, cap winding,
bad topology/coordinates and insufficient caller buffers. No tests have been run
for this addition; user instruction defers build/testing until implementation is
complete. No new engine SHA, browser receipt or approved asset sample is claimed.

Inventory current symbols on the pinned Qualia revision before adding any call. The entries below record what was visible in source inspection on 2026-10-11; they are deliberately conservative.

| Capability | Existing inspection evidence | Game status | Required next proof |
|---|---|---|---|
| Camera, lighting, sky, AO and .10d load | GamePortal wraps Qualia camera/sky/light/AO/shadow controls; game submits camera and style/quality state | Bound; visual result unreviewed | Record the six camera frames on named browser/backend/device configurations |
| HMC mesh | Game loader verifies the HMC manifest and referenced bundles, decodes scene assets, preserves authored frames and reports loaded meshes/water | Pack validated; browser residency unreviewed | Capture actual HMC scene load, material bindings, stable picks and campaign-state switching |
| Material/texture package | Recipes export stable material families and roles; Qualia decodes MAT4 materials, sampler state and submesh ranges. HMC texture dependencies/round-trip are still open | Materials partial; textures unverified | Demonstrate real maps, colour space, roughness/normal/alpha and dependency residency after hydration |
| Shadows | Game quality bridge calls Qualia-owned shadow enablement when admitted on high quality | Bound conditionally; effect unreviewed | Compare admitted and fallback browser frames and record the renderer receipt |
| Screen-space AO | Game quality bridge controls enablement, radius, strength, bias and sample count through Qualia | Bound conditionally; effect unreviewed | Capture matched on/off frames and record availability for each backend |
| Temporal AA | Game asks Qualia for a temporal schedule and carries actual producer/execution receipts; the ordinary-tick fallback remains explicit | Scheduling bound; moving geometry acceptance open | Show actual producer history and multi-frame execution on an animated fixture; otherwise preserve ordinary tick |
| Environment probe | GamePortal exposes Qualia's environment-probe-data setter; no authored probe dataset is currently submitted by the game | API exposed, content not integrated | Supply bounded authored probe data through the same game-owned renderer and compare a scene frame |
| Water | Game splits `renderRole=water` geometry into Qualia's animated water pass; curved channels and banks use Qualia spline geometry, and a GamePortal setter controls colour, waves and quality by campaign state | Curved scene and engine pass bound; rendered shoreline/depth response unreviewed | Capture flowing/high-tide states and verify reflection, foam, depth interaction and actual waves in the game path |
| Vegetation | MAT4 carries a foliage-wind material bit; Qualia's mesh shader reads engine frame time and displaces authored foliage vertices | Animated foliage path implemented; browser motion and shadow match unreviewed | Capture visible motion at overview/walk scale; check stable grounding, shadows and lower-tier fallback |
| Character rigs and clips | Residents are individually authored static task poses; no serialized animated-skin playback is wired through GamePortal/HMC | Static models; character motion remains open | Add the supported rig/clip export/import path and record deformation, contact, attachment and clip transitions |
| LOD, instancing and terrain streaming | Game authors Qualia-built heightfields and validated fixed scene packs; the route/world is still a bounded illustrative fixture | Static terrain bound; regional streaming/LOD unimplemented | Connect Qualia spatial pack APIs, then record adjacent seam, visibility, LOD and bounded-memory results |
| VibeScript composition | Game parser builds one shape for sphere/cylinder/water/vegetation/box/probe | Partial | Bind authored data to production scene/material/renderer APIs; probe currently maps to a sphere recipe |
| VibeScript game simulation | `drive-simulation.vibe` is checked in WASM and resolves drive movement, hazard stops, thermal damage and mirror state; action IDs also pass through VibeScript before the SHACL gate | Native runtime path implemented; WASM and browser checks pending | Verify named deterministic ticks, representative hazard/heat cases and ordinary drive controls in the built game |
| Backend recovery and quality | Portal tier/recovery controls and browser capability code exist | Partial | User-facing tier, truthful receipt, resize/loss/recovery playback in game |
| Direct Drive feedback | Qualia HUD supports colored labels/meters and a rear-view mirror widget; the VibeScript simulation supplies engine temperatures, following traffic and bend reveal | Implemented as an illustrative driving fixture; captured appearance unreviewed | Review rising/cooling alerts, towing occlusion, and bend reveal in actual rendered captures |
| Graphics review fixtures | Six camera presets live in a versioned JSON fixture; review mode pins style/rig/lighting/ecology and disables persistence | Deterministic setup implemented | Capture all six views at a recorded viewport/backend and compare them to the reference rubric |
| Profile-selected Australian flora | `flora.rs` authors seven Australian plant groups through Qualia mesh/10D tooling; profile string keys choose the groups before scene compilation | Models exported; corridor mix remains illustrative | Review all seven silhouettes and have each geographic pack select only source-supported local taxa |

The Vibe host's Render.* invocation must be traced through the full host, active game instance and WASM renderer. A successful host handler test does not demonstrate that it changes the GamePortal canvas.

When a feature is complete, replace the next-proof text with exact full engine/game SHAs, public symbol, exported WASM API, fixture, browser/backend and observable result.
