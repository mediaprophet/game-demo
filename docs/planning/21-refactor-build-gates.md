# QualiaDB refactor build gates for Maslows Challenge

The failing release runs used QualiaDB `1b2c944f08ab0e73ddac257c420dd2ab79825836`
(tag `v0.0.40.12`). The game now pins the later `0.0.40.7` commit
`e19423df474482df24fc8e0aba35328412f63f19`. The game uses the
**full QualiaDB `wasm-full` engine**, including QualiaPortal rendering. These
are generic upstream build issues; changing the game to the WebCivics profile
would remove required engine capabilities.

| Gate | Evidence | Required QualiaDB work |
| --- | --- | --- |
| WASM Release and Qualia Pages | [WASM run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684727), [Pages run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684694): `webizen-lite-wasm` fails when `gpu_context.rs` compiles `wgpu` code with `gpu-runtime` disabled. | The generic feature gate fix is published as `e19423df` on `0.0.40.7`; the branch [Pages rerun](https://github.com/mediaprophet/qualiaDB/actions/runs/37194859765) is validating it. The old release tag still points to the failing commit. |
| QDNF MSVC and Linux | [Earlier run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684692): `qualia-peer` failed in the same GPU feature gate. | The generic `gpu-runtime` gate in `e19423df` now passes both [MSVC and Linux jobs](https://github.com/mediaprophet/qualiaDB/actions/runs/37194859701). |
| Anatomy asset release | [Run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684676): `qualia-client-core` passes a `qualia_cooperative_core::RecordEnvelope` where its host API expects the distinct `wellfare_core::record::RecordEnvelope`. | Choose one canonical record ABI, or add an explicit conversion at the boundary. Confirm the anatomy producer's ownership after app decoupling. |

The game repository now pins a published QualiaDB commit in `qualia.ref` and
builds its own WASM, `.10d` assets, and HMC pack in its Pages workflow. A later
QualiaDB pin update must pass the game build and browser self-test before the
published game is changed. The game assets and generated runtime remain owned
by this repository.
