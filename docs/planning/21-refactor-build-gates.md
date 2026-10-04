# QualiaDB refactor build gates for Maslows Challenge

Checked against QualiaDB `1b2c944f08ab0e73ddac257c420dd2ab79825836`
(`0.0.40.7`, tag `v0.0.40.12`) on 4 October 2026. The game uses the
**full QualiaDB `wasm-full` engine**, including QualiaPortal rendering. These
are generic upstream build issues; changing the game to the WebCivics profile
would remove required engine capabilities.

| Gate | Evidence | Required QualiaDB work |
| --- | --- | --- |
| WASM Release and Qualia Pages | [WASM run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684727), [Pages run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684694): `webizen-lite-wasm` fails when `gpu_context.rs` compiles `wgpu` code with `gpu-runtime` disabled. | Gate shared GPU initialization by `gpu-runtime`. The verified fix is local QualiaDB commit `e19423df`; it is not on the remote branch or release tag. Rerun both workflows after an authorised upstream publication. |
| QDNF MSVC and Linux | [Run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684692): `qualia-peer` fails in the same GPU feature gate. | Use the same generic `gpu-runtime` gate; keep the non-GPU CPU path compilable. |
| Anatomy asset release | [Run](https://github.com/mediaprophet/qualiaDB/actions/runs/37190684676): `qualia-client-core` passes a `qualia_cooperative_core::RecordEnvelope` where its host API expects the distinct `wellfare_core::record::RecordEnvelope`. | Choose one canonical record ABI, or add an explicit conversion at the boundary. Confirm the anatomy producer's ownership after app decoupling. |

The game repository now pins a published QualiaDB commit in `qualia.ref` and
builds its own WASM, `.10d` assets, and HMC pack in its Pages workflow. A later
QualiaDB pin update must pass the game build and browser self-test before the
published game is changed. The game assets and generated runtime remain owned
by this repository.
