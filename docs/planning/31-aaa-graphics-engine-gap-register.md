# AAA and cinema-grade graphics: engine specification and gap register

Status: **proposed requirements and implementation guidance**, revised **2026-10-07**.
Owner: QualiaDB/Webizen engine programme; game integration: Maslows Challenge.
No capability is marked complete by this document. Research recommendations are
implementation candidates with mandatory qualification gates, not measured Qualia results.

The target is a coherent, cinematic world with polished surfaces, lighting,
characters, movement and composition at both strategic and Walk distances.
Support **stylized animated-film and photorealistic rendering**, with the stylized
vertical slice first. Both use the same engine, asset contracts and semantic world.
Support **native QualiaDB/Webizen and browser WASM runtimes**, including content
delivered through WASM and processed by local Qualia pipelines. Quality scales with
device capabilities and measured budgets; unsupported enhancements have explicit fallbacks.

The art direction remains [doc 22](22-animated-film-visual-direction.md): original
Storybook, Earthlight and Community Grounds styles, the
[community-grounds concept](https://dev.civics.au/assets/communitygrounds.jpg),
and the three proposed reference views. Film references describe finish and visual
language; they do not supply production assets. The reference views still need
approved captures and manifests before they can become visual acceptance baselines.

## 0. Implementation tracker

Updated **2026-10-07**. This programme is in active implementation; the register is
not a claim that the full graphics stack is complete. Existing engine code and
uncommitted work are treated as candidates until their capability-specific gates
pass. Status means: **In progress** = code or tests are actively being changed;
**Not started** = no qualifying implementation slice recorded here; **Verified**
requires the complete stated acceptance evidence, not a compile or one native test.

| Capability | Status | Recorded evidence / next gate |
| --- | --- | --- |
| AG-01 Rendering correctness | In progress | Native GLES offscreen occlusion/non-additive and explicit-zero-light pixel fixtures pass. The no-GPU CPU picking fallback now mirrors the projector WGSL PGA transform (including camera-dependent bilateral pull), orbit view-projection, temporal cut, clip volume, alpha-scaled point footprint and strict nearest-depth selection; zero-width temporal slices now have defined edge behavior in WGSL. Four regressions cover overlap/depth, pixel footprint, behind/near-camera and temporal rejection, invalid extents/pointers. Native and WASM renderer test-target checks compile these paths; the four focused native tests pass (4/4), and the projector WGSL Naga parser test passes (1/1). The native GLES offscreen empty-viewport fixture now passes with adapter execution required: it remains opaque black through 32×24, 65×31 and 1×1 resize, and a zero-width resize request safely retains a 1×1 target. The native adapter-required GLES GPU/CPU differential now passes against the production one-pixel R32Uint pick readback for the nearer overlapping node and a no-hit texel; temporally hidden and behind-camera nodes are included as rejection cases (1/1). The adapter-required native GLES CPU pixel oracle also passes: it independently classifies robust interior pixels for clear, far-blue and near-red surfaces using barycentric coverage and interpolated depth (1/1). The independent geometry/depth pixel oracle also passes on the explicitly selected native DX12 backend with adapter execution required (1/1). Next: real browser readback, broader material/AO/shadow scene oracles, and measured performance qualification. |
| AG-02 Versioned `.10d` material model | In progress | MAT3 v3 has 448-byte records with six SHA-256 references, six per-map UV0 transforms, six per-map glTF sampler states and contiguous triangle ranges; MAT1 v1 and MAT2 v2 remain readable with their historical defaults. GLB import preserves material factors, embedded image digests/payloads and primitive bindings; compilation attaches MAT3. Portal and Webizen draw opaque and alpha-mask per-submesh materials through a padded dynamic-uniform buffer, merging adjacent ranges and capping draws at 65,536. WGSL samples base colour/alpha, emissive, tangent-space normal, metallic-roughness (G=roughness, B=metallic), AO and stylized ramp textures. Alpha MASK coverage discards below the authored cutoff in forward shading, shadow casting and AO depth/normal prepass; surviving Opaque/Mask fragments write full alpha. Alpha BLEND now preserves material/eligible vertex/base-texture alpha, uses a depth-tested read-only-depth pipeline, and sorts submeshes back-to-front each frame from cold-computed bounds with an in-place allocation-free index sort; blended geometry is excluded from opaque AO/shadow depth inputs. Direct lighting uses GGX alongside the stylized three-band path; the vertical slice remains stylized first. HMC loading verifies bundle, `.10d` and texture digests, bounded-decodes PNG/JPEG, uploads sRGB/linear variants and rolls back new residencies on failure. Native and WASM SDK test-target checks pass; Naga validates the current mesh, AO and shadow WGSL. Adapter-required native GLES and DX12 pixel fixtures verify base-colour texture residency/binding, UV0 sRGB interpretation, and alpha-mask cutoff discard versus surviving opaque coverage (1/1 per backend). The shadow comparison confirms at least 12 additional receiver red-channel levels through a cutout versus the solid reference; the AO packed-surface comparison confirms the rear receiver depth survives behind a cutout by more than 100 packed depth codes. WASM test-target checking passes; browser GPU execution remains open. Per-map pixel qualification, semantic picking coverage parity, transparent ordering/compositing pixel qualification (native runtime is toolchain-blocked here), intersecting-surface limitations and weighted OIT, KTX2/Basis/WebP transcode, URI/data-URI loading, per-map sampler pixel qualification, alternate UV sets and environment/probe lighting remain open. |
| AG-03 Texture compilation and residency | In progress | GLB import preserves embedded PNG/JPEG/WebP/KTX2 as SHA-256-deduplicated dependencies; MAT1/MAT2/MAT3 HMC bind those digests outside the 42 MiB semantic arena. Shared caller-buffered PNG/JPEG decode enforces encoded-byte, dimensions, decoded-byte, output and PNG scratch limits. HMC loading verifies `.10d` and image dependencies, reads versioned MAT1/MAT2/MAT3 texture roles, decodes/uploads RGBA8 in required sRGB/linear interpretations, and removes newly resident images if scene loading fails. GPU residency validates adapter dimensions, deduplicates by digest, colour-space interpretation, mip semantic and (for alpha masks) canonical alphaCutoff, reserves full-chain bytes as TextureResidency and path-appropriate UploadStaging (base-level plus bounded alpha scratch for runtime generation, full chain for supplied mips), generates GPU render-pass mips when no precomputed chain is supplied, or validates and directly uploads complete caller-provided RGBA8 mip chains; generated mips use sRGB-linear colour averaging, linear-data averaging or normal-vector renormalization, and applies deterministic per-mip alpha-coverage correction computed from bounded alpha-only scratch; the upload API can return the planned base coverage and fixed-size per-level signed residual array by value, without retaining per-texture diagnostic state; these are CPU predictions, not GPU readback measurements. HMC loading and material binding select the same cutoff-keyed chain, so shared source images with different mask thresholds do not alias; staging releases after submission and explicit eviction releases residency. TEXCOORD_0 survives MikkTSpace splitting, round-trips through CRC-protected UV01, uploads as a GPU vertex stream, and is sampled for all six material roles. Native core and renderer plus WASM renderer test-target checks pass; Naga validates current material and mip shaders. A native GLES test verifies resident upload/binding and UV0-selected base-colour sampling with sRGB decode (1/1). The standalone production alpha-coverage-plan tests pass 4/4, including signed residual checks. The adapter-required native GLES minified alpha-mask fixture now passes and directly reads back GPU-generated mip levels 1–3; a measured 203/255 threshold-rounding failure was fixed with CPU/GPU-synchronized UNORM headroom. Native test-target checking and WASM renderer test-target checking pass. Decoder/HMC rollback assertions, other texture-role pixel fixtures, broader cutoff/NPOT coverage and browser GPU qualification remain open. KTX2/Basis and WebP transcode, URI/data-URI loading, HMC ingestion/transcoding for precomputed KTX2/Basis mip chains, coarse-to-fine streaming, sampler pixel qualification, alternate UV sets, semantic picking parity and BLEND, compressed adapter formats and visual fallback remain open. Runtime alpha coverage uses cutoff-keyed residency and GPU mip generation with CPU-derived correction scales; the direct RGBA8 full-chain upload endpoint is implemented; HMC packaging/resolution for precomputed corrected chains remains open [R32]. Direct core WASM with defaults remains blocked by native-only `cudarc` through `wgsl-forge-cuda`; renderer WASM is the supported no-default-features path. |
| AG-04 Authored normals and direct lighting | In progress | GLB NORMAL/TANGENT streams are validated and normalized, tangent frames orthogonalized with handedness retained, and uploaded in raw vertex streams. `.10d` FRM1 preserves optional normal/tangent frames; compiler, decoder and Portal carry them while QuantizedMesh/Tensor10DNodes stay stable. Missing GLB tangents are generated with MikkTSpace when TEXCOORD_0 exists, with deterministic vertex splits for differing corner frames and finite fallback on degenerate UVs. The material shader uses tangent-space normal maps with normal-scale support. Native and WASM `webizen-render --tests` checks pass. Codec/import assertions compile but remain unexecuted; runtime shader/pixel qualification, deformation frame updates, OBJ/STL frames and seam preservation through LOD/skinning remain open. |
| AG-05 Shadows and ambient occlusion | In progress | Sun-shadow baseline: two budget-admitted `Depth32Float` camera-range maps with 1024/512/256 per-map fallback, texel-stabilized light fits, material caster/receiver gates, static-map invalidation, a blended overlap, and direct-light 3×3 comparison PCF. The AO baseline adds a separately admitted half-resolution `Depth32Float` depth-test attachment + `RGBA8Unorm` packed surface + `R8Unorm` visibility set (9 bytes per half-resolution pixel). The packed surface stores octahedral normal in RG8 and normalized linear view depth in BA16, so evaluation and upsampling sample a portable float texture rather than relying on depth-texture loads; the depth attachment still supplies visibility/depth testing. A retained-mesh prepass feeds bounded 4/8/12-tap camera-basis reconstruction from linear depth and source-aspect pixel coordinates, with depth/normal-aware upsampling. AO attenuates indirect ambient only; baked and dynamic visibility combine by minimum, so they are not multiplied together and emissive/direct light stay unchanged. Both passes are skipped when disabled or unavailable; AO resources rebuild on resize and failure preserves the non-AO renderer. `QualiaPortal` exposes WASM controls and target availability/resolution. Native and WASM SDK test-target checks pass; a full portal WASM dev build plus wasm-bindgen web glue generation exports `portal_init_webgpu` and AO availability/configuration methods. Browser GPU/pixel execution remains open. Naga validates the AO prepass, AO evaluator and material WGSL. All three production AO math/budget tests pass in an isolated `rustc --test` harness. The focused native offscreen GPU test now executes on a GLES adapter and passes after AO surface sampling was changed from depth-texture loads to portable packed RGBA8 normal/view-depth data. An ambient-only occluder/receiver pixel comparison also passes: AO darkens at least one pixel by at least two 8-bit levels, preserves alpha and does not brighten RGB by more than one level (1/1). Complex-scene AO quality and browser AO/shadow pixel execution remain open. AO currently covers the retained mesh only. Its prepass applies MAT1 texture/vertex alpha and cutoff; an adapter-required GLES/DX12 packed-depth differential confirms the rear receiver survives behind cutout holes. Non-mesh scene geometry remains open. Shadow cascades now cover the retained mesh. The light-space depth convention now maps light-facing surfaces nearer under Less, and adapter-required GLES/DX12 cutout receiver comparisons pass. Multi-object/instance caster culling (current bounds are per material range within the retained mesh), contact shadows, AO temporal/mip/deinterleaved quality tiers, GPU/pixel qualification and measured performance fixtures remain open. |
| AG-06 Sky, atmosphere and distance depth | In progress | Native and WASM presets resolve through shared `render::atmosphere::AtmospherePreset` values for clear colour, sun direction/radiance, ambient irradiance and authored fog controls. WebGPU renders a camera-oriented fullscreen analytic sky in direct-colour and HDR+bloom passes, with day/dusk/night gradients, horizon/ground haze and sun/moon disk/halo. The optional WebGL2 fallback now uses a generated fullscreen triangle with the same orbit-camera FOV/basis and shared sun/ambient state; it also applies bounded distance/height fog to its mesh fragments. Explicit clear-colour overrides disable the analytic sky while retaining the selected fog profile, and WebGL2 sky initialization failure degrades to the preset clear. This remains art-directed, not calibrated atmospheric scattering. Next: calibrate sky/atmosphere units, align environment irradiance with sky output and exposure, and qualify browser GLSL, WebGPU/WebGL2 parity, gameplay-size pixels and performance.
| AG-07 Exposure and colour management | In progress | Native WebGPU and WebGL2 scene paths render to an offscreen scene target and apply bounded manual exposure plus PBR Neutral v1 after scene rendering. WebGPU direct and bloom paths use RGBA16F when their capability gates pass; WebGL2 selects RGBA16F when `EXT_color_buffer_float` is present and the framebuffer is complete, otherwise it uses a budget-admitted RGBA8 fallback. Both expose output-transform and highlight-preservation capability separately. CPU output oracle tests pass 9/9; a clean WASM `portal,webgl2` library check passes; Chrome compiled/linked the production WebGL2 shaders, rendered a pixel with identity white balance and no GL errors, and verified complete RGBA16F and RGBA8 targets on the tested adapter. Exposure is applied once in the shared SDR transform. Bounded artistic white-balance controls are implemented across direct WebGPU, bloom, WebGL2, and the WASM portal. Cross-backend pixel parity, resize/budget fallback automation, native GPU execution, HDR display output, LUT grading, golden captures and performance qualification remain open. |
| AG-08 Instancing, batching, culling and LOD | In progress | The WebGPU renderer now submits bounded instanced indexed draws across direct/HDR material, shadow, and AO passes. The native/WASM record is versioned ABI v2 (128 bytes): column-major transform, CPU-precomputed inverse-transpose normal frame, lossless 64-bit semantic identity, reflection orientation and padding. General finite, invertible affine transforms including non-uniform scale and reflection are supported; malformed or forged normal frames fail closed. WASM exposes caller-buffered ABI packing, packed-byte upload, SoA CPU frustum selection, and alignment-independent visible-record compaction, enabling a no-allocation cull → compact → submit path while preserving stable source order and identity. Storage capacity grows geometrically to 10,000 records under GPU resource reservations; multi-instance BLEND is refused pending sorted transparency. The WASM `portal,webgl2` library check passes. MSVC native tests pass 12/12 for frustum selection, affine-bound visibility, packed selection/compaction, identity/order preservation and 10,000-instance stability; ABI-v2 transform/normal-frame tests pass 4/4 for non-uniform scale, reflection, singular rejection, forged-frame rejection and the scale-relative determinant guard. The WASM test-target check cannot complete because of 64 unrelated test-build errors (native-gated APIs, missing imports/dependencies and stale test references); no diagnostics were reported from the changed renderer modules. The MSVC native `phenomenal_shader_modules_parse` test now performs full Naga validation (not syntax-only parsing) and passes for mesh ABI v2, sun-shadow, AO prepass/generation, output, bloom, ambient and projector WGSL; adapter pipeline creation, runtime pixels and performance remain open. Instance identity picking, camera-driven culling invocation in the frame loop, independently bounded mesh batches, LOD, GPU compaction/indirect draws, pixel qualification and measured 10,000-instance performance remain open.
| AG-09 Terrain and geography streaming | Not started | Deliver source-aligned terrain, precision and residency/eviction gates. |
| AG-10 Water and shore interaction | Not started | Implement material, shoreline and depth interactions with quality fallback. |
| AG-11 Rigs, clips and animation runtime | Not started | Specify/implement versioned rig and clip data plus bounded playback. |
| AG-12 Vegetation and secondary motion | Not started | Add scalable vegetation deformation and device-tier behavior. |
| AG-13 Particles and environmental effects | Not started | Add bounded pooled effects and event-driven presentation. |
| AG-14 Production compiler and interchange | In progress | GLB-to-`.10d` mesh/material/UV/normal import and deterministic versioned sidecars are implemented in slices. Next define and implement v2 structured identity/profile records: a scoped 10D address plus declared domain/profile, coordinate semantics and typed shape/behavior parts; hashes remain optional indexes/integrity aids. Add canonical identity resolution, collision/scope tests, round-trip fixtures, and preserve v1 compatibility. Full authoring interchange, scientific-field transport and broader malformed-input fixtures remain open. |
| AG-15 Character and botanical families | Not started | Produce reusable families only after their renderer and compiler foundations close. |
| AG-16 LOD and mesh optimization | Not started | Add deterministic LOD compilation, quality checks and runtime transitions. |
| AG-17 Style and art direction | Not started | Implement versioned style profiles and stylized-first acceptance assets. |
| AG-18 Selection and world feedback | Not started | Integrate readable selection, decals and semantic feedback across quality tiers. |
| AG-19 Accessibility and performance admission | Not started | Add accessibility floors, telemetry and resource admission behavior. |
| AG-20 Frame graph and resource ownership | In progress | Owned Geometry/FrameTarget reservations cover mesh positions, generated normals, colours, indices and the padded MAT1 uniform buffer; sun-shadow depth/map matrix; half-resolution AO depth, normal and visibility targets (9 bytes per AO pixel) plus AO uniform; bloom, depth, picking, offscreen colour, pooled compute bindings, portal fixed uniforms/model buffer, EMF parameters and SDK vertex/offscreen targets. MAT1 material uniforms are included in pre-upload Geometry reservation; ordered draws merge adjacent equal materials and reject more than 65,536 submissions. UploadStaging covers Portal/SDK readback, pick-map, uniform ring, pooled compute readback, and base-level texture upload bytes held until GPU-generated mip work completes; full mip-chain bytes are held as TextureResidency; mesh pose vertex/normal queue writes remain to be integrated into staging admission. FieldResidency covers Tensor10D records/particle projection and EMF field cells; particle pool size degrades to admitted capacity. The field contract is multidomain: EMF `[α,μ,σ]` is one family; astrophysics/N-body gravity, waves, diffusion, depth-aware and time-dependent fields retain typed 10D/PGA manifold semantics. Native and WASM SDK test-target checks pass. Wave/diffusion/astrophysics GPU residency adapters, frame graph, CPU asset/model working-set caps, shared asset/frame/staging/Sentinel/inference accounting, per-adapter ledger identity, surface swapchain residency, and the no-Qualia SDK budget remain open; independent surface devices must not be treated as covered by Portal reservations. |
| AG-21 Anti-aliasing and reconstruction | Not started | Implement qualified AA/reconstruction paths with motion and disocclusion fixtures. |
| AG-22 Environment lighting and probes | Not started | Add probe baking/runtime and portable analytic fallback. |
| AG-23 Reflection and transmission | Not started | Add bounded opaque/transparent/refraction paths and glass acceptance. |
| AG-24 Volumetric atmosphere and media | Not started | Add tiered volumetric path and non-volumetric fallback. |
| AG-25 Hero surface fidelity | Not started | Add skin, foliage, cloth, eye and layered material models. |
| AG-26 Native/WASM negotiation and recovery | In progress | Native portable/render-only fallback is implemented and test-target type-checked; pixel execution is blocked by the host MinGW aws-lc-sys nanosleep64 link. WASM has capability reporting, VibeScript initial and recovery backend policy, WebGPU/WebGL2 loss detection, fresh-canvas recreation, and retained tensor/reduced-mesh re-upload; the full WASM target type-checks. Webizen Studio’s VibeScript viewport lifecycle is present, but simulation-field output is not yet bridged into the live viewport. Forced-loss browser qualification, suspend/resume, full-fidelity pack retention/delivery and native/WASM parity fixtures remain open. `render/portal/mod.rs` remains an oversized multi-lifecycle owner; decompose portal input/selection and presentation lifecycle into focused child modules before further substantial behavior is added. |
| AG-27 Cinematic camera and capture | Not started | Add shot timeline and capped deterministic capture/export. |
| AG-28 Observability and visual regression | Not started | Version receipts and reproducible image/content diagnostics. |
| AG-29 Optional native ray queries | Not started | Evaluate only after portable path; require explicit native eligibility and fallback. |

### Implementation log
**2026-10-07 follow-up:** Implemented the first alpha-BLEND runtime slice in the shared Portal/Webizen renderer. MAT1 Blend coverage preserves material alpha, the configured vertex-alpha multiplier and resident base-colour texture alpha; opaque and mask draws remain in the depth-writing pass, while blended draws use alpha compositing with `LessEqual` depth testing and depth writes disabled. Blended submeshes are ordered far-to-near using model-space bounds computed during upload, current camera/model depth keys computed once per draw, and a deterministic in-place `sort_unstable_by` over a preallocated draw-index vector. The WGSL change is shared by native and WASM. Blend surfaces are omitted from opaque AO depth and shadow caster passes pending a separate translucent-shadow model. `cargo check -p qualia-core-db --tests --locked --offline` and `cargo check --target wasm32-unknown-unknown -p webizen-render --tests --locked --offline` pass; targeted rustfmt checks and direct Naga parsing/validation of the updated mesh, AO-prepass and sun-shadow WGSL pass. An adapter-required overlapping-pane pixel oracle was added and type-checks, but could not execute on this Windows GNU host: the default profile cannot link missing `d3d12`/`dxgi` import libraries, while the portable GL profile reaches an unrelated `aws_lc_sys` unresolved `nanosleep64` symbol. No native alpha-BLEND pixel result is claimed. Classic bounds sorting is approximate for intersecting/concave transparent submeshes; weighted OIT and browser pixel qualification remain open.
**2026-10-07 follow-up:** Comparative alpha-mask qualification now covers native forward, shadow and AO behavior on GLES and DX12. The shadow receiver fixture compares a transparent-left/opaque-right caster with a fully opaque reference; the receiver red channel through the cutout is at least 12 levels brighter on both adapters. The AO fixture copies one packed RGBA8 surface pixel from the production half-resolution prepass: the cutout path records the rear receiver, more than 100 depth-code values behind the front caster, on both adapters. Both tests require a GPU adapter; the AO readback copy usage is test-only. These comparisons exposed and fixed reversed light-space depth ordering in the sun projection: surfaces nearer the directional light now receive smaller zero-to-one depth and win the Less depth test. Six allocation-free shadow-math tests pass, including the nearer-surface regression. WASM test-target checking passes; browser AO/shadow runtime and broader-scene quality remain open.
**2026-10-07 follow-up:** AG-02 adds alpha-mask coverage to the shared native/WASM material WGSL. MAT1 material uniforms carry the mask enable bit in the existing 256-byte aligned record; below-cutoff samples discard and surviving Opaque/Mask fragments write full alpha. Alpha Blend continues to fail closed until a sorted transparent pass exists. Adapter-required offscreen pixel fixtures pass on GLES and explicitly selected DX12 (1/1 each), verifying that the transparent texel reveals the clear colour while the opaque texel remains shaded. The WASM webizen-render test-target check passes. The initial fixture exercised forward, AO depth/normal and cascaded shadow draws while checking forward coverage. Comparative shadow/AO hole-lighting checks and results are recorded in the newer entry above; semantic picking parity, browser GPU execution and alpha Blend remain open.
**2026-10-07 follow-up:** Ran render::gpu::mesh_pixel_tests::native_mesh_frame_matches_independent_cpu_coverage_and_depth_oracle with gpu-native-dx12, QUALIA_RENDER_WGPU_BACKEND=dx12 and QUALIA_REQUIRE_GPU_TESTS=1. The explicitly selected DX12 adapter executed the renderer and matched the independent CPU pixel-coverage/depth oracle (1/1; Cargo test exit 0). This qualifies that fixture on native DX12; it does not qualify DX12 material maps, AO, shadows, browser/WebGPU presentation, broader scenes or performance. Browser readback and those backend-specific pixel fixtures remain open.
**2026-10-07 follow-up:** Added adapter-required native GLES MAT1 base-colour texture sampling coverage in `render/gpu/mesh_pixel_tests.rs`. The test uploads a two-texel red/green sRGB resource through `upload_resident_texture_rgba8`, checks its resident binding, attaches its digest to a material record, binds UV0 at the red texel centre, and compares actual center-pixel output. The frame returns `[47, 0, 0, 255]`; the regression allows a backend-tolerant red range of 35–70 while requiring G/B ≤2 and opaque alpha, distinguishing the sRGB sample from linear interpretation, the adjacent green texel and semantic white fallback. Native required-adapter execution passes (1/1). This verifies one base-colour role and one UV/sampler path, not all six texture roles, mip behavior, HMC decoding/rollback, compressed formats or browser parity.

**2026-10-07 follow-up:** Added `native_screen_space_ao_only_reduces_ambient_receiver_pixels` to `render/gpu/mesh_pixel_tests.rs`. A retained broad receiver plane and raised patch render first with ambient-only lighting and AO disabled, then with AO enabled and all other inputs fixed. Required-adapter GLES execution confirms at least one pixel darkens by 2 or more 8-bit levels, RGB channels never brighten by more than one quantization step, and alpha coverage remains unchanged (1/1). This proves the AO pass reaches receiver shading; it does not establish complex-scene AO quality, artifact-free temporal behavior, alpha-mask/deformed/non-mesh coverage, cross-backend parity or performance.

**2026-10-07 follow-up:** Added `render/gpu/mesh_pixel_tests.rs`, a controlled offscreen CPU image oracle separate from the renderer implementation. It projects test triangles through the active view-projection, evaluates pixel-center barycentric coverage and linearly interpolated NDC depth, excludes a documented edge-precision band, and predicts clear/far/near surface ownership. Adapter-required GLES execution matches all classified samples (1/1), with explicit minimum coverage of 500 clear, 20 far-only and 20 near/depth pixels. The fixture uses diagnostic red/blue vertex colours and a black clear; it verifies geometry coverage and depth ownership rather than full material shading, anti-aliasing or cinematic image quality. More complex clipping/material oracles, DX12 and browser readback remain open.

**2026-10-07 follow-up:** Added adapter-backed GPU/CPU differential coverage in the separate `render/gpu/picking_tests.rs` module. The fixture submits four Tensor10D nodes through the production projector and R32Uint picking pass, queues fractional pointer coordinates through `queue_pick`, and compares the mapped one-pixel readback with `cpu_pick_node_at_camera`. A nearer overlapping node wins; temporal and clip-volume rejects do not interfere; an adjacent no-hit texel returns the sentinel as `None`. Required-adapter GLES execution passes (1/1), and the native test-target check compiles the fixture. The browser async readback path, broader transformed/temporal differential vectors, independent CPU rendered-pixel oracle and DX12 remain open.

**2026-10-07 follow-up:** Enforced native GLES offscreen pipeline creation exposed that depth-texture `textureLoad` is not portable through the GLES GLSL backend. AO now retains `Depth32Float` only as a depth-test attachment and samples a packed `RGBA8Unorm` surface record (oct-normal RG8, normalized linear view depth BA16); total admitted AO target storage remains 9 bytes per half-resolution pixel. The shader also renamed a WGSL-reserved local identifier found during pipeline creation. A GLES test with adapter execution required passes, proving pipeline creation and opaque-black empty-frame behavior across 32×24, 65×31 and 1×1 resize (1/1); it does not exercise visible AO samples. AO math/budget tests pass (3/3), Naga shader parsing passes (1/1), native core test-target check and WASM `webizen-render --tests` check pass. Complex-scene AO pixel quality, DX12/browser execution, measured performance, alpha-mask and non-mesh coverage remain open.

**2026-10-07 follow-up:** AG-01 CPU picking fallback now evaluates Tensor10D nodes with the same PGA motor, observer/time inputs, orbit view-projection, homogeneous clip range, alpha-scaled shader disc, top-left pixel-center sampling and strict depth ordering as `projector.wgsl`. Zero-width temporal windows now avoid undefined WGSL `smoothstep` edges and hide the zero-fade boundary. Four regression tests exercise depth ordering, pixel footprint, near/behind-camera clipping, temporal cut and invalid pointer/extent inputs. The four focused native tests pass (4/4), and Naga parses the projector WGSL (1/1); native core and WASM renderer test-target checks compile the path. Browser GPU/CPU differential and rendered-pixel qualification remain open.
**2026-10-07 follow-up:** AG-05 first sun-shadow slice is implemented in the native/WASM Portal: budget-admitted Depth32Float target (tries 1024, 512, 256), material-selected casters/receivers, a texel-snapped directional projection fitted to transformed scene bounds, direct-light-only 3x3 comparison PCF, receiver slope bias and pipeline caster bias. Static maps are retained until geometry, pose, light matrix or enable state changes; failure to admit the lowest target retains the unshadowed path. Allocation-free projection tests pass in an isolated Rust harness (3/3). Native and WASM `webizen-render --tests` cargo checks pass; direct Naga validation passed for the mesh and sun-shadow WGSL modules. A focused cargo test build reached native linking but could not execute because this host MinGW `aws_lc_sys` link reports unresolved `nanosleep64`; GPU/pixel qualification remains open. Two camera-range cascades with a bounded blend band have since been added; see the next follow-up. Multi-object caster culling, alpha-mask/deformation parity and contact shadows remain future AG-05 work. Method researched against Microsoft CSM guidance and WGSL/WebGPU semantics [R08]/[R28]/[R29]/[R30]; this is a portable design reference, not a measured Qualia performance claim.
**2026-10-07 follow-up:** `.10d` writes header version 2 for the shared, extensible container while preserving its 64-byte physical header and dual-version v1/v2 reader. V2 defines semantic identity as a canonical, typed structure: a domain/profile and universal 10D address interpreted together with intrinsic characteristics and references to versioned parts that express shape, topology, field values and behavior. The 10D address is not itself a globally unique digest; equality and resolution follow the declared profile, coordinate representation, units and identity scope. Stable IDs may reference this structure, while hashes/digests are auxiliary indexes, cache keys or integrity checks and never replace its meaning or data. Distinct parts remain inside one `.10d` envelope so a record can carry multiple domain facets without collapsing them into one scalar or colour projection. “Quantum signature” is an analogy for the structured identity-bearing pattern only, not a claim of quantum computation, uniqueness or cryptographic proof. This is the normative v2 contract; the header/version compatibility is implemented, while explicit identity/profile codecs and remaining typed-part transports stay tracked in AG-14/AG-20. Native core test-target and native/WASM renderer test-target checks pass; these compile checks do not demonstrate runtime rendering or field transport.
**2026-10-07 follow-up:** Replaced the single sun map with two independently rendered/budgeted `Depth32Float` camera-range maps. The near split follows 4× orbit zoom clamped to 8–48 world units; the far range is 200 units. Both fits include the retained scene-mesh bounds, use the existing stable texel snap, and render only when shared inputs change. Receivers interpolate across a clamped 8% split overlap and continue to apply 3×3 comparison PCF to direct sun only. Added allocation-free cascade fitting tests (5/5 pass in an isolated `rustc --test` harness); native and `wasm32-unknown-unknown` `webizen-render --tests` checks pass; direct Naga parsing/validation passes for mesh and shadow WGSL. GPU/pixel and measured performance qualification remain open, as do multi-object caster culling, comparative alpha-mask shadow/AO fixtures, deformation parity, contact shadows and higher-fidelity tiers. This portable two-map slice uses 2× the corresponding single-map target bytes and makes no measured performance claim.

**2026-10-07 follow-up:** Added optional half-resolution screen-space ambient visibility in `render/gpu/ao.rs` with math/sizing isolated in `ao_math.rs`. A retained-mesh depth/normal prepass feeds a world-space 4/8/12-tap WGSL evaluator; the mesh material shader performs depth/normal-aware upsampling and applies visibility to indirect ambient only. Baked/dynamic visibility uses a minimum bound; emissive and direct lighting remain untouched. Separate FrameTarget admission covers 9 bytes per half-resolution pixel and resize recreates resources. Native and WASM `webizen-render --tests` cargo checks pass; direct Naga validation passes for AO prepass, AO evaluator and mesh shaders; formatting and whitespace checks pass. All three production AO math/budget tests pass in an isolated `rustc --test` harness. At the time of this initial baseline entry, the full Cargo test executable had not linked because of the host MinGW `aws_lc_sys` `nanosleep64` issue; the focused enforced GLES test later linked and ran successfully, as recorded in the following follow-up. GPU/pixel qualification, performance measurements, alpha-mask parity, non-mesh AO, and CACAO-style deinterleaving/hierarchy remain open. Research reference [R31] informed the reduced-resolution strategy; this is an implementation baseline with no measured performance claim, not a CACAO-equivalent port.
- **2026-10-07 — AG-02/03/04 HMC texture-to-shader slice:** Added a bounded shared PNG/JPEG decode boundary, digest-verified HMC material dependency loading, sRGB/linear resident textures with separate staging/resident reservations and failure rollback, plus six fixed material texture slots with semantic fallback texels. GLB TEXCOORD_0 now survives MikkTSpace vertex splitting, round-trips through CRC-protected UV01 `.10d` data and enters the native/WASM GPU vertex stream. WGSL samples UV0 for base colour/alpha, emissive, tangent-space normal, metallic-roughness, AO and stylized ramp textures; GLB material factors and `KHR_texture_basisu` source preservation remain represented in the asset path. `cargo check -p webizen-render --tests` and `cargo check --target wasm32-unknown-unknown -p webizen-render --tests` pass, including the HMC loader. These checks compile tests but do not run decoder/residency assertions or validate/present the latest shader on a GPU. KTX2/Basis/WebP transcoding, URI/data URI sources, mipmaps/streaming, sampler/UV transform support, alpha MASK/BLEND, and pixel/visual qualification remain open.
  - **2026-10-07 — AG-26 WASM test dependency boundary:** Restricted `proptest` to its standard feature set, removing the native `rusty-fork`/`wait-timeout` path, and made Criterion native-only because benchmarks do not run in browser WASM. This removes dependency-level WASI compile errors. `cargo check --target wasm32-unknown-unknown -p qualia-core-db --no-default-features --features wasm-scientific --tests` now reaches existing project test modules but fails where tests reference native-only APIs/modules (including volume/mmap/query and host services) gated out of WASM. The supported `webizen-render --tests` WASM check passes. Native QualiaDB test-target check passes with `--locked --offline`.
  - **Historical AG-04 UV-derived tangent fallback (superseded):** GLB import now reads TEXCOORD_0 VEC2 in FLOAT or normalized unsigned-byte/unsigned-short form, validates counts, strides, view bounds and finite values, and derives a deterministic tangent/handedness from triangle positions and UV derivatives when TANGENT is absent. Mirrored UV orientation is retained; degenerate UV triangles fall back to a stable orthogonal frame. Added derivative, mirrored-sign, degenerate-UV and accessor-bound tests plus a GLB-to-FRM1 `.10d` round-trip fixture. This initial lightweight fallback was not MikkTSpace and did not close production normal-map baking; it is superseded by the MikkTSpace implementation recorded below. Native `cargo check -p qualia-core-db --tests --locked --offline` passes. An isolated `rustc --test` harness compiled the exact production UV accessor and tangent-generation functions and ran 2 behavioral tests successfully. WASM `webizen-render` test-target check passes; full `qualia-core-db` WASM test-target compilation now reaches unrelated tests that import native-only APIs/modules and fails there; see the AG-26 dependency-boundary log. The full native test executable cannot link because MinGW `aws_lc_sys` references unresolved `nanosleep64`, so the new end-to-end GLB-to-FRM1 fixture is compile-checked but not executed on this host.
  - **2026-10-07 — AG-04 `.10d` tangent-frame round trip:** Extended FieldSidecar with FRM1, a vertex-aligned combined frame encoding that stores optional octahedral SNORM16 normals and tangent XYZ plus signed handedness as SNORM16 values. Legacy normals-only NRM1 remains readable and byte-stable; compiler asset paths now include imported tangents, decoder APIs return optional normals/tangents, and Portal validates counts and uploads both. Added codec and compiler round-trip coverage for normal+tangent and tangent-only data, including mirrored handedness. Native portable-profile core and WASM renderer test-target checks pass. An isolated Rust test harness ran all 8 codec tests successfully and validated the mesh WGSL with Naga; broad native test-binary execution remains unverified because the host MinGW linker reports unresolved `aws_lc_sys` symbol `nanosleep64`. Normal-map material use and deformation refit remain open; MikkTSpace generation is recorded in the later 2026-10-07 entry.
  - **2026-10-07 — AG-04 GLB tangent-frame upload:** GLB import now reads FLOAT VEC4 `TANGENT`, validates position-aligned count and handedness, normalizes directions and orthogonalizes them to the resolved normal. Mixed primitives receive deterministic fallback directions; a fourth vertex stream is admitted and bound for both SDR/HDR mesh pipelines, and the shader transforms its direction through the rigid model transform while retaining handedness. Added fixture assertions for tangent orthogonalization and invalid handedness, plus a MESH_WGSL Naga smoke test (compiled but not executed on this host). Native portable and WASM test-target checks pass. At the time of this entry, tangent `.10d` persistence was still open; the later FRM1 entry above supersedes that status. Normal-map material use and deformation updates remain open; MikkTSpace generation is recorded in the later 2026-10-07 entry.
  - **2026-10-07 — AG-04 `.10d` normal-field preservation:** Activated the reserved FieldSidecar section type 8 for a versioned per-vertex normal stream. The NRM1 codec stores octahedral SNORM16 pairs, validates counts and non-finite/zero-length vectors, and requires exact payload length. The compiler attaches imported normals while retaining byte-stable QuantizedMesh and Tensor10DNodes payloads; Portal decodes and forwards the optional stream, generating normals through the existing path when absent. Four codec tests, two compile-10d tests, and end-to-end GLB import-to-container-to-decode coverage were added. Unknown optional field-sidecar kinds remain available to separate scientific-field consumers. Native portable-profile and WASM render test-target checks pass; runtime execution still cannot link on this MinGW host because `aws_lc_sys` references unresolved `nanosleep64`. `.10d` tangent/handedness, OBJ/STL authored normals, hard-edge/UV-seam preservation, LOD/skinning and visual qualification remain open.
  - **2026-10-07 — AG-04 GLB authored-normal import:** Added an additive `ImportedMesh` route that preserves finite, non-zero glTF/GLB `NORMAL` VEC3 attributes after normalization without changing the existing `Mesh` struct API. Primitives without normals receive area-weighted geometric normals; malformed count, layout or zero-length authored normals fail closed. Portal carries authored normals through to the initial mesh vertex stream; deformed meshes recompute from geometry using the incremental normal workspace. Native portable-profile core test-target and WASM `webizen-render` test-target checks pass. Focused importer test execution could not link: the local Windows GNU linker reports unresolved `aws_lc_sys` symbol `nanosleep64`; the native and WASM test-target checks still compile successfully. OBJ/STL authored-normal ingestion, tangents/hard-edge topology, material/style profiles and visual qualification remain open.
  - **2026-10-07 — AG-04 incremental pose-normal update:** Replaced full-mesh normal recomputation on animation updates with a cold-built CSR vertex-to-face adjacency workspace, area-weighted face accumulators, preallocated dirty-vertex/face marks, and incremental updates limited to faces incident to the changed position span. Dirty vertices are sorted and coalesced into contiguous normal-buffer queue writes. Three focused Rust tests pass via an isolated `rustc --test` harness. Native portable-profile `cargo check -p qualia-core-db --tests` and WASM `cargo check --target wasm32-unknown-unknown -p webizen-render --tests` pass. This lowers update work for local poses but does not yet cover transient queue-write staging in the VRAM ledger or provide an adapter-measured performance gate.
  - **2026-10-07 — AG-04 generated-normal lighting slice:** Added bounded checked area-weighted vertex-normal generation for indexed mesh geometry, a dedicated vertex stream on both SDR and HDR mesh pipelines, and linear Lambert direct-light evaluation from interpolated normals. Geometry reservation includes the new normal stream. Pose updates retain the cold source positions/indices, update positions and recompute normals into preallocated storage before queue upload. Two focused normal-generation tests pass in an isolated `rustc --test` harness; the native portable-profile core test target and WASM `webizen-render` test target pass `cargo check`; Naga parses and semantically validates the mesh WGSL. This does not close authored normal/tangent import, hard-edge/smoothing-group preservation, physical/stylized material profiles, CPU asset memory admission, per-frame staging admission, or visual lighting fixtures.
  - **2026-10-07 — AG-20 SDK admission and WASM feature-boundary slice:** `webizen-render` Qualia builds now own Geometry reservations for fixed vertex buffers, FrameTarget reservations for offscreen colour targets, and UploadStaging reservations for readback; replacement targets are admitted before allocation and the old target/reservation survive refusal or resize failure. The separate surface-device path still needs per-adapter accounting and swapchain residency estimates; standalone no-Qualia builds retain no shared Qualia ledger. The portable WGSL Forge feature no longer implies CUDA; the default Qualia profile opts into the new `wgsl-forge-cuda` aggregate so existing native defaults retain CUDA. WASM browser engine features are target-scoped. Native `cargo check -p webizen-render --tests --locked --offline` and WASM `cargo check --target wasm32-unknown-unknown -p webizen-render --tests --locked --offline` both pass. The WASM slice also gates native PNG/readback tests and APIs out of the browser target and makes zero-copy JS typed-array views explicitly unsafe with lifetime/alignment requirements. GPU pixel execution remains host-link/device dependent; these checks do not qualify runtime output or close AG-20/26.

- **2026-10-06 — AG-01:** Mesh occlusion/non-additive and explicit-zero-light offscreen pixel regressions both pass on native GLES (`QUALIA_WGPU_BACKEND=gl`) using a temporary Vulkan/GLES-only build; the existing default-DX12 occlusion test had also passed before these changes. `cargo check -p qualia-core-db --tests` passes. The default GNU native relink is unavailable here (`ld` cannot find `-ld3d12` / `-ldxgi`). A temporary Vulkan-only execution exited with `0xc0000005` on this host; Vulkan is not qualified. All temporary Cargo manifest changes were restored; the shipped/default backend settings are unchanged. These results do not close AG-01 or the wider programme.
- **2026-10-06 — AG-20/26 native fallback slice:** Added gpu-native-dx12 (enabled by default) and gpu-native-portable profile support; portable wgpu enables GLES/Vulkan/Metal/WebGPU without DX12. QUALIA_RENDER_WGPU_BACKEND is a strict render-only pin; offscreen rendering tries a bounded platform order and can initialize a renderer device independently if the shared inference device is unavailable or unsuitable. Windows build-script D3D12/DirectML link and staging are now conditional on the DX12 feature; the existing default profile still uses the vendored files under vendor/directml. cargo check -p qualia-core-db --no-default-features --features profile_target_1024,zk-culling,gpu-runtime,gpu-native-portable,wgsl-forge,privacy-he --tests --locked --offline passes, and the touched Rust files pass rustfmt --check. Offscreen test linking now gets past missing D3D12 import libraries but stops at the local MinGW aws-lc-sys unresolved nanosleep64; pixel fixtures were not re-run through the durable profile. The earlier temporary GLES run remains evidence for two fixtures only. Native/shared device-loss recovery, effective capability reporting and per-class resource reservations remained open at the time of this native slice.
- **2026-10-06 — AG-26 WASM slice:** Added a bounded read-only browser graphics probe reporting compiled profile/capabilities separately from WebGPU API and adapter availability, WebGL2 context creation, Canvas2D availability and recommended fallback order; WebGL probe releases its temporary context. The browser loader now consults this report, automatically allows WebGL2 when the probe confirms it (unless explicitly disabled), and replaces a failed WebGPU canvas before trying WebGL2/Canvas2D. Primary canvas consumers receive the replacement canvas. WASM portal cargo check passes; wasm-pack compiled the WASM module; wasm-bindgen JS post-processing remains unverified because the default installer could not create a temp directory and the workspace-temp retry was blocked by access denied to the toolchain dlltool executable under AppData. JS syntax and wasm-fetch loader tests pass. The wasm --tests profile remains blocked by the pre-existing wait-timeout dev dependency lacking a wasm implementation. At the time of this WASM capability slice, device-loss restoration, local asset-pack validation/delivery, and browser parity captures remained unimplemented; see the subsequent recovery entry below.
- **2026-10-06 — AG-26 device-loss recovery slice:** The browser portal now observes wgpu device loss and WebGL2 context loss at a frame boundary, drops invalid backend resources/readbacks, signals the host, replaces the context-bound canvas, and uses VibeScript to select WebGPU → WebGL2 → Canvas2D recovery. A recreated GPU backend re-uploads the retained semantic tensor and the retained CPU body's reduced index set, with reduced mesh detail reported. The WASM target type-checks; JS syntax, fetch tests and formatting checks pass. Forced-loss browser execution, tab suspend/resume, retaining/replaying the original full-fidelity validated pack, and semantic/image parity remain unverified/open.
- **2026-10-06 — AG-20 geometry admission slice:** WebGL2 mesh residency now holds a geometry-class VRAM reservation for its resource lifetime. Anatomy upload reports backend truth, retries admission with a compact per-organ reduced mesh, and requests VibeScript-governed canvas replacement when a context-bound backend refuses even reduced geometry. WebGL2 refusal forces the replacement path to Canvas2D. WASM portal cargo check and native portable-profile test-target check pass; JS syntax, rustfmt and diff whitespace checks pass. The reduced-mesh regression compiles in the native test target but was not executed on this host. Browser refusal/recovery and device-memory accounting against concurrent telemetry changes remain unqualified; shared accounting is still an estimate pending full AG-20 ownership integration.
- **2026-10-06 — AG-20 HDR frame-target admission slice:** `BloomChain` now owns a `FrameTarget` VRAM reservation covering the full-resolution RGBA16F HDR surface, both half-resolution blur targets and the 1×1 dummy target. Byte sizing is checked for overflow and admission happens before texture creation; resize holds old and replacement reservations through the peak. Bloom is omitted on refusal and the same bytes are no longer counted again in render telemetry. Native portable-profile test-target and WASM portal checks pass; size/overflow regression tests compile but are not executed here. Browser resize/refusal behavior, depth/picking/offscreen targets, staging, and hardware allocation measurements remain open.
- **2026-10-06 — AG-20 base frame-target admission slice:** Depth, picking and RGBA8 offscreen targets now hold an owned `FrameTarget` reservation. Admission precedes creation and replacement resize reserves the new extent before releasing old attachments. Offscreen targets halve resolution until admission succeeds and `Render.gpu_resize` returns the effective dimensions; an unadmittable browser surface extent reports failure and requests the existing Canvas2D recovery policy. Checked byte sizing has exact 1080p/overflow regression cases. Native portable-profile test-target and WASM portal checks pass; regression tests compile but are not executed. Driver allocation measurements and real browser refusal/resize fallback remain unqualified.
- **2026-10-06 — AG-20 upload-staging admission slice:** Offscreen pixel readback, the 256-byte pick-map slot, and the uniform upload ring now own `UploadStaging` reservations. Readback reservation includes checked 256-byte row alignment and height; offscreen extent selection admits both target and readback sizes. The uniform ring halves its slot count to one on native allocation pressure, while WASM budgets a bounded queue-depth horizon for `queue.write_buffer`. Native portable-profile test-target and WASM portal checks pass; sizing tests compile but are not executed. Other compute/readback staging paths, driver measurement and full shared-ledger admission coordination remain open.
- **2026-10-06 — AG-20 compute allocation admission slice:** Pooled compute binding buffers reserve `FrameTarget` bytes and pooled readback buffers reserve `UploadStaging` bytes before allocation; RAII ownership follows the pool and releases on replacement/key reset. Readback binding validation and checked four-byte alignment reject malformed or overflowing requests before dispatch resource construction. Corrected the uniform-belt WASM documentation and clarified that the EMF view is one scientific field-family projection; its complete manifold record stays intact and display colour remains a projection. Native portable-profile test-target and WASM portal checks pass. The focused checked-alignment test was attempted, but the test binary cannot link on this Windows GNU host because `aws-lc-sys` references unresolved `nanosleep64`; runtime test execution remains unverified. Persistent scientific-field GPU uploads, other GPU resource owners, driver measurements, browser runtime behaviour and global admission/telemetry reconciliation remain open.
- **2026-10-06 — AG-20 multidomain field-residency slice:** Added a dedicated `FieldResidency` reservation class for viewport storage data. Tensor10D raw records and their particle projection reserve replacement capacity before allocation; portal startup halves the optional particle pool until the budget/device binding limit admits it. EMF cell uploads validate checked grid counts, cell-byte overflow and storage limits, reserve the GPU copy, and construct the replacement binding before swapping, preserving the complete 10D cell record. EMF remains one field-family adapter, not the manifold/astrophysics renderer. Native portable-profile test-target and WASM portal checks pass; sizing regressions compile, but native test binaries still cannot link here because MinGW `aws-lc-sys` references unresolved `nanosleep64`. Runtime GPU allocation behaviour is not qualified. Wave/diffusion/astrophysics field GPU adapters, remaining static renderer allocations, driver measurement and shared-ledger reconciliation remain open.
- **2026-10-06 — AG-20 fixed renderer-buffer admission slice:** Portal ambient, telemetry, camera, observer and model uniform buffers now share an owned fixed-size `FrameTarget` reservation (384 bytes from ABI `size_of` values). Bloom residency now includes its 32-byte uniform block as well as HDR, blur and dummy textures. Removed portal particle/tensor `record_render`/`record_tensor` overwrite gauges where owned reservations now account the actual buffers, preventing stale estimates from replacing telemetry. Native portable-profile test-target and WASM portal checks pass; size regressions compile but cannot be executed because native test linking fails at the host MinGW `aws-lc-sys` unresolved `nanosleep64`. Static shader/pipeline and other remaining resource measurements stay open.
- **2026-10-06 — AG-20 SDK-adapter resource audit:** `webizen-render` uses the shared Qualia device for its qualia-enabled offscreen constructor, but its surface constructor requests a separate adapter/device. Its fixed vertex buffers, offscreen target replacement and transient readback buffer are not yet admitted through owned resource reservations; this is an open adapter gap, not evidence that Portal accounting covers every renderer.
- **2026-10-06 — AG-26 10D/field integration audit:** Existing engine support includes the versioned 10D tensor ABI, a 4D EMF field grid tagged with 10D manifold coordinates and depth sampling, plus separate astrophysics N-body gravity, wave and diffusion solvers. EMF is only one field family; its `[α,μ,σ]` spectrum is canonical data and display colour is a derived human-visible projection. In `qualiaDB`, fixed Webizen Studio’s tensor reader to decode Q42 header + packed f32 records while retaining the historical f64 preview reader; the digest no longer computes a colour. Tests were added, but native test execution is blocked before project compilation by local Windows GNU `gcc.exe`/`dlltool.exe` Access Denied. The Studio viewport already calls its generic lifecycle through VibeScript; the typed bridge from scientific solver outputs into the viewport is still open. The Studio-side implementation and progress are tracked in `crates/webizen-studio/STUDIO_ROADMAP_PROGRESS.md`.
- **Next slice update:** add owned target/vertex/readback reservations and budget-aware resize to `webizen-render`, accounting separately for its independent surface device and shared offscreen device; extend FieldResidency to wave/diffusion/astrophysics and other structured fields; close admission races across separate devices and reconcile telemetry with owned allocations. Then bridge typed multidomain simulation outputs—including the earlier high-dimensional astrophysics programme—through VibeScript into the Studio/Portal viewport without replacing solvers. Qualify browser resize/refusal, forced recovery, full-fidelity pack replay, native/WASM parity and remaining AG-01 image oracles.
- **Next slice (owner-directed):** complete and qualify the remaining AG-20 resource owners, then implement the typed VibeScript-driven 10D/multidomain field bridge. Preserve historical scientific work and explicitly distinguish design/history from validated runtime capability. Continue native/WASM fallback, loss/recovery and semantic/image parity qualification; return to AG-01 pixel oracles as dependencies permit.

## 1. Scope, authority and operating rules

This document specifies reusable engine capabilities, their implementation methods,
integration contracts and evidence. Content production and artistic review remain
essential: adding rendering effects alone cannot certify cinema-grade art.

The [blueprint](17-rts-aaa-uplift-blueprint.md),
[development contract](13-qualiadb-only-development-contract.md),
[QG work orders](19-qualiadb-upstream-gate-work-orders.md),
[asset catalogue](18-asset-production-catalog.md), and
[geospatial pipeline](30-procedural-geospatial-rendering-pipeline.md) remain linked
programme requirements. Generic rendering, import, compilation and runtime work
belongs in QualiaDB; game recipes, reviewed assets and style content belong in game-demo.
The present work is a specification revision, not an engine implementation or release.

**MUST/SHALL** identify acceptance requirements. **SHOULD** identifies the preferred
method; an alternative needs equivalent quality and measured evidence. **MAY** is
an optional enhancement. A profile cannot advertise a feature until its stated
runtime/device tests pass. A supported low-quality profile is different from an
unsupported feature; both must be reported accurately.

Preserve **AG-01–AG-19** identifiers and meanings. New gaps are **AG-20–AG-29**.
AG numbers describe capabilities, not execution order. Scheduled implementation
receives an upstream work order linked back here. Allocate the next **unused** QG
identifier after inspecting all registers: doc 19 already uses QG-20 for Portal API
terminology, while the capability ledger also uses QG-20 for the world atlas.
Resolve that collision in the owning registers before issuing new orders; this
revision does not silently renumber either programme.

Every work order SHALL:

1. Inspect the existing public API, format, shaders, feature gates and selected
   native/WASM route; record the exact source and built-artifact identities.
2. Establish a small failing fixture, including malformed input, missing
   dependencies, exhausted budgets and incompatible versions where applicable.
3. Extend the engine API, format specification, capability declaration and both
   runtime adapters together. An existing candidate may need integration rather
   than replacement.
4. Integrate one non-game consumer and the full `wasm-full` game; demonstrate the
   same pack in a native Webizen consumer for portable features.
5. Close only with correctness, visual, performance, memory, identity and recovery
   receipts. Update the [capability ledger](15-living-qualiadb-capability-ledger.md).

## 2. Evidence baseline and corrections to the original register

Inspection checkpoint: QualiaDB HEAD
`7284afeffd7370dfa1460d7853be3d79bd493fd7`; game-demo HEAD
`020a53443628d51dccc07cc094a48075f4f9b1cf`. Both working trees contain local
changes. These hashes identify the base commits inspected, **not reproducible
pins for those changes or proof of the current built WASM package**. Historical
game receipts cite `32ef0175`; a sibling Cargo path can consume a different tree.
Implementation SHALL reconcile source, local diff, compiler and package digests first.

Paths in this table are relative to `C:/github/qualiaDB`, except the game entry.

| Area | Evidence inspected | Interpretation and next proof |
| --- | --- | --- |
| Canonical renderer | `crates/qualia-core-db/src/render/gpu/`, `gpu_context.rs`, `crates/webizen-render/` | Cross-platform Portal renderer and native offscreen/readback paths exist. Extend this engine; SDK/serde/codecs are adapters. |
| Geometry and `.10d` | `render/compile_10d.rs`, `render/assets/`, `container_10d/`, computational geometry | Quantized meshes, import and optional topology/spatial sections exist. Primitive sufficiency for blockouts does not prove production normals, UVs, rigs, LOD quality or close-up topology. |
| Surface colour | Game `crates/rolling-commons-shell/src/lib.rs` SRD1 authoring | Per-vertex colour variation exists. It is not a material/texture/BRDF contract. Preserve it as a vertex-colour multiplier. |
| Mesh lighting | `crates/qualia-core-db/src/shaders/viewport/mesh.wgsl` | The fragment shader already evaluates lighting **per pixel**, using derivative-derived **flat face normals**. The gap is authored normals/tangents and a production lighting/material model, not the absence of a fragment lighting stage. |
| Tone mapping | `shaders/viewport/bloom.wgsl`, `render/gpu/bloom.rs` | HDR bloom, exposure parameters and a Reinhard composite exist. The gap is a coherent, configurable colour/exposure/output contract and conformance across bloom/fallback paths. |
| Lights, motion and effects | `render/scene_graph/light.rs`, `scene_graph/ik.rs`, `render/gpu/particles.rs` | Light types, kinematics and ambient particles are candidates. They do not establish packed skeletal animation, bounded crowd playback or a general event-linked effects system. Audit allocation and integration before reuse. |
| Current game scale | Existing doc-22/register reports approximately 340 asset variants per style | Historical content counts are fixture inputs, not throughput evidence. Record actual visible instances, unique meshes, triangles, materials and bytes in every new run. |
| Correctness history | QG-12 black-viewport/ghost-surface reports | Upload counts and passing gameplay checks do not prove visible, correctly occluded pixels. Reproduce against the chosen build; do not assume a historical failure remains or has been fixed. |
| Maintainability | `render/gpu/mod.rs` is about 2,181 lines at inspection | Substantial new behaviour requires tracked decomposition under AG-20, following engine `AGENTS.md`; new systems must not accumulate in this file. |
| Independent memory domains | `gguf_bridge/wasm_cpu/mod.rs`, `inference/runtime/budget/{model,pools}.rs`, `q42/asset_import/budgets.rs` | Inference explicitly separates its memory domain from the semantic Sentinel arena. Asset imports already distinguish total job bytes from the 42 MiB per-chunk limit. Apply the owner's clarified graphics residency contract in §3.2. |

Evidence statuses: **candidate**, **gap reproduced**, **specified**, **in progress**,
**upstream verified**, **runtime qualified**, **game integrated**. Native-only
qualification never implies browser qualification; an optional native feature can
close its explicitly native scope while its browser fallback is separately tested.
This revision records candidates and proposed requirements, not new qualification.

## 3. Non-negotiable contracts

### 3.1 One semantic engine, two runtime adapters

`qualia_core_db::render` owns canonical planning, draw contracts and shader behaviour.
`webizen-render` and Portal/WASM expose that engine. Keep the existing 10D/PGA
projection, semantic identity, depth and picking contracts. Do not introduce a
game-only renderer, alternate simulation, or device-specific asset authority.

Q42 owns facts, commands and stable entity IDs. `.10d` owns compiled geometry and
versioned presentation sections; HMC owns immutable distribution dependencies.
Material/texture/rig payloads are referenced resources, not additions to `NQuin`.
Keep the six-u64, 48-byte ABI, hash rules, parity and existing metadata ownership.
Do not repurpose semantic bits for transient render state.

Portable features SHALL share schemas and material semantics across native and
WASM. Backend scheduling and selected quality may differ. Exact authoritative
world/replay digests must agree; floating-point GPU images are compared within
declared tolerances rather than assumed byte-identical across drivers.

### 3.1.1 Ten-dimensional manifold and multidomain field preservation

The graphics programme SHALL extend the existing 10D/PGA and scientific-field
work; it SHALL NOT replace it with pane geometry, a colour-only point cloud or an
EMF-only renderer. Preserve the full `[q,v,w,x,y,z,t,α,μ,σ]` tensor and its
topological/domain interpretation through native and WASM scene ingestion,
selection, picking, replay and fallback. Use the versioned Q42 tensor-buffer ABI
(32-byte header; 40-byte `10×f32` record, subject to its version/stride fields)
or a newer explicitly versioned contract; legacy decoding may remain as a
compatibility path, never as the canonical writer.

Use the revised `.10d` v2 container/envelope for the manifold scene and its typed field parts;
field families and coordinate records are section kinds inside that shared format, not
competing outer file formats. Keep the 64-byte v1 physical header layout unless a separately
reviewed ABI change is required; v2 is the current writer version, and readers SHALL continue
to accept valid v1 files during migration. Live native/WASM transport SHOULD carry the same
bounded v2 section descriptors and payload semantics, using zero-copy slices or caller-owned
buffers where available. New part kinds are registered/versioned in the `.10d` section registry,
with checked length, alignment, limits, integrity and explicit required/optional behavior.

The v2 model SHALL keep `Tensor10D` as the universal manifold coordinate/address:
`[q,v,w,x,y,z,t,α,μ,σ]`. Its axes and declared manifold/domain conventions jointly
express an entity's manifold identity and address, with characteristics from which
shape and behavior are interpreted through PGA, field-family and simulation
conventions. The tensor coordinates are identity-bearing structure, not a hash used
to look up identity. A Q42 stable entity ID may provide a compact reference and
continuity handle across edits and replay; it does not define or replace that
manifold identity.

This gives a Tensor10D record the role of a compact **manifold signature**: an
entity's structured multidimensional identity and state in its declared manifold.
“Quantum signature” is a useful conceptual analogy for that identity-bearing,
coupled descriptor; it does not mean a cryptographic signature or claim that this
record alone is a complete physical quantum state.

Do not require duplicate, standalone shape or behavior records when those
characteristics are already implicit in the tensor plus its declared manifold and
domain. `.10d` v2 uses typed, versioned parts only for payloads that need explicit
storage: for example, renderable mesh topology, sampled scalar/vector/tensor/spectral
fields with native units and axes, material/rig data, solver state, or a
VibeScript/existing-solver behavior reference. Such parts attach to the relevant
Tensor10D node/record by stable section and record references; they do not redefine
its axes or duplicate the solver in the renderer. Parts may be absent when their
data is derivable or irrelevant. Bound, verify and stream each explicit payload
within the same `.10d` v2 envelope. This is the v2 composition contract, not a claim
that every part codec or runtime bridge is implemented.

Within that envelope, keep the repository's two 10D record types semantically and
structurally distinct. `Tensor10D` (`[q,v,w,x,y,z,t,α,μ,σ]`) is the universal
spatial/query/render coordinate record; `ManifoldCoordinate10D`
(`scale, attention_depth, epistemic_weight, topological_spin, temporal_decay,
entropy_bias, spatial_phase, recurrence_frequency, density_threshold,
manifold_curvature`) is a separate epistemic/attention record. They are typed parts
of one container, not interchangeable layouts or alternate names for the same axes.
Field samples may reference either or both by declared part kind and stable
record/section index; unit-bearing field-grid payloads retain their own schema. No
part may be flattened into or inferred from a different 10D record without an
explicit, versioned mapping. Rendering, packing or observer conversion MUST NOT
overwrite one part with another or discard provenance.
EMF is one field family in the existing scientific stack. Astrophysics/N-body
gravity, wave and diffusion fields, depth-aware sampling, time-dependent fields
and future field families must share a typed field/axis/manifold contract without
being coerced into `[R,G,B]`. Keep each field’s native units, coordinate axes,
time/depth, manifold tags and provenance until a declared observer/output
projection. EMF `[α,μ,σ]` is canonical spectral-field data, not an RGB tuple or
colour-map input. A viewer may request colour, audio, wavelength, or another
sensor projection, but this is a derived observation and MUST NOT replace or
mutate the field data, clamp its stored coordinates, or become the only available
view. The presence of an EMF solver/shader MUST NOT be reported as GPU/render
support for astrophysics, wave, diffusion, or other field families; each adapter
has its own implementation and qualification status.

Previously authored high-dimensional astrophysics work is part of the programme baseline and MUST be carried forward and reconciled with current implementation evidence. Distinguish historical/design work from validated runtime support: current code includes the versioned Tensor10D/container field substrate, an astrophysics N-body solver and a WGSL N-body kernel, while the inspected N-body solver documents a 2D direct-summation model. A missing presentation bridge or unverified higher-dimensional solver is not evidence that the earlier domain work was absent. Audit prior artifacts and current modules before proposing replacement; preserve their domain intent, units, invariants and dated status, and record any genuinely unimplemented dimension as a new implementation gap.

VibeScript SHALL own Studio orchestration where its invoke surface supports the
operation. One scriptable request should be able to pass a bounded simulation
result to the selected renderer, report the field dimensions/format and selected
backend, and preserve the same semantic result if the browser uses a CPU/Canvas
fallback. Do not invent renderer-local physics or duplicate an existing solver.
Where native and WASM scientific capability gates differ, report the unavailable
capability and route through the documented fallback instead of presenting a
mock field as calculated output.

### 3.2 Allocation, memory and lifetime

Follow QualiaDB `AGENTS.md`'s two-tier model: caller-owned flat buffers and zero
engine heap allocation in hot kernels/frame traversal; bounded cold construction
and caller-buffered outputs through `GeometryWorkspace` or the owning arena.
Preallocate frame lists, pose palettes, particle pools, upload rings and readback
slots; report overflow before writing. Driver-internal allocation is measured
separately from the engine allocation counter.

**Owner clarification, 2026-10-06: the 42 MiB ceiling does not cap asset payload
size, total pack size or aggregate asset residency.** Use the same architectural
distinction as inference: the semantic Sentinel arena and bounded governed passes
retain their limits, while graphics owns explicitly sized resource domains. Large
geometry, textures, rigs, clips and baked lighting remain external payloads reached
through validated references; they do not have to fit into an `NQuin` or `SlgArena`.

| Memory domain | Applicable contract |
| --- | --- |
| Semantic/Sentinel execution | `SlgArena` and work explicitly admitted to that domain remain within **42 × 1024 × 1024 bytes**. Semantic metadata and bounded hot kernels retain existing ABI/allocation rules. |
| Asset source and CPU residency | Pack/source/decoded geometry, texture, rig, clip and probe payloads may individually or collectively exceed 42 MiB. Use a declared asset CPU/WASM budget, streaming/mapping, validated ranges, eviction and checked size arithmetic. |
| Governed import/construction scratch | Keep existing per-chunk ingestion and `GeometryWorkspace` pass limits, including the 42 MiB admitted pass ceiling. Total jobs/output assets may span many chunks/passes. Do not exempt scene-building scratch or materialize the entire large asset as unbudgeted temporary copies. |
| GPU asset residency | Mesh/texture/rig/probe device resources use a separate graphics-asset VRAM budget with reservations and eviction; 42 MiB is not the total scene VRAM cap. |
| Graphics frame resources | Render targets, shadow maps, temporal histories, compute outputs and capture accumulation use explicit render-resource budgets. They are not asset payloads, but likewise are not placed in the semantic arena or capped by its total size. Declare their own transient/resident caps and use tiling when those caps require it. |
| Decode/upload/readback staging | Charge each allocation to its actual owner: governed chunk scratch obeys its pass limit; a graphics staging ring uses its declared staging budget. Count simultaneous copies and queue depth. Relabelling an execution-arena allocation does not waive its ceiling. |
| Inference resources | Keep inference's existing independent host/device budget. Shared rendering/inference admission accounts for weights, KV, scratch, graphics residency, frame resources and runtime headroom together. |

AG-20 SHALL implement this classification and separate counters/reservations;
AG-26 SHALL expose effective budgets on native and WASM. Synchronize affected
upstream memory specifications/manifests with this scoped owner instruction when
implementing it; this document does not claim existing renderer APIs already
enforce every separate quota. The exception concerns payload/resource ownership,
not hot-path allocation, deterministic construction or unbounded scene creation.

At admission, total live graphics/inference/device reservations plus headroom
must fit the declared device budget. Host/WASM residency has its own checked cap;
a 32-bit WASM address space and browser/device limits still apply. Native mmap
does not make resident pages free: distinguish mapped extent, live heap, resident
pages where observable and GPU copies. Handle shared/aliased allocations without
double-counting physical bytes or omitting logical ownership reservations.

Every resource class SHALL have a cap, admission estimate, high-water counter and
release path. Account for simultaneous old/new resources during resize, style
switch, streaming and device recovery; count mip chains, multisamples, histories
and staging copies. GPU retirement waits for completed work. Cache keys include
asset/format/compiler/shader/profile identities and relevant adapter/backend data.

Temporary compiler/capture files use uniquely owned RAII directories and byte
budgets. Retention promotes validated output to a caller-selected artifact
directory. Cleanup is marker-verified and limited to the configured run parent;
tests cover success, failure and unwind. No unscoped root logs or model variants.

### 3.3 Space, surface and colour conventions

Publish one convention for units, handedness, up axis, matrix layout, winding,
texture origin, tangent handedness and clip/depth ranges. Import explicitly
converts source conventions. Use tile/local origins for geographic precision;
retain original CRS/source positions in the semantic/geospatial pipeline.

The minimum material contract uses linear lighting and explicitly tagged colour
inputs: base-colour/emissive colour textures decode from sRGB where authored as
such; normal, roughness, metalness, occlusion and masks are data textures. One
output transform applies display encoding once. Do not grade normals or apply
gamma in both shader and sRGB attachment. HDR scene rendering does not by itself
mean an HDR display/output mode is supported. See [glTF colour conventions][R02]
and [ACES display encoding][R04].

Opaque, masked, blended and transmissive surfaces have separate depth, shadow and
pick policies. Alpha blending alone does not implement refractive glass.
Document material-space versus world-space normals, non-uniform transform handling
and all roughness/opacity ranges. Legacy SRD1 assets receive a declared default
material; unsupported required sections produce an explicit incompatibility.

### 3.4 Time, replay and selection

Authoritative time is the fixed simulation tick. Presentation uses an explicit
tick plus interpolation/subframe and can be reset or sought. Weather/wind/effect
seeds derive from stable identities and event/timeline data. Decorative ambient
effects need a declared scene seed/time, not a fabricated gameplay event.

Quality, culling, camera effects and dynamic resolution cannot change commands,
collision, navigation or replay state. GPU particles and cloth may be presentation
approximations; do not claim cross-device bitwise simulation or feed them back into
rules. A replay receipt proves identical events/seeds/timeline and state, while a
visual receipt proves the selected rendering policy within tolerances.

Picking resolves stable Q42 identity across batching, LOD, rig deformation,
streaming, style switch and device recovery. Specify screen-to-render coordinate
conversion for DPR, resize, jitter and dynamic resolution. Never temporally average
ID buffers. A transparent surface's pick-through policy is authored and tested.
Draw order or compacted GPU instance indices are not persistent semantic IDs.

## 4. Device profiles and graceful degradation

Profiles describe **capabilities plus budgets**, not vendor names or automatic
promises based on GPU branding. Query actual features, limits and format usages;
request only what a selected path consumes. Optional features differ between
native wgpu and WebGPU, as documented in [wgpu's feature contract][R01]. Measured
performance and user preference further constrain the selected path.

| Profile | Intended presentation | Proposed target to qualify | Typical fallback policy |
| --- | --- | --- | --- |
| P0 Accessible fallback | Qualia-owned map/low-detail view with all required entities, actions and state cues | Device-specific interactive floor to declare | Static authored cues, simple lighting, minimal effects; a view that omits the game scene is not action-equivalent. |
| P1 Portable low | Complete stylized raster scene | 1280×720 output, stable 30 fps | Low texture mips/LOD, one sun shadow or qualified contact proxy, baked environment/probes, spatial AA, analytic fog, static secondary motion. |
| P2 Portable standard | Finished stylized slice; core photoreal materials | 1920×1080 output, 60 fps | Bounded clustered lights, cascaded sun shadows, AO, probes, temporal AA/upscaling where qualified, limited reflections. |
| P3 Enhanced | Higher-fidelity photoreal/stylized rendering | 2560×1440 output, 60 fps initially | Better GI/reflections, volumetrics, surface lobes and crowd detail, selected only when supported and within budget. |
| P4 Cinematic capture | Fixed-camera/shot output, accumulation and export | 3840×2160 offline capture target; no interactive-fps claim | Tiled/bounded capture; optional native ray queries/path tracing; portable raster capture remains available. |

These resolutions/rates are **proposed test targets**, not certified minimum
hardware, achieved results or universal browser support. Native and WASM may each
qualify P1–P3 at different feature levels. P4 may have a narrower output-format or
backend scope; state it explicitly. Do not block portable cinema-quality art on
hardware ray tracing, mesh shaders, bindless resources or an upscaler vendor SDK.

### 4.1 Required capability and quality report

Expose a versioned report through both runtime adapters containing runtime,
backend, adapter category/available identity, requested/effective features and
limits, supported target usages, selected profile, output/internal resolution,
feature implementation and fallback, memory caps, timing method and failure reason.
Do not infer unavailable browser driver/VRAM details; label them unavailable.

Each feature resolves to **full**, **reduced**, **substitute**, or **unavailable**,
with a reason and authored policy. Examples: SSR → reflection probe; dynamic GI →
baked probes; volumetric fog → analytic fog; skeletal secondary motion → authored
pose; compressed texture → bounded uncompressed/low-mip variant. If a substitute
would remove a gameplay cue, retain a static marker or use P0. Missing essential
resources cannot be converted into a success receipt.

Separate hard eligibility from adaptive quality. Use hysteresis, cooldowns and
bounded transitions for sustained frame/memory pressure. Preserve player settings
and accessibility floors. Reduce optional sample counts, effects and resolution
before compromising interaction or state readability. Record quality changes;
invalidate affected histories. Recovery/replay tests force each transition.

### 4.2 Budget and measurement contract

Before a work order starts, freeze its device/runtime matrix and a budget manifest.
Required fields: output/internal sizes; p50/p95/p99 frame interval; CPU preparation,
simulation and GPU-pass targets; load/decode/upload time; pick latency; resident
CPU/WASM bytes; GPU ledger bytes; transient/staging/cache caps; asset/draw/light/
joint/particle limits; upload bytes per frame; and capture output budget.

For initial P2 allocation, evaluate **16.67 ms p95 frame interval**, CPU render
preparation ≤3 ms and GPU rendering ≤12 ms, leaving headroom in their respective
overlapping schedules. For P1 evaluate 33.33 ms, ≤6 ms preparation and ≤25 ms GPU.
These are planning targets. They are not additive proof of present latency and
must be revised openly if the qualified device cannot meet them. Pick completion
target: ≤2 displayed frames p95; immediate selection/command acknowledgement
target: the next displayed frame. Existing QG requirements still apply.

Use at least 120 warm-up frames and three 1,800-frame samples per steady-state
scenario, plus separate cold-load and 20-minute streaming/thermal runs. Freeze
camera/timeline/quality for comparisons. Record stalls separately; do not hide
shader compilation, streaming or recovery in warm-up exclusions. VSync-paced frame
intervals and GPU timestamps answer different questions. If timestamps are absent,
label CPU submit/completion-clock measurements accurately; never call them GPU pass time.

Browser GPU memory is an application resource-ledger estimate unless independently
measured, not observed total VRAM. WASM linear-memory high-water may not shrink
after free; test live allocations and repeated-cycle plateau as well as pages.
Hardware caps and numeric per-domain memory budgets remain open until AG-20/26
baseline qualification; their separation from the semantic 42 MiB arena is settled
by §3.2. Unspecified numeric budgets block closure, not planning or interface design.

## 5. Renderer architecture and execution order

### 5.1 Preferred portable architecture

Evaluate **clustered forward rendering** as the initial portable path. It can
avoid a large material G-buffer while supporting many lights and transparency;
[Filament's design][R05] supplies a primary implementation reference. This is a
Qualia scheduling recommendation, not permission to embed another engine.
Retain a simple forward path for P1. Deferred/visibility-buffer paths need an
equal-quality bandwidth/frame-time comparison before becoming an additional backend.

Build a bounded frame/resource graph with pass inputs, outputs, formats, load/store
policy, lifetimes and profile variants. A minimal frame is conceptually:

```text
validated scene snapshot + tick/subframe + selected capability profile
  -> bounded visibility / LOD / pose / light lists
  -> shadow updates and optional depth/normal/velocity prepass
  -> opaque + alpha-tested forward material lighting
  -> shared depth hierarchy / AO / qualified reflection or GI inputs
  -> transparent water/glass + effects + volumetric composition
  -> temporal reconstruction or spatial AA + selected HDR post effects
  -> one exposure/look/output-transform chain
  -> accessible interaction overlays / UI + present
  -> asynchronous pick/readback/telemetry receipts
```

This is a dependency sketch, not a fixed universal pass order. Specify which
lighting inputs must precede opaque shading, how opaque colour is captured for
refraction, and which post effects operate before/after reconstruction. Split or
fuse passes only when dependencies and measurement permit it. ID picking uses a
consistent depth/geometry policy independent of post effects and the displayed UI.

Reuse depth pyramids, scene normals, velocity, exposure and history validation
across consumers where their conventions agree. HZB culling, reflection traversal
and other consumers may need different min/max reductions: declare this rather
than reusing an incompatible hierarchy. Minimize unnecessary attachment stores
and fullscreen bandwidth, guided by [tile-based rendering practice][R06]; native
Vulkan subpasses or extensions are not assumed to be available in WebGPU.

### 5.2 Cold planning versus hot execution

Cold planning validates content, selects feature/format variants, compiles
pipelines, sizes arenas and prepares immutable draw/material bindings. Hot
execution updates preallocated records, encodes selected passes and submits bounded
work. Loading and shader preparation are staged; no first-use compilation on an
interactive critical path. A cache miss has an explicit temporary compatible
variant or loading state, never a black-frame success.

AG-20 must decompose GPU ownership, frame planning, execution, upload/residency,
readback, backend construction, receipts and tests into focused libraries. Suggested
owners such as `render/material/`, `render/lighting/`, `render/visibility/`,
`render/animation/`, `render/effects/` and `render/post/` are proposals to reconcile
with existing modules. New implementation files should remain below 500 lines;
oversized existing modules require the tracked ownership/decomposition review.

### 5.3 Corrected dependency order

| Stage | Capabilities and dependency boundary |
| --- | --- |
| Foundation A | AG-01 first fixture; minimal AG-20 resource planning, AG-26 capability negotiation and AG-28 diagnostics accompany it. AG-19 floors begin now. AG-01 must not wait for the complete frame graph. |
| Surface/light B | AG-02 + AG-04 base shading; AG-03 textures; AG-05 shadows; AG-06 sky; AG-07 full colour pipeline; AG-22 IBL/probes. A declared simple output transform already belongs to AG-01. |
| Production E, started early | AG-14 compiler increments alongside B, then AG-16 LOD tooling. AG-15 characters/foliage consumes AG-11/12/25 as applicable. Avoid producing a large unvalidated catalogue before pipeline proof. |
| Scale/world C | AG-08 portable instancing/culling; AG-09 terrain/streaming; AG-10 water adds AG-23. CPU visibility/basic LOD does not wait for temporal AA or GPU occlusion. |
| Motion D | AG-11 QG-03/13 rig/playback; AG-12 wind does **not** require skeletal clips; AG-13 effects need depth/alpha contracts, not shadow maps. AG-21 consumes all deformation velocities as they arrive. |
| Cinematic quality | AG-21 temporal stability; AG-23 reflections/transparency; AG-24 volumetrics; AG-25 advanced surfaces. AG-17 styles and AG-18 interaction polish are integrated throughout. |
| Capture | AG-27 portable cinematic camera/export after a qualified slice. AG-29 optional native/reference ray rendering never blocks portable exits. |

Surface, pipeline and scale work can overlap once their specific dependencies
pass. AG-19, AG-26 and AG-28 gate every exit. AG-16 is LOD tooling; AG-18 is
selection feedback. These correct the swapped labels in the original dependency sketch.

## 6. Stable capability register: AG-01–AG-19

Each item below inherits §§3–5 and the optimization requirements in §8. Its
portable core needs native plus full-WASM evidence; enhancements declare narrower
qualification. Acceptance includes the relevant fixtures in §9 and receipts in §11.

### AG-01 — Rendering correctness conformance

Extends QG-12. Establish independent opaque near/far cubes, colour patches,
overlapping surfaces, resize and bloom-on/off tests before judging art. Compare
unlit diagnostic output against a CPU colour/output-transform oracle; a lit
material is not expected to display its raw authored albedo unchanged.

Specify clears, depth compare/write, winding, blend modes, target formats and
linear/display output. Test zero sun/ambient intensity explicitly; existing shader
default handling must not make zero indistinguishable from an unset value. Cover
black viewport, ghost surfaces, near-plane clipping, alpha, non-square targets and
fallback transitions. Use native readback and a real browser readback/capture path.

**Close:** expected colour tolerances and full occlusion pass on selected native
and browser devices; no validation errors; picks agree; every historical failure
has a regression case or a recorded non-reproduction. CPU/headless software GPU
checks supplement rather than replace qualified hardware receipts.
**Depends:** none. Enables all visual comparisons.

### AG-02 — Versioned `.10d` material model

Extends QG-02. Specify stable material IDs, submesh/face ranges and vertex-colour
multiplication; base colour, roughness, metalness/specular policy, emissive,
normal/occlusion slots, opacity mode, cutoff, sidedness and shadow/pick policy.
Use a metallic-roughness microfacet baseline; stylized ramps/lobes are an explicit
model with compatible resource semantics, not ad hoc changes to unrelated shaders.

Sections define endian/alignment, lengths/count caps, defaults, required/optional
flags, dependency digests and migration. Old static assets still render through
defaults; old readers reject required unknown semantics. Per-face materials compile
to bounded submesh ranges; no material allocation or texture bind per triangle.
**MAT3 v3 layout now implemented:** section type `Materials=13`, 16-byte aligned; 24-byte little-endian header; 448-byte material records; 24-byte submesh ranges. MAT3 keeps MAT2 UV transforms and adds six 8-byte sampler records in bytes 400..447. Each sampler stores wrapS, wrapT, magFilter and minFilter enum bytes plus four reserved zero bytes; MAT3 rejects unknown enum values or non-zero reserved bytes. Readers preserve MAT1 v1 (256-byte records, identity UV transforms and glTF-default samplers) and MAT2 v2 (448-byte UV-transform records, glTF-default samplers). Material IDs are stable, non-zero `u64` hashes in ascending order and imported IDs are scoped by asset identity. Six 32-byte SHA-256 references separately bind base-colour, normal, metallic-roughness, occlusion, emissive and optional stylized-ramp resources; payload bytes remain external dependencies. Core embedded GLB image bufferViews are digested during import; external/data URI images still require the AG-03/HMC resolution path. Factors are linear; records carry base colour, emissive, metallic, roughness, specular, normal scale, occlusion, alpha cutoff, shading/opacity model, two-sidedness, shadow/pick policy, vertex-colour multiplication, six UV0 transforms and per-map sampler state. Current caps are 4,096 materials and 1,048,576 ranges. Ranges are ordered, contiguous, triangle-aligned and cover the complete index stream; unknown versions/models/opacity modes/flags, invalid references, non-finite/out-of-range factors, malformed sizes and CRC failures reject. Decode writes to caller buffers. A missing material section uses the legacy white dielectric default; a present unknown required section fails closed. No per-triangle material allocation or texture bind is allowed.

**Close:** bark, plaster, metal, soil and cloth differ under the same rig; CPU/GPU
BRDF samples and white-furnace/energy tests cover the physical model. Glass receives
its own AG-23 acceptance. One building/prop round-trips through HMC, both runtimes,
stable picks and repeat compilation. **Depends:** AG-01; AG-20 resource ABI.

### AG-03 — Texture compilation, sampling and residency

Extends QG-02/14. Specify UV sets/transforms, sampler/wrap/filter/mipmap policy and a bounded anisotropy tier, usage
colour tags, mip chain, GPU format alternatives, texture dependency digests and
rights. Prefer offline mips and compressed distribution; capability-select BC,
ETC2 or ASTC only where supported, with a bounded uncompressed/low-mip alternative.
KTX2/Basis is a distribution/transcode route, not universal GPU format support. The compiler SHOULD retain one verified source payload and select a GPU-native transcode target from the active adapter intersection (ASTC, BC, or ETC2), with a bounded uncompressed/low-mip fallback. For glTF-compatible inputs, colour maps (base colour/emissive) SHALL use sRGB interpretation; normal, metallic-roughness and occlusion data SHALL remain linear. Metallic-roughness reads G=roughness and B=metallic; vertex COLOR_0 is an additional linear base-colour multiplier. Offline mip pyramids are preferred; Basis/KTX2 may carry mip levels and stream coarse-to-fine, while decoder/transcoder work and peak scratch bytes are separately budgeted. For Basis targets, ETC1S is the compact colour candidate and UASTC the higher-quality non-colour candidate; unsupported compressed formats transcode to an adapter-supported format, with non-colour data allowed a bounded RGBA fallback. Small uploads MAY use direct queue texture writes; large/streamed uploads SHALL use bounded staging and account transient and resident bytes independently. Verify channel semantics, transfer function, sampler/wrap/UV selection and mip completeness before GPU allocation.

Validate decoded dimensions, blocks, levels and decompression/transcode budgets
before allocation. Generate normal-map mips with declared normalization. For alpha-mask materials, preserve the base-level covered fraction at each authored alphaCutoff independently; alphaCutoff is part of the mip-residency identity, so incompatible thresholds MUST NOT alias one corrected chain. If a small mip cannot represent the target fraction exactly, choose the nearest attainable covered-pixel count deterministically and report the residual rather than claiming exact preservation. Keep RGB filtering in the declared colour space and alpha linear. Prefer corrected mips prepared in the bounded local asset pipeline and carried with the asset; runtime GPU generation MAY derive cutoff-specific correction scales from alpha-only scratch, but MUST NOT require GPU readback or unbounded synchronization. This follows the established generate-then-scale coverage method [R32]. Prevent atlas mip bleed with padding;
use array/atlas layouts only when they preserve sampling/material semantics. Embedded source images SHALL be emitted as separate HMC resources keyed by SHA-256, with MIME metadata retained; resolution SHALL verify the HMC entry digest against MAT1 and return a zero-copy slice before decode/transcode. Multi-asset HMC assembly MUST de-duplicate equal digests deterministically.

**Close:** textured building/foliage at both zooms; no mip halos, missing-dependency
success or colour/data confusion; native/WASM load and forced format fallback;
resident/staging/upload accounting and license/digest integrity. MAT3 sampler wrap/filter state SHALL be preserved per texture role; unsupported sampler enums fail closed. The shared sampler cache SHALL be bounded and keyed by normalized state. Filtering and non-filtering WebGPU bindings SHALL remain type-correct; coverage/shadow passes SHALL use the base-colour sampler. Mip selection must match available levels, and anisotropy remains at 1 until an adapter-capability tier is measured and qualified. **Depends:** AG-02.

### AG-04 — Authored normals, tangents and production direct lighting

Extends QG-12. Preserve smooth/flat authored normals, tangent/sign, UV seams and
hard edges through compile/decode/LOD/skinning. Generate missing tangents in the cold compiler using the [MikkTSpace convention][R07] or a declared equivalent. A simple UV-derivative tangent is permitted only as a labelled fallback for preview/non-normal-mapped materials; it MUST NOT be presented as MikkTSpace-compatible. Production normal-map baking/runtime SHALL use matching MikkTSpace tangent generation, preserve UV and hard-normal discontinuities by splitting corner vertices where required, and retain mirrored handedness. Degenerate UVs SHALL produce a deterministic fallback and a diagnostic that disables or downgrades normal mapping.
Specify inverse-transpose normal transforms and mirrored/non-uniform transforms.

The `.10d` compiler SHALL preserve vertex-aligned authored shading frames in optional FieldSidecar section type 8 without changing the QuantizedMesh or Tensor10DNodes payload. Normals-only version-1 NRM1 retains its 16-byte header and stores two little-endian signed-normalized 16-bit octahedral coordinates (4 bytes per vertex). Version-1 FRM1 has a 16-byte little-endian header (`FRM1`, version, flags, vertex count, zero reserved field); flag bit 0 means an octahedral SNORM16 normal pair (4 bytes), and bit 1 means tangent XYZ plus handedness as four SNORM16 values (8 bytes), interleaved per vertex in that order. Valid flag values are 1, 2 or 3; normal-only encoders SHOULD retain NRM1 for byte compatibility, while FRM1 supports tangent-only and combined streams. Both formats require exact vertex-count alignment and exact payload length; directions must be finite and non-zero, and tangent handedness SHALL decode to exactly -1 or +1. NRM1 legacy containers and mesh-only containers remain valid. Consumers SHALL dispatch on self-describing field magic/kind, ignore unsupported optional field kinds and absent streams, and fail closed on malformed NRM1/FRM1 data; shading frames MUST NOT replace or collapse scientific manifold or field records.

Integrate bounded directional, point and spot lights, explicit physical/artistic
units, range attenuation and key/fill policy. Reuse existing scene light data after
qualification. Adopt O-01's clustered path for light scale; small scenes retain a
cheap simple forward path. Rim response must be view-dependent if intended as a
view rim; world-Z facing is not a general camera-relative Fresnel model.

**Close:** rounded pod curvature, textured sphere, mirrored UV seam, hard-edged
building and transformed/skinned normals pass fixed cameras; night lamps light
nearby surfaces; bounded light-list overflow is deterministic and reported.
**Depends:** AG-02; AG-03 for maps; AG-20 for scalable lights.

### AG-05 — Stable shadows and ambient occlusion

Extends QG-12. The portable floor starts with one sun `Depth32Float` map using
`RENDER_ATTACHMENT | TEXTURE_BINDING` and a comparison sampler; caster geometry is
rendered once to depth, then receivers use bounded 3×3 PCF with explicit level-0
comparison. Use `textureSampleCompareLevel` where per-fragment range checks are
divergent: WGSL specifies that it avoids derivative computation and does not
require uniform control flow [R28]. Select a light-space orthographic fit from
scene bounds, snap its center to shadow texels, and combine caster depth bias with
receiver slope bias. Reuse retained geometry/index buffers; omit the map pass when
caster/material/light state is unchanged. Admit the target before allocation and
try a declared descending resolution ladder; if admission fails, disable the pass
and preserve the unshadowed renderer.

The next tier adds camera-range cascades for Map/Walk, stable fitting, overlap and
smooth transitions [R08]. Shadow alpha tests and deformed geometry must match the
visible material before those paths are declared supported. Higher-cost contact
or softening methods are optional and separately budgeted.

AO remains a separate indirect-visibility approximation with radius/strength and depth/normal inputs. The portable floor SHALL evaluate at reduced resolution and reconstruct with depth/normal-aware filtering. Its bounded tap count and target dimensions MUST follow the active graphics profile and FrameTarget admission; on pressure, reduce sample count/resolution or omit AO while preserving the rest of the renderer. Under the current fixed-FOV perspective camera, reconstruction SHALL use pixel-centre NDC, the shared projection aspect and camera basis to recover view/world positions from linear depth; it MUST NOT perform a full inverse-view-projection transform and ray normalization for every AO tap. Odd-sized viewports use the source camera aspect, not the rounded half-resolution dimensions. Any projection/FOV change updates the shared camera contract. Compare this path against projection/reprojection tests and adapter pixel fixtures; claim a performance gain only after target-device measurement.
The initial profile SHALL use half-resolution depth/normal inputs, a bounded 4/8/12-tap evaluator, and bilateral edge-aware reconstruction. Higher profiles SHOULD evaluate CACAO-style cache-friendly deinterleaved depth/normal layers and a depth mip hierarchy when benchmarked quality gains justify the extra residency and pass bandwidth [R31]. This is a method reference, not a requirement to copy CACAO code or claim equivalent performance.
Dynamic visibility SHALL attenuate indirect ambient only; it MUST NOT darken direct illumination or emissive output, and it MUST NOT multiply baked material occlusion as a second copy of the same visibility term. AO is separate from and does not replace directional shadows.

**Close:** worker feet, canopy shade, fine foliage and sloped ground remain grounded
without acne, detached shadows, cascade swimming or AO halos; light/instance churn
and reduced profiles meet budget. **Depends:** AG-04 and AG-20 depth/resource
contract. AO does not substitute for directional shadows.

### AG-06 — Sky, atmosphere and distance depth

Extends QG-12. Provide authored sky/analytic fog first, then an optional LUT-based
scattering path. Sky, sun direction/radiance and environment lighting must share
one scene state. Define horizon continuity, height/distance fog, time/weather
parameters and exposure interaction. Keep LUT resolution/update caps and banding
control explicit. Stylized colour authoring may override the physical look through
the declared style model without changing source geography.

**Close:** horizon, sunset, night and camera-height changes at actual game size;
no sky/ground discontinuity or unacceptable bands; matched environment lighting
and analytic fallback. **Depends:** AG-01; AG-07/22 for final coherent lighting.

### AG-07 — Exposure, tone mapping, colour management and grading

Extends QG-12. The working space for the current material renderer is scene-linear
Rec.709. Manual exposure is specified in stops, bounded, resettable and held fixed
for reference captures; automatic exposure is a later optional feature and must
exclude HUD/selection. Preserve requested manual exposure across resize and device
recovery, report when a fallback cannot apply it, and expose active-path capability.
Bloom threshold/intensity use pre-exposure scene-linear units. Render all opaque and
transparent geometry into a scene target, then apply exposure and the output transform
once after blending and before display encoding; do not transform object fragments
individually. Keep HUD/selection outside scene exposure. Preserve an RGBA16F scene
target when supported; fallback targets must expose their reduced highlight capability. On WebGL2, gate RGBA16F on `EXT_color_buffer_float` and framebuffer completeness, reserve colour plus depth bytes before allocation, and fall back to a budgeted RGBA8 target or direct canvas rendering if admission/allocation fails. Release the old-size target before admitting its replacement so resize does not require a transient double-target budget. Expose active output-transform and extended-range capability separately.

**Selected portable SDR output v1:** implement Khronos PBR Neutral with the exact
reference coefficients and operation order [R33]. The transform accepts non-negative
scene-linear Rec.709 and returns display-linear Rec.709 in [0,1]. An sRGB surface encodes on write; a linear UNORM fallback explicitly applies the same sRGB transfer function in the composite, so the display encoding occurs exactly once. `SDR_OUTPUT_TRANSFORM_PBR_NEUTRAL_V1`
identifies this contract. Preserve a CPU oracle and golden grey/saturated/highlight
ramps. Do not label a different filmic curve as ACES: an ACES output transform is a
separate scene-to-display rendering plus display-encoding pipeline [R34]. PBR Neutral
v1 does not define HDR/wide-gamut output. That path requires negotiated display
capabilities, an explicit output transform/encoding and independent reference tests.
Stylized looks may add versioned grading before display encoding; fuse compatible
operations only when measured and preserve one display encoding. The current shared
artistic white-balance control is a bounded diagonal gain in scene-linear Rec.709:
temperature and tint each clamp to ±1 stop, zero maps to identity gains, and the gains
apply once alongside exposure before PBR Neutral v1. This is a look control, not a
Kelvin-based camera calibration or an ACES Input Transform/IDT [R36]. Any future
calibrated camera workflow must name its source white/encoding and implement a separately
versioned chromatic adaptation/input transform; it must not silently reinterpret these
artistic controls. Keep LUT grading a later versioned operation with explicit domain,
shaper, precision, and fallback requirements.

**Close:** grey ramps, saturated colours, emissive highlights, indoor/outdoor
transition and three sky states remain coherent with bloom on/off and profile
changes. Golden images use fixed exposure/output metadata. **Depends:** AG-01/06;
AG-02/04/22 for final lighting captures; minimal correctness transform stays early.
### AG-08 — Instancing, batching, culling and runtime LOD

Extends QG-12. Separate immutable meshes/materials from instance transforms,
semantic IDs and state. Batch by compatible mesh/material/pass; retain stable
ID indirection. Start with caller-buffered CPU frustum/LOD lists and instanced
indexed draws. Add GPU compaction/indirect submission and conservative HZB culling
only if O-06 proves benefit at the declared workload.

**CPU visibility contract v2:** `render::instance_culling::select_visible_instances` and its WASM-friendly `_packed` variant accept caller-owned bounds/transforms, a column-major WebGPU view-projection matrix and caller-owned `u32` source-index output. They preflight worst-case capacity, preserve input order, use the WebGPU `[0,w]` clip-depth range, and cull only when an AABB support-radius test proves it lies outside a homogeneous clip plane. The six planes are extracted once per view and transformed into local space. Non-finite/inverted bounds and non-finite transforms fail open. The packed API uses three bound floats and 16 column-major transform floats per instance; all arrays must have matching counts. Both culling APIs are bounded to 10,000 instances. Bounds for skinned, wind-deformed or displaced geometry must include conservative deformation margins.

`GpuInstanceRecord` ABI v2 is 128 bytes: 64-byte column-major world transform, a 48-byte CPU-precomputed inverse-transpose normal frame, two `u32` semantic-ID words, determinant orientation sign and padding. Construction is cold-side; renderer validation rejects non-affine, singular, non-finite or forged normal frames. This preserves correct normals under non-uniform and reflected scale without per-vertex inversion. `compact_visible_records` and its alignment-independent packed-byte variant validate capacity and all source indices before copying and preserve order and semantic identity without allocating.

The GPU renderer now uploads the caller-owned packed stream through a budget-accounted storage buffer, grows it transactionally up to 10,000 instances, and issues instanced indexed draws in the direct/HDR material path, shadow pass and AO depth prepass. The WASM portal exposes ABI version/stride, caller-buffered record packing, culling, compaction and upload; hosts can run cull → compact → submit with no JavaScript record-layout copy. Empty upload restores the legacy identity instance. Multi-instance BLEND is refused until sorted transparent instance submission exists. The renderer does not yet call the CPU selector automatically from its frame loop, and instance identity is not yet returned by picking. GPU compaction/indirect draws remain benchmark-gated by O-06.

Use projected error/coverage, hysteresis and explicit transitions, not distance
alone. Bounds include wind, skinning and displacement. Cuts/teleports invalidate
occlusion history. An uncertain visibility test draws the object; false-negative
culling of visible gameplay objects is unacceptable. Meshlets/virtual geometry
are optional measured extensions, not a requirement for all devices.

**Close:** 100 agents, 50 structures, 500 props plus terrain at both zooms; also
stress 10,000 repeated instances to expose batching limits. Record actual mesh,
triangle/material/draw counts, p95/p99 and allocations. Picks remain correct after
compaction, LOD, tile churn and camera cuts. **Depends:** AG-01/02/20; AG-16 supplies
production LODs. Base culling need not wait for the GPU depth hierarchy.

### AG-09 — Joined terrain, geography precision and streaming

Extends QG-12/17 and doc 30. Compile terrain from declared source coordinates,
height data and validity masks; separate physical/source height from authored
visual exaggeration. Use tile-local origins, deterministic neighbour boundaries,
seam/LOD stitching and continuous roads, waterways and material layers.

Use O-07's measured tile/quadtree or clipmap selection. Residency separates
requested, validated, staged, resident and retiring assets, with bounded queues,
cancelation, incremental uploads and deterministic eviction ties. Render holes
have a declared low-detail parent; navigation/collision never read an absent
high-detail render tile. Offline packs and a teleport/thrashing test are mandatory.

**Close:** joined four-tile scene and multi-sector route with no visible cracks,
height-query/pick/navigation disagreement or memory growth; local-origin rebasing
preserves identities. **Depends:** AG-08/14; AG-03 for layered surfaces; QG-17 owns
source geography, placement and terrain derivation.

### AG-10 — Water surfaces and shore interaction

Add flow/ripple normals, depth/shore tint, roughness/specular response, foam and
optional refraction/reflection. Prefer O-08's analytic/texture-driven shallow
water for creek/canal; FFT ocean simulation requires a separate evidenced need.
Tide/flow parameters are authored or rule-driven facts; decorative waves do not
claim flood/hydrology accuracy. Displacement expands bounds and supplies velocity.

**Close:** Saltwind low/high tide, shallow/deep shore, bridge overlap and underwater
view policy at both zooms; no refraction sampling the wrong foreground layer,
reflection-edge popping or pick ambiguity. Probe/simple-water fallback preserves
tide and navigability cues. **Depends:** AG-03/04/09/23; AG-21 for temporal motion.

### AG-11 — `.10d` rigs, clips, deformation and animation runtime

This is QG-03/13's owner, referenced here rather than duplicated. Specify hierarchy,
bind/inverse-bind poses, capped influences, normalized weights, clip sampling,
blend state, articulation/morph sections and compatibility. Root motion is
explicitly reconciled with authoritative movement; animation cannot move rules
or navigation by itself. Provide stable previous/current pose data for velocity.

Use O-09's measured vertex versus compute skinning and shared pose/clip strategy.
Preserve zero-heap playback, bounded interpolation, reduced-motion poses, seek/
pause and crowd LOD. Existing kinematic/IK functions require allocation audit.

**Close:** one original worker idle/walk/work and an articulated facility;
100-agent crowd, retarget/invalid-rig diagnostics, save/replay/seek and correct
shadow/pick/deformation bounds. **Depends:** QG-03/13, AG-02/20; AG-04 verifies lit
normals, AG-21 consumes velocities. Clip-format work can precede final lighting.

### AG-12 — Vegetation and secondary motion

Specify species assets, two-sided leaf response, alpha mask, wind weight/bend
channels, seeded phase and scene time. Prefer analytic vertex deformation and
instanced clumps/cards with coverage-preserving LOD; author species variation.
Near-detail geometry, distant cards/impostors and static fallback share identity.
Measure alpha overdraw, not only triangles. Procedural wind is independent of rigs.

**Close:** gum, lavender and garden family at both distances; matched deformation
in shadow and velocity passes; no leaf halos or LOD thinning; deterministic
timeline/seek and reduced-motion static cue. **Depends:** AG-02/03/04/08;
AG-21 for temporal qualification, AG-11 only for skeletal secondary motion.

### AG-13 — Particles and event-linked environmental effects

Specify emitter descriptors, scene/event seeds, fixed presentation steps or
analytic age, capacity/overflow priority, spawn/retire and seek policy. Existing
ambient GPU particles are a candidate, not a general emitter contract. Start with
bounded CPU/analytic emitters; GPU update/compaction is conditional on scale.

Use depth-aware soft intersections, instanced quads, culled emitters and
screen-coverage/overdraw budgets. Reduced-resolution smoke requires edge-aware
composition. Gameplay effects originate from authoritative receipts; ambient
weather originates from a declared scene timeline. Reconstruct from seed/time
after seek rather than frame-rate-dependent accumulated randomness.

**Close:** rain plus work-site dust/sparks and canal splash; pool exhaustion,
replay seek, reduced motion, transparent overlap and foreground-depth tests.
**Depends:** AG-20 depth/time contracts and AG-02 alpha; AG-23 transparency;
AG-04 for lit effects. Shadow maps are not a prerequisite for soft particles.

### AG-14 — Production compiler and interchange ingestion

Extends QG-02/14. Extend the existing OBJ/STL/GLB/import/`.10d` compiler into a
validated, complete material/texture/rig/LOD dependency pipeline. Qualia-native
procedural authoring remains first-class. Standard interchange is an input to
Qualia, not a dependency on external DCC software or a second runtime renderer.

Support a declared glTF/GLB subset and extension policy; reject unsupported required
extensions. Preserve units, transforms, pivots, UVs, tangent seams, material ranges,
skins, clips and provenance. Use bounded validation of offsets/counts, NaNs,
degenerates, graph cycles, external references and decoded resource sizes.
No hidden runtime fetch for a supposedly offline pack.

**Close:** hero building and worker, deterministic repeated compile under pinned
tool/configuration, complete native/WASM HMC load, readable diagnostics, corrupt
input/oversize/cancel tests and incremental dependency rebuild. Cross-machine
byte identity is qualified only with canonical math/ordering/toolchain controls.
**Depends:** QG-01/02/14; AG-02/03; animated increment uses AG-11. Begin early.

### AG-15 — Reusable character and botanical production families

Extends QG-03/13 and AG-14. Specify character topology, rig/influence limits,
face morphs/expression conventions, hands/clothing, attachments and reusable
animation sets. Botanical families include source dimensions, species morphology,
growth variants, card/geometry LOD, normal/alpha/wind channels and deterministic
variation. Asset QA includes strategy silhouette, Walk detail and screen coverage.

Hero skin/eyes/hair/cloth select AG-25 features with authored lower-tier variants;
no requirement to run strand hair or cloth simulation on every crowd member.
**Close:** original worker and foliage family produced through one pipeline,
idle/walk/work/expression and static variants, pick/shadow quality and crowd
budgets. **Depends:** AG-11/12/14/16; AG-25 for selected advanced surfaces.

### AG-16 — LOD compiler, mesh optimization and validation

Extends QG-02. Reuse existing decimate/remesh/quality modules; add attribute-aware
error bounds, seam/material/skin preservation, rig/leaf coverage quality and
semantic pick anchors. O-05 defines vertex/cache/fetch/quantization optimization.
LODs carry bounds, source relationship, compiler settings, max error and digests.

Do not accept triangle reduction that destroys silhouette, texture seams,
normals, face expressions or selection proxies. Impostors are optional authored
render representations; they do not replace collision/semantic geometry.
**Close:** building, worker, foliage and terrain LOD chains pass fixed screen-space
error/coverage thresholds, reproducible compile and AG-08 transitions/picks.
**Depends:** AG-14 increment; AG-11 for skin/morph-sensitive chains.

### AG-17 — Style profiles and art-direction management

Generic style-profile machinery belongs in the engine; Storybook/Earthlight/
Community Grounds names and content remain game-authored. A versioned profile
binds material variants, palette/look transform, light/sky/environment script,
effect authoring and feature fallbacks. Style changes are presentation preferences
and cannot alter facts, source geography or campaign saves.

Avoid recompiling shaders or duplicating identical textures/geometry per style;
use content-addressed sharing and bounded variant/prewarm sets. Preserve current
separate HMC compatibility until an explicit deduplication migration is tested.
**Close:** identical camera/world displayed in all three styles, daylight/night
and reduced profiles; repeated swaps plateau in live memory and reset histories
correctly. **Depends:** AG-02/07/22/26; incremental style scaffolding starts earlier.

### AG-18 — Selection, decals and world feedback

Extends QG-10. Provide semantic-ID-driven outlines, terrain-conforming rings,
command markers, placement previews and decals. Depth/polygon offset and alpha
policy prevent flicker or unintended occlusion. Feedback remains legible at
strategy scale and after reconstruction; choose explicit scene/display-space
composition policy, with stable text/UI at output resolution.

Colour cannot be the sole state cue. Offer shape/pattern/label and reduced-motion
variants; author visibility against every style/day/night/background. A 2D
fallback can convey the same entities/actions without pretending to render decals.
**Close:** 100 mixed selections, transparent overlap, slopes, LOD, DPI/resize and
dynamic resolution; interaction latency and accessibility evidence.
**Depends:** QG-10, AG-01/20/26; AG-04/05 improve lit grounding but do not gate basic cues.

### AG-19 — Accessibility, quality floors and performance admission

Extends QG-18. Every exit includes reduced motion, persistent user controls,
photosensitivity-reviewed effects, independent camera shake/motion blur/DoF
controls, legible markers and action-equivalent fallback. Declare baseline
hardware and qualification, resolution scaling limits and frame/pick budgets.

Automatic degradation preserves essential cues and cannot silently downgrade a
feature's acceptance scope. Test transitions under forced load, thermal/memory
pressure and unsupported-capability reports. Human review supplements automated
contrast/flash checks; colour-blind simulation alone is not accessibility proof.
**Close:** every declared supported profile passes its floors on its device/runtime
matrix, with known limits recorded. **Depends:** continuous from AG-01; all features
contribute evidence. This is not a final-phase-only task.

## 7. Added foundations and cinematic capabilities: AG-20–AG-29

### AG-20 — Frame graph, resource ownership and shader lifecycle

Extends QG-12. Define bounded passes/resources and a reusable material/light/scene
ABI; separate cold planning from frame execution. Decompose the oversized GPU
module while preserving behaviour. Track attachment bandwidth, working-set and
residency budgets, upload/readback rings, cache invalidation and safe retirement.
Use Naga validation and existing shader/Forge infrastructure where applicable;
do not require arbitrary generated shader repair or new shaders for every style.

**Close:** minimal opaque/depth/pick/post graph, optional pass omission and resize
work native/WASM; no stale-resource reads, bound overruns or hot engine allocation;
shader variants are prewarmed; separate semantic/pass, asset, staging and render
resource quotas follow §3.2, with no global 42 MiB cap on scene assets.
**Depends:** AG-01's initial fixture; work alongside it. Portable foundation.

### AG-21 — Anti-aliasing, motion vectors and reconstruction

Extends QG-12. Provide spatial AA first, then qualified temporal AA/upscaling.
Declare jitter, velocity units/sign, depth convention, history formats and
disocclusion/reset policy. Velocity includes camera, transforms, skin/morph,
wind and displaced water. Transparency/reactive masks prevent smearing.
Cuts, resize, style/exposure discontinuity and device recovery reset histories.

Use O-11; a native upscaler SDK is not assumed WGSL-compatible. Provide a portable
implementation or an explicit native enhancement plus spatial browser fallback.
**Close:** thin wires, leaf masks, reflective surfaces, crowd movement and camera
pan remain stable over clips; screenshots alone cannot close this gate. Picking
is unjittered/coordinate-correct and never filtered. **Depends:** AG-20/04;
AG-11/12/10 supply deformation coverage. Portable AA; temporal path qualified per device.

### AG-22 — Environment lighting, probes and indirect light

Extends QG-12. Add prefiltered specular environment maps, diffuse irradiance
representation and a BRDF integration LUT; bake deterministic variants offline.
Support spatial probes/lightmaps with ownership, visibility/leak controls,
normal/scale conventions and streaming. Dynamic objects receive the same environment.
Optional dynamic GI consumes a bounded probe-update budget with explicit stale-data
policy; screen-space-only bounce is not complete world GI.

**Close:** coloured bounce/occluded alcove, interior-to-exterior, moving worker,
sun/time change and tile seams; compare light transport to a fixed reference and
check leaks. Baked-probe fallback is acceptable when honestly labelled and within
the selected profile's quality bar. **Depends:** AG-02/03/04/06/20; AG-29 only for
optional accelerated updates/reference rendering, never the portable baseline.

### AG-23 — Reflections, transparency, transmission and refraction

Extends QG-12. Baseline reflections use prefiltered probes with local bounds and
roughness response; evaluate planar water reflections and SSR as optional paths.
SSR has confidence/miss handling and a probe fallback for off-screen/disoccluded
content. Hardware-ray reflections are optional AG-29 enhancements.

Separate alpha-mask, sorted blend, selected order-independent blend and transmission
models. Glass defines index/absorption/thickness policy and limitations of screen
refraction. Weighted blended OIT is an approximation for suitable effects, not
order-correct refractive multilayer glass. Sort/overlap policies remain explicit.

**Close:** glass panes, leaves, smoke, metal, water, nested transparent surfaces and
off-screen reflector; no ID bleed/depth disagreement. Forced unsupported-format
and reduced paths retain essential cues. **Depends:** AG-02/03/04/20/22;
AG-21 for temporal reflection/denoise. Portable baseline, enhanced variants optional.

### AG-24 — Volumetric atmosphere and local participating media

Extends AG-06/13. Provide bounded local fog/smoke/light shafts using clustered
froxel volumes or an equivalent measured reduced-resolution method. Specify
density/extinction/scattering units, depth composition, light/shadow injection,
history validation and quality caps. Reuse shared light lists when appropriate.
Volumetric clouds are a separate optional increment with their own ray-step/
coverage budgets, not a hidden requirement of basic atmosphere.

**Close:** sunlit canopy haze, local smoke against foreground geometry and camera
motion, no temporal trails or double-applied fog; analytic fog/effect fallback
preserves scene state. **Depends:** AG-06/20/04; AG-05 for shadowed shafts;
AG-21 for temporal accumulation. Enhanced profile; analytic portable floor.

### AG-25 — Skin, foliage, cloth, eyes and layered surface fidelity

Extend AG-02 with explicit, bounded models for thin-surface transmission, cloth
sheen, clear coat and optional anisotropy/subsurface approximation. Hero skin/eyes
require close-up tests across skin tones and lighting; foliage requires two-sided
transmission without fake view-dependent glow. Hair cards are the portable path;
strand hair and simulated cloth are optional detail classes with authored fallbacks.

Separate material lobes from costly deformation simulations. Variant counts and
texture/sample costs are capped; geometric/specular AA limits sparkle. Avoid
performing hero-only lobes and subsurface passes on the entire distant crowd.
**Close:** worker face/eyes/clothing and backlit gum leaves at Walk plus reduced
Map LOD; no energy blowout, sorting or shadow mismatch. **Depends:** AG-02/03/04/22/23;
AG-11/15 for character content. Qualify individual lobes, not one blanket “cinematic” flag.

### AG-26 — Native/WASM capability negotiation, content delivery and recovery

Extends QG-12/14. Expose §4's report and profile resolver through native and full
WASM adapters. Sharing a native device with inference is the default existing
architecture: render/inference/upload residency and scheduling need one admission
policy, with interaction/render latency protected under compute pressure.

Content delivered through WASM follows this boundary:

```text
WASM-delivered immutable pack bytes / declared asset sections
  -> local Qualia integrity + format + rights + budget validation
  -> canonical HMC/.10d/material/rig dependency resolution
  -> selected native Webizen or browser-WASM resource preparation
  -> same semantic scene and profile-qualified rendering
```

WASM delivery does not force native rendering through a browser backend. Data-only
delivery is the default. If a package contains an executable generator, use the
existing governed WASM host with declared imports, fuel/time/memory/output limits
and no direct GPU/graph mutation; generated output re-enters normal validation.
The exact embedded-section/extraction ABI is an open contract to specify, not an
assumption that arbitrary modules already expose HMC. Core play cannot depend on
online code execution or a remote renderer.

Handle device loss, surface reconfiguration, tab suspend/resume, failed upload,
cache mismatch and context recreation. Rebuild from validated retained assets,
reset histories, cancel stale readbacks and preserve entity/action/save state.
**Close:** identical delivered pack extracted/validated and consumed by native and
WASM; missing optional features select known fallbacks; forced loss/recovery and
inference pressure pass budgets without semantic divergence. **Depends:** AG-01/20,
QG-01/14; profile/report scaffolding starts immediately.

### AG-27 — Cinematic cameras, shot timelines and bounded capture/export

Define perspective/orthographic camera, focal length/sensor or equivalent FOV,
focus/aperture, exposure, shot cuts, easing and tick/subframe timeline. Provide
photo mode and optional DoF/motion blur; HUD and selection remain separately
controlled. Gameplay defaults prioritize legibility and accessibility.

Capture supports fixed timestep, pinned content/scene/style/camera, deterministic
sample sequence, bounded accumulation, tiled high-resolution export where needed,
async readback/backpressure, cancellation and disk/output caps. Mandatory initial
output is declared SDR still/frame sequence plus metadata. HDR scene-linear export
and video codec/audio synchronization are separate qualified increments. Do not
apply a display transform twice or export display pixels as scene-linear data.

**Close:** original 10-second camera/worker/water shot plus still on native and
portable WASM capture paths; tile overlap avoids bloom/DoF seams, no missing frames,
time drift or semantic state mutation; cancellation cleans scratch.
**Depends:** AG-07/11/20/21/26; AG-29 only for optional reference render mode.

### AG-28 — Graphics observability, visual regression and content diagnostics

Instrument passes, resource ledger, draws/triangles, visibility/LOD, lights,
deformations, overdraw proxies, uploads, compile/cache hits and history resets.
Debug views include albedo, normals/tangents, roughness, linear depth, velocity,
shadow cascades, AO, probe influence, reflection confidence and semantic ID.
Timestamp/query use is capability-gated; unavailable measures are labelled.

Keep diagnostics, benchmark runners and capture artifacts separate from hot
execution. CI pairs shader/format/CPU oracles with real GPU lanes and playable
clips. Pin goldens by scene/profile/output-transform/backend family and tolerance;
never normalize away a black viewport or accept all images by broad thresholds.
**Close:** a deliberately injected blend/normal/history/budget failure produces
actionable evidence and fails the relevant gate; disabled instrumentation stays
within overhead/allocation budget. **Depends:** AG-01/20/26; begins with the first fixture.

### AG-29 — Optional native ray queries and reference path rendering

Native acceleration may improve GI/reflections/shadows and offline reference
images. Eligibility, experimental status, acceleration-structure caps, build/refit
cost, material subset, sample/bounce limits and denoising are explicit. Current
wgpu 30 exposes experimental native ray facilities; this is not proof of backend
availability or portable browser ray tracing. Validate the actual dependency/API
version before adoption. See [wgpu features][R01] and [DDGI research][R13].

Qualify equal-quality benefit against raster/probe paths, including structure
memory and dynamic rebuild cost. Reference tracing uses deterministic sampling and
declared convergence limits; denoised output is labelled, and unsupported materials
never disappear. This remains a mode of the canonical engine, not a new game renderer.
**Close:** qualified native adapter + unsupported-adapter fallback, fixed scene
comparison, animated refit and bounded capture. **Depends:** AG-20/22/23/25/26/27
as relevant. Optional; does not gate portable stylized or photoreal material exits.

## 8. Researched optimization methods: implementation requirements

Research accessed **2026-10-06**. The sources are primary specifications, author
papers, library implementation guidance and official GPU documentation. Older
algorithm references remain useful, but their historical APIs/timing figures are
not transferred to Qualia. External source code/SDKs require upstream dependency,
licence, portability, boundedness and integration review; citing an algorithm is
not an instruction to install its engine or copy it into the game.

### 8.1 Optimization qualification protocol (applies to O-01–O-17)

For each method, the work order SHALL state the bottleneck, baseline, candidate,
target profiles, memory/work caps and expected quality. Implement a simple
correct reference first. Measure equal-quality fixed fixtures and sustained
real-scene runs using §4.2. Publish CPU/GPU/frame p95/p99, working set/residency,
upload/bandwidth proxy, visual error and implementation limits for each runtime.

Select the method that meets the quality bar and budgets with the best supported
trade-off. A faster image obtained by silently dropping shadows, resolution,
visible entities or material lobes is a different profile, not an optimization
win. An optimization that loses on a device is disabled there. Record chosen
thresholds and alternatives in the profile/compiler manifest; retain a tested
reference/fallback path. No claim of universal “fastest” or guaranteed speedup.

| Requirement | Preferred method and implementation obligations | Fallback / qualification | Owning AG / research |
| --- | --- | --- | --- |
| **O-01 Light evaluation** | Evaluate clustered forward: partition view depth into bounded froxels, intersect light bounds, build capped lists, shade only affecting lights. Cache immutable light data; update dirty lists. Cap per-cluster and global index counts with deterministic priority/overflow receipts. | Simple forward for few lights; compare a deferred candidate only if dense lights justify its G-buffer cost. Tune cluster sizes on discrete and tile GPUs. | AG-04/20/24; [Filament][R05], [tile bandwidth][R06]. |
| **O-02 Texture cost** | Offline usage-aware mips; KTX2/Basis or qualified equivalent; choose device compression at load; stage/transcode under caps; stream low mips first. Share by content digest and sampler compatibility. Avoid repeated full decode/upload per instance/style. | Bounded RGBA/low-mip alternative. Compare ETC1S/UASTC or other encodings for download, transcode time and visible normal/mask error; no universal codec winner. | AG-03/14/26; [KTX2/Basis][R03]. |
| **O-03 Shadows** | Stable cascades, caster culling, tightly bounded coverage, texel snapping, bias controls, small PCF; cache unchanged static coverage with explicit invalidation. Budget cascade updates and shadow distance separately. | One map/contact proxy in low tier; PCSS/higher sample filters only after quality/time comparison. Test pans, sun movement and dynamic casters. | AG-05; [Microsoft CSM][R08]. |
| **O-04 Ambient visibility** | Evaluate adaptive compute AO on depth/normals at reduced resolution, with edge-aware filtering/reconstruction. Allocate samples by quality/importance; expose radius in world units. | Baked material occlusion + contact cue; do not call it dynamic AO. Test thin geometry, silhouette halos and moving history. | AG-05/21; [CACAO][R09]. |
| **O-05 Mesh fetch and size** | Cold compile: index/deduplicate while preserving attributes, reorder for vertex reuse, optional measured overdraw ordering, optimize vertex fetch, quantize within error, validate after quantization. Use smallest legal indices and explicit GPU decode layout. | Existing validated layout if a stage worsens quality/time. Overdraw reordering can harm other caches and transparent order; qualify per mesh/pass. | AG-14/16/08; [meshoptimizer pipeline][R10], [glTF quantization][R11]. |
| **O-06 Visibility and submission** | Instanced indexed draws first. At scale, evaluate GPU frustum/LOD + bounded compacted lists/indirect draws and conservative HZB tests. Keep counts GPU-resident; use ID indirection; avoid same-frame visibility readback stalls. | Caller-buffered CPU culling/direct draws; occlusion disabled for cuts/uncertain bounds. Benchmark submission plus compaction/hierarchy overhead, including small scenes. | AG-08/20/28; [occlusion stall analysis][R12], [SSSR hierarchy][R14] as a hierarchy reference. |
| **O-07 Terrain locality** | Compare bounded quadtree tiles against nested geometry clipmaps for large continuous heightfields. Reuse grid/index buffers, update newly exposed strips/tiles, geomorph/stitch LOD edges; work in local origins. | Simpler compiled tiled mesh/parent LOD. Clipmaps do not represent caves/overhangs; retain mesh features and source semantics. Compare streaming churn and construction working set. | AG-09; [geometry clipmaps][R15]. |
| **O-08 Water workload** | Shallow canal/creek: analytic wave superposition or authored flow-normal textures; compute only necessary displacement/detail by projected size. Reuse opaque depth/colour and probes; one qualified planar reflection where justified. | Static ripple/specular/tide-tinted material. FFT/open-ocean simulation requires an explicit scale/spectrum need and benchmark. | AG-10/23; [water models][R16]. |
| **O-09 Crowd deformation** | Share meshes, clips and immutable rig data. Evaluate vertex palette skinning versus compute once/reuse across passes; share quantized pose cache when error permits. Use joint/influence/animation-rate LOD with tick-correct sample time and previous poses. | Bounded CPU palettes + vertex skinning; baked clip/pose or impostor distant variants. Include shadow/pick/velocity reuse and pose-buffer bandwidth in the comparison. | AG-11/15; [animated crowd instancing][R17]. |
| **O-10 Sky and volumetrics** | Sky uses small transmittance/multiscattering/sky-view/aerial LUTs or a qualified simplified subset. Local media use bounded reduced-resolution froxels, capped light/step work and validated reprojection. Rebuild only inputs that changed. | Authored sky gradient + analytic fog; no per-pixel unbounded integration. Test height/sun changes, history invalidation and fog/shadow composition. | AG-06/24; [Hillaire atmosphere][R18], [Wronski volumetrics][R19]. |
| **O-11 Temporal reconstruction** | Stable jitter; camera/object/deformation velocities; depth-based disocclusion; bounded history clipping and reactive masks; exposure-aware history; deterministic reset rules. Evaluate internal-resolution reduction only after temporal correctness. | Spatial AA/upscale with output-resolution UI. Frame generation does not prove simulation or input latency targets. Do not assume an HLSL/native SDK is a WebGPU implementation. | AG-21/27; [FSR2 input/history contract][R20]. |
| **O-12 Indirect light** | Bake diffuse irradiance and roughness-prefiltered environment/BRDF LUTs offline. Tile/local probes use visibility-aware interpolation and measured density. Dynamic probe updates are amortized by influence/change under a strict ray/work budget. | Authored/baked probe sets for day/night; stale update state reported. Screen-space bounce supplements rather than replaces off-screen transport. | AG-22/29; [IBL design][R05], [DDGI][R13]. |
| **O-13 Reflection traversal** | SSR candidate classifies reflective tiles, uses hierarchical depth traversal, caps ray steps and roughness-dependent sampling, then denoises with confidence/history validation. Misses blend to probes; planar reflection has culling/update caps. | Probe reflection; optional native rays for qualified misses. Measure hierarchy/classification/denoise overhead, not traversal alone. | AG-23/21; [SSSR][R14]. |
| **O-14 Transparency/effects** | Alpha-mask foliage avoids unnecessary blend order; blended surfaces use bounded sort. Evaluate weighted blended OIT for smoke/particles when it improves cost/order artefacts. Cull emitters and cap screen coverage; depth-aware reduced-resolution effects where qualified. | Sorted blend/simple effects. Weighted OIT is approximate and unsuitable as a universal refractive-glass solution. Test overlap, high opacity and attachment/blend-format eligibility. | AG-12/13/23; [weighted blended OIT][R21], [glTF transmission][R22]. |
| **O-15 Shared post/downsample work** | Reuse compatible pyramids; fuse compatible colour/LUT/output operations; keep bloom multiscale/reduced resolution with explicit energy. Evaluate SPD-style mip reduction with workgroup-memory fallback and qualified f16/subgroup variants. | Multi-dispatch portable pyramid or existing validated Kawase path. Do not assume global thread synchronization/progress guarantees from an HLSL port; prove WGSL synchronization and reduction semantics. | AG-07/20/21/23; [SPD][R23], [wgpu eligibility][R01]. |
| **O-16 Surface specialization** | Author capped material variants; select expensive coat/sheen/subsurface/anisotropy by material and projected importance. Use baked/detail maps, thin-sheet leaf transmission and hair cards before global screen passes/strands. Prewarm pipeline variants. | Physical/stylized base lobes and authored opaque/alpha cards. Validate lobe energy and normal/specular aliasing; never silently substitute one model under the same qualified feature label. | AG-02/25; [surface models][R05], [material interchange][R02], [transmission][R22]. |
| **O-17 Cinematic post/capture** | Compute signed circle of confusion, classify tiles and run bounded reduced-resolution near/far DoF with edge handling; motion blur uses correct velocity and a sample cap. Capture queues use ring readbacks/backpressure and tiled accumulation with overlap for filter support. | Effects off/manual focus; portable still/frame sequence. Benchmark image quality/edge leakage and full capture memory/output time, not just shader milliseconds. | AG-27/21; [FidelityFX DoF][R24], [temporal inputs][R20]. |

These are requirements to implement or qualify the named methods and their
fallbacks. A work order may choose a better method with a linked primary reference
and equivalent comparison receipts. Persistent ownership, deterministic authoring,
zero-heap hot paths and bounded resources remain mandatory whichever method wins.

### 8.2 Minimum algorithm and data-layout contracts

The implementation work order SHALL specify these details before shader/API work.
Names below describe proposed contracts, not existing exported Qualia symbols.

**Scene and resource views (AG-20/26):** use immutable resource handles with a
generation or equivalent stale-handle check. Keep semantic ID, resource ID and
instance index distinct. GPU records use explicit `repr(C)`/alignment/stride and
matching WGSL layouts verified by size/offset tests. Portable u64 identity is a
pair of u32 words with a declared ordering; require native u64 shader arithmetic
only in a separately qualified variant. CPU slice length, device binding size,
offset arithmetic and format usage are checked before submission. Cross-WASM
memory views have explicit copy/borrow lifetime; zero-copy is claimed only for
the specific measured boundary that actually avoids a copy.

**Cluster lists (O-01):** specify tile width/height, depth-slice function, near/far
bounds, light-index capacity and the overflow policy. Logarithmic depth slicing
is the first perspective candidate; orthographic Map uses an appropriate bounded
linear/world distribution. Build list counts, bounded offsets and light indices
without out-of-range atomic writes. A parallel count/scan/fill or fixed-stride
list is selected by measurements; its ordering cannot change authoritative
state. Every dropped lighting contribution is counted against the profile's
quality contract. Invalid light ranges/NaNs are rejected at admission.

**Visibility and LOD (O-05/06/07):** store local-space bounds and deformation
margin; select LOD from projected geometric/attribute error in output pixels.
Record max allowed error and hysteresis thresholds per profile and camera type.
Conservative HZB tests compare the object's nearest possible depth with the
appropriate occluder reduction, account for screen bounds and near-plane crossing,
and fall back to visible for uncertainty. Previous-frame occlusion requires a
motion/disocclusion policy; use current-frame occluder data when required for
correctness. Do not apply temporal occlusion assumptions to shadow caster lists.

**Deformation (O-09):** define influence count, palette stride, inverse-bind order,
normal/tangent transformation and current/previous pose lifetime. Compute skinning
is attractive when several passes reuse deformed vertices, but adds output memory
and dispatch cost; vertex skinning avoids that buffer and repeats work per pass.
Benchmark both with the actual shadow/pick/velocity schedule. Animation-rate LOD
samples the authoritative timeline at its selected rate and interpolates for
display; it cannot drift based on how many frames happened to render.

**Depth and reconstruction (O-04/06/11/13/15):** one descriptor defines standard or
reversed depth, clear/compare, perspective/orthographic reconstruction, pyramid
reduction and background sentinel. A reversed-Z path is a measured candidate,
not a silent change to existing picking/camera conventions. Reuse a pyramid only
where consumers agree on these semantics. Record velocity sign/units and whether
jitter is included. Reject history for out-of-bounds reprojection, depth/normal
disagreement, invalid resource generation, cuts and declared appearance changes.

**Indirect/reflection sampling (O-12/13):** specify probe volume/bounds, residency,
visibility and interpolation; prevent interpolating illumination through thin
walls by a tested visibility policy. For SSR declare thickness, maximum steps,
ray length, confidence and roughness cutoff. A miss resolves to a known probe or
authored fallback, not a black sample. Denoising cannot turn a missing reflector
into a passed geometric-reflection fixture.

**Transparent composition (O-14):** define alpha premultiplication, blend factors,
depth writes, sorting key/ties and which layers are available to refraction.
Weighted OIT specifies accumulation/revealage formats and blend support, and has
an overlap/opacity error fixture. Soft particles reconstruct scene/particle depth
in the same space before applying fade. Refraction clamps/distorts lookups with
foreground rejection; limitations for multiple layers remain visible in receipts.

**Memory sizing and upload (all methods):** compute full allocated bytes before
admission, including alignment and every mip/layer/sample/history. For uncompressed
targets, sum `width × height × layers × samples × bytes-per-texel` over levels;
compressed textures use rounded block counts. A 1080p RGBA16F surface alone is
about 15.8 MiB; one 4K RGBA16F target is about 63.3 MiB before depth/history/bloom.
That target may be admitted under the separate graphics-resource budget; it does
not violate the semantic arena limit merely by exceeding 42 MiB. Size from
allocated/decoded bytes, not compressed download bytes. Partition governed
ingestion/construction scratch under its pass cap and tile rendering/capture when
their own resource budget requires it. Enforce a per-frame staging/upload
allowance and bounded queue/backpressure; partial
residency never exposes invalid bindings. Schedule shader/asset preparation off
the interactive path and retire replaced resources after GPU completion.

### 8.3 Public interface outcomes

Extend existing APIs with a coherent contract rather than creating parallel
facades. Both adapters SHALL provide equivalents of:

- **Inspect capabilities and select a profile:** requested/effective features,
  device limits, budgets, rejected enhancements and reasons.
- **Prepare validated content:** immutable pack/resource identity, required
  dependencies, admission estimates, bounded progress/cancellation and diagnostics.
- **Submit a scene snapshot:** stable identities, caller-owned instance/light/
  pose/effect views, camera, tick/subframe, profile and capacity checks.
- **Render/poll:** bounded encoding/submission, explicit pending/completed/failed
  status and receipts. A submission success does not certify a displayed image.
- **Pick/capture asynchronously:** request ID/frame identity, output capacity,
  bounded pending slots, top-left/size metadata, cancellation and no stale result
  after scene generation or device changes. Browser APIs cannot synchronously
  block waiting for a native-style readback.
- **Reconfigure/recover/release:** safe resize/style/profile changes, history reset,
  device rebuild and resource accounting; preserve semantic state.

Return typed errors for capacity, malformed/unsupported/versioned content,
capability mismatch, device loss and canceled work. Keep hot-path errors and
telemetry bounded; formatting/JSON/artifact writing belongs to the cold adapter.
Expose versioned receipts and defaults so a second non-game SDK consumer can use
the same contracts without importing game-specific types.

## 9. Fixtures and visual acceptance

Fix cameras, timeline, scene state, content/style hashes, output size and profile.
Store diagnostic outputs as well as beauty images. Numeric fixtures compare
declared absolute/relative colour/depth/normal/velocity errors; art frames use
reviewed tolerances and human judgment. Lock thresholds before implementation;
changing a tolerance requires a recorded reason and retained failing evidence.

| Fixture | Required content and checks | Primary gates |
| --- | --- | --- |
| **F01 Correctness room** | Opaque near/far cubes, unlit grey/colour ramp, intersecting depth, zero-intensity light, alpha modes, bloom toggle, resize, DPR and fallback; known pixel/depth/ID oracles. | AG-01/07/20/26 |
| **F02 Material/light lab** | Roughness/metalness spheres, bark/plaster/cloth, emissive lamp, smooth/hard normals, mirrored UVs and scaled transforms; grazing views, white-furnace test, moving lights. | AG-02/03/04/22/25 |
| **F03 Grounding/world** | Canopy/pod/worker on sloped joined tiles; cascade transitions, terrain seams, creek/canal tide, glass/foliage/smoke, interior/exterior and horizon. | AG-05/06/09/10/23/24 |
| **F04 Motion/temporal** | Worker idle/walk/work, morph, gum/grass wind, waves, rain/dust, thin wire; static and pan/cut/teleport clips, replay seek, history invalidation, reduced motion. | AG-11/12/13/21 |
| **F05 Representative sector** | 100 active agents, 50 structures, 500 props plus terrain; annotated light/material/triangle/texture counts; separate 10,000-instance submission test; 20-minute travel/streaming run. | AG-08/09/16/19/26/28 |
| **F06 Failure/recovery** | Missing/corrupt/incompatible pack, decompression/arena cap, light/particle overflow, upload error, device loss, tab resume, low feature limits, style churn, inference pressure. | AG-03/14/19/20/26 |
| **F07 Content and cinematic round trip** | Hero building/worker/foliage compiled twice, WASM-delivered pack consumed locally native/browser, 10-second shot + bounded still/sequence, cancel/cleanup and provenance verification. Include a pack and synthetic individual payload larger than 42 MiB on a profile with sufficient declared residency: governed chunk/pass scratch stays within its cap, while payload bytes remain in the asset domain. Forced asset/frame-budget exhaustion must refuse or degrade by that domain's policy. | AG-14/15/16/20/26/27; optional AG-29 |

In addition, lock the four art views: **Kestrel strategy**, **garden Walk with
worker**, **Saltwind low tide**, and **Community Grounds pod under solar shade**.
Capture each in three styles, at P1 and P2 on qualified native/WASM routes, in
day/night where relevant. Review silhouette, material differentiation, grounding,
atmospheric depth, faces/hands, botanical identity and selection readability at
actual display size. A photoreal variant of F02/F03 qualifies physical surface
behaviour; it does not delay the first stylized slice or certify all future content.

Temporal acceptance includes recorded moving clips and comparisons of static
regions over time. Reject noticeable shadow swimming, specular shimmer, ghosting,
leaf disappearance or LOD popping even if isolated images look good. Reduced
profiles have their own quality floors; their reference imagery cannot be labelled
as the enhanced output.

## 10. Executable work sequence and delivery packets

1. **Packet A: trustworthy foundation.** AG-01 F01, minimal AG-20 graph/resource
   ownership/decomposition plan, AG-26 runtime report, AG-28 receipts. Qualify
   native plus one real full-WASM browser device and define P1/P2 budget candidates.
2. **Packet B1: one finished surface asset.** AG-02/04, then AG-03; AG-14 compiler
   increment. Deliver one hero building through native/WASM, normals/tangents,
   textures, default legacy material and malformed/budget cases. Run F02.
3. **Packet B2: coherent illumination.** AG-05/06/07/22; key/fill/night lights,
   shadows, environment lighting, colour/output and analytic fallbacks. Adopt the
   four art views and first AG-17 style bundle; run F03.
4. **Packet C: continuous place at scale.** AG-08/16 first, then AG-09. Profile
   the representative sector, source-aligned terrain/height queries and streaming;
   add AG-10/23 water/probe/transparency increment. Run F05/F06.
5. **Packet D: life and temporal stability.** QG-03/13 + AG-11, independent AG-12
   wind, AG-13 effects, AG-21 velocities/reconstruction and AG-15 content family.
   Run F04; add selected AG-25 hero surfaces before close-up sign-off.
6. **Packet E: finished portable slice.** Complete AG-17/18 feedback/styles,
   AG-19 floors and all native/WASM round trips. Portable slice must work from an
   offline validated HMC, including WASM-delivered content, without a second engine.
7. **Packet F: cinematic/enhanced increments.** AG-24 volumes, stronger AG-22/23
   updates, AG-25 lobes and AG-27 shot/export; evaluate AG-29 only for eligible
   native profiles. Each enhancement has an independent fallback/performance gate.

These are acceptance slices, not fixed durations or a promise that a packet fits
one agent session. Break each into bounded work orders with an owner, interfaces,
tests and dependencies. Do not defer the compiler, accessibility, recovery or
instrumentation until “polish,” and do not expand the catalogue until its hero
asset/pipeline slice survives the same runtime and budget gates.

## 11. Work-order template and completion receipts

An implementation agent should be able to execute a work order without guessing
which contract, device, quality level or artifact proves completion. Use this template:

```text
Work order: <unused QG ID>; capability: <AG IDs>; owner/status:
Player/SDK behaviour and concrete before/after case:
Scope: portable core / named enhancement / named runtime:
Inspected APIs, code owners, source revision + local-diff identity:
Required format/API changes + compatibility/migration:
Chosen O-methods, primary sources, baseline and alternatives:
Cold construction / hot execution / resource ownership:
Device/runtime matrix + capability and budget manifests:
Failure/fallback/recovery policy:
Fixtures + visual/numeric thresholds + allocation checks:
Public native API and WASM binding + non-game consumer:
Game integration and offline/WASM-delivery route:
Receipts required; outstanding decisions; known limits:
```

Receipt schema SHALL be versioned and include:

- Work-order/AG/fixture IDs, test command or runner, timestamp and result.
- Exact Qualia/game revisions, source diff identity if dirty, toolchain/features,
  built WASM/native artifact digests, format/API/compiler/shader versions.
- Source/HMC/asset/material/style digests, scene/world/replay seed and tick range,
  camera/shot/exposure/output-transform metadata.
- Runtime/browser/OS/backend and adapter identity where available; effective
  capabilities, selected features/fallbacks, output/internal resolution.
- p50/p95/p99 frame and pick metrics, labelled GPU/CPU timing method, cold/stall
  results, live/high-water arena/CPU/WASM/GPU-ledger/staging/cache bytes.
- Draw/visible/culled/LOD/triangle/material/light/joint/particle/upload counters,
  allocation evidence, validation errors and recovery results.
- Expected/actual semantic IDs and replay digests; reference and captured
  image/clip digests; numeric errors, visual/art/accessibility review.
- Artifact destination, output byte cap, cleanup/retention outcome and unresolved
  limitations. No passing summary without its underlying failed/skipped cases.

Apply the repository-required checks for the actual code changes. At minimum,
new portable rendering behaviour needs focused native/format/CPU-oracle tests,
shader validation, a `wasm32` check of the relevant full profile, real native GPU
and browser fixture runs, and the rebuilt game integration. Use the engine's
existing test/CI commands and hardware lanes; do not invent unimplemented CLI
commands or treat `cargo check` as a render-quality test.

Close portable items only after both adapters and the declared game path pass.
Close device-only enhancements with an explicit eligibility matrix and passing
fallback. Record skipped hardware tests as unqualified, never passed. Reopen a
gate when a new scene/device/version reproduces a failure.

## 12. Decisions to resolve before implementation closure

The owner has selected both styles with stylized first, native and WASM runtimes,
device-dependent features, graceful degradation and locally processed WASM-delivered
content. These are requirements, not open questions. The following remain design
or qualification decisions; independent specification/compiler work can proceed:

| Decision | Required resolution / owner |
| --- | --- |
| Baseline hardware and browser matrix | Engine/integration owner names actual P1/P2 devices, OS/browser/backend versions and qualification coverage; no arbitrary minimum GPU is invented here. |
| Numeric per-domain residency caps | The owner has settled that asset payload/residency and graphics resources are separate from the 42 MiB semantic arena. Engine owner selects CPU/WASM asset, GPU asset, frame-resource, staging and shared inference headroom caps per profile and implements reservations under §3.2. |
| Working/output colour transform | Rendering/art owners choose and version SDR scene/display spaces and curve; HDR display/export is separately scoped. |
| Material/rig section allocation | Format owner selects `.10d` section IDs/versions, endian/alignment and legacy reader policy after inspecting the canonical registry. |
| WASM content extraction ABI | Runtime/format owners define data sections/exports, immutable pack identity and optional governed generator policy for local native/browser consumption. |
| Native host presentation surface | SDK owner qualifies native interactive embedding as well as offscreen capture; offscreen tests alone do not prove an interactive native product. |
| Cinematic deliverable | Art/export owners select still/sequence/HDR/video scope, shot lengths, storage budget and any audio-sync increment; 4K is a proposed offline target. |
| Advanced methods and dependencies | Qualify measured wins and upstream library/portability review. Ray tracing, virtual geometry, strand hair and simulation remain individually eligible enhancements. |
| QG identity collision | Programme owner reconciles doc-19/ledger QG-20 before assigning new upstream numbers. AG IDs in this document remain stable. |

## 13. Research references

These links support the named methods, not measured Qualia speedups. Pin exact
source versions/commits and coefficients in implementation work orders.

| Ref | Primary source | Applied requirement |
| --- | --- | --- |
| R01 | [wgpu 30 feature eligibility and native/experimental distinctions][R01] | Adapter-intersected features, optional native enhancement; inspect actual pinned API. |
| R02 | [Khronos glTF 2.0 specification][R02] | Material/texture colour conventions and bounded declared interchange subset. |
| R03 | [Khronos KTX texture container and Basis distribution][R03] | Compressed distribution with device-dependent transcode variants. |
| R04 | [ACES display encoding][R04] | Distinguish scene rendering, output transform and display encoding. |
| R05 | [Google Filament rendering design][R05] | Clustered forward, microfacet/IBL and bounded surface-model candidates. |
| R06 | [Khronos Vulkan tile-based rendering best practices][R06] | Attachment bandwidth, stores and tiler-sensitive measurement; no assumed WebGPU subpasses. |
| R07 | [MikkTSpace author implementation interface][R07] | Tangent basis consistency through baking/import/shading. |
| R08 | [Microsoft cascaded shadow maps][R08] | Coverage, filtering, stabilization and bias trade-offs. |
| R09 | [AMD FidelityFX CACAO][R09] | Adaptive/reduced-resolution AO candidate and quality comparison. |
| R10 | [meshoptimizer author guidance][R10] | Ordered cache/fetch/quantization stages and measured mesh optimization. |
| R11 | [Khronos KHR_mesh_quantization][R11] | Attribute quantization and explicit decode/error contract. |
| R12 | [NVIDIA Efficient Occlusion Culling][R12] | Avoid synchronous visibility-query stalls; contemporary HZB remains a qualified candidate. |
| R13 | [Majercik et al., Dynamic Diffuse GI with Ray-Traced Irradiance Fields][R13] | Visibility-aware probes and optional bounded dynamic updates. |
| R14 | [AMD FidelityFX SSSR][R14] | Hierarchical traversal, tile classification and denoise stages. |
| R15 | [GPU Gems 2: Geometry Clipmaps][R15] | Reused terrain grids, nested resolution and incremental updates. |
| R16 | [GPU Gems: Water Simulation from Physical Models][R16] | Analytic wave/normal detail baseline; avoid unnecessary ocean workloads. |
| R17 | [GPU Gems 3: Animated Crowd Rendering][R17] | Shared clip/pose data, instancing and crowd LOD. |
| R18 | [Hillaire's atmosphere research implementation and paper links][R18] | LUT-based sky/aerial perspective candidate. |
| R19 | [Wronski, SIGGRAPH volumetric fog][R19] | Compute/froxel local participating-media candidate. |
| R20 | [AMD FidelityFX temporal reconstruction input contract][R20] | Velocity, jitter, reactive masks, depth and history correctness. |
| R21 | [McGuire/Bavoil, Weighted Blended OIT][R21] | Bounded approximate blend candidate; limitations explicit. |
| R22 | [Khronos transmission material extension][R22] | Transmission is distinct from alpha blend; declared glass approximation. |
| R23 | [AMD FidelityFX SPD][R23] | Downsample reuse, optional wave/f16 variants and portable synchronization review. |
| R24 | [AMD FidelityFX depth of field][R24] | Bounded reduced-resolution cinematic DoF candidate. |
| R25 | [Rust mikktspace crate interface][R25] | Rust callback interface for MikkTSpace; generated corner results must be preserved without averaging incompatible indexed vertices. |
| R26 | [Khronos KHR_texture_basisu][R26] | Basis ETC1S/UASTC semantics, usage colour-space rules, mip guidance and runtime GPU-format transcoding. |
| R27 | [WebGPU texture and queue-write specification][R27] | Validate texture dimensions/layout and account texture write/upload paths against device limits. |
| R28 | [WGSL texture comparison and explicit-level semantics][R28] | Use comparison-depth sampling; explicit level avoids derivative/uniform-flow requirements for bounded PCF. |
| R29 | [WebGPU shadow-mapping sample][R29] | Portable depth-texture/comparison-sampler pipeline reference; adapt with Qualia admission and shader contracts. |
| R30 | [wgpu texture usage flags][R30] | Shadow depth target needs render-attachment and texture-binding usages. |
| R31 | [AMD FidelityFX CACAO guidance][R31] | Adaptive half-resolution AO preparation, depth/normal deinterleaving, and quality-tier reference; Qualia first pass is a custom WGSL baseline, not a full CACAO port. |
| R32 | [Microsoft DirectXTex alpha-coverage mip guidance][R32] | Generate the mip chain, then adjust each level's alpha to preserve coverage at the alpha-test reference; use as the content-pipeline baseline and conformance oracle. |

[R01]: https://docs.rs/wgpu/30.0.0/wgpu/struct.Features.html
[R02]: https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html
[R03]: https://www.khronos.org/ktx/
[R04]: https://docs.acescentral.com/system-components/output-transforms/technical-details/display-encoding/
[R05]: https://google.github.io/filament/main/filament.html
[R06]: https://docs.vulkan.org/guide/latest/tile_based_rendering_best_practices.html
[R07]: https://github.com/mmikk/MikkTSpace/blob/master/mikktspace.h
[R08]: https://learn.microsoft.com/en-us/windows/win32/dxtecharts/cascaded-shadow-maps
[R09]: https://gpuopen.com/manuals/fidelityfx_sdk/techniques/combined-adaptive-compute-ambient-occlusion/
[R10]: https://github.com/zeux/meshoptimizer
[R11]: https://github.com/KhronosGroup/glTF/blob/main/extensions/2.0/Khronos/KHR_mesh_quantization/README.md
[R12]: https://developer.nvidia.com/gpugems/gpugems/part-v-performance-and-practicalities/chapter-29-efficient-occlusion-culling
[R13]: https://research.nvidia.com/publication/2019-05_dynamic-diffuse-global-illumination-ray-traced-irradiance-fields
[R14]: https://gpuopen.com/manuals/fidelityfx_sdk/techniques/stochastic-screen-space-reflections/
[R15]: https://developer.nvidia.com/gpugems/gpugems2/part-i-geometric-complexity/chapter-2-terrain-rendering-using-gpu-based-geometry
[R16]: https://developer.nvidia.com/gpugems/gpugems/part-i-natural-effects/chapter-1-effective-water-simulation-physical-models
[R17]: https://developer.nvidia.com/gpugems/gpugems3/part-i-geometry/chapter-2-animated-crowd-rendering
[R18]: https://github.com/sebh/UnrealEngineSkyAtmosphere
[R19]: https://bartwronski.com/wp-content/uploads/2014/08/bwronski_volumetric_fog_siggraph2014.pdf
[R20]: https://gpuopen.com/manuals/fidelityfx_sdk/techniques/super-resolution-temporal/
[R21]: https://www.jcgt.org/published/0002/02/09/
[R22]: https://github.com/KhronosGroup/glTF/blob/main/extensions/2.0/Khronos/KHR_materials_transmission/README.md
[R23]: https://gpuopen.com/manuals/fidelityfx_sdk/techniques/single-pass-downsampler/
[R24]: https://gpuopen.com/manuals/fidelityfx_sdk/techniques/depth-of-field/
[R25]: https://docs.rs/mikktspace/0.3.0/mikktspace/
[R26]: https://github.com/KhronosGroup/glTF/blob/main/extensions/2.0/Khronos/KHR_texture_basisu/README.md
[R27]: https://gpuweb.github.io/gpuweb/
[R28]: https://gpuweb.github.io/gpuweb/wgsl/#texture-builtin-functions
[R29]: https://github.com/webgpu/webgpu-samples/tree/main/sample/shadowMapping
[R30]: https://docs.rs/wgpu/30.0.0/wgpu/struct.TextureUsages.html
[R31]: https://gpuopen.com/manuals/fidelityfx_sdk/techniques/combined-adaptive-compute-ambient-occlusion/
[R32]: https://github.com/microsoft/DirectXTex/wiki/ScaleMipMapsAlphaForCoverage

## 14. Revision record

**2026-10-06 follow-up:** implemented WASM device/context loss detection and frame-boundary recovery. VibeScript selects the retry/fallback backend; the browser host performs the API calls and replaces the context-bound canvas. WebGPU/WebGL2 reinitialization reuses retained Tensor10D data and a reduced CPU mesh, then reports the reduced detail. WASM cargo check and Vibe test-target checks pass. Real-browser forced-loss and suspend/resume qualification, original full-detail source-asset replay, asset-pack delivery and parity remain open.

**2026-10-06 follow-up:** clarified the manifold/physics contract after inspection of Webizen Studio and existing scientific modules. Preserve 10D/PGA semantics and multiple field families (including astrophysics and waves/diffusion); EMF `[α,μ,σ]` is field data, while colour is only an observer-facing projection. Studio now decodes the canonical engine tensor ABI with backward-compatible legacy reads; a VibeScript-driven field-output-to-viewport bridge, native tests, and browser field parity remain open. No renderer qualification is implied by the adapter change.

**2026-10-06 follow-up:** moved renderer preference ordering from JavaScript into a checked VibeScript policy consumed by the WASM capability probe. Host-side tests cover WebGPU, WebGL2, Canvas2D and unavailable branches; `cargo check -p vibe --tests` passes, while test execution is blocked by host access denied to MinGW `dlltool`. Browser API probing and context acquisition remain host responsibilities. WASM cargo check passes; wasm-bindgen post-processing remains environment-blocked as recorded above. Device-loss restoration, retained-scene GPU reupload, asset-pack delivery and browser parity remain open.

**2026-10-06:** expanded the original 19-item register into a native/WASM cinematic
engine specification; preserved AG identifiers; added ten foundation/cinematic
gaps, 17 researched optimization requirements and 24 primary references. Corrected
per-pixel/flat-normal and existing tone-map/import/effects baseline descriptions,
dependency errors and QG allocation assumptions. Added format/space/colour/time/
memory contracts, device eligibility and fallback policy, budget qualification,
WASM content delivery, recovery, fixtures, execution packets and closure receipts.
Recorded the owner's memory clarification: packs/asset payloads, residency and
graphics resources have separately budgeted domains comparable to inference;
42 MiB remains the semantic arena and applicable governed chunk/construction
pass ceiling. Added a large-asset fixture and explicit shared-device admission.
The specification remains proposed; no new renderer implementation, performance
qualification or capability completion is asserted by this revision.


**2026-10-07 follow-up:** Implemented owned resource reservations for the Qualia-enabled `webizen-render` SDK vertex, offscreen target and readback allocations; separated portable WGSL Forge from opt-in CUDA while retaining CUDA in the default native profile; and completed native/WASM SDK test-target type checks. The 10D scientific field contract remains multi-family: EMF `[α,μ,σ]` is one data family and colour remains a derived observer projection. Surface-device budget identity, swapchain estimation, other scientific field-family residency, runtime browser/pixel qualification, and full shared resource accounting remain open in AG-20/26.
**2026-10-07 follow-up:** Added generated vertex normals and direct-light interpolation to both WebGPU mesh passes; geometry accounting includes the new normal buffer. The normal generator and shader semantic validation pass, and native portable/WASM test-target checks pass. GLB authored normals now survive import and `.10d` round-trip; OBJ/STL authored normals, MikkTSpace tangent generation, hard-edge topology, complete material/style paths, frame-time staging admission, and visual lighting qualification remain open; GLB tangents now persist through FRM1. The manifold stays multi-family: EMF is one field family and displayed colour is derived from scientific field data.
**2026-10-07 follow-up:** Pose-driven normal work now uses precomputed vertex/face adjacency and updates only affected faces and vertices; sparse normal-buffer writes are coalesced. The focused tests, native portable-profile test-target check, WASM SDK test-target check, Naga validation, formatting, and whitespace checks pass. Staging admission for per-frame pose uploads, authored normals/tangents and material/style profiles remain open.
**2026-10-07 follow-up:** Replaced the preview UV-derivative tangent fallback with production MikkTSpace generation for GLB primitives lacking authored `TANGENT` but providing `TEXCOORD_0`. The importer preserves per-corner frames and splits indexed vertices only where tangent signatures differ (including mirrored handedness), remapping full vertex streams deterministically. Added standard, mirrored UV, shared-corner split, degenerate-UV finite-frame, accessor and GLB-to-FRM1 coverage. The implementation uses `mikktspace` 0.3.0; `HashMap` is lookup-only and output indices follow source face/corner order. Native portable core test-target and WASM `webizen-render` test-target checks pass; isolated tests compiling the production tangent module pass. Full native test execution remains host-linker blocked by MinGW `aws_lc_sys` `nanosleep64`. Research basis: MikkTSpace author interface and Rust API [R07]/[R25]. Normal-map shading/material integration and deformation/LOD seam qualification remain open. The multidomain 10D/PGA contract is preserved: astrophysics/N-body gravity, waves/diffusion, depth and time axes, and EMF are distinct data families; EMF `[α,μ,σ]` is not the whole manifold and colour remains a derived observation.

**2026-10-07 follow-up:** Continued AG-02 from the MAT1 codec into source ingestion. MAT1 v1 now uses 256-byte records with six separate SHA-256 resource references, including independent metallic-roughness and occlusion images; 24-byte ranges continue to partition the triangle index stream exactly. GLB import preserves core material factors, alpha/double-sided state, embedded BIN image digests and primitive material assignments. Standard asset compilation scopes local material identities to the asset and attaches MAT1. Native `cargo check -p qualia-core-db --tests` passes, including the new source-to-container fixtures; test execution remains unverified because the host test binary has the known MinGW `aws_lc_sys` `nanosleep64` link failure. WASM `cargo check --target wasm32-unknown-unknown -p webizen-render --tests` passes; full pixel shading, external/data URI and HMC dependency resolution, texture processing/residency and visual qualification remain open. The 10D/PGA scientific substrate remains multidomain: astrophysics/N-body, wave/diffusion, depth/time and EMF are distinct typed fields; EMF is not the whole manifold and RGB remains a derived observer projection. Method follows the metallic-roughness interchange model and bounded material parameterization in [R02]/[R05].

**2026-10-07 follow-up:** Completed the first MAT1-to-pixel material consumption slice. `webizen-render` now decodes MAT1 from `.10d` assets and submits opaque factor-only submeshes through 256-byte aligned dynamic uniforms; adjacent equal-material ranges merge and submissions are capped at 65,536. The mesh shader applies base colour, emissive and vertex-colour policy with direct-light GGX metallic-roughness shading plus a three-band stylized mode. GPU material bytes are included in the pre-allocation Geometry reservation. Unsupported texture references and MASK/BLEND modes fail closed until texture residency and coverage passes land. Native core and SDK test-target checks pass, WASM renderer test-target check passes, Naga validates the shader, and Rust formatting/whitespace checks pass. Native test execution remains blocked by MinGW `aws_lc_sys` unresolved `nanosleep64`; no pixel-level GPU qualification is claimed. Image-based lighting, shadows, textures, visual profile controls and broader frame-graph admission remain open. Renderer decomposition moved upload lifecycle and material shading into focused files under 500 lines. The graphics substrate remains multidomain 10D/PGA: EMF `[α,μ,σ]` is one field family, astrophysics/N-body gravity, waves/diffusion, depth and time remain first-class typed domains, and RGB is a derived observer projection.

**2026-10-07 follow-up:** Researched and tightened AG-03 texture requirements against Khronos glTF/KHR_texture_basisu and the WebGPU specification [R02]/[R26]/[R27]: preserve sRGB versus linear map semantics, metallic-roughness channel mapping, adapter-selected Basis transcode targets, mip completeness/coarse-to-fine streaming, and separate resident versus bounded staging/transcode accounting. This is normative guidance; texture decoding, transcoding and GPU sampling remain open implementation work.

**2026-10-07 follow-up:** GLB asset ingestion now retains embedded image payloads as `CompiledAsset.texture_dependencies`, content-addressed by the same SHA-256 digests written to MAT1, deduplicates shared image payloads in digest order, validates MIME type and bounded source-BIN byte totals, and selects `KHR_texture_basisu` KTX2 or `EXT_texture_webp` image sources when present. A GLB-to-compiled-asset regression covers KTX2 byte/digest preservation. Native `cargo check -p qualia-core-db --tests` and WASM `cargo check --target wasm32-unknown-unknown -p webizen-render --tests` pass. This is source preservation only: payload HMC packaging/resolution, transcode/decode, mip construction, GPU sampling and texture residency remain open.

**2026-10-07 follow-up:** The focused KTX2 import-to-compiled-asset native regression was attempted after the native/WASM test-target checks passed. It does not reach test execution: the default Windows GNU link fails because `ld` cannot find `-ld3d12` or `-ldxgi`. Runtime behavior remains unverified; keep the checks distinguished from executed tests. Standard and developmental asset compilation both preserve MAT1 and the extracted texture-dependency records.

**2026-10-07 follow-up:** Added `CompiledAsset::build_hmc_bundle` in the focused `render/asset_package.rs` module to package its `.10d` and original texture dependencies into a single HMC bundle. Image entries use deterministic `textures/sha256/<digest>` keys and MIME kind metadata; packaging rechecks SHA-256. `resolve_hmc_texture_resource` returns a zero-copy payload only after verifying HMC entry integrity and equality between the entry digest and the MAT1 resource digest. The embedded-KTX2 regression now covers bundle output and resolution. Native and WASM test-target checks pass; runtime test execution remains blocked by the Windows GNU linker missing `-ld3d12`/`-ldxgi`. External/data URI resolution, image decode/transcode, mip handling, GPU texture binding/sampling and pixel qualification remain open. Multi-asset HMC ordering, shared-image de-duplication and MIME conflict rejection are implemented; their regression tests compile but could not run on this host due the Windows GNU linker limitation recorded above.

**2026-10-07 follow-up:** Extended HMC construction to multiple compiled assets. Asset entries are sorted by key before serialization, equal image digests are stored once in digest order, payload SHA-256 is rechecked, and equal digests with conflicting MIME metadata fail closed. Added regression coverage for order-independent bundle bytes, shared-texture de-duplication and metadata conflict rejection. Native and WASM test-target checks pass; test execution remains host-linker blocked. HMC multi-asset assembly is now implemented; URI resolution and GPU-ready image processing remain open.

**2026-10-07 follow-up:** Added
ender::texture_decode::decode_hmc_texture_rgba8_into as a shared native/WASM cold path for PNG and JPEG. It decodes into caller-owned RGBA8 output (and caller-owned PNG scratch), applies explicit encoded-byte/dimension/decoded-byte caps, and rejects undersized buffers before pixel conversion. PNG preserves alpha; JPEG expands RGB/grayscale to opaque RGBA. KTX2/Basis and WebP remain content-addressed HMC sources but are not yet transcoded. This is not texture residency or material sampling: colour-space/channel mapping, mip generation/streaming, adapter-specific compressed targets, GPU residency/accounting and visual tests remain open. The renderer's 10D/PGA field remains multidomain; EMF [α,μ,σ] is one field family alongside astrophysics/N-body, waves/diffusion, depth and time, with colour as a derived observation.
**2026-10-07 follow-up:** Added first-level shared GPU texture residency in
ender::gpu::PortalGpu. Upload admission checks dimensions against active device limits and exact RGBA8 byte counts, selects sRGB or linear UNORM interpretation, deduplicates equal source-digest/interpretation pairs, reserves resident texture bytes before texture creation, reserves upload staging separately until submitted queue work completes, and exposes explicit eviction to release resident accounting. Upload currently retains one base mip and one repeat/linear sampler; it is not yet sampled by MAT1 materials because mesh UV streams and texture bind groups are not wired. Native core library check and WASM Webizen renderer test-target check pass after the change; native test assertions still have not executed due the long test-harness compile and known GNU linker limitations. The graphics model remains full 10D/PGA and multidomain: EMF [α,μ,σ] is one field family among astrophysics/N-body, waves/diffusion, depth and time; RGB remains a derived observer projection.
**2026-10-07 follow-up:** Preserved GLB TEXCOORD_0 through the tangent-frame split path and compiled it into a versioned, CRC-protected .10d UV01 sidecar. Added caller-buffered UV readback with exact vertex-count, payload-layout, finite-value and coordinate-range validation. The Webizen .10d loader now forwards the optional stream into the native/WASM Portal mesh upload; the GPU vertex ABI and SDR/HDR pipelines carry UV0, include it in geometry byte admission, and retain a zero-coordinate fallback for legacy assets. A regression covers GLB import through compiled-sidecar readback. Native core and WASM Webizen renderer test-target checks pass; no unit-test assertions have executed because native test-harness compilation was stopped before linking. UV0 is now resident but not yet sampled: UV transforms/UV1, material texture bind groups and actual shader sampling remain open. The scientific 10D/PGA data model remains complete and multidomain; EMF [α,μ,σ] is one field family alongside astrophysics/N-body, waves/diffusion, depth and time, and colour is a derived observer projection.
**2026-10-07 follow-up:** Advanced the material section to MAT2 v2 while preserving MAT1 v1 decoding. MAT2 adds six KHR_texture_transform-compatible per-map UV0 transforms; GLB import reads offset, scale and rotation for base colour, normal, metallic-roughness, occlusion and emissive maps, and applies them consistently in forward, alpha-coverage/AO and shadow shaders. Alternate TEXCOORD sets fail closed because only UV0 is imported and resident. The aligned material uniform carries the transform values without per-frame allocation. Regressions cover MAT2 round-trip, MAT1 identity-transform compatibility, transform packing, component parsing and unsupported UV-set rejection. Native `cargo check -p qualia-core-db --tests --locked --offline` and WASM `cargo check --target wasm32-unknown-unknown -p webizen-render --tests --locked --offline` both pass, and Naga validates all three changed shaders. Native runtime pixel qualification remains pending because the Windows GNU linker lacks the D3D12/DXGI libraries required by the test harness. MAT2 material/texture metadata is outside the 42 MiB semantic Sentinel budget; existing bounded asset/residency budgets still apply to encoded and decoded payloads.
**2026-10-07 follow-up:** Advanced the material writer to MAT3 v3 to preserve format-version semantics: MAT2 v2 remains decoded with its reserved bytes required to stay zero, while MAT3 reuses those 48 bytes for six compact, validated sampler records without growing the 448-byte material record. MAT1 v1 and MAT2 v2 default to the historical glTF sampler state. GLB import now preserves wrapS/wrapT, magFilter and all six minFilter modes per texture role, applying glTF defaults when omitted and rejecting invalid values. A renderer-wide fixed cache covers the 108 normalized wrap/filter combinations; each map binds filtering and non-filtering samplers separately, with a per-material uniform mask selecting the correctly typed path in forward, AO coverage and shadow shaders. Native and WASM test-target checks pass and Naga validates the changed shaders. Pixel-level sampler qualification remains open; test execution is constrained by the Windows GNU linker lacking D3D12/DXGI libraries.
**2026-10-07 follow-up:** AG-03 now creates a complete mip chain during GPU texture admission. A persistent three-pipeline render-pass generator handles sRGB colour in linear light, linear data, and normal maps with vector renormalization; odd dimensions use area-weighted source texels, and the final 1×1 level is included. Residency admission charges the full chain before texture allocation, while UploadStaging covers only the uploaded base level until all submitted generation passes complete. The texture identity includes mip semantic so content reused across roles does not share incompatible normal/data/color pyramids. HMC material loading chooses the role-specific policy. Native core and Webizen renderer test-target checks and WASM renderer test-target checking pass; Naga validates mip, mesh, AO and shadow WGSL. Pixel execution remains host-link blocked. Coverage-preserving alpha mips, ingestion of precomputed KTX2/Basis levels, and coarse-to-fine streaming remain open. Texture payloads and mip residency are governed by encoded/decoded and GPU resource budgets, not by the 42 MiB semantic Sentinel.

**2026-10-07 follow-up:** Implemented the first alpha-mask mip coverage slice. TextureMipSemantic::AlphaMask carries canonical cutoff bits, and residency keys include that value. The CPU derives deterministic per-level alpha scales from an alpha-only pyramid using the same area footprint as the GPU downsampler; the GPU applies each scale while rendering the mip in sRGB colour space. HMC source discovery and material binding both select the same cutoff-specific texture identity. UploadStaging reserves bounded correction scratch before allocation and releases it after scale computation; the full mip chain remains charged to TextureResidency. Native core and Webizen renderer test-target checks and the WASM renderer test-target check pass; Naga validates mip_generate.wgsl. The standalone production alpha-coverage-plan tests pass 4/4, including signed post-RGBA8 residuals and the nearest attainable 2x2 result when 75% exact coverage is impossible. Integrated GPU pixel coverage qualification remains open: the targeted test binary reaches linking, where the renderer-only MinGW profile fails on unresolved AWS-LC `nanosleep64`; this profile does not require D3D12/DXGI import libraries. Follow-up work: add GPU-measured coverage comparison and broader cutoff/NPOT fixtures, then ingest precomputed corrected chains for the preferred zero-readback asset-pipeline path. Method basis: Microsoft's ScaleMipMapsAlphaForCoverage guidance [R32].


**2026-10-07 follow-up:** Added an adapter-required minified alpha-mask pixel fixture. It uploads a 75%-coverage texture with cutoff 0.8, forces mip level 2, reads back the production renderer output and requires visible pixels; ordinary box-filtered mips would fall below cutoff and disappear. The targeted native test build compiled the test crate but did not link: in the renderer-only MinGW profile, `aws_lc_sys` references unresolved `nanosleep64`, so no GPU pixel result is claimed. `cargo check -p qualia-core-db --lib --tests --no-default-features --features profile_target_1024,zk-culling,gpu-runtime,gpu-native-portable,wgsl-forge,privacy-he --locked --offline` passes, confirming the native test target compiles. Executing the test still requires a working host linker; browser GPU comparison and multi-cutoff/NPOT fixtures remain open.

**2026-10-07 identity clarification:** `.10d` v2 treats the Tensor10D record as a structured, identity-bearing manifold coordinate/address together with declared profile/domain and its implicit characteristics—not as a digest lookup key. Q42 stable IDs provide durable references across edits/replay; cryptographic hashes remain optional integrity/deduplication/index aids. Typed sections in the same versioned file hold explicit mesh, scientific-field, material/rig and solver payloads when they cannot be represented by the universal tensor plus declared conventions. Identity resolution and versioned codec fixtures remain tracked under AG-14.
**2026-10-07 follow-up:** Added `AlphaCoverageDiagnostics` and `upload_resident_texture_rgba8_with_mips_and_diagnostics`. Alpha-mask uploads return the baseline coverage, mip-level count and fixed `[f32; 32]` signed post-RGBA8 residual array by value; non-alpha uploads return `None`. The plan is computed once, used by GPU mip generation, and not retained in the residency map, avoiding extra persistent per-texture state. Native `cargo check -p qualia-core-db --lib --tests` with the portable renderer feature set and WASM Webizen renderer test-target check pass. The four isolated production alpha-plan tests pass, including base coverage and residual values. These diagnostics predict the CPU reference filter; GPU readback coverage qualification remains open, as do broader cutoff/NPOT cases and precomputed corrected-chain ingestion.
**2026-10-07 follow-up:** Added `PortalGpu::upload_resident_texture_mip_chain_rgba8` as the direct runtime endpoint for offline-generated chains. It validates exact floor-halved dimensions through 1×1, full mip count, per-level RGBA8 byte lengths, adapter limits and semantic/colour-space compatibility before allocation. It reserves the entire resident chain and the entire transient upload separately, writes each supplied level directly, and skips runtime mip render passes and alpha-plan scratch. For alpha masks, returned diagnostics measure the supplied RGBA8 levels against base coverage. Native and WASM renderer test-target checks pass, including the validator tests; Rust formatting passes. Runtime GPU upload/pixel execution remains blocked by this host's MinGW AWS-LC `nanosleep64` linker failure. HMC/`.10d` representation and resolver integration, KTX2/Basis level decoding/transcoding, and coarse-to-fine streaming remain open.

**2026-10-07 follow-up:** Closed the native GLES minified alpha-mask pixel gap for the current runtime mip path. The first adapter run exposed that GLES stored the nominal 0.8 cutoff at 203/255, below the authored threshold; GPU readback across mip levels 1–3 confirmed the failure. The bounded CPU planner and GPU mip pass now share a two-code UNORM rounding bias while planning one code above the authored cutoff, and the adapter-required test directly asserts generated mip alpha bytes meet the cutoff before verifying the minified mask remains visible. Material textures now bind one sampler each to avoid GLES rejecting a texture used with multiple samplers; nearest minification uses explicit texel/mip selection while magnification follows authored nearest/linear state. The native GL pixel test passes (1/1); native `qualia-core-db` library/test-target check passes; WASM `webizen-render` test-target check passes. Test builds add `COPY_SRC` only for direct mip assertions; production residency usage remains unchanged. Browser GPU parity, broader cutoffs/NPOT patterns, other sampler combinations, transcode/HMC precomputed-chain integration and image/performance qualification remain open. The earlier AWS-LC `nanosleep64` linker note is superseded: switching the portable feature graph to the explicitly selected ring provider enabled the native test link; the Vulkan route remains unsuitable on this host due a driver access violation, so this evidence is specifically from GL.

**2026-10-07 follow-up:** Broadened the native GLES material qualification after the alpha-mask fix. The adapter-required pixel suite now passes 5/5: minified coverage plus direct GPU mip-byte cutoff checks, below/above alpha cutoff shading, AO receiver depth through a cutout, direct sun through a shadow cutout, and sorted two-layer transparency. Fixtures now request cutoff-keyed alpha-mask residency rather than ordinary colour mips. The blend oracle asserts visible contribution/order and opaque final coverage against the actual linear-target values. The four CPU alpha-planner tests pass, including neutral scales for fully opaque/transparent chains; the sampler-uniform packing test passes. Native `qualia-core-db` test-target checking and WASM `webizen-render` test-target checking pass. GL is the qualified native adapter here; browser GPU parity, DX12 pixel execution, expanded sampler/address combinations, cutoffs/NPOT coverage distribution, KTX2/Basis/HMC precomputed chains and visual/performance measurements remain open.
**2026-10-07 follow-up:** Extended the adapter-required AG-05 AO pixel fixture to qualify all three specified tap tiers (4/8/12) on native GLES. Each tier visibly darkens receiver pixels, never brightens the ambient-only fixture beyond one 8-bit code, and preserves alpha coverage. Focused native test passes 1/1; formatting and `git diff --check` pass. This verifies correctness and graceful quality selection on this fixture only; it is not a comparative quality study or performance measurement. Browser parity, complex-scene/contact-shadow quality, temporal/deinterleaved AO, and per-profile timing remain open.
**2026-10-07 follow-up:** Hardened AG-05's internal AO uniform boundary so arbitrary tap requests normalize to the supported 4/8/12 tiers even when callers bypass the Portal setter. A regression covers low, boundary, intermediate and overflow requests. The unit test and the 4/8/12 adapter-required GLES receiver fixture both pass; WASM `webizen-render --tests` check and Rust formatting pass. This closes the profile-contract validation gap, not browser pixel parity or comparative performance evidence.

**2026-10-07 follow-up:** Built the full `qualia-core-db` `portal` WASM package in dev mode, then generated its web-target wasm-bindgen glue from the locally cached 0.2.125 CLI. The linked module is 24,681,441 bytes in dev mode; generated JS exports `portal_init_webgpu`, `QualiaPortal.set_screen_space_ao_enabled`, `set_screen_space_ao_sample_count`, and `screen_space_ao_available`. This proves package linking and JS API presence, not browser GPU execution or pixels. `wasm-pack` could not install its CLI into the locked-down Cargo home, so the same version was built offline into `target` and invoked directly. No browser pixel result is claimed; this environment's browser automation failed to initialize and no browser executable was available on PATH.
**2026-10-07 follow-up:** Added conservative per-material-range caster bounds and refresh them when retained-mesh geometry is deformed. Shadow draws now test model-transformed AABBs against each camera-receiver-fitted cascade, while light-depth bounds remain conservative; this keeps off-camera casters that can affect the visible receiver region and skips ranges outside the cascade. The focused cascade/culling math harness passes 7/7 and the portal WASM library check passes. The updated adapter-required native GLES pixel test could not be compiled because the C: volume ran out of disk while writing rustc incremental cache; after deleting only that failed working-cache directory, 1.2 GiB is free. Native pixel execution and performance impact are therefore unverified. Multi-object/instance culling remains open.
**2026-10-07 follow-up:** Optimized the AO evaluator position reconstruction around the existing fixed 45-degree camera contract. A CPU-built orthonormal camera basis and shared aspect/FOV let each tap reconstruct world position directly from pixel-centre NDC and linear depth, removing full inverse-view-projection ray reconstruction and its per-frame matrix inversion. The AO uniform shrinks from 144 to 112 bytes. Projection/reprojection and basis tests pass in a standalone harness (11/11, including the actual camera module); Naga validates mesh, AO-prepass and AO-evaluator WGSL; native and portal-WASM library cargo checks pass for the updated AO layout/path. The final shared-FOV constant refactor is covered by the standalone camera/AO harness; complete cargo recheck and adapter pixel/performance qualification remain open because this volume has under 80 MiB free. The CACAO deinterleaved/mip hierarchy remains a separately benchmark-gated higher tier [R31]; no runtime speedup is claimed.
**2026-10-07 follow-up:** Added the camera-oriented analytic sky pass. `sky.wgsl` reconstructs a perspective ray from the existing camera matrix and view basis, then evaluates an authored day/dusk/night palette with horizon haze and a sun/moon disk/halo. Native SDR and HDR+bloom pipelines share the shader; selecting a sky preset enables it, while explicit clear-colour input returns to the flat background and is preserved on WebGL2/recovery paths. Naga validates sky, AO and material shaders; a standalone Rust compile validates the sky pipeline helper against workspace wgpu 30. Camera-ray reprojection, AO and atmosphere contract tests pass 14/14 in a standalone harness, including projection-row aspect/FOV checks over three orbit poses. Full crate checks and adapter/browser pixel/performance tests remain open because C: has under 75 MiB free. Fog colour/density are data only so far; distance/height fog, calibrated atmosphere, environment/exposure coherence and WebGL2 sky parity remain open.

**2026-10-07 follow-up:** AG-06 now consumes the shared fog profile in both opaque and blended WebGPU mesh shading. A 32-byte atmosphere uniform carries linear fog colour/density and height controls; fragment shading applies bounded Beer-Lambert transmittance with exponential altitude falloff and a 0.92 maximum opacity. Preset changes update the uniform once, and portal recovery preserves the selected fog profile even when a caller overrides the sky clear colour. The Rust/WGSL binding contract now covers group 6. The WASM `qualia-core-db` library check passes (`--no-default-features --features portal`), Naga parses and validates the updated mesh shader, and a standalone uniform-mapping harness passes. A clean native check is blocked before renderer compilation by MinGW `dlltool` access denied; no new GPU pixel, browser-parity, visual-quality or performance claim is made. Distance/height fog is no longer data-only; calibrated atmospheric scattering, exposure/environment coherence, WebGL2 fog parity and adapter-level qualification remain open.
**2026-10-07 follow-up:** Added an optional fullscreen analytic sky to the WebGL2 fallback, using a generated triangle and the shared orbit-camera FOV/basis, sun direction and ambient state; the authored day/dusk/night palette now follows the camera on both browser GPU backends. WebGL2 mesh fragments also consume the shared profile through bounded distance/height fog. Explicit clear colour disables the sky while retaining the selected fog profile, and sky shader initialization failure degrades to the preset's flat clear. A fresh WASM `qualia-core-db` library check with `portal,webgl2` passes. Browser GLSL compilation/pixels, WebGL2 visual parity and runtime/performance qualification remain open; the shader-string path is Rust-compiled but has not been exercised in a browser. A WASM `--tests` type-check was attempted and is currently blocked by existing unrelated test-only references to native-gated query/volume modules and missing test imports, so it does not qualify this feature.

**2026-10-07 follow-up:** Began AG-07 with bounded manual HDR exposure compensation. `PortalGpu` accepts finite −8…+8 EV, maps stops to `2^EV` once at configuration, retains the value when HDR is unavailable, reapplies it when HDR bloom targets are rebuilt, and the WASM facade exposes configured EV and active HDR availability. Bloom threshold remains pre-exposure scene-linear; the final composite applies exposure before the existing Reinhard curve and surface encoding. Stop conversion/default-preservation tests pass 2/2, and the WASM `qualia-core-db` library check passes with `portal,webgl2`; native clean rebuild remains blocked by MinGW `dlltool` access denied. Direct SDR/WebGL2 exposure parity, a selected versioned tone/output transform, white balance/LUT grading, HDR display negotiation and golden-ramp captures remain open.
[R33]: https://github.com/KhronosGroup/ToneMapping
[R34]: https://docs.acescentral.com/system-components/output-transforms/
[R35]: https://registry.khronos.org/webgl/extensions/EXT_color_buffer_float/

**2026-10-07 follow-up:** Selected and implemented the first versioned portable SDR output transform for AG-07. The HDR scene composite now applies the bounded manual EV compensation then Khronos PBR Neutral v1 (scene-linear Rec.709 to display-linear Rec.709); sRGB attachments encode on write; linear UNORM fallback surfaces encode in the composite shader, so either path applies the transfer once. The Rust CPU oracle follows the reference operation order and passes 5/5 focused tests for stop scaling, default preservation, PBR Neutral mid-grey/highlight behavior and sRGB encoding. A clean `cargo check --target wasm32-unknown-unknown -p qualia-core-db --lib --no-default-features --features portal,webgl2 --offline` succeeds (four unrelated dead-code warnings). `git diff --check` passes. The existing Naga bloom-shader smoke test is not yet executed: native test compilation stops before this crate because MinGW `dlltool.exe` returns access denied, and the installed MSVC Rust target lacks `link.exe`. This does not yet share exposure/transform with direct SDR or WebGL2 scene rendering, and it does not implement HDR display output, white balance/LUT grading or browser/GPU pixel qualification. Next: route all scene paths through a shared output stage, qualify shader output on native/browser adapters, then add fixed-exposure reference captures.

**2026-10-07 follow-up:** Tightened the exposure capability report to reflect the active composite path rather than merely allocated targets. It now checks that the HDR target, required HDR scene pipelines and current bloom policy are all available; the renderer uses the same gate to choose the HDR composite. Manual EV remains configured if any gate becomes unavailable, while the setter returns false until the composite is active. Added a regression test for each missing gate; its native execution remains unverified because MinGW dlltool.exe is denied and the MSVC target lacks link.exe. The WASM portal,webgl2 library check passes after the change. Direct SDR and WebGL2 paths still require the shared final output stage.
**2026-10-07 follow-up:** Routed direct WebGPU SDR rendering through the shared final output transform. The renderer now draws to a budget-admitted scene target, completes opaque and transparent blending, then applies manual EV and the shared PBR Neutral v1/sRGB encoding pass. It uses RGBA16F scene storage whenever the HDR pipelines are available even when bloom is disabled; a format-matched SDR intermediate remains the fallback and `hdr_scene_available` reports whether extended highlights survive to mapping. `exposure_transform_available` reports whether the active output path applies exposure; the original HDR-named methods remain compatibility aliases. The HDR bloom and direct output shaders now share one WGSL transform implementation. Naga parse/binding coverage was added to the existing shader contract test. The CPU oracle passes 5/5 and the WASM `portal,webgl2` library check passes; native Naga/pixel execution remains unverified because MinGW `dlltool.exe` is denied and the MSVC target lacks `link.exe`. WebGL2 still bypasses the final scene output stage; HDR display encoding, grading, fixed-exposure captures and pixel/performance qualification remain open.

**2026-10-07 follow-up:** Completed the WebGL2 final-output slice. The fallback now renders its analytic sky and mesh into a lazily resized, VRAM-ledger-reserved framebuffer with a depth attachment, then composites through the same bounded EV and PBR Neutral v1 SDR operation order. `EXT_color_buffer_float` gates the RGBA16F target; allocation/completeness or budget failure falls back to RGBA8, and output-target failure retains direct canvas rendering. Exposure is converted to a linear scale when configured, not per pixel/frame, and portal capability reporting includes the WebGL2 output target. Source basis: Khronos documents RGBA16F as color-renderable under `EXT_color_buffer_float` [R35]. A clean rebuild after `cargo clean` passes `cargo check --target wasm32-unknown-unknown -p qualia-core-db --lib --no-default-features --features portal,webgl2 --offline`; scoped `git diff --check` passes. Browser GLSL compilation, output pixel comparison against the CPU oracle, resize/budget fallback behavior and native/browser performance remain unqualified. An offline Naga GLSL validation attempt was blocked because its parser dependency `pp-rs` is absent from the local Cargo cache; no dependency was added, preserving offline builds. HDR display negotiation, grading and golden captures remain open.

**2026-10-07 follow-up:** Added fixed CPU output-transform vectors spanning black, middle-grey, SDR grey, saturated red/blue highlights and neutral highlights. A standalone `rustc --test` harness ran the production `render/output.rs` module and passed all 7 oracle tests, including the six new ramp points; this tests the CPU reference only and is not a GPU screenshot/cross-backend match. A native offline Naga GLSL test was explored, but the required `pp-rs` package is absent from Cargo's local cache, so the parser dependency was not enabled and offline builds remain unchanged. Browser shader compilation and pixel comparison are still open.

**2026-10-07 follow-up:** Fixed an exposure double-application in the bloom composite: bloom now combines in scene-linear space and the shared final SDR transform owns the single exposure/WB application. Added bounded ±1-stop artistic temperature/tint gains in linear Rec.709 and wired them through direct WebGPU, bloom, WebGL2, and the WASM portal; zero is identity. The control is explicitly not Kelvin camera calibration or an ACES Input Transform [R36]. The production CPU output module passes 9/9 tests; offline Naga parse/validation passes for the shared composite and bloom WGSL; the clean WASM library check passes. Chrome headless WebGL2 compiled and linked the production vertex/fragment GLSL, rendered center pixel `(185,185,185,255)` at EV 0 with identity WB, reported no GL errors, and completed RGBA16F (`EXT_color_buffer_float`) and RGBA8 fallback targets. The WASM `--tests` check fails on 64 test-target compilation errors outside this rendering slice (native-gated modules, missing test imports/dependencies and incompatible offscreen constructor references); native focused tests remain blocked before crate compilation by MinGW `dlltool.exe` access denied. Cross-backend golden pixel parity, actual resize/budget fallback injection, native GPU execution, calibrated camera input transforms, LUT grading, HDR display output and performance qualification remain open.

[R36]: https://docs.acescentral.com/system-components/input-transforms/


**2026-10-07 follow-up:** Added `render::instance_culling`, an allocation-free CPU frustum selector with stable source indices, caller-owned output, worst-case capacity preflight, six WebGPU homogeneous clip-plane tests and fail-open invalid-input behavior. Standalone tests pass 8/8, including a 512-case deterministic comparison against an eight-corner reference oracle, including a perspective near/far/behind-camera/crossing-bound fixture and a 10,000-instance stable-index caller-buffer exercise; `cargo check --target wasm32-unknown-unknown -p qualia-core-db --lib --no-default-features --features portal,webgl2 --offline` passes with four unrelated dead-code warnings. The frustum planes are extracted once per view; each instance transforms six planes into local space and evaluates AABB support radii instead of transforming and classifying all eight bounds corners against six planes. This optimization is checked against the corner oracle; no runtime speedup is claimed without benchmark data. This is not yet connected to renderer submission. Next: keep immutable mesh/material data separate from per-instance transform and semantic-ID streams, integrate stable visible-index indirection into mesh, shadow, AO and pick passes, then validate 100/50/500 production scenes and a 10,000-repeat batching stress case with draw counts, frame percentiles, allocations and picks. GPU compaction/indirect draws remain benchmark-gated by O-06; no performance improvement is claimed yet.


**2026-10-07 follow-up:** Defined the native/WASM mesh-instance upload record as an 80-byte `#[repr(C)]`/`Pod` record with a 64-byte column-major transform, lossless semantic `u64` split into two shader-portable `u32` words, and explicit zero padding. Added caller-buffered `compact_visible_records`, which checks output capacity and all indices before writing so failures leave output unchanged. The isolated host Rust suite passes 10/10, including layout/identity round-trip, stable compaction and atomic failure tests; the WASM portal/webgl2 library check passes and compiles the `Pod` ABI. This is data/visibility infrastructure only: GPU storage-buffer binding, indexed instanced draws and the shadow/AO/picking consumers remain the next integration gate.


**2026-10-07 follow-up:** Wired a bounded GPU instance stream into the retained mesh renderer. The 80-byte caller ABI is uploaded directly from borrowed native/WASM records into a Storage buffer; geometric growth reserves replacement bytes before releasing the old reservation and refuses overflow or budget denial without losing the active stream. Main direct/HDR, shadow, and AO indexed draws now use the stream's instance count. Validation accepts finite affine rigid transforms and positive uniform scale while rejecting transforms whose normal handling is not yet correct. The portal reports record stride and rejects malformed packed input; multi-instance transparent material upload is refused until a sorted path exists. cargo check --target wasm32-unknown-unknown -p qualia-core-db --lib --no-default-features --features portal,webgl2 --offline passes. Production mesh, sun-shadow, and AO WGSL modules pass Naga parse and validation. MSVC could not be exercised on this host: Visual Studio C++ tools, cl.exe, and link.exe are absent. A Windows GNU-target check was also attempted but stopped in dependency build because dlltool.exe returned access denied. Renderer adapter/pixel validation and performance measurements are outstanding; instanced picking, CPU visibility submission wiring, non-uniform normal transforms, and transparent sorting are follow-up work.

**2026-10-07 follow-up:** Upgraded the instance data contract to ABI v2 (128-byte records) to support non-uniform scale and reflected transforms without vertex-stage matrix inversion. Rust cold-side packing computes the inverse-transpose normal frame and determinant sign; the WebGPU stream verifies the supplied normal frame against the transform before upload. Mesh lighting uses the normal matrix and re-orthogonalizes tangents, while mesh, AO and sun-shadow storage declarations share the updated record stride. Added an explicit ABI-version query/check and caller-buffered WASM pack operation. The WASM portal,webgl2 library check passes after the API/layout update. Focused v2 layout, inverse-transpose and forged-frame tests are present but have not run on this host, and Naga validation/runtime pixel checks for the changed WGSL are still required. I retried MSVC with Visual Studio 18 and the installed MSVC Rust toolchain: cl.exe and link.exe resolve, but link fails at the first Rust build script with LNK1181 because no Windows SDK kernel32.lib is installed/discoverable; this is an environment prerequisite, not a renderer source diagnostic.

**2026-10-07 follow-up:** Retried MSVC after Visual Studio 18 and the MSVC Rust toolchain became available. cl.exe and link.exe resolve inside VsDevCmd, but the build stops while linking dependency build scripts because no Windows SDK libraries are installed/discoverable (LNK1181: kernel32.lib); MSVC engine compilation and native tests are therefore still unverified. The final WASM portal,webgl2 library check passes. cargo check --target wasm32-unknown-unknown -p qualia-core-db --lib --tests --no-default-features --features portal,webgl2 --offline was also attempted to compile the new unit tests, but the repository's current test target fails with 64 unrelated errors (native-only modules/APIs, missing imports/dependencies and stale test references); no diagnostics pointed to the changed renderer files. ustfmt --check and scoped git diff --check pass. Naga validation after the ABI-v2 WGSL change and execution of v2 unit/pixel tests remain open.

**2026-10-07 follow-up:** Added WASM exports for the allocation-free SoA visibility selector and byte-oriented record compactor. Hosts provide caller-owned bounds, transforms, view-projection, index output, source records and compacted output; the selector fails open on invalid geometry, and both selection and compaction preflight shape/capacity before writing. These functions complete the portable caller-controlled cull → compact → upload sequence without JS-side ABI casts or record-layout duplication. The caller and renderer still need to invoke selection with the active camera and appropriate per-mesh bounds each frame; this is not yet automatic frame-loop culling. The renderer limit is shared at 10,000 records across packing, culling and GPU upload. The library WASM check passes; the broad test-target check again fails on the known unrelated 64 repository test errors, without diagnostics in the modified render modules.

**2026-10-07 MSVC verification:** The Windows SDK is now installed and selected by Visual Studio 18 VsDevCmd (WindowsSDKLibVersion=10.0.28000.0; Kits library directories 10.0.22621.0, 10.0.26100.0 and 10.0.28000.0 are present). cargo +stable-x86_64-pc-windows-msvc check --target x86_64-pc-windows-msvc -p qualia-core-db --lib --offline passes with the normal default GPU feature profile (7m09s). An initial reduced --no-default-features --features portal,webgl2 MSVC check failed with 16 existing module/feature-gating errors; it was not an SDK/linker failure. Native library compilation and native `cargo check --tests` are verified, and focused native CPU tests pass 16/16 across culling/compaction and ABI-v2 transforms. Native GPU adapter/pixel validation and performance qualification remain separate gates. Earlier log entries below record the environment before the SDK became available.

**2026-10-08 follow-up:** MSVC focused runtime checks now pass: 12/12 ender::instance_culling tests (including packed cull/compact, affine world-space bounds and a 10,000-instance stable-order stress case), 4/4 ABI-v2 instance-stream tests (non-uniform scale, reflection, singular and forged-frame rejection), and 1/1 full Naga validation test over the production viewport WGSL modules. These are CPU/compiler validation results; no adapter-backed pipeline, render-pixel or performance result is claimed. Next AG-08 work is to invoke visibility selection from the camera/frame path and complete semantic instance picking, then qualify supported GPU adapters.
**2026-10-08 implementation checkpoint — renderer portability and runtime foundations:** A coordinated implementation batch updated the QualiaDB work order in `qualiaDB/docs/planning/32-aaa-graphics-swarm-implementation-work-order.md`. The WebGPU mesh, sun-shadow, AO prepass and pick pipeline layouts now fit portable bind groups 0–3, with material factors and the six texture/sampler slots combined. A native GPU picking bug was fixed by reserving R32Uint zero for a miss and biasing tensor hit IDs by one; node index zero remains selectable. The renderer now has a bounded caller-buffered texture-mip residency planner, fixed-memory quality hysteresis policy and shared VibeScript initial tier selection. A versioned `.10d` v2 manifold identity/typed-field manifest and outer section-envelope binding were added; entity identity is explicitly independent from the optional integrity/index digest, with EMF represented as one field family and extensions allow-listed. Texture decode limits now preserve PNG limit errors, and corrected render pixel/accounting oracles match the actual sRGB output and admitted GPU resources.

Verification on this workstation: MSVC `cargo test --lib --offline --target x86_64-pc-windows-msvc -p qualia-core-db render:: -- --test-threads=1` passed **476/476** in 20.21 seconds after a 5m22s build. A second focused MSVC `container_10d` run passed **152/152**, including the five identity-v2 codec tests, two outer-envelope tests and the v2 section/version gate. Browser WASM `cargo check --offline --target wasm32-unknown-unknown -p webizen-render --no-default-features --features qualia` passed in 4m49s. These results validate native test behavior and browser compilation only; they do not claim browser GPU execution, WASM pixel parity, game vertical-slice integration, or performance improvement.

**Still open:** run the browser app and exercise adapter capability/refusal/recovery paths; integrate KTX2/Basis transcode and the deterministic mip planner into the live asset-loading scheduler; gather same-scene native/browser visual, CPU/GPU timing and memory receipts; complete photorealistic lighting and the stylized game vertical slice; update and qualify the asset pipeline and game-side acceptance manifests. The overall AAA programme remains **in progress**; no capability is promoted to Verified by these checks alone.

**2026-10-08 follow-up — MID2 integration and validation:** QualiaDB now has an opt-in caller-buffered mesh compiler adapter that appends the v2 manifold identity section to a verified `.10d` output while preserving existing section bytes and re-sealing the whole-file CRC. The MID2 codec now requires explicit extension-kind allow-lists on both encode and decode, defines `StableEntityId` as an authority-issued globally unique continuity handle rather than a content digest, and canonicalizes signed zero in its ten coordinates. Existing compile output remains opt-in and unchanged. MSVC focused runs passed **153/153 `.10d`** and **478/478 renderer** tests; browser-target `webizen-render` check passed. The full 9,241-test library binary compiled, but the full suite is not green on this machine: one CUDA test requires an unavailable `cuda.dll`/`nvcuda.dll`, and a separate ternary GPU test terminated with `STATUS_ACCESS_VIOLATION`. These failures are outside the rendering/MID2 test subsets. Texture mip planning remains standalone; HMC still uploads full decoded chains. The next tracked slice is deterministic budget-prefix admission and physically smaller coarse-first residency, followed by safe refinement/rebinding, KTX2/Basis transcoding, browser GPU execution, measured quality tiers, and game vertical-slice qualification. Programme status remains **in progress**.

**2026-10-08 follow-up — budgeted coarse-first HMC textures:** Added an allocation-free texture metadata preflight, deterministic best-effort mip-prefix planner with explicit deferred byte/count reporting, caller-buffered CPU mip reduction for linear-light colour/data/renormalized normals, and GPU APIs that allocate only a selected coarse mip suffix. Native HMC loading now accepts per-load texture residency/upload budgets, plans before pixel decode, uploads from the selected physical base extent, returns an admission report, and binds typed neutral per-slot fallback maps when budget or GPU reservations defer a texture. Legacy HMC loading remains full-chain by default. Alpha masks remain full-resolution-only when admitted because a caller-buffered coarse coverage correction is still missing; frame-time refinement/eviction awaits safe bind-group rebuild or indirection.

Latest evidence: MSVC renderer tests **491/491**, `.10d` tests **153/153**, native Webizen renderer tests **63/63**, and browser `webizen-render` WASM check all pass. The HMC CPU reducer fixtures verify coarse dimensions, linear-light reduction, and explicit alpha deferral; actual end-to-end HMC budget fixture and browser GPU execution remain open. KTX2/Basis transcoding, safe refinement, same-scene performance receipts, visual quality comparison, and game vertical slice remain open. Overall programme status: **in progress**.

**2026-10-08 follow-up — alpha-mask degradation and KTX2 structural inspection:** HMC coarse-first admission now supports caller-buffered alpha-mask reduction that preserves authored base coverage at the actual coarse bytes/cutoff, including odd dimensions and closest-quantized coverage. The CPU stage deliberately avoids applying the GPU-generated-mip rounding margin twice. Added an allocation-free KTX2 container inspector with checked header/level ranges, section order and padding, DFD outer framing, and standard supercompression rules. This is only structural inspection: DFD sample/texel semantics, Basis/Zstd/ZLIB decode/transcode, target GPU format selection, and GPU upload remain open. Registered vendor schemes are rejected; zero-level block-compressed validation is also open pending DFD interpretation. The partial mip planner audit found deterministic admission/deferred accounting, with resident-byte accuracy still caller-supplied and eviction still whole-texture only.

Verification: MSVC renderer/core library tests **505/505**; native `webizen-render` library tests **63/63**; browser `webizen-render` WASM check passes. One all-target Cargo attempt exceeded the Windows paging file while mapping the large core `.rlib` for unrelated integration targets; scoped library commands passed. No browser GPU run, Basis transcode, frame-time mip refinement, performance qualification, or game vertical slice is claimed. Programme remains **in progress**.

**2026-10-08 follow-up — constrained KTX2 RGBA8 and HMC fallback:** The shared HMC decoder now routes `image/ktx2` with case-insensitive MIME matching and parameter/whitespace tolerance through allocation-free KTX2 inspection and caller-buffered base-level copy. Supported scope is exactly structural validation by the existing KTX2 parser plus uncompressed `VK_FORMAT_R8G8B8A8_UNORM`/`_SRGB` (37/43), 2D, non-array, one face, `typeSize=1`, no supercompression. Native HMC loading treats unsupported image preflight/decode as a per-resource deferral and binds the corresponding typed neutral fallback while preserving mesh/other supported maps; package resolution and digest failures remain fatal. Latest recorded checks: MSVC core renderer **510/510**, native `webizen-render` **63/63**, WASM target check passed. Decoder fixtures cover format inspection, MIME parameters, supported formats and refusal cases; a live end-to-end HMC unsupported-KTX2 fallback fixture remains outstanding. No general KTX2/Basis support is claimed: DFD sample semantics, other `vkFormat`s, compressed formats, BasisLZ, Zstd/ZLIB, transcoding, mip-chain decode, GPU compressed upload, browser GPU execution, and performance evidence remain open.

**2026-10-10 recovery-branch follow-up:** On `qualiaDB` branch `0.0.40.7-codexfailurerecovery`, added a renderer-owned temporal scheduling contract with explicit motion/reactive admission, history reset/publication state, a CPU resolve/output oracle, and final-output ordering; added a persistent scene-depth owner and bounded HMC water upload path whose transparent pass does not write authoritative depth; added verified browser HMC mesh resolution plus digest-backed resident-upload generations; and added a backend-independent terrain seam oracle covering horizontal/vertical joins at four resolutions and 256 projected edge samples. QualiaDB core check passes; focused temporal tests pass **5/5**, frame-graph tests **9/9**, terrain seam acceptance **1/1**; native `webizen-render` check and browser WASM `webizen-render` check both pass. These slices do not claim GPU motion-vector/reactive producers, persistent GPU history texture binding, native adapter pixels, browser GPU runtime, or platform performance qualification. Overall status remains **In Progress**.

**2026-10-10 swarm follow-up:** Added a renderer-owned, fail-closed temporal GPU helper and WGSL path with linear-depth disocclusion, motion reprojection, reactive weighting, history reset/publication, and final-output handoff; indexed uncompressed RGBA8 KTX2 levels now select and upload deterministically from projected footprint and shared budgets, with typed deferral and end-to-end coarse-level/unsupported-format fixtures; and added bounded environment-probe lighting with roughness-aware specular interpolation, stylized controls, and direct-light fallback. QualiaDB `webgl2` library checking passed; focused lighting tests passed **11/11**, temporal/Naga tests **3/3**, HMC tests **5/5**, and the browser WASM renderer check passed. The temporal helper remains an explicit API seam because real motion/reactive producers and frame-graph ownership are not yet wired; compressed/Basis KTX2, GPU probe binding, adapter-backed runtime pixels, Windows MSVC/DX12, macOS/Metal, Android Chrome, iOS Safari/WebKit, and performance receipts remain open. Programme status remains **In Progress**.

**2026-10-10 swarm increment:** The recovery branch now carries a renderer-owned temporal submission contract with explicit resolve → history publication → final-output ordering, resize/invalidation handling, and fail-closed producer admission. HMC water now preserves authoritative opaque scene depth, avoids redundant geometry uploads, validates finite/indexed geometry against an explicit budget, and exposes bounded normal/reflection/foam/shore quality tiers. Ambient vegetation/effects now use stable seeds, world-frame wind, a fixed-capacity event pool, and event-only GPU refresh with explicit CPU-degraded and disabled fallbacks. Central verification passed the QualiaDB `webgl2` library check; frame-graph **10/10**, temporal/Naga **3/3**, water **3/3**, HMC **7/7**, vegetation **4/4**, telemetry **1/1**; native default/no-default renderer checks; and browser WASM renderer checking. This does not claim real motion/reactive producer wiring, sampled depth/SSR/reflection bindings, compressed/Basis KTX2, GPU probe bindings, adapter-backed pixels, Windows MSVC/DX12, macOS/Metal, Android Chrome, iOS WebKit, game vertical-slice acceptance, or performance receipts. Overall status remains **In Progress**; next steps are real temporal producers/history resources, probe/material and texture capability integration, game vertical-slice wiring, then cross-platform runtime/pixel/memory/performance qualification.

**2026-10-10 vertical-slice increment:** QualiaDB now exposes a concrete renderer-owned scene-colour temporal path with an explicit host contract for genuine linear-depth, motion-vector, and reactive-mask producers; missing producers, invalid extents, resize, and camera discontinuities fail closed or reset history without treating hardware depth as linear depth. HMC texture residency now supports generation-stamped coarse-to-fine replacement, deterministic omission eviction, material rebind, bounded staging/retention, and explicit refusal for unsupported compressed/Basis-like inputs. game-demo now derives deterministic scene/water/effects/quality/camera presentation state from story facts, prefers verified HMC scene snapshots when the rebuilt WASM API exists, and retains the generated-package `.10d` fallback. QualiaDB verification passed frame-graph **11/11**, temporal/Naga **4/4**, texture lifecycle **15/15**, HMC **7/7**, texture refinement/rebind/eviction **1/1**, native checks, and browser WASM checking. game-demo WASM/Rust checks passed; worker Chromium self-test passed **91/91**, HMC verification **341** entries, and scene smoke checks. WebGPU runtime, generated `web/pkg` refresh, actual temporal producer supply, compressed/Basis KTX2, GPU probe binding, platform adapter pixels, and performance receipts remain open. Overall status remains **In Progress**.
