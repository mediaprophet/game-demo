# Graphics overhaul evidence

Evidence for [the production plan](../../planning/35-community-grounds-graphics-overhaul.md) belongs here as an index with links to CI runs, deterministic capture metadata, review frames and performance reports.

Baseline captured: **no**. A real browser/GPU run must record the actual browser, adapter/backend, game commit, pinned Qualia commit, WASM hash, HMC hash, art style, fixture, camera, viewport and quality tier. Do not treat manifest hashes or a CI compile as pixel execution.

| Evidence | Status | Details |
|---|---|---|
| Supplied visual reference | retained | ../../reference/communitygrounds.jpg |
| Versioned review camera setup | ready for capture | ../../../web/fixtures/graphics/review-cameras.json; six named views plus a validated 30-second read-only showcase route; `?graphics-view=<view-id>` uses the seed world and disables save/load/persistence |
| Guided in-game showcase | implemented; visual run pending | Optional route visits the hub, workshop, garden, touring rig, bridge and highlands, pauses campaign controls, and restores the prior view on completion or cancel. No browser execution or rendered capture has been claimed. |
| Direct Drive instruments and mirrors | implemented; visual run pending | web/game.html shows simulated oil and water/coolant temperatures with risk colours and rising/cooling trends, ambient temperature, grade, surface, load and engine health. Separate overheat and road-hazard alerts keep engine risk visible. The Qualia HUD shows following traffic and trailer occlusion, with a bend reveal. No browser execution or rendered capture has been claimed. |
| Release WASM and style packs | built and structurally validated | 2026-10-11; game HEAD `9c52d96`; Qualia HEAD `9463dc6` with local changes; WASM SHA-256 `3f0afa620894f8e8222094487af989cf803b936f09c61ca716487c3d2216d389`; Storybook, Earthlight and Community Grounds each contain 578 assets and 40 scene states, including rounded touring-rig bodies and the resident model pass. Exporter validation is not rendered-browser evidence. |
| Game baseline captures | pending | Record the six reference cameras and current campaign states |
| Browser WebGPU material fixture | pending | Demonstrate actual per-map binding and colour-space behaviour |
| Moving temporal fixture | pending | Record real producers and history execution |
| Water shoreline / foliage motion | pending | Run through the shipped browser game path |
| Resident rig/clip | pending | Validate skinning, contact, attachment and transitions |
| Performance and recovery | pending | Named adapters, cold/cached traversal, repeatable route and resource totals |
