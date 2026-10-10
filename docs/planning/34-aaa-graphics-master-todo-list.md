# AAA Graphics Capability: Master Implementation To-Do List & Gap Analysis
_Branch: `0.0.40.7-codexfailurerecovery` | Updated: 2026-10-10_

This document reconciles:
- [`31-aaa-graphics-engine-gap-register.md`](file:///c:/github/game-demo/docs/planning/31-aaa-graphics-engine-gap-register.md)
- [`32-aaa-graphics-swarm-implementation-work-order.md`](file:///c:/github/game-demo/docs/planning/32-aaa-graphics-swarm-implementation-work-order.md)
- [`33-aaa-graphics-completion-tracker.md`](file:///c:/github/game-demo/docs/planning/33-aaa-graphics-completion-tracker.md)

---

## 1. Executive Summary & Reconciliation

| Capability Area | Register Status | Current Engine Status | Primary Open Gap |
|:---|:---:|:---:|:---|
| **AG-21 Anti-Aliasing & Temporal Reconstruction** | Foundation Only | Math & WGSL Shaders Implemented | Real motion-vector & reactive-mask texture pass in `PortalGpu` |
| **AG-22 Environment Lighting & Probes** | Foundation Only | CPU Model & WGSL Structs Implemented | GPU binding update API & offscreen pixel oracle test |
| **AG-03 Texture Package & Streaming (KTX2)** | In Progress | RGBA8 & Zstd/Zlib Decompressor Tested | BasisLZ / Block (BC7/ASTC) transcoding bridge |
| **AG-10 Water & Shore Interaction** | Foundation Only | Finite Mesh & Transparent Depth Tested | Shoreline depth-difference foam & refraction |
| **AG-12 Vegetation & Secondary Motion** | Foundation Only | Wind Buffer & Vertex Wave Tested | Live actor deformation & foliage instancing |
| **AG-26 Native/WASM & Browser Qualification** | In Progress | Native 95/95 Tests, WASM Library Checks Pass | Headless Chromium WebGPU DOM & render receipt |
| **AG-19/20 Quality Admission & Performance Telemetry**| Foundation Only | Fixed Uniforms & FrameGraph Implemented | Frame-time profiling, draw-call counter, golden image tests |

---

## 2. Master Work Order: Ranked Implementation Waves

### Wave 1: Temporal Reconstruction & Live Producer Integration (AG-21)
- [ ] **T1.1: Motion Vector Render Target & Pass:**
  - Allocate `R16G16_FLOAT` / `R16G16_SNORM` motion target in `PortalGpu`.
  - Wire `temporal_motion.wgsl` with camera previous-vs-current projection matrix uniform.
  - Draw static and moving geometry to output screen-space `[du, dv]` per-pixel motion vectors.
- [ ] **T1.2: Reactive Mask Render Target & Pass:**
  - Allocate `R8_UNORM` reactive mask target in `PortalGpu`.
  - Wire `temporal_reactive.wgsl` with alpha-tested geometry, particle instances, and water surfaces.
  - Output normalized reactivity values `[0, 1]` to attenuate history blending.
- [ ] **T1.3: Feed Live Producers into Temporal Resolve:**
  - Connect real `motion_vectors` and `reactive_mask` texture views to `TemporalResolveInputs`.
  - Enable history accumulation in `temporal_resolve.wgsl` without failing closed.
- [ ] **T1.4: Multi-Frame Motion & Disocclusion Test:**
  - Construct an offscreen test with a translating mesh across 2 frames.
  - Assert that motion vectors match expected delta and that moving edges accumulate smoothly.

---

### Wave 2: Environment Probes & Shader Pixel Qualification (AG-22)
- [ ] **E2.1: `PortalGpu::set_environment_probes` API:**
  - Add setter on `PortalGpu` accepting `&EnvironmentProbeSet`.
  - Pack into `EnvironmentLightingGpu` (544 bytes) and copy via uniform belt to `@group(2) @binding(13)`.
- [ ] **E2.2: Live Shader Binding Verification:**
  - Ensure material bind groups in `materials.rs` bind the active environment uniform buffer.
- [ ] **E2.3: Adapter-Backed Pixel Oracle Test:**
  - Render offscreen sphere with zero directional light and pure blue probe irradiance.
  - Read back pixels and assert non-zero blue channel confirming probe illumination.

---

### Wave 3: KTX2 Basis Universal & Block Transcoding Bridge (AG-03)
- [ ] **K3.1: Lightweight Zero-Heap / Bounded Transcoder Adapter:**
  - Implement a bounded transcoder module in `crates/qualia-core-db/src/render/texture_transcode/`.
  - Support transcoding BasisLZ / UASTC payloads to RGBA8 (portable fallback) and BC7 (native desktop).
- [ ] **K3.2: Wire Transcode Request in `texture_decode.rs`:**
  - Convert `TextureDecodePlan::RequiresTranscode` into an executed caller-buffered decode pass.
- [ ] **K3.3: Container Transcode Test Suite:**
  - Test round-trip transcoding of a synthetic BasisLZ fixture into valid RGBA8 texels.

---

### Wave 4: Water Shoreline & Vegetation Scene Uplift (AG-10, AG-12)
- [ ] **W4.1: Shoreline Foam Depth-Difference:**
  - In `water.wgsl`, sample opaque scene depth and compare with current fragment depth.
  - Generate soft white foam band where depth disparity is within shoreline threshold.
- [ ] **W4.2: Vegetation Collision & Foliage Instancing:**
  - Extend `vegetation_effects.rs` with character cylinder collision displacement.
  - Implement grass instance batching using ABI v2 instance buffer.

---

### Wave 5: Platform & Browser Qualification (AG-26, AG-19)
- [ ] **B5.1: Headless Chromium WebGPU Execution:**
  - Run headless Chrome with `--enable-unsafe-webgpu` and `--use-angle=vulkan` on `web/game.html?selftest`.
  - Capture DOM test receipts and confirm `TEST WebGPU runtime execution: PASS`.
- [ ] **B5.2: Multi-Platform Qualification Matrix:**
  - Capture compilation and execution receipts for Windows DX12/Vulkan and WASM WebGPU/WebGL2.
- [ ] **B5.3: Performance & Telemetry Harness:**
  - Add frame-time microsecond timer, draw-call counter, and VRAM memory report to `SystemTelemetry`.
  - Measure steady-state frame times and draw counts under 1,000+ instances.
- [ ] **B5.4: Golden-Image Visual Regression Suite:**
  - Export reference PNG frames of offscreen scenes; assert PSNR > 40 dB across builds.
