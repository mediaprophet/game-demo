# QualiaDB tensor marker rendering

## Decision

Gameplay location markers are the colored, categorized icons authored by the game HUD. QualiaDB's tensor nodes remain uploaded as semantic pick data, but their debug sprite projection must be hidden in the game.

## Generic upstream capability

`QualiaPortal::set_tensor_projection_enabled(bool)` controls tensor-node drawing independently from tensor residency and picking. The default remains enabled so existing QualiaDB applications keep their current presentation. The control applies to both WebGPU and canvas fallback rendering, and it is retained when the portal initializes its GPU renderer.

The game calls the control with `false` after uploading the site tensor. Its existing category markers remain visible in overview, survey, map, and first-person camera modes.

## Upstream integration gate

- Carry the renderer control into a QualiaDB commit and publish that revision.
- Update `qualia.ref` in this game repo to the published revision, then rebuild the WASM package with the standard `wasm-pack` release command.
- Verify on both WebGPU and canvas fallback that tensor sprites are absent while site selection still resolves all 11 semantic locations.
