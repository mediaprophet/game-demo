# ADR 0001: Application surface, WASM profile, and pinned QualiaDB revision

Status: Accepted — spikes 1–5 verified in Chrome headless (2026-10-02) at
pinned revision `27d1644a`: semantic/rules/OPFS + `.10d` compile→ingest→
semantic pick + Vibe eval/denied-capability, 10/10 checks. Remaining open
proofs: real-GPU tier-2 path, HMC pack, animation contract, save/replay
versioning, frame-time budgets.
Date: 2026-10-02

## Context

Phase 0 requires selecting the Webizen/QualiaDB application surface (D-018),
the WASM capability profile (D-017), and a pinned revision, with measurements
recorded before production build (D-022).

## Options considered

| Surface / profile | Contents | Assessment |
|---|---|---|
| `webizen-lite-wasm` (`wasm-ontology`) | Ontology MCP JSON-RPC kernel only | Deliberately limited; no graph mutation, no SHACL bridge, no render path. Insufficient alone. |
| `qualia-core-db` `wasm-webcivics` | Semantic interchange, SHACL, modal logic, SPARQL/Q42 kernels, civics receipts, `OpfsVfs` storage, device-storage policy. No GPU/LLM. | Best fit for the authoritative simulation surface: rules + explanation + persistence without GPU weight. Compiled clean to `wasm32-unknown-unknown`. |
| `qualia-core-db` `portal` / `wasm-full` | Adds WebGPU render portal, scientific stack, optional `wasm-llm` | Needed only for the storybook-3D presentation and local inference; heavier bundle. Deferred pending D-019/D-020 spikes. |
| `webizen-web` (`qualia-wasm`) | `wasm-full` engine + canvas2d stub + portal | Existing combined package; likely the eventual host shell, but `wasm-full` exceeds first-playable needs. |
| `webizen-render` | PGA/N-dimensional renderer, `RenderQuin.semantic_id` picking, wgpu | Presentation surface for the 3D profile; WASM build under verification. |
| `vibe-wasm` | VibeScript parse/check/eval/compile + `capability_invoke` | Bounded authoring host for QG-04; integrated in Phase 5 spike. |

## Decision

- Game shell crate `rolling-commons-shell` depends on `qualia-core-db` by path
  with `features = ["portal", "wasm-webcivics"]` plus `vibe-wasm`: the portal
  feature supplies the `.10d`/render/pick surface the storybook-3D lead needs,
  while `wasm-webcivics` keeps the semantic/rules/storage surface. The
  standalone `webizen-render` crate is **not** used — the portal-internal
  `PortalGpu` path is the renderer.
- The **storybook-3D presentation profile leads** (user decision, D-032),
  verified at spike level: authored prop → `compile_mesh_to_10d_with_nodes` →
  `QualiaPortal::load_10d` → tensor-buffer semantic pick. An illustrated/2D
  profile remains the required low-spec fallback (D-036); the portal's tier-1
  CPU path already functions without a WebGPU device.
- Pin QualiaDB commit **`27d1644a`** (branch `0.0.40.5`, fast-forwarded onto
  `rolling-commons/phase0`), replacing `6356bb5a` after the DNS work landed;
  every ledger row references this commit until a later verified pin replaces
  it.
- Upstream capability work happens on a dedicated branch in
  `C:/github/qualiaDB`, pinned back here once verified (QualiaDB-only contract).
- Picking is tensor-node oriented: `.10d` mesh organs are display geometry;
  semantic pick targets must be authored as `Tensor10D` nodes (embedded via
  `compile_mesh_to_10d_with_nodes` and uploaded via `upload_tensor_buffer`).

## Consequences

- The first playable is playable without GPU features; the 3D profile is a
  presentation layer over the same projection, not a second engine.
- LLM inference (`wasm-llm`) stays out of the base build; D-020 spike decides.
- Any gap found in `wasm-webcivics` becomes a QG upstream task before the game
  feature proceeds.
