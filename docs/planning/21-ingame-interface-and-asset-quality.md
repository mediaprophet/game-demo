# In-game interface and asset quality

## Architecture

The visible player interface lives inside the world viewport. The browser hosts two canvases:
the Qualia Portal WebGPU scene and a transparent Qualia HUD canvas above it. QualiaDB's generic
`render::hud::QualiaHud` owns menu painting, hit testing, and keyboard focus. The game supplies
declarative menu text and action IDs; `GameSession` alone decides whether orders are legal and
records them in the Qualia world. Save, load, replay, and VibeScript continue to use Qualia
capabilities. The surrounding HTML is a host for canvases and an invisible legacy data view;
it is not the visible player interface.

The HUD renderer is generic upstream work, not a game-specific rendering fork. Its bounded
document accepts panels, labels, buttons, and meters in canvas coordinates, plus world-space
markers projected with the same Qualia camera matrix as the Portal scene. It deliberately
does not implement campaign rules. Future users of Qualia Portal can supply their own document.

## Site navigation

The game scene receipt supplies each semantic site's normalized world position in the same
order as the pickable Qualia tensor nodes. Game data assigns each site a readable name,
category color, and icon. Qualia HUD projects and hit-tests the markers. Selecting a marker
opens the site inspector and eases the Portal camera toward that site. The in-game `+`, `−`,
and map-view buttons control scale and return to the current territory overview. Orbit,
pan, wheel zoom, and arrow key pan remain available. Camera motion is a presentation concern;
it does not change world state or save data.

Generic upstream follow-up: HUD markers currently have no scene depth test or occlusion
policy. Qualia should expose a reusable projected-annotation policy (always visible,
occluded, or edge-clamped) before adding dense worlds. It should also offer a generic
camera transition API so applications can request target/zoom easing without their own
frame interpolation. Keep both in Qualia Portal/HUD rather than a game-only renderer.

The game maps each semantic marker to its available project orders. Selecting
a marker focuses the camera, describes the site's current condition, and shows
relevant actions in the Qualia HUD. The field dispatch can locate the site for
its suggested next order. These presentation mappings use the existing Qualia
`GameSession`; blocked orders still return Qualia's rule explanation and
accepted orders enter the same replay tape. Both desktop and compact HUD
layouts were checked in the browser.
Accepted and blocked orders now focus their affected site so model changes
and rule explanations are visible where the player acted.

Either campaign ending now opens a Qualia HUD result panel over the map. It
reports the chosen outcome and campaign totals, offers replay verification,
and lets the player explore or begin again. A completed trade-route playthrough
was rendered in the browser; its replay matched the world byte for byte, and
the new-campaign control reset the world and camera to Kestrel Flats.

## Current art direction

Aim for a warm, readable, stylized strategy world with distinct silhouettes and rich colour.
Keep game-owned source recipes separate from the engine. Continue to compile meshes through
Qualia computational geometry and seal them as `.10d`; use the full Qualia WASM renderer. The
resident upgrade uses a revolved tapered coat and separate limbs, facial marks, and hair instead
of three sphere or box parts. The workshop annex and chimney, hall bell tower, repaired bridge
planks, pump blades, and boat rail give major sites clearer silhouettes. This is an asset
iteration, not a claim of finished production art. The game now emits Qualia's
per-vertex `.10d` surface reading (`SRD1`) for deterministic colour variation,
preserving the public compiler and provenance path.

The next building pass splits the workshop, community hall and Saltwind barn
into distinct wall and gabled-roof assets and adds workshop facade and barn
eave details. The selected-site camera frames the buildings more closely.
All five new asset variants were built by Qualia and packaged into the
game-owned HMC; the scene remains stylized blockout art.

## Quality work still needed

1. Inspect both rendered territories after the WebGPU depth and occlusion fix. Tune camera,
   lighting, terrain scale, colour separation, and shadow response from actual screenshots.
2. Replace workshop, shelter, market, hall, bridge, pump, boat, and resident blockouts with
   authored, coherent multi-part models. Preserve stable semantic asset IDs where practical.
3. Add production character rigging, animation clips, transitions, and visual feedback through
   Qualia's animation and `.10d` path. Generic missing capabilities belong in QualiaDB.
4. Add terrain variation, paths, vegetation sets, prop families, construction states, and
   readable visual effects. Profile draw time and asset load time with the full Qualia package.
5. Extend the generic HUD with text input and accessibility semantics before moving the
   VibeScript authoring workbench into the in-game canvas. Do not build a second page UI engine.
6. Replace the invisible legacy DOM data view with direct HUD presentation data once the
   canvas migration is proven. Remove old CSS and markup at that point.

## Generic Qualia render gap found in the browser

`upload_tensor_buffer` provides the ten semantic pick targets used by the game, but it also
enables visible tensor projectors. The projector pass still draws bright square markers after
`set_ambient_enabled(false)`, because that switch only controls the particle field. Add a
generic, independent Portal projector-visibility setting (or a general render-layer mask) while
preserving the tensor buffer and GPU pick readback. Verify with a non-game fixture that hidden
projectors do not alter picks, and with a game screenshot that no markers cover the landscape.
This belongs upstream; removing the tensor upload would sacrifice Qualia picking.

## Acceptance checks

- World visible under WebGPU; two distinct territories can be orbited and picked.
- Every visible player control is inside the Qualia HUD canvas.
- HUD actions result in the same SHACL-validated world and byte-for-byte replay.
- Save/load survives a reload; tide, bridge, and both endings remain reachable.
- No WebCivics branding or licence changes appear in the full Qualia build.
