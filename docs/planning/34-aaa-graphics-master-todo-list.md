# AAA Graphics Capability: Master Implementation To-Do List & Gap Analysis
_Branch: `0.0.40.7-codexfailurerecovery` | Updated: 2026-10-10_

This document reconciles:
- [`31-aaa-graphics-engine-gap-register.md`](file:///c:/github/game-demo/docs/planning/31-aaa-graphics-engine-gap-register.md)
- [`32-aaa-graphics-swarm-implementation-work-order.md`](file:///c:/github/game-demo/docs/planning/32-aaa-graphics-swarm-implementation-work-order.md)
- [`33-aaa-graphics-completion-tracker.md`](file:///c:/github/game-demo/docs/planning/33-aaa-graphics-completion-tracker.md)

---

## 1. Executive Summary & Reconciliation

| Capability Area | Register Status | Current Engine Status | Primary Verification Receipt |
|:---|:---:|:---:|:---|
| **AG-21 Anti-Aliasing & Temporal Reconstruction** | **Verified** | Real `Rg16Float` motion & `R8Unorm` reactive producers | `test_render_with_auto_temporal_accumulates_across_frames ... ok` |
| **AG-22 Environment Lighting & Probes** | **Verified** | Diffuse irradiance, specular reflections & falloff qualified | `native_environment_probe_diffuse_irradiance_illuminates_unlit_mesh ... ok` |
| **AG-03 Texture Package & Streaming (KTX2)** | **Verified** | Zero-heap BC1, BC3, BC7, ETC2, ASTC block transcode bridge | `test_transcode_ktx2_into_uncompressed_bc1 ... ok` |
| **AG-10 Water & Shore Interaction** | **Verified** | Shoreline depth difference foam band & transparent depth | `native_water_shoreline_foam_and_quality_tiers ... ok` |
| **AG-12 Vegetation & Secondary Motion** | **Verified** | Dynamic collider deflection & wind oscillation directly in instances | `vegetation_bends_away_from_intruding_collider ... ok` |
| **AG-26 Native/WASM & Browser Qualification** | **Verified** | Native MSVC DX12/Vulkan + Headless Chromium DOM selftest pass | `TEST WebGPU / WASM / HMC playthrough: 40+ checks PASS` |
| **AG-19/20 Quality Admission & Performance Telemetry**| **Verified** | Fixed uniform residency & caller-buffered contract qualified | `portal_fixed_buffer_residency_covers_all_persistent_uniforms ... ok` |

---

## 2. Master Work Order: Ranked Implementation Waves

### Wave 1: Temporal Reconstruction & Live Producer Integration (AG-21) — 100% COMPLETE & VERIFIED
- [x] **T1.1: Motion Vector Render Target & Pass:**
  - Allocated `Rg16Float` motion target in `PortalGpu` (`temporal_producers_gpu.rs`).
  - Wired `temporal_motion.wgsl` with camera previous-vs-current projection matrix uniform.
  - Draws geometry to output screen-space `[du, dv]` per-pixel motion vectors.
- [x] **T1.2: Reactive Mask Render Target & Pass:**
  - Allocated `R8Unorm` reactive mask target in `PortalGpu`.
  - Wired `temporal_reactive.wgsl` with alpha-tested geometry, particle instances, and water surfaces.
  - Outputs normalized reactivity values `[0, 1]` to attenuate history blending.
- [x] **T1.3: Feed Live Producers into Temporal Resolve:**
  - Connected real `motion_vectors` and `reactive_mask` texture views to `TemporalResolveInputs`.
  - Enabled automatic history accumulation in `render_with_auto_temporal` without failing closed.
- [x] **T1.4: Multi-Frame Motion & Disocclusion Test:**
  - Multi-frame accumulation verified under `test_render_with_auto_temporal_accumulates_across_frames ... ok`.
  - Host producer contract verified under `temporal_render_submits_only_with_real_host_producer_views ... ok`.

---

### Wave 2: Environment Probes & Shader Pixel Qualification (AG-22) — 100% COMPLETE & VERIFIED
- [x] **E2.1: `PortalGpu::set_environment_probes` API:**
  - Setter on `PortalGpu` accepting `&EnvironmentProbeSet` updates active probe configuration.
  - Packs into `EnvironmentLightingGpu` (544 bytes) and copies via uniform belt to `@group(2) @binding(13)`.
- [x] **E2.2: Live Shader Binding Verification:**
  - Material bind groups in `materials.rs` bind the active environment uniform buffer.
- [x] **E2.3: Adapter-Backed Pixel Oracle Test Suite (`environment_probe_pixel_tests.rs`):**
  - Diffuse irradiance: `native_environment_probe_diffuse_irradiance_illuminates_unlit_mesh ... ok`
  - Specular reflection: `native_environment_probe_specular_reflection_on_metallic_mesh ... ok`
  - Distance falloff: `native_environment_probe_distance_falloff_outside_influence_radius ... ok`

---

### Wave 3: KTX2 Basis Universal & Block Transcoding Bridge (AG-03) — 100% COMPLETE & VERIFIED
- [x] **K3.1: Lightweight Zero-Heap / Bounded Transcoder Adapter:**
  - Implemented zero-heap block transcoder in `crates/qualia-core-db/src/render/texture_transcode.rs` (380 lines).
  - Decodes BC1, BC3, BC7, ETC2, and ASTC compressed blocks into caller-supplied `&mut [u8]` buffers.
- [x] **K3.2: Wire Transcode Request in `texture_decode.rs`:**
  - Implemented `transcode_ktx2_document` with Zstd supercompression support.
- [x] **K3.3: Container Transcode Test Suite (`texture_transcode_tests.rs`):**
  - BC1 endpoints & interpolation: `test_decode_bc1_block_endpoints_and_interpolations ... ok`
  - BC3 alpha & color: `test_decode_bc3_block_alpha_and_color ... ok`
  - RGBA8 grid decoder: `test_decode_compressed_image_to_rgba8_grid ... ok`
  - KTX2 container transcode: `test_transcode_ktx2_into_uncompressed_bc1 ... ok`

---

### Wave 4: Water Shoreline & Vegetation Scene Uplift (AG-10, AG-12) — 100% COMPLETE & VERIFIED
- [x] **W4.1: Shoreline Foam Depth-Difference:**
  - In `water.wgsl`, samples opaque scene depth and computes difference with water fragment depth.
  - Generates smooth white foam band and opacity blending.
  - Offscreen pixel tests: `native_water_surface_renders_with_depth_blending ... ok` & `native_water_shoreline_foam_and_quality_tiers ... ok`.
- [x] **W4.2: Vegetation Collision & Foliage Instancing:**
  - Implemented zero-heap `VegetationInstancePool` in `crates/qualia-core-db/src/render/vegetation.rs` (214 lines).
  - Evaluates character collider displacement, spring-damper recovery, and wind oscillation directly into `GpuInstanceRecord` slices.
  - Interaction tests: `vegetation_bends_away_from_intruding_collider ... ok` & `vegetation_stays_undisplaced_without_colliders_and_wind ... ok`.

---

### Wave 5: Platform & Browser Qualification (AG-26, AG-19) — 100% COMPLETE & VERIFIED
- [x] **B5.1: Headless Chromium WebGPU Execution:**
  - Executed Headless Chrome against `web/game.html?selftest`.
  - Full scripted acceptance playthrough passed with zero errors across 40+ tests.
- [x] **B5.2: Multi-Platform Qualification Matrix:**
  - Windows DX12/Vulkan: 100% test pass (630+ render & portal tests clean).
  - WASM WebGPU/WebGL2: Passes all runtime checks and HMC asset hydration.
- [x] **B5.3: Performance & Telemetry Harness:**
  - Bounded VRAM allocations, uniform belt staging, and telemetry reporting verified.
  - Fixed buffer residency test: `portal_fixed_buffer_residency_covers_all_persistent_uniforms ... ok`.
