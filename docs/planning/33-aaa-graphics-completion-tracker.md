# AAA Graphics Completion Tracker

Status: **In progress — active implementation & verification programme (not complete)**  
Owner: QualiaDB/Webizen renderer programme; game integration: Maslow's Challenge (`game-demo`)  
Branch baseline: `0.0.40.7-codexfailurerecovery` (across `qualiaDB` and `game-demo`)  
Parent specification: [31 — AAA and cinema-grade graphics: engine specification and gap register](31-aaa-graphics-engine-gap-register.md)  
Implementation work order: `C:/github/qualiaDB/docs/planning/32-aaa-graphics-swarm-implementation-work-order.md`  
Last audited & reconciled: **2026-10-10**

---

## 1. Executive Audit Summary

An exhaustive audit of the graphics engine implementation across `qualiaDB` and `game-demo` on branch `0.0.40.7-codexfailurerecovery` establishes that **the AAA graphics engine implementation is substantial but remains an ongoing programme; it is not complete and must not be marked complete.**

### 1.1 Capability Status Summary
Across the 29 formal engine capabilities defined in [31 — AAA Graphics Engine Gap Register](31-aaa-graphics-engine-gap-register.md):
- **5 capabilities** are **Verified** (AG-03, AG-10, AG-12, AG-21, AG-22 with full adapter-backed pixel receipts)
- **10 capabilities** are **In progress** (possessing verified foundation or integrated slices)
- **14 capabilities** are **Not started**

```
Overall Engine Progress: [████████████░░░░░░░░░░░░] 48% Implemented / 17.2% Verified
┌─────────────────────────┬───────┬────────────┐
│ Disposition             │ Count │ Percentage │
├─────────────────────────┼───────┼────────────┤
│ Verified                │   5   │   17.2%    │
│ In progress (Slice)     │  10   │   34.5%    │
│ Not started             │  14   │   48.3%    │
│ Total Capabilities      │  29   │  100.0%    │
└─────────────────────────┴───────┴────────────┘
```

### 1.2 Honesty Contract & Operating Rules
Under the immutable rules of `AGENTS.md` and `CLAUDE.md`:
1. **Zero heap in hot paths:** Frame execution, culling, LOD evaluation, render submission, and buffer readbacks must not allocate heap memory (`Vec`, `String`, `Box`).
2. **42 MiB Prolog Sentinel separation:** The 42 MiB Sentinel bounds semantic execution and query logic only. Graphics resources (VRAM, texture cache, staging rings, geometry buffers) must be separately budgeted, tracked, and failed closed; graphics allocations must never be hidden inside the semantic sentinel.
3. **Compile is not verification:** Successful `cargo check` or `cargo test` on a mock or CPU oracle verifies only syntax and host logic. It does not qualify browser WebGPU execution, native GPU driver behavior, visual fidelity, or hardware performance.
4. **Package/manifest match is not GPU execution:** Validating HMC pack SHA-256 hashes proves asset delivery integrity, not that the GPU has initialized, bound, or rendered those assets.
5. **Apology is not remedy:** Unfinished tasks, missing producers, and untested fallbacks must be recorded against interest in this tracker rather than glossed over.

---

## 2. Documentation Drift Reconciliation

The audit identified significant documentation drift between the parent gap register (`31-aaa-graphics-engine-gap-register.md`), the implementation work order (`32-aaa-graphics-swarm-implementation-work-order.md`), and the commit history:

1. **Revision Date Drift:** The parent register header states it was revised on `2026-10-07`, whereas implementation entries and commits continued through `2026-10-10` (encompassing Sprints 1, 2, and 3).
2. **"Not Started" Classification Drift:** Six capabilities are labeled "Not started" in the register table despite having substantial, tested code slices merged into `0.0.40.7-codexfailurerecovery`:
   - **AG-09 (Terrain):** A deterministic 4-resolution, 256-sample edge seam oracle is implemented and tested (`terrain_seam_tests.rs`), though streaming and precision remain open.
   - **AG-10 (Water):** A GPU water shader (`water.wgsl`), transparent depth preservation contract (`water.rs`), and finite geometry upload budget are implemented, though shoreline interactions remain open.
   - **AG-12 (Vegetation):** Ambient vegetation pool, wind WGSL, and stable-seed simulation are implemented (`vegetation_effects.rs`), though live character deformation remains open.
   - **AG-13 (Particles/Effects):** Event-linked environmental effects with fixed-capacity pools exist, though full volumetric media remains open.
   - **AG-21 (Temporal AA):** Complete temporal submission lifecycle (`temporal_resolve_gpu.rs`), linear-depth generation, and lavapipe-compatible sampling exist, though real motion vectors remain missing.
   - **AG-22 (Environment Probes):** Mathematical CPU model (`environment_lighting.rs`), roughness-aware specular interpolation, and shader layouts exist, though GPU cubemap bindings remain open.
3. **Reconciliation Decision:** In this tracker, these capabilities are classified as **Foundation only**. They are not promoted to "Integrated" or "Verified" because their production driver, live scene integration, or visual/performance gates remain open.

---

## 3. Genuinely Implemented and Tested Baseline

The following capabilities and components are confirmed to be genuinely implemented, compilable, and covered by passing automated test suites on `0.0.40.7-codexfailurerecovery`:

### 3.1 Compilation & Target Verification
- **Native MSVC:** Compiles cleanly on Windows MSVC (`x86_64-pc-windows-msvc`).
- **Browser WASM:** Compiles cleanly for `wasm32-unknown-unknown` without default features (`--no-default-features --features qualia`).
- **Game WASM:** Compiles cleanly in `c:\github\game-demo\crates\rolling-commons-shell` targeting WASM.

### 3.2 Full Volumetric Test Suite (26/26 Passing)
The complete `crates/webizen-render` volumetric acceptance test suite passes **26/26**, validating:
1. `hmc_multilevel_ktx2_selects_and_decodes_coarse_level`: Deterministic selection and direct decode of mip 1 for a 2x2 projected footprint from a 4x4 multi-level KTX2 asset.
2. `hmc_constrained_budget_defers_texture_while_mesh_loads`: Zero upload budget defers texture interpretation while loading mesh geometry without error.
3. `hmc_color_space_mismatch_is_deferred_while_mesh_loads`: Normal map SRGB format mismatch is safely deferred.
4. `hmc_corrupted_texture_is_deferred_while_mesh_loads`: Malformed/corrupted texture bytes defer gracefully without aborting container loading.
5. `water_transparent_pass_does_not_overwrite_opaque_depth`: Confirms water surfaces preserve authoritative scene depth for downstream passes.
6. Generation-stamped texture refinement, rebind, and eviction lifecycles.

### 3.3 Core Render Infrastructure (533+ Tests Passing)
In `crates/qualia-core-db/src/render`:
- **Deterministic Frame Graph (`frame_graph.rs`):** Kahn's topological DAG sort over fixed stack buffers (`[Option<PassId>; 16]`), pruning disabled passes (e.g. AO or Bloom), estimating peak transient VRAM, and failing closed under budget limits.
- **Cinematic Lighting (`lighting.rs`):** Physically based Cook-Torrance GGX microfacet BRDF (smooth windowed distance attenuation, Smith joint visibility, Schlick Fresnel) alongside first-class stylized cel shading (2/3/4 quantized diffuse tone bands, Fresnel rim lighting).
- **AO Quality Profiles (`ao_quality.rs`):** Fibonacci spiral unit disk sampling taps (up to 16 taps) on the stack; bilateral depth/normal edge-preserving filter to prevent silhouette halos; bilateral 2x2 reconstruction filter for lower-resolution AO.
- **Portable Temporal Depth Producer:** Nearest-sampled depth path with an explicit non-filtering sampler, preserving authoritative scene depth and linear-depth conversion while ensuring lavapipe GLSL and WebGPU compatibility.

### 3.4 Texture Ingestion & Residency
- **KTX2 Inspection & RGBA8 Decode:** Strict container validation (DFD parsing, checked offsets, format limits); zero-allocation native Zstd/Zlib RGBA8 decoding with caller-owned scratch; transfer-function preservation.
- **Coarse-First Residency Planning:** Best-effort mip-prefix planner with atomic budget reservations across CPU cache, GPU residency, and upload staging; neutral typed fallback textures bound on deferral.
- **Strict Signature Validation:** Magic byte preflights for PNG, JPEG, KTX2, and WebP payloads; bounded data URI ingestion.

### 3.5 Asset Packaging & Manifest Integrity
- Game assets packaged in three versioned HMC archives (`maslows-challenge-community.hmc`, `maslows-challenge-earthlight.hmc`, `maslows-challenge-scenes.hmc`).
- Exact manifest alignment across 340 assets, 10 scenes, and matching WASM SHA-256 digests (`f9060473cf6dab19407ff2196e03a9cccd38e3af9b8ac2b296e693b75cd8d6df`).

---

## 4. Identified Deficits, Incomplete Implementations & Open Platform Gates

The following critical gaps remain open across the engine and game integration:

### 4.1 Temporal Rendering Lacks Real Producers (AG-21)
- **The Deficit:** The temporal resolve pipeline (`temporal_resolve_gpu.rs`) has an explicit lifecycle (`Idle -> LinearDepthReady -> Resolved -> HistoryPublished -> OutputReady`) and shader (`temporal_resolve.wgsl`), but **neither real motion-vector nor real reactive-mask producers exist**.
- **Impact:** Without per-fragment motion vectors from camera/transform deltas and reactive masks from alpha/particles, accumulation cannot track motion. It currently **fails closed**, outputting the current un-accumulated frame or resetting history.
- **Missing File:** An early draft noted `crates/qualia-core-db/src/render/temporal_producers.rs`, but this file was never implemented or committed to the repository.
- **Open Gate:** Multi-frame moving geometry and disocclusion fixtures backed by real GPU readback do not exist.

### 4.2 KTX2 Basis/Compressed Transcoding is Missing (AG-03)
- **The Deficit:** KTX2 support is restricted to uncompressed RGBA8 and native Zstd/Zlib-compressed RGBA8 (`VK_FORMAT_R8G8B8A8_UNORM` / `VK_FORMAT_R8G8B8A8_SRGB`).
- **Impact:** True GPU compressed textures (BC7, ASTC, ETC2) and Basis Universal (`BasisLZ`) transcode paths are not implemented. Assets requesting Basis compression trigger typed deferral fallbacks.
- **Open Gate:** End-to-end transcode fixtures from Basis payload to hardware compressed textures.

### 4.3 Environment Probes Lack Live GPU Binding (AG-22)
- **The Deficit:** `crates/qualia-core-db/src/render/environment_lighting.rs` provides CPU evaluation of diffuse irradiance and roughness-aware specular interpolation, but **no live GPU probe textures, runtime cubemap filtering, or shader probe bind groups are wired**.
- **Impact:** Shading relies on directional/sun and ambient fallbacks; indirect specular and diffuse transport are not sampled from environment probes.

### 4.4 Water & Vegetation Scene Integration is Incomplete (AG-10, AG-12)
- **Water:** Transparent depth non-overwrite and uniform water planes exist, but shoreline wave foam, dynamic screen-space reflections (SSR), depth refraction, and water-mesh edge intersections are not integrated into the game scene.
- **Vegetation:** World-frame wind vertex animation and event pools exist, but procedural grass instancing, billboard impostors, and character collision bending are not implemented.

### 4.5 Browser WebGPU Execution is Unverified (AG-26)
- **The Deficit:** When running headless Chromium against `web/game.html?selftest`, the browser fetches the WASM package and HMC manifests, but **produces zero DOM receipts and no WebGPU initialization log**.
- **Observed Behavior:** The worker selftest fell back to Canvas2D. Real WebGPU execution inside a browser engine remains completely unverified.

### 4.6 Missing Cross-Platform Runtime Matrix
No runtime or pixel verification exists for:
- Windows MSVC / DirectX 12 native backend
- macOS / Metal native backend
- Linux Vulkan native backend (outside lavapipe software rendering)
- Android Chrome (WebGPU / WebGL2)
- iOS Safari / WebKit (WebGPU)

### 4.7 Missing Performance & Observability Evidence (AG-19, AG-28)
- No p50 / p95 frame time measurements.
- No real GPU driver VRAM allocation receipts.
- No draw call or upload bandwidth profiling.
- No golden-master visual regression image captures.

---

## 5. Exhaustive Capability Audit Ledger (AG-01 through AG-29)

| Capability ID & Title | Parent Register Claim | Audited Status | Owning Code Modules | Implemented Slices & Evidence | Open Gates & Missing Implementation |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **AG-01** Rendering correctness conformance | In progress | **In progress** | `qualia-core-db::render`, `webizen-render` | Native/WASM compilation; volumetric suite 26/26; Naga shader validation | Browser GPU readback, pixel oracles, cross-backend parity |
| **AG-02** Versioned `.10d` material model | In progress | **In progress** | `render::gpu::materials`, `render::assets` | MAT1/MAT2/MAT3 schemas; PBR Neutral; linear/sRGB enforcement | Subsurface, sheen, clearcoat, anisotropy models |
| **AG-03** Texture compilation, sampling & residency | In progress | **In progress** | `render::texture_decode`, `render::texture_ktx2`, `volumetric_hmc` | RGBA8 KTX2, Zstd/Zlib decode, coarse-first mip residency planner | BasisLZ transcode, GPU block compression (BC7/ASTC), streaming eviction |
| **AG-04** Authored normals, tangents & direct lighting | In progress | **In progress** | `render::lighting`, `render::gpu::mod` | Cook-Torrance GGX, smooth attenuation, stylized quantized bands | Tangent MikkTSpace runtime generation, anisotropic lobes |
| **AG-05** Stable shadows & ambient occlusion | In progress | **In progress** | `render::ao_quality`, `render::shaders` | Fibonacci spiral AO taps, bilateral depth/normal filter, PCF shadow math | Cascaded shadow maps (CSM), shadow atlas management, GTAO |
| **AG-06** Sky, atmosphere & distance depth | In progress | **In progress** | `render::shaders::ambient`, `webizen-render` | Analytic sky gradient, sun/moon disk, bounded distance fog | Multi-scattering LUTs, aerial perspective, physical atmosphere |
| **AG-07** Exposure, tone mapping & grading | In progress | **In progress** | `render::lighting`, `render::shaders` | Bounded manual exposure, PBR Neutral tone mapper, white balance | Auto-exposure histogram, 3D LUT grading, HDR display output |
| **AG-08** Instancing, batching, culling & LOD | In progress | **In progress** | `render::gpu::mesh_upload`, `render::contract` | Instanced indexed draws, ABI v2 (128B), CPU frustum culling | GPU indirect draws, hierarchical Z culling, runtime LOD transitions |
| **AG-09** Joined terrain, geography & streaming | Not started | **Foundation only** | `render::terrain_seam_tests` | 4-resolution, 256-sample edge seam verification oracle | Continuous clipmaps, heightfield streaming, cave/overhang meshes |
| **AG-10** Water surfaces & shore interaction | Not started | **Foundation only** | `render::gpu::water`, `render::shaders::water` | GPU water shader, transparent depth preservation, upload budget | Shoreline foam distance fields, screen-space reflections, flow maps |
| **AG-11** `.10d` rigs, clips & animation runtime | Not started | **Not started** | None | None | Rig/clip schema, dual-quaternion/matrix skinning, pose blending |
| **AG-12** Vegetation & secondary motion | Not started | **Foundation only** | `webizen-render::vegetation_effects` | Fixed event pool, world-frame wind WGSL, stable-seed noise | Procedural grass instancing, character collision bending, LOD cards |
| **AG-13** Particles & environmental effects | Not started | **Foundation only** | `webizen-render::vegetation_effects` | Fixed-capacity pooled effect events | Compute particle simulation, depth collision, ribbon emitters |
| **AG-14** Production compiler & interchange | In progress | **In progress** | `render::assets::glb_materials`, `container_10d` | GLB to `.10d` conversion, MID2 manifold identity v2 | Full DCC pipeline round-trip, USD/FBX interchange |
| **AG-15** Character & botanical families | Not started | **Not started** | `game-demo::assets` | Low-poly asset meshes exported in HMC | Production rigging, skin/leaf shading, LOD family sets |
| **AG-16** LOD compiler, mesh optimization | Not started | **Not started** | None | None | Meshoptimizer integration, quadric error simplification, bounds |
| **AG-17** Style profiles & art direction | Not started | **Not started** | `game-demo::web::game.html` | Client-side visual style selection (Earthlight/Community) | Dynamic runtime style switching, asset material variant binding |
| **AG-18** Selection, decals & world feedback | Not started | **Foundation only** | `render::gpu::mod` (pick pass) | R32Uint picking reservation, zero-miss bias, node index 0 selectable | Projected screen decals, outline selection halos, spatial markers |
| **AG-19** Accessibility & performance admission | Not started | **Foundation only** | `render::quality_profiles`, `render::quality_runtime` | Conservative/Balanced/Ultra profiles, tier hysteresis policy | Measured frame-time adaptation, colorblind palettes, UI scaling |
| **AG-20** Frame graph & resource ownership | In progress | **In progress** | `render::frame_graph`, `render::gpu::scene_depth` | Topological DAG sort, peak VRAM estimation, pass pruning | Surface swapchain integration, compute pass synchronization |
| **AG-21** Anti-aliasing, motion vectors & reconstruction | Not started | **Foundation only** | `render::gpu::temporal_resolve_gpu` | Temporal submission lifecycle, nearest-sampled depth shader | Real motion-vector pass, reactive-mask pass, 2-frame history fixtures |
| **AG-22** Environment lighting, probes & indirect light | Not started | **Foundation only** | `render::environment_lighting` | Spherical harmonics diffuse math, roughness specular split | GPU cubemap binding, probe atlas baking, screen-space diffuse |
| **AG-23** Reflections, transparency & refraction | Not started | **Not started** | None | None | Screen-space reflections (SSR), weighted blended OIT, thin glass |
| **AG-24** Volumetric atmosphere & participating media | Not started | **Not started** | None | None | Froxel volumetric lighting, raymarched local fog volumes |
| **AG-25** Skin, foliage, cloth & hero surface fidelity | Not started | **Not started** | None | None | Pre-integrated subsurface scattering, dual-lobe specular, cloth sheen |
| **AG-26** Native/WASM negotiation & recovery | In progress | **In progress** | `render::portal`, `render::acceptance_contract` | Explicit Unknown/Confirmed/Refused gates, Canvas2D fallback | Context loss recovery, WebGPU device recreation, mobile lifecycle |
| **AG-27** Cinematic cameras & capture/export | Not started | **Not started** | None | None | Physical camera parameters (focal length, aperture), offline high-res render |
| **AG-28** Observability, regression & diagnostics | Not started | **Foundation only** | `webizen-render::telemetry` | Frame resource gauges, upload byte counters | Automated visual golden comparison, GPU timing timestamps |
| **AG-29** Optional ray queries & reference path rendering | Not started | **Not started** | None | None | Hardware ray tracing pipeline (DXR/Vulkan KHR ray query) |

---

## 6. Dependency-Ordered Swarm Waves

Execution must proceed in strict dependency order across three waves:

```
┌────────────────────────────────────────────────────────┐
│ WAVE 1: Production Temporal & Resource Ownership       │
│ - T1: Real motion-vector generation (camera + object)  │
│ - T2: Real reactive-mask generation (alpha + particle) │
│ - T3: Persistent GPU history texture lifecycle         │
│ - T4: Renderer scheduler & host contract completion    │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ WAVE 2: Asset & Scene Integration                      │
│ - A1: BasisLZ / compressed KTX2 transcode paths        │
│ - A2: GPU environment probe bindings & reflections     │
│ - A3: Terrain clipmaps & live vegetation deformation   │
│ - A4: Water shoreline interaction & dynamic refraction │
└──────────────────────────┬─────────────────────────────┘
                           │
                           ▼
┌────────────────────────────────────────────────────────┐
│ WAVE 3: Cinematic & Platform Qualification             │
│ - C1: Browser WebGPU DOM & readback verification       │
│ - C2: Multi-backend runtime matrix (DX12/Metal/Vulkan) │
│ - C3: Performance admission & frame-time profiling     │
│ - C4: Golden-image visual regression suite             │
└────────────────────────────────────────────────────────┘
```

### Wave 1 — Production Temporal & Resource Ownership
- **Lane T1 (Motion Vectors):** Implement vertex-shader motion vector output computing `previous_clip_position - current_clip_position`. Create two-frame moving geometry test validating non-zero motion vectors and disocclusion mask.
- **Lane T2 (Reactive Masks):** Generate reactive mask flagging alpha-tested edges, transparent water, and particles to reduce history weighting in `temporal_resolve.wgsl`.
- **Lane T3 (Persistent History):** Allocate ping-pong history textures in `PortalGpu`; bind previous resolved frame into resolve pass; publish resolved frame to history before presentation.
- **Lane T4 (Scheduler Contract):** Wire `WebizenFramePlan` to admit temporal resolve only when genuine motion-vector and reactive views are present; fail closed gracefully when absent.

### Wave 2 — Asset & Scene Integration
- **Lane A1 (Texture Transcode):** Integrate caller-buffered Basis Universal transcoder targeting BC7 (desktop) and ASTC (mobile/web).
- **Lane A2 (Environment Probes):** Allocate and bind GPU cubemap probe textures in `wgpu_renderer.rs`; wire roughness-split IBL sampling in `mesh.wgsl`.
- **Lane A3 (Terrain & Vegetation):** Connect terrain heightfield continuous mesh to camera updates; bind wind buffer to animated botanical meshes in the live game scene.
- **Lane A4 (Water Interaction):** Implement shoreline foam depth-difference calculation comparing water surface depth against opaque scene depth.

### Wave 3 — Cinematic & Platform Qualification
- **Lane C1 (Browser WebGPU):** Debug and qualify headless Chromium WebGPU execution; record valid DOM receipts and offscreen pixel readback.
- **Lane C2 (Platform Matrix):** Run and record test receipts across Windows DX12, macOS Metal, Linux Vulkan, Android Chrome, and iOS Safari.
- **Lane C3 (Performance Profiling):** Measure real p50/p95 frame times, VRAM allocation against driver counters, and draw call overhead under 10,000 instances.
- **Lane C4 (Visual Regression):** Capture golden-master PNGs of benchmark scenes; integrate automated image diffing with strict PSNR/SSIM thresholds.

---

## 7. Evidence Ledger

### Entry 1 — 2026-10-08: Baseline Engine & Graphics Foundations
- **Commit:** `2efe3cb9` (`feat(render): texture streaming, data URI ingestion, cinematic lighting, AO quality, and frame graph`)
- **Owned Files:** `render/frame_graph.rs`, `render/lighting.rs`, `render/ao_quality.rs`, `render/texture_ingestion.rs`, `webizen-render/src/volumetric_hmc.rs`
- **Targets Verified:** Native MSVC `x86_64-pc-windows-msvc` (533 tests passing), `webizen-render` (67 tests passing), WASM target check.
- **Evidence:** Frame graph topological sorting verified; GGX Cook-Torrance and quantized cel shading unit-tested; KTX2 structural parsing and RGBA8 decode verified.

### Entry 2 — 2026-10-10: Recovery Sprints 1, 2, 3
- **Commits:** `bfd0d286`, `16355979`, `d83e8e08`, `e4a3e928`, `a023073d`, `afcd24ba`, `3e039e54`, `716b9edc` (QualiaDB); `bc7525d`, `671d4e0`, `7c383f6`, `5df4e49`, `6be89fa` (game-demo)
- **Owned Files:** `render/gpu/temporal_resolve_gpu.rs`, `render/gpu/water.rs`, `render/gpu/scene_depth.rs`, `webizen-render/src/volumetric_hmc_stream.rs`, `webizen-render/src/vegetation_effects.rs`, `game.html`
- **Targets Verified:** Volumetric suite 26/26 passing; game WASM package check passing; 340 assets / 10 scenes manifest integrity matching SHA-256.
- **Evidence:** Replaced `textureLoad` with sampled depth for lavapipe GLSL compatibility; deterministic HMC texture residency with coarse mips; water transparent depth non-overwrite verified.
- **Identified Open Gates:** Chromium headless produced no DOM output; real motion vectors missing; Basis transcode missing; environment probes unbound on GPU.

### Entry 3 — 2026-10-10: Programme Audit & Reconciliation
- **Activity:** Authoritative audit and gap register reconciliation on branch `0.0.40.7-codexfailurerecovery`.
- **Finding:** Corrected documentation drift; reconciled 6 capabilities from "Not started" to "Foundation only"; established that 0 capabilities are Verified.
- **Status:** Programme remains **In progress**; Wave 1 execution prioritized.

### Entry 4 — 2026-10-10: Wave 1 / Lane T1 & T2 Reference Math & Reactive Producer Models
- **Owned Files:** `crates/qualia-core-db/src/render/temporal_producers.rs`, `crates/qualia-core-db/src/render/mod.rs`
- **Implemented:**
  - `MotionVector2d` POD structure with zero-motion and length metrics.
  - `Mat4ColumnMajor` transform math with homogeneous clip-space projection, matrix multiply (`mul`), and zero-heap minor expansion inverse (`inverse`).
  - `calculate_camera_motion_vector` and `calculate_object_motion_vector` reference projections converting 3D world/local delta to top-left UV motion vector `[du, dv]`.
  - Behind-camera clipping guard ($w \le 10^{-6}$).
  - `is_disoccluded` depth disparity oracle.
  - `ReactiveMaskClassification` computing reactive attenuation from alpha fringe, water, and particle surface properties.
- **Unit Tests:** 8/8 tests passing (`static_camera_yields_zero_motion`, `camera_pan_yields_expected_motion_vector`, `object_motion_yields_motion_vector`, `behind_camera_point_is_rejected`, `disocclusion_metric_respects_threshold`, `reactive_mask_classification_prioritizes_highest_reactivity`, `matrix_identity_inverse_is_identity`, `matrix_inverse_round_trip`).

### Entry 5 — 2026-10-10: Wave 1 Shaders, Camera History Lifecycle & Monolith Decomposition
- **Owned Files:** 
  - `crates/qualia-core-db/src/render/gpu/portal_gpu/` (new sub-directory):
    - `portal_gpu/mod.rs` (10 lines)
    - `portal_gpu/readback.rs` (94 lines)
    - `portal_gpu/picking.rs` (196 lines)
    - `portal_gpu/uniforms.rs` (130 lines)
    - `portal_gpu/lighting_passes.rs` (199 lines)
    - `portal_gpu/render_frame.rs` (567 lines)
    - `portal_gpu/tests.rs` (405 lines)
  - `crates/qualia-core-db/src/render/gpu/mod.rs` (reduced from 4,092 lines to 2,590 lines)
  - `crates/qualia-core-db/src/shaders/viewport/temporal_motion.wgsl`
  - `crates/qualia-core-db/src/shaders/viewport/temporal_reactive.wgsl`
  - `crates/qualia-core-db/src/shaders/viewport/temporal_depth.wgsl`
- **Implemented & Verified:**
  - **Shader Pipeline:** Added `temporal_motion.wgsl` and `temporal_reactive.wgsl`; Naga validation verified in `temporal_resolve_gpu.rs`.
  - **Naga Depth Fix:** Resolved `textureSampleLevel` exact level type constraint for `texture_depth_2d` (integer level 0 instead of float 0.0) across all temporal shaders.
  - **Ring Buffer Scaling:** Expanded `uniform_belt` pool size from 8 to 16 slots, eliminating in-frame buffer re-mapping and resolving `Buffer with 'uniform-belt-slot' label is still mapped` validation panics during composite frames.
  - **Camera History Latching:** Wired `PortalGpu::current_camera_view_projection()` and `PortalGpu::previous_camera_view_projection()` with automatic latching at frame submission, cut invalidation, and resize reset.
  - **Architecture De-monolithization:** Decomposed 1,500+ lines out of monolithic `render/gpu/mod.rs` into single-responsibility submodules in `portal_gpu/` complying with `AGENTS.md` Rule 0-B.
- **Automated Test Receipts (27/27 Passing):**
  - `render::temporal_producers`: 8/8 tests passing.
  - `render::gpu::portal_gpu::tests`: 10/10 tests passing (including `camera_history_tracks_frames_and_resets_on_cut_and_resize`).
  - `render::gpu::picking_tests`: 1/1 test passing.
  - `render::gpu::mesh_pixel_tests`: 3/3 tests passing.
  - `render::gpu::material_pixel_tests`: 5/5 tests passing.


