# Community Grounds art bible

See [the production plan](../planning/35-community-grounds-graphics-overhaul.md) for the detailed family specifications and [the supplied reference](../reference/communitygrounds.jpg). This document stores production decisions and approved sample evidence.

## Identity and visual language

- Subject: a welcoming, practical regional Australian community ground with homes, workshop, shared green, food growing, water and renewable energy.
- Look: illustrated realism. Ordinary human proportions, purposeful everyday objects, softened but credible construction detail, warm daylight, mature local vegetation and layered rural distance.
- Reference hierarchy: overall view and activity composition; central shed and solar canopy; mobile dwellings; communal work/growing; mature gums; grass/path colour variation; soft daylight and hills.
- Do not make the game ant-themed, a toy miniature, a technical CAD rendering or a photorealistic catalog scene. Avoid thick outlines and excessive saturation.
- Keep real-world labels, text and narrative UI separate from meshes so language is legible and localisable.
- Gameplay actions, resource counts and rules remain authoritative in the existing Qualia world.

## Scale and composition

- World units are source-world metres. Calibrate to a resident near 1.7 m and an ordinary door near 2 m; verify through the current scene transform before changing dimensions.
- Kestrel comprises five connected zones: arrival/mobile homes; solar workshop and learning hub; raised gardens/shared water; open social green; service/storage.
- Preserve clear routes, building entrances, interaction pads and selected-site sightlines at Map, Survey and Walk camera scales.
- Use mature gum forms to frame edges and distant transitions. Keep central hub, tank, shelter, mast and canal legible.
- Each hero model must read in front, side, three-quarter and overhead/ordinary gameplay views.

## Palette starting points

Tune these against the supplied image in matched captures; these values are only starting swatches.

| Surface | sRGB start |
|---|---|
| Main foliage | #657448 |
| Dry grass | #A4A36A |
| Pale eucalyptus bark | #C4BE9D |
| Earth / gravel path | #B4976F |
| Cream vehicle / painted metal | #E3DFC7 |
| Solar cell | #314C67 |
| Galvanised frame | #87918B |
| Structural dark | #46504B |
| Warm timber | #94704A |
| Skin and clothing | authored per character; never derive from the global foliage tint |

The palette must retain clear separation between foliage, soil, timber, painted metal, skin, fabric, water and dark rubber. Avoid applying a single style tint to all assets.

## Materials

Make authored material parameters explicit: colour space, metallic/roughness response, supported map role, alpha mode/cutoff, UV scale, mip policy and fallback. Metals alone receive high metallic response. Use geometry for silhouette and large folds; reserve normal/detail maps for bark, shallow panel-cell relief, cloth and ground microstructure. Review under neutral daylight and shaded workshop fill.

Priority production set: painted cream enamel; galvanised steel; dark rubber; clear and tinted glass; solar cell/glass; timber; pale gum bark; soil; gravel; wet/dry water; leaf; grass; cloth; plaster. Reuse engine-supported material and texture identities. Do not invent a game-only material container.

## Hero standards

1. Workshop: open bays with visible roof thickness/framing, panel rails/cells, sensible posts, doors, benches/shelves/tools, battery/control area, back and side construction.
2. Home on wheels: coherent chassis, tyres, wheel arches, framed glazing, visible entry, roof load, awning, steps, hitch and small domestic details.
3. Resident: adult body proportions, face/hair silhouette, layered clothing, readable hands and footwear; believable task poses and ground contact.
4. Gum tree: tapered branched trunk, irregular crown and hanging leaf clusters; three distinct canopy silhouettes.
5. Garden: timber raised beds, soil, individual leaf groups, varied planting, irrigation and adjacent crates.

Complete these together in one scene before scaling the method to the roster and the second territory.

## Animation and soundness

Only engine-supported clips and bounded motion are used. Character pose must put hands on tools/work surfaces and feet on the terrain. A renderer effect never grants game resources. Cosmetic time does not advance campaign day, alter the tide rule or change replay history. Trees vary by species and stiffness; avoid uniform synchronized wind.

Reduced-motion and quality preferences can turn down secondary movement and particles without hiding a valid world state or interaction.

## Evidence

There are no approved production samples at baseline. Link the first accepted workshop, resident, gum, vehicle and garden review images here. A source mesh or successful pack export does not count as visual approval.
