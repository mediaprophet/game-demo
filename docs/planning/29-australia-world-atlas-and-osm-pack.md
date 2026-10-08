# Australia world atlas, OSM foundations, and travel model

## Purpose and current state

Maslows Challenge is Australia-first and needs a navigable world beyond the current single rendered scene. The current scene contains three authored local areas (Kestrel Flats, Saltwind Reach, Northern Highlands). They are not an Australian geographic atlas: they share one local coordinate space, there is no inter-region route graph, and no OSM-derived geography is presently imported. Character place selection is a spawn choice, not a journey simulation.

Build the world as a multi-scale, offline-first model with two connected layers:

1. **Australia atlas:** state/territory, settlement, locality, region, and journey nodes; routes connect places and carry mode, distance, time, cost, access, reliability, and seasonal/service constraints.
2. **Local playable grounds:** streamed or loaded geographic tiles for terrain, roads, water, land use, building footprints, civic places, vegetation context, and authored game assets. One ground may span several engine render tiles; render tiles are a technical LOD and memory boundary, not a political or narrative boundary.

The intended first community-development playtest must use a geographically grounded real Australian locality, with fictional or consented narrative cases layered over it. The existing fictional Kestrel/Saltwind/Highlands ground remains a mechanics fixture and visual-development sandbox; it is not sufficient as the only place users can work in. A real geographic pack can mark attributes as unknown and omit sensitive or private features rather than invent or expose them. Do not teleport a character into a different place while showing the same scenery and implying it is that destination.

## Geographic data roles

Use OpenStreetMap as a geographic foundation where its coverage and feature tags support the game question. It can provide mapped roads and paths, waterways, land-use areas, building footprints, amenities, settlements, and named places. It does not by itself provide a complete elevation model, vegetation inventory, building interiors, condition/access status, or photorealistic building models. Those layers need appropriate separately licensed sources or original authored game content, with their own provenance.

| Layer | Game use | Representation |
|---|---|---|
| Administrative/place hierarchy | Australia atlas and named destinations | State/territory, locality, settlement and region entities with parent/within links |
| Roads, tracks, paths, rail, ferry | Journey options and local navigation | Network edges with mode and access tags; never assume a mapped path is currently safe/open |
| Waterways/coast | Natural boundaries, crossings, local orientation | Lines/polygons with scale-appropriate geometry |
| Land-use and natural areas | Establish ground character and possible stewardship contexts | Generalised polygons, preserve source tag and interpretation separately |
| Buildings and large facilities | Identify halls, depots, markets, stations, civic grounds, and named large structures | Footprint plus source tags and stable source identity; a footprint is not a finished 3D model |
| Elevation/terrain | Hills, valleys, slope, local scene relief | A separately sourced DEM or authored terrain surface, aligned to the same declared CRS |
| Vegetation/ecology | Botany and habitat context | Separate ecological/land-cover source, or clearly labelled authored stylisation |
| Community/game layer | Stories, projects, upgrades, access agreements and gameplay state | QualiaDB entities linked to, but never overwriting, source geography |

Convert large mapped buildings to semantic place/facility anchors and a footprint for spatial fit and orientation. Use reviewed Qualia `.10d` assets for their visible models. Do not extrude every OSM polygon into the same generic block; classify, generalise, and add authored variation by feature type and scale. If no suitable model exists, use an explicit low-detail placeholder with a label rather than present a crude model as a realistic object.

## World graph and travel

The canonical world graph is geographic and semantic, not a flattened list of scene coordinates. Each place has a stable ID, name, type, administrative parent, geographic extent or anchor, local-pack reference, provenance, and availability. Routes are first-class directed or bidirectional edges connecting places. A route records:

- supported modes (walking, wheelchair-accessible walking where evidenced, cycling, private vehicle, coach/bus, rail, ferry, or game-authored local traversal);
- source geometry/reference and calculated distance, while retaining that distance is an estimate at the source's mapping resolution;
- scheduled or estimated duration and transfer/wait time as separate values;
- player-borne cost, shared/group cost, required equipment, carrying capacity, and resource costs where the scenario defines them;
- access restrictions and uncertainty, seasonal or event conditions, service availability, and whether the route is verified, inferred, fictional, or unavailable;
- alternatives and causal effects (arrival date, fatigue, cash, missed work/care, companions, and which projects or services become reachable).

Travel should be an explicit choice among feasible routes, not an instantaneous camera jump. The player can inspect why an option is unavailable or costly, choose a slower/cheaper route, wait for a service, travel with a group, or revise the destination. Record selected and rejected alternatives in the event ledger. Never infer a real person's movement. Character journeys are player-authored scenario actions and stay in the local save unless explicitly exported by the player.

For the first atlas slice, implement a connected Australian network of real destination hubs with source and provenance. At least one real locality must have a locally detailed ground pack before calling the community-development play loop geographically grounded. Do not imply that all Australia is fully playable because the atlas can name a place. Clearly distinguish atlas-only destinations, data-shaped destinations, and fully playable grounds.

## Spatial tiles and large objects

Use a hierarchy: Australia overview → state/territory and corridor view → locality/ground → local render tile → feature/asset. Tile boundaries are derived from a declared spatial scheme and CRS, with stable IDs, bounds, neighbour links, source coverage, and pack/version references. Adjacent render tiles must share edge samples or use a documented transition so DEM/road/water geometry does not crack at seams. Spatial queries should retrieve by bounded extent and LOD; never load the whole Australian dataset into browser memory.

Geographic source tiles are not the same object as game asset tiles. A source tile stores referenced/derived geographic features and provenance. A render tile is a bounded scene chunk. A 10D asset is a reusable visual object. An HMC pack bundles a versioned playable region and its references; it does not erase the independent licence/provenance of embedded source layers.

Feature IDs must retain source dataset and source identity where available. Keep source feature facts separate from interpreted game roles. For example, an OSM `amenity=community_centre` tag is evidence of a mapped tag, not proof the building is open, accessible, available to the player, or governed by a particular committee.

## OSM acquisition, licence, and attribution

Acquire a dated regional extract or bounded source snapshot during pack authoring, not from an unbounded live request during play. Prefer an Australia/state extract for repeatable source acquisition, then transform only the geographic layers and extents needed for a reviewed pack. Store retrieval time, source URL/provider, extract date or replication sequence, checksum, OSM object IDs/versions where available, source tags, transformation code/version, target CRS, generalisation scale, and licence notice in pack provenance.

OSM data is available under the Open Database License (ODbL). The game must display “© OpenStreetMap contributors” with a link to the OSM copyright/licence page wherever OSM geography is presented, and the data pack must preserve the ODbL notice and applicable share-alike obligations for a derivative database. Keep OSM-derived geographic data identifiable and separable from original character/story content, QualiaDB code, game-authored models, and unrelated layers. Before distributing a merged/derived database or HMC containing OSM-derived geometry, review the exact database and produced-work obligations; do not describe the entire game or QualiaDB as ODbL merely because one sourced geographic layer is OSM-derived. Do not use bulk downloads of the standard OSM raster tile service as an extract/import method.

Authoritative references:

- OpenStreetMap copyright and licence: https://www.openstreetmap.org/copyright
- Open Database License 1.0: https://opendatacommons.org/licenses/odbl/1-0/

## QualiaDB dependency and upstream work

All spatial storage, geometry validation/generalisation, coordinate transforms, spatial indexing, terrain construction, `.10d` asset production, and HMC packaging must use QualiaDB ecosystem capabilities. Do not add a game-only OSM parser, mesh pipeline, geospatial database, or geometry fallback. The game repo may contain authored geographic style profiles, pack manifests, fixtures, VibeScript scenarios, and tests that exercise public QualiaDB interfaces. The current OSM adapter parses basic element fields and tags into quins, but does not reconstruct way/relation geometry or render roads/building footprints. The DEM adapter does not yet fetch or pipe a heightfield through to a playable tile. Track those as upstream gaps instead of describing OSM/DEM as already integrated.

Before an OSM data-shaped tile pack can ship, verify that the selected QualiaDB/WASM profile provides:

1. Bounded read-only range queries over a packaged Q42/HMC volume, with spatial-index lookup and paging.
2. Import of OSM PBF or a documented intermediate format through a generic QualiaDB ingestion tool; preserving source tags, object type/ID/version, attribution, and ODbL provenance.
3. CRS transforms, geometry validity/repair reports, polygon clipping/generalisation, route graph construction, and distance/nearest/intersection queries.
4. Seam-safe DEM and line/polygon clipping across render tiles, with explicit units, CRS, vertical datum, and missing-data handling.
5. HMC pack creation and browser loading for geographic data plus linked `.10d` assets, with digest/version verification and bounded memory.
6. Spatial query and scene streaming across low/medium/high LOD, including dateline/antimeridian and state-boundary cases relevant to the Australia extent.

Record any missing generic operation in the QualiaDB upstream capability ledger and test it against more than this game. Until it exists, keep fictional geometry as a labeled mechanics fixture and do not claim the real-place experience or OSM-derived grounds are implemented. Never silently substitute a JavaScript or game-only spatial pipeline.

## Delivery sequence

1. **World schema and route graph:** define world/place/route/tile/feature provenance in Q42/HMC terms; add validation and replay tests for route choice, cost, delays, alternatives, and unavailable routes.
2. **Procedural geographic renderer:** implement the generic QualiaDB vector-to-scene path and author original, switchable game render profiles over stable geographic source facts.
3. **First real local pack:** acquire a dated, licensed snapshot for a selected Australian locality; preserve roads, paths, water, land-use/buildings and provenance; combine a separately sourced elevation model; test geometry and spatial queries.
4. **Playable travel and local action loop:** add atlas/map HUD, route choices with time and cost, persistent arrivals, and local projects anchored to real mapped facilities/grounds without asserting unverified access or conditions.
5. **Large-object and botany pass:** map important facilities to reviewed `.10d` models; layer local trees/shrubs/herbs from authored regional assets or separately licensed ecological evidence.
6. **Expand Australian destinations:** add multiple local grounds, transport options, travel constraints, and route alternatives; retain offline snapshots and deterministic replay.

## Acceptance criteria

- The atlas makes it possible to choose among multiple Australian destinations, inspect route modes, constraints, time and costs, and travel without unexplained teleportation.
- The first geographically grounded community-development play loop runs in a real Australian locality; at least two separately packaged local grounds have distinct terrain and visible feature sets before any UI calls them fully playable.
- Local terrain/map rendering loads only the selected ground and nearby tiles, with stable seams and bounded memory.
- Large OSM features remain linked to their source identity/provenance and use appropriate authored visual assets; mapped tags are not misrepresented as verified operating/access facts.
- Every geographic layer has source, date/version, licence, CRS, transformations, uncertainty/coverage, and attribution.
- OSM attribution and ODbL notice are visible in-game, and distribution of derived geographic databases has passed an explicit licence review.
- Travel events record route alternatives, selected mode, time, economic/resource costs, and arrival state so consequences are measurable and replayable.
- The world can add Australian local packs without changing game-core character, project, or community semantics.
