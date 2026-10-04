# Maslows Challenge: Asset Production Catalog

Status: production backlog and first source batch, 2026-10-03. Follow the
[RTS uplift blueprint](17-rts-aaa-uplift-blueprint.md) and the
[QualiaDB-only contract](13-qualiadb-only-development-contract.md). This
catalog is game content, not a QualiaDB engine module or a licence change.

## Art direction: original animated storybook world

Kestrel Flats should feel warm, optimistic and inhabited at strategy zoom:
rounded silhouettes, readable colour families, expressive residents, lively
market and garden props, and a miniature landscape with clear project
landmarks. Use original character and environment designs. The current game
content is a geometry and palette exploration, not finished character art.
Each authored asset retains a stable semantic ID and game-owned source and
licence provenance; Qualia builds the geometry, seals `.10d`, and renders the
scene. Finished soft materials, sky, daylight, shadows, character rigs and
animation require the generic Qualia renderer and format gates in work order
QG-12/13/14. Do not approximate those engine features with page overlays.

## What exists today

`crates/rolling-commons-shell/src/asset_catalog.rs` now produces 93
always-present original Kestrel Flats scene organs, with additional solar and
garden variants. They cover terrain colour zones, paths, detailed buildings,
market stock, garden soil and crops, a rounded tank, trees, lamps, salvage
drums, vehicle detail, residents, flowers, background forms and infrastructure.
The shell uses QualiaDB's bounded
primitive assembler and its full computational-geometry authoring APIs:
deterministic cylinders, spheres, tori, transforms and parametric profile
revolution. Every organ is sealed through the Qualia `.10d` compiler with a
game-owned source recipe and provenance sidecar, then rendered by full
QualiaPortal. Stable `rc:asset/` IDs remain available. These are **detailed
blockouts for composition and interaction testing**, not finished AAA art.
The current game export now emits loose `.10d` snapshots and a provisional
Qualia core QBDL/HMC pack containing those intact assets and a JSON scene
manifest. It verifies the pack with Qualia's reader. QG-01/02/14 remain the
production gate: the canonical HMC variant, Q42 identity/rights manifest,
browser pack loading, and complete state/animation packaging are not yet proven.

Saltwind Reach adds 26 always-present organs on a second connected land tile:
canal, bridge, roads, barn, market, quay, boat, wind pump and orchard. Bridge,
pump and fruit variants follow the SHACL-validated project states. The full
map is 119 persistent `.10d` organs before conditional variants. Qualia's
public camera target and sky preset APIs provide territory navigation and
daylight presentation in the full WASM game.

The current asset export contains 158 distinct `.10d` variants across eight
scene states in the game-owned HMC pack. A focused building pass separates the
workshop, hall and barn roofs from their walls, adds facade and eave detail,
and makes the roof colours readable at strategy zoom. The Qualia camera now
frames selected sites closer. These improvements were inspected in the full
WebGPU game, and still need finished materials and richer authored models.

## Asset families and build order

The counts below are **planning estimates** for a substantial first RTS
territory, not claims that assets exist or approved scope for v0.1. Count a
family as complete only when its variants, semantic link, LODs, validation,
browser view, and rights record pass.

| Priority | Family | Target distinct sources | Required variants and animation | First production sample |
|---|---|---:|---|---|
| P0 | Terrain and ground | 12-20 | height/biome/ground wear, cliffs, water edge, road transition, LOD | One 256 m sector with two elevations and build zones |
| P0 | Roads, paths, utility links | 20-35 | straight, bend, junction, bridge, damaged, under construction, power/water/data overlays | Road crossing plus powered workshop link |
| P0 | Buildings and facilities | 20-30 families | foundation, scaffold, operating, unpowered, fault, repair, upgraded, LOD | Workshop, market, hall, battery, tank, depot |
| P0 | Workers and civilians | 10-16 character sources | original silhouettes, skin/clothing variety, idle, move, carry, work, build, repair, interact, reduced motion | One complete worker rig and seven actions |
| P0 | Vehicles and equipment | 12-20 | stopped, travelling, loaded, damaged, repaired, lights, LOD | Repair van and cargo cart |
| P0 | Resources and props | 60-100 | stockpile amounts, condition, carried form, salvage/reuse form | Solar panel, battery, water drum, tool kit, timber |
| P0 | UI and world icons | 100-160 | selection, command, resource, alert, state, overlay, high contrast | Core RTS command set plus power flow |
| P0 | Audio | 80-140 events | select/order/deny/complete, construction, machinery, weather, ambience, music states, captions | Workshop/camp/market sound set |
| P1 | Vegetation and ecology | 25-40 | age/season/water stress, clustering, LOD | Three original local plant families |
| P1 | Effects and weather | 20-35 | dust, rain, power flow, fault, construction, smoke/steam where relevant, reduced motion | Power on/off and build completion |
| P1 | Portraits and story art | 20-40 | role/expression/accessibility, localisation-safe text separation | Three Kestrel Flats characters |
| P1 | Cinematics and campaign maps | 8-15 scenes | opening, chapter transitions, map/territory changes, subtitles | Short first-territory introduction |

Do not mass-produce low-quality mesh variations just to reach these counts.
First complete one **building**, one **character**, one **terrain sector**, one
**vehicle**, one **resource**, and one **UI/audio command set** to final
quality. Measure time, memory, file size, browser appearance, and iteration
cost. Then use those templates to scale production.

## First asset batch: Kestrel Flats sector

The blockout IDs give a concrete replacement queue. `P0-finish` means the
asset must be rebuilt as a finished source and packed before calling the
sector visually complete. `P0-system` means the visible variant is tied to
authoritative Q42 state.

| Existing source ID | Replacement goal | State/semantic requirement |
|---|---|---|
| `rc:asset/ground`, `rc:asset/main-road` | Sculpted local terrain, road material, edges, markings, drainage | buildability, passability, survey, route cost |
| `rc:asset/camp-shelter`, `rc:asset/camp-platform` | Original tent/swag and modular platform with interior-readable silhouette | shelter condition and upgrade stage |
| `rc:asset/workshop`, `rc:asset/workshop-doors` | Distinctive workshop exterior/interior glimpse, signage, tools, bay | open/closed, power, occupancy, fault, project stage |
| `rc:asset/solar-array`, `rc:asset/energy-battery`, `rc:asset/utility-cable` | Installable panels, battery enclosure, routed connection, readable flow effect | count, charge, grid connection, fault, capacity |
| `rc:asset/market-stall` | Trading post building and stock display | opening, listing, inventory, fulfilment |
| `rc:asset/salvage-pile`, `rc:asset/salvage-rack` | Recognisable reusable materials and storage fixtures | stock/condition and collectable identity |
| `rc:asset/committee-sign` | Original community noticeboard, not generic authority icon | pending/approved/review state |
| `rc:asset/water-tank`, `rc:asset/tank-stand` | Tank, pipes, fill indication, safe platform silhouette | water amount, access, maintenance |
| `rc:asset/garden-beds`, `rc:asset/garden-fence` | Modular garden beds, crops, tools, paths | planted/growing/harvest/maintenance |
| `rc:asset/repair-van`, `rc:asset/van-windows` | Original service van with wheels, cabin, load bay, damage/repair variants | move, carry, repair, route, condition |
| `rc:asset/community-hall`, `rc:asset/hall-entry` | Social focal point with readable access and activity | occupancy, agreement, event, upkeep |
| `rc:asset/communications-mast`, `rc:asset/mast-node` | Distinct comms equipment and status light | power, connectivity, fault |

Add the missing high-priority sources: 1 complete worker; depot; cargo cart;
tool bench; solar panel item; battery item; water drum; timber/metal stock;
road junction/bridge; three local plant species; terrain rock/soil set;
power-line pole; construction scaffold; UI selection rings; command icons;
alert sounds; map ambience. The sector should show people working and
materials moving, not just more buildings.

## Per-asset source contract

Every authored asset gets a record with:

1. stable game ID and intended Q42 entity/type binding;
2. original source/recipe path, author, revision, licence and provenance;
3. dimensions, units, pivot/origin, forward axis, placement footprint,
   collision/nav contribution, selection bounds;
4. mesh/material/texture/rig/animation sources and named state variants;
5. geometry and texture budgets for close, mid, and strategic LOD;
6. `.10d` compiler/format version, content digest, HMC entry and manifest
   reference, and any HCF descriptive content;
7. visual review at close/strategic zoom, accessibility and reduced-motion
   check, and test receipt on the selected full Qualia WASM browser profile.

Keep source content editable and **separate from derived `.10d`/HMC output**.
Generated assets are never authoritative merely because an LLM produced them.
Qualia validation catches topology, bounds, units, corrupt sections, semantic
binding, incompatible versions, and missing rights evidence. Human review
judges style, readability, animation, cultural context, and gameplay value.

## Production pipeline and upstream gates

1. Write a short asset brief, concept sheet, silhouette and reference board
   with original or rights-cleared inputs. Approve the family style before
   making its variants.
2. Author geometry, materials, rig, and motion through Qualia-owned creation
   and import tools. Extend Qualia tooling where source authoring or fidelity
   is inadequate. Preserve source parameters and provenance.
3. Compile and validate `.10d`; bind stable Q42 IDs, spatial bounds, states,
   and pick targets. Animation waits for QG-03/QG-13.
4. Build a versioned HMC pack with manifests, digests, licences, and dependency
   checks through the canonical Qualia producer/reader (QG-01/QG-14).
5. Inspect in the actual full Qualia WASM game at close and strategic zoom.
   Verify state changes, command selection, LOD, frame time, memory, and
   offline reload. Reject assets that only look right in an isolated preview.
6. Promote the pack, pin the Qualia revision and asset digests, and record the
   content receipt. Rebuild dependent variants when the source changes.

No separate game-side 3D engine, geometry compiler, HMC imitation, or WebCivics
profile is part of this pipeline. A missing feature becomes a QualiaDB
upstream task and resumes here when tested. The existing blockout catalog can
remain useful for rapid composition and as a baseline test while finished
assets replace it family by family.
