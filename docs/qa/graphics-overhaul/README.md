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

## Single-asset viewer browser qualification

This is evidence for the standalone asset viewer, not a new full-game art or
performance baseline. The existing WASM package was reused; no game build,
asset regeneration, engine edit, or dependency installation was performed.

- Latest result: [asset-review-1791861101828/result.json](asset-review-1791861101828/result.json).
- Rendered sample: [asset-review-1791861101828/banksia.png](asset-review-1791861101828/banksia.png), showing one banksia branch component, not a complete plant.
- Final capture: [asset-review-1791861101828/final.png](asset-review-1791861101828/final.png).
- Vibe-authored human note: [asset-review-1791861101828/review-note.json](asset-review-1791861101828/review-note.json), disposition `not-reviewed`.
- Runtime: installed Chrome 154.0.8037.98, headed, local origin on port 8090.
  Engine receipt reports WebGPU, upload success and a presented frame. The
  screenshot was inspected; this is not approval of the asset's appearance.
- Seven functional assertions passed: no startup model/HMC fetch; one selected
  asset decodes with authored RDF clearly labelled; metadata layout remains
  within the viewport; camera input passes through Vibe; switching requests
  only the two selected model paths and no HMC; note identity/disposition are
  preserved; corrupted response bytes are refused before render.
- Textures are the bounded shared manifest set, not an exact per-model subset.
  RDF is a Vibe source preview, not Q42-decoded evidence. Browser console
  records include a missing-resource 404; no uncaught page exception was
  recorded in the passing result.

Earlier attempts remain retained: `asset-review-1791860120776`,
`asset-review-1791860227754`, `asset-review-1791860329116`, and
`asset-review-1791860565684` record startup failures. Corrections renamed an
identifier rejected by the loaded parser and made loop bounds explicit. A
small helper budget had clamped the caller's shared budget; using an existing
typed-list bound removed that unintended clamp without changing the engine's
overall limits. `asset-review-1791860812420` passed the six original assertions
but its screenshot exposed metadata overflow; the latest result adds a layout
assertion after the CSS correction.

The updated server and headed review browser were left running for manual
UAT at `http://127.0.0.1:8090/asset-review.html`. The pre-existing port-8089
server was left untouched and does not serve the new asset-file routes.
