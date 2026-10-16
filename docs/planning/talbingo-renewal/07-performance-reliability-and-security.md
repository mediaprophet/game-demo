# 07 — Performance, reliability and security

Status: proposed work. None of the targets below is a measured property of the current build. Read [architecture](03-architecture-state-and-platform.md), [art](06-art-audio-and-rendering.md), and [testing](08-testing-and-release.md) together with this chapter.

## 1. Define supported experiences before optimizing

The starting tiers are **Low** (30 FPS, reduced decorative detail) and **Standard** (60 FPS). A later high-quality tier is optional and must not postpone the two supported tiers. Both retain the same collision, simulation, interaction reach, wildlife warnings and accepted outcomes. Presentation quality is never allowed to change game rules.

During M0, select and record actual reference machines: one integrated-GPU laptop, one ordinary desktop/laptop GPU, and one physical mobile device on a supported browser. Record OS/browser versions, GPU/driver, renderer tier reported by Qualia, viewport, device pixel ratio and power mode. A resized desktop browser proves layout only. Do not promise all browsers or mobile hardware from feature detection alone.

If no supported Qualia renderer starts, show a clear capability/retry panel and a read-only trip guide. Do not report that screen as a playable renderer fallback. If another Qualia render tier works, test its visual and semantic parity explicitly. Investigate upstream device-loss handling; the WebGPU API's device lifecycle belongs behind the renderer boundary, not in a second game renderer.

## 2. Starting performance budget

Ratify or revise these provisional thresholds after M0 profiling, with a recorded reason. Lowering a target requires a new supported-device decision, not silently changing the acceptance report. [Art allocations](06-art-audio-and-rendering.md) fit within these envelopes.

| Metric / measurement | Low | Standard | How to interpret |
|---|---:|---:|---|
| Frame interval, p95 over five active minutes | ≤33.3 ms | ≤16.7 ms | End-to-end interval, not just JS submission time |
| Frame interval, p99 | ≤50 ms | ≤33.3 ms | Exclude explicit loading; include traversal and actor animation |
| Simulation + contact + command handling, p95 per 20 Hz tick | ≤8 ms | ≤4 ms | Instrument separately from rendering and DOM |
| Sustained >100 ms stalls in an ordinary five-minute walk/drive | 0 | 0 | Record all >50 ms tasks; investigate causes |
| Input-to-visible reaction, p95 on reference device | ≤150 ms | ≤100 ms | Measure high-speed capture or reliable browser tracing; dispatch time alone is insufficient |
| First usable scene, warm cache | ≤3 s | ≤2 s | User-visible ready state after assets are validated |
| First usable scene, cold 20 Mbit/s / 80 ms latency fixture | ≤15 s | ≤12 s | Report engine/pack transfer sizes; do not hide download time |
| Already-installed destination transition | ≤3 s | ≤2 s | UI stays responsive and cancellable throughout |
| Estimated resident texture bytes | ≤32 MiB | ≤64 MiB | Include mipmaps; use actual encoded GPU format, not PNG file size |
| WASM linear memory high-water target | ≤256 MiB | ≤512 MiB | Includes asset construction; measure actual memory pages |
| Combined JS + WASM + estimated GPU target | ≤512 MiB | ≤1 GiB | Report components; APIs do not provide universally accurate process totals |
| Initial site art payload, compressed | ≤12 MiB | ≤24 MiB | Excludes shared engine; loading budget still includes both |
| Durable save acknowledgement, p95 | ≤500 ms | ≤250 ms | Async operation; do not block input awaiting storage |

The existing WASM file is roughly 9.7 MB uncompressed on disk. That is not a measured transfer size. Keep bootstrap, shared engine, shared art and each site separately reported. Adopt progressive pack installation through Qualia's supported volume/pack APIs only after verification. An offline claim requires a tested install-complete receipt and a network-disabled reload, not merely a warm browser cache.

Qualia's 42 MiB Sentinel bound applies to its semantic evaluator arena. Asset construction, JS objects, audio, textures and renderer resources need their own explicit budgets. Do not put a global “zero heap” or “42 MB game” claim on the UI. Follow upstream hot/cold allocation requirements when exposing new APIs.

## 3. Measurement implementation

Add a proposed `web/talbingo/diagnostics.mjs` presentation adapter and `scripts/profile-talbingo.mjs` harness after shell extraction. Use the existing portal receipts and producer report where available; do not fabricate GPU timing from a `performance.now()` wrapper around submission.

Collect a bounded ring of samples (e.g. 300 frames), aggregate p50/p95/p99, and export a small local diagnostic receipt on request. Do not write a JSON object, console line or persistent log every tick. Authoritative event logging is separate and follows the session contract.

Instrument these spans:

1. Module fetch, WASM instantiation and capability initialization.
2. Manifest validation, payload read, unpack, geometry preparation and GPU upload, separately.
3. Input sampling, simulation/contacts, receipt projection, actor pose and HUD updates.
4. Portal render submission; GPU timing only when its verified profiling API supports it.
5. Save queued, durable acknowledgement and recovery read.
6. Scene load/unload and live resource counts before/after each transition.

Run cold and warm tests separately. For each tier run five warm-ups, then three fixed-seed five-minute routes with identical input tapes. Report each run and the aggregate; do not report only the fastest. Include all six scenes, The Pines arrival, dense vegetation, water, a towing maneuver and repeated transitions. Store trace files under an explicitly budgeted QA run directory; reject oversized traces before writing.

## 4. Optimize the actual path

The current page rebuilds a site synchronously via `m.talbingo_scene(site)`, serializes full drive input with JSON each tick, repeatedly converts maps, updates some HUD elements every tick and applies actor poses at simulation rate. Measure these costs before changing APIs.

Implementation sequence:

1. Move constant site/config data out of repeated work and cache immutable compiled assets by content digest within a bounded Qualia cache.
2. Publish HUD updates on value change or a modest display cadence, while warning/state transitions update immediately. Batch DOM reads/writes.
3. Ask upstream for a versioned caller-buffered simulation/projection interface if JSON/Map conversion is material. Keep the public ABI documented and tested; never add a private binary layout only understood by this page.
4. Interpolate render transforms between accepted simulation states through the verified presentation interface. Collision and decisions remain at the fixed simulation tick.
5. Precompile/package site content where supported. Move expensive cold work to Qualia's bounded/cancellable loader. A web worker is a host scheduling option only behind a supported upstream contract.
6. Stream/load the active site and required neighboring corridor segment, with an explicit overlap budget during transition. Do not keep all six full scenes resident.
7. Use measured tier controls for resolution, vegetation density, shadows, AO, water and audio voices. Preserve silhouettes/landmarks and collision; degrade decorative details first.

Do not skip simulation ticks silently to improve FPS. Keep the existing 20 Hz step initially; profile and validate before changing it. Limit catch-up work (proposed maximum five ticks per visual frame); if overloaded, pause with a recoverable message instead of accumulating an unbounded backlog. On tab hiding, clear held inputs, pause the canonical clock according to the session policy and resume explicitly without simulating elapsed wall time. Seed gameplay randomness; audio noise may be nondeterministic only if it cannot change state, cues or replay verification.

## 5. Resource ownership and recovery

Create one runtime owner for a portal, loader, scheduler, input subscriptions and audio presentation session. Every acquired handle must have a corresponding cancellation/disposal path. Use the actual upstream resource lifecycle; add one upstream if unavailable. Garbage collection is not a GPU unload policy.

Proposed lifecycle: `idle → preparing → active → pausing → paused → disposing → disposed`; failures carry an error code and recoverable checkpoint. Scene transition state is specified in chapter 03 and must not be reimplemented in this controller.

| Failure | Required behavior | Verification |
|---|---|---|
| Missing/mismatched asset | Stop activation; retain previous scene/checkpoint; identify failed pack and retry | Delete or corrupt one fixture asset |
| Compilation/loading exceeds budget | Cancel safely; offer validated lower tier or return to selection | Artificial low byte/time budget |
| GPU initialization failure | Try only verified Qualia fallback; otherwise capability screen | Disable/unavailable adapter fixture |
| GPU device/context loss in play | Pause input/clock, preserve valid state, dispose invalid resources, reconstruct from checkpoint | Fault injection at drive and camp |
| Vibe/runtime error | Stop commands and audio; keep diagnostic ID; retry/reload from safe checkpoint through a restarted scheduler | Inject bounded simulation error |
| Save quota/denial | Keep current game in memory; explicitly show unsaved state and recovery/export option if supported | Denied write and quota fixture |
| Partial write or incompatible save | Validate checksum/version; retain last good checkpoint; never overwrite it with failed migration | Truncation and newer-version fixture |
| New scene selected while loading | Cancel old generation; only latest accepted request may commit | Rapid select/cancel/load sequence |
| Audio unavailable or suspended | Continue silently; indicate disabled setting; captions remain | Reject AudioContext resume |
| Controller removed or focus lost | Clear only that source's held intents; pause where appropriate | Disconnect while accelerating |

At scene disposal release actor roots, mesh/texture handles, spatial queries, audio emitters and listeners; cancel pending uploads; remove old UI listeners. A session checkpoint persists across renderer recreation. Recreating a scene must not reset rig damage or award an activity twice.

Leak acceptance: cycle all six sites ten times, then return to the same idle scene. After warm-up/caches stabilize, live asset/actor/audio counts return to the expected bounded baseline. Memory has no sustained positive trend; retained resource growth over the last five cycles must be explained. WASM linear memory may retain its high-water allocation, so distinguish capacity from live allocations rather than requiring its size to shrink.

## 6. Content and save trust boundaries

There is no need to add account authentication to this single-player roadmap. The relevant boundaries are content packs, external reference links, save imports and the application origin.

- Validate manifest versions, IDs, hashes, declared byte lengths, decompression sizes, counts, finite coordinates, index ranges and capability requests before constructing runtime objects. Hash equality detects corruption; it does not establish a trusted publisher.
- Use Qualia pack validation and capability enforcement. Allow reviewed fixture modules; do not execute arbitrary URLs or imported save strings as Vibe/JavaScript. A save references known rules and pack versions, not executable code.
- Bound archive entry count, path length, unpacked bytes, texture dimensions, geometry complexity, route nodes and simulation work. Reject traversal paths, duplicate identities and cyclic dependencies where forbidden.
- Display source text with text nodes. Permit only reviewed link schemes/hosts for outward links; keep `noopener` for new windows. Pack metadata is data, never HTML or instructions.
- Adopt a production Content Security Policy after extracting inline code/style; test exact WASM/worker requirements on supported browsers. Avoid adding broad exceptions to solve an uninvestigated integration failure. Do not copy the developer server's permissive CORS headers into a production policy blindly.
- Keep progress local by default. Diagnostic export is explicit, bounded and previewable; redact local paths, user identifiers and imported private source records. No geolocation permission is required for the illustrative scenario.
- Delete/reset controls clearly identify their scope and require confirmation only when losing progress. Export before destructive migration when the upstream API supports it. Imported saves never replace the last good local save until validation and activation succeed.

For future collaborative or runtime-AI extensions, create a new threat model and authority boundary first. Do not prebuild a backend, secret store or telemetry service for the present plan.

## 7. Implementable work packages

| ID | Steps and concrete output | Dependencies | Acceptance |
|---|---|---|---|
| PERF-01 | Add measurement spans and bounded receipt; extend browser harness; establish device matrix and baseline | ARC-01, QA-01 | Three comparable runs, no missing metric described as zero |
| PERF-02 | Profile allocations/serialization/DOM; implement change-driven projections and verified buffered calls where justified | ARC-02/03, PERF-01 | Same replay result; p95 budget improves with before/after receipt |
| PERF-03 | Use bounded upstream asset cache and staged loading; record unload counts and transition peak | ARC-04, DATA pack work, PERF-01 | Ten six-site cycles, no unbounded resource growth; cancellation works |
| PERF-04 | Map Low/Standard settings to verified renderer/audio controls; preserve semantic geometry | ART work, SIM-02, PERF-01 | Same command outcomes across tiers; five-minute budgets met |
| REL-01 | Specify/install runtime owner, disposal and scheduler lifecycle; audit every listener/resource | ARC-02/04 | Re-enter runtime repeatedly with one active scheduler and one input dispatch |
| REL-02 | Implement fault states/retry/checkpoint recovery table through public Qualia APIs | ARC-04/05, REL-01 | Each injected failure leaves a usable recovery action and preserves last good save |
| REL-03 | Separate input sources; handle hide/blur/disconnect; add master mute/voice lifetime checks | UX input work, ART audio work | No stuck throttle; muted bus is silent; resume causes no catch-up jump |
| SEC-01 | Add negative pack/save cases, URL/text safety and bounded loader validation | ARC-03/05, DATA schemas | Corrupt, oversized and untrusted content rejected before activation |
| SEC-02 | Ship tested origin/CSP/offline-cache policy and explicit diagnostic export | ARC-06, QA-05 | Clean deployed load; offline behavior matches labels; no unexpected network requests |

Each package should be split into 1–3 day reviewable changes. An upstream API change may take longer and needs its own issue/receipt. Avoid a single “optimize everything” patch.
