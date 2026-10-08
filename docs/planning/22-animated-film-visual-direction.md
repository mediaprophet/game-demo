# Animated-film visual direction

The reference is the **overall finish of DreamWorks' *Antz***, not a request to
make an ant-themed game. The garden ants are optional environmental life in
*Maslows Challenge*. The people, buildings, plants, hills and water carry the
visual identity. Create original designs for this game's SDG setting; use the
film as a quality and visual-language reference, not as a source of characters,
sets or copied frames. This is one selectable aesthetic direction, not a lock
on the final game's look.

## Selectable styles

The in-game System panel offers Storybook, Earthlight and Community
Grounds. Storybook is the current bright palette. Earthlight is an initial
warmer, more restrained study toward the animated-film target. Community
Grounds carries the muted sage, olive, warm tan, terracotta and cream palette
sampled from the civics.au community-ground concept illustration, with the
camp rebuilt as a rounded home-on-wheels pod parked under a tilted solar
shade structure. The choice changes game-owned `.10d` surface readings and
Qualia sky lighting; each style has a separate HMC pack. It is a
presentation preference, independent of saved campaign state. None of the
presets is a finished film look yet. Add future styles through the same style
registry, asset authoring path and in-game selector, with a separate
reviewed HMC per style. Let players preview the same place and camera pose
when comparing them.

## Target on screen

- A coherent, dimensional world with designed silhouettes at strategy distance
  and credible detail in Walk view. Eliminate the floating square tile and
  toy-block reading.
- Distinct, appealing residents and workers with readable faces, hands,
  clothing and motion. Their design should express their role and current
  activity without relying on the HUD.
- Recognisable gum trees, shrubs, herbs and crops with branched structure,
  species-specific leaves, bark, growth states and natural variation.
- Buildings with shaped foundations, doors, windows, eaves, joinery and
  weathering. Avoid one cuboid per building as the final asset.
- Controlled surface response: bark, soil, plaster, timber, cloth, water and
  leaves should react differently to light. Use a consistent colour script for
  daylight, low tide, night and weather.
- Contact shadows, soft daylight, atmospheric depth and balanced exposure that
  preserve readable gameplay silhouettes and selection markers.

## What must change in the game art

1. Establish three approved original reference frames: Kestrel strategy view,
   a garden-level Walk view with a worker, and Saltwind at low tide. Record the
   camera pose, time of day, scene state and asset manifest hash for each.
2. Replace the rectangular earth skirts and four isolated plate edges with
   joined terrain and natural transitions into the creek, canal and distant
   landscape. Keep collision, picking and the Walk height query aligned with
   the rendered mesh.
3. Model one hero worker and one hero building to final silhouette and surface
   quality first. Derive LODs, rig variants and reusable material families from
   that pair before multiplying the catalogue.
4. Replace round-canopy and rectangular-bed stand-ins with botanical asset
   families. Use verified geometry and texture detail that survives both Map
   and Walk distances.
5. Animate workers, vegetation and environmental life with authored clips and
   state transitions. Garden ants can remain a small detail; they should not
   become the visual theme or a substitute for resident character work.
6. Capture and compare the same three frames after every asset and renderer
   pass. Reject a pass if the art improves only in a close crop but the game
   still reads as blocks from its ordinary camera positions.

## QualiaDB gates, shared beyond this game

All runtime rendering uses the full Qualia WASM package. Source art, generated
`.10d` assets and HMC packs belong in `game-demo`. Implement a missing generic
capability in QualiaDB rather than creating a game-only renderer or replacing
the Qualia path. The QualiaDB repository remains read-only for this game task
except for work separately authorised by the owner.

| Need | Generic Qualia acceptance evidence | Existing work order |
| --- | --- | --- |
| Surface materials and texture sampling | `.10d` material slots, HMC texture dependencies and Portal bindings render at least bark, soil, cloth, foliage and water with distinct light response in a non-game fixture and the full game WASM build. | QG-02, QG-12, QG-14 |
| Lighting and depth | Tunable key/fill/ambient light, shadow casting and receiving, exposure and atmospheric depth render without disabling semantic picks or the in-game HUD. | QG-12 |
| Character and foliage motion | Authored mesh/rig clips, clip transitions and instance transforms load through `.10d`/HMC and play deterministically through the public WASM surface. | QG-03, QG-13 |
| Large connected environment | Streaming, LOD and residency hold frame time and memory within measured browser budgets while preserving stable semantic IDs and pick targets. | QG-12, QG-17 |
| Ground camera | Configurable near plane, collision and terrain-aware motion permit close inspection of small props and characters without clipping. | QG-10, QG-11 |

## Review gate

The current 340-variant packs (one per style) and 61/61 campaign checks prove
packaging and gameplay compatibility, **not** film-quality art. Accept the
visual target only
after the three reference frames show original polished characters,
architecture, botany and terrain at actual game size; movement and material
response are visible in the full Qualia WASM renderer; and the campaign,
semantic picks, HMC integrity and browser performance still pass.
