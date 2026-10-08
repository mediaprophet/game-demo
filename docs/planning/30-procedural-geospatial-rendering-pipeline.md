# Procedural geospatial rendering pipeline

## Goal and boundary

Render recognizable Australian places from source geography while keeping the geographic facts stable across the game's selectable visual styles. The render pipeline changes the presentation of mapped features; it must not invent roads, waterways, parcels, building use, ownership, public access, or local conditions. A fictional story can use a real geographic base, but story events and game-built changes remain a separate overlay.

All GIS parsing, coordinate transforms, geometry processing, spatial indexing, terrain construction, mesh generation, `.10d` creation, and HMC packing must run through generic QualiaDB capabilities. Game content may provide declarative feature-to-style profiles and authored asset libraries. No JavaScript geometry implementation, external rendering engine, or game-only OSM parser.

## Inputs

1. **Vector geography:** dated/licensed OSM or other appropriate vector snapshot. Preserve source object type/ID/version, tags, relation membership, coordinates/geometry, retrieval date, source digest, CRS, and licence.
2. **Elevation:** a separate DEM/DSM grid for landform height. Record cell size, vertical datum, capture/processing method, valid-data mask, source/licence, and uncertainty. For Australia, Geoscience Australia's elevation catalogue points users to ELVIS and includes products with different coverages and resolutions; select the actual product for the chosen locality and inspect its terms instead of assuming one national resolution/licence fits every tile.
3. **Optional layers:** ecology/vegetation, official administrative boundaries, transit services, aerial imagery used only where its licence permits, and public aggregate indicators. Each layer keeps its own provenance and never silently inherits OSM's licence.
4. **Game art profile:** versioned declarative rules that bind semantic feature classes and scale ranges to QualiaDB geometry/material/asset recipes. The art profile cannot change source coordinates, feature identity, or confidence.
5. **Game overlay:** fictional or consented scenario entities such as projects, temporary works, characters, local needs, and proposed structures. These are separate graph/asset entities linked to geography, never edits to source features.

## Pipeline

```text
1. Discover + snapshot sources
   -> immutable source files, licences, dates, checksums, CRS, source manifest
2. Parse + preserve source geometry
   -> nodes / ways / multipolygons / relations, IDs, tags, topology and provenance
3. Validate + diagnose
   -> coordinate ranges, missing way nodes, ring closure, self-intersections,
      invalid holes, duplicate/crossing roads, bounds, feature confidence
4. Transform + localise
   -> geodetic coordinates -> declared projected metric CRS -> tile-local ENU
5. Build semantic world graph
   -> places, buildings, road/path network, water, land-use, administrative links
6. Derive terrain + align vectors
   -> DEM no-data handling, surface mesh, contours/slope, riverside/road alignment
7. Generalise + tile
   -> extent query, simplification by screen scale, clipping, seam stitching,
      stable IDs, neighbour edges, LOD bands
8. Apply declarative render profile
   -> feature class + level of detail + uncertainty -> geometry/asset/material recipe
9. Assemble scene content
   -> validated geometry -> .10d meshes + Q42 identity/provenance -> HMC pack
10. Browser load + deterministic replay
   -> bounded nearby tiles, styled scene, semantic picking, map/game overlay,
      versioned offline travel/event state
```

Each stage emits a receipt: input digest, capability and format version, parameters, warnings, output digest, and provenance links. Failed geometry remains inspectable and is not silently repaired. A selected repair produces a new derived feature with a relationship to its original.

## Semantic treatment and stylization

### Elevation, hills and valleys

Interpolate the sourced heightfield with QualiaDB's DEM terrain functions. Derive slope, aspect, contours, drainage basins and line/terrain intersections through its geospatial and computational-geometry APIs. Stylization can exaggerate relief by an explicitly declared factor for readability, but keep a source-height query so the rendered exaggeration cannot be mistaken for real elevation. Water flow or flood claims require a hydrology model and appropriate evidence; a blue line drawn on a DEM is not a flood simulation.

### Rivers, creeks and water bodies

Preserve line/polygon geometry and source waterway/natural tags. Use width/area where sourced or infer a render width only as a tagged visual estimate. Render with curved ribbons/banks, shallow edge gradients, vegetation overlays, bridges/culverts where mapped, and authored details. Do not route paths through water simply because a 3D river mesh is decorative. Do not label seasonal/perennial state unless the source supports it.

### Roads, tracks and paths

Build a connected routable graph from way geometry and access/mode tags. Keep traversal constraints distinct from visual width. For the local scene, make a styled road ribbon with shoulders/verges, authored edge wear and markings; derive these from a seed plus stable source-feature ID so re-bakes are deterministic. Lower LOD merges markings and minor edges, but route graph IDs and connectivity remain unchanged. A mapped road is not proof it is open, maintained, safe, or publicly accessible now.

### Buildings and large objects

Reconstruct building footprints and multipolygons with holes. Use source height/levels/material tags if present, with a confidence/provenance label. For untagged heights, use a declared local typology estimate and visibly lower confidence. Generic extrusions are acceptable only for distant low-detail LOD; named or scenario-important buildings use curated Qualia `.10d` assets fitted/oriented to footprints, with original game artistry and no false claim of architectural accuracy. Retain public-facility tags and source IDs for map interaction and community-development mechanics.

### Land use, vegetation and botany

Use polygons and separately sourced ecological layers as placement constraints and context. Generate instance positions deterministically inside valid areas, respecting roads, buildings, water, steep slopes, and explicit exclusion zones. Select gumtrees, shrubs, herbs, crops, and garden vegetation through Australian biome/season profiles rather than scattering generic green props. OSM landuse and natural tags describe mapped categories; they do not establish species or ecological condition.

## Declarative game render profile

A profile contains no executable JS or user-supplied shader. It identifies:

- profile ID/version, visual style name, palette/material family and lighting preset;
- semantic feature rules with a closed QualiaDB-supported operation list;
- min/max map scale or projected pixel size for each rule;
- geometry treatment (ribbon, surface patch, footprint asset, marker, or exclusion);
- widths/heights/relief multipliers tagged as sourced, authored, or estimated;
- stable seeded variation ranges and style asset references;
- fallbacks, uncertainty treatment, accessibility contrast, performance/vertex budget;
- source-neutrality assertion: style selection cannot alter IDs, coordinates, tags, route graph, or gameplay facts.

The user can select a storybook/animated-film look, field-atlas look, or naturalistic look. All styles render the same underlying geography and gameplay semantics. An atlas palette may prioritize contour/water/route readability; a cinematic profile may add soft banks, layered canopy, weathered materials, and atmospheric distance. Film-inspired language is a quality target, not imitation of a particular studio's protected characters or art assets.

## World map and journey integration

Use the same semantic route graph at Australian atlas, corridor, locality and ground scales. An overview route is a generalized rendering of local geometry, not a separately invented connection. Selecting a mapped place focuses its local pack; travel picks among mode-specific route options with cost/time/access constraints, then loads the distinct destination pack when available. Atlas-only places remain route destinations without fabricated local 3D scenes. Game scenarios attach project/community overlays to source-linked public features while preserving unknown and contested fields.

The render scene and route planner consume one versioned world-pack reference. Never use hand-authored scene positions to override a geographic route when a sourced graph is available. Gameplay changes remain replayable and separate from immutable source geography.

## QualiaDB capability status and required generic work

Repository inspection shows useful foundations: `OsmAdapter` builds a consent-gated Overpass request and parses basic element fields/tags into Q42 quins; DEM terrain functions construct meshes and geodetically anchored tiles; computational geometry exposes polygon validation, triangulation, topology, simplification and mesh operations; `.10d` compilation can include provenance sidecars. These are ingredients, not an end-to-end map pipeline.

Specific upstream gaps found in the current source:

- OSM adapter is documented as a stub: way/relation node references and geometry are not reconstructed into usable lines/polygons; `fetch_region` executes a request but drops the body and does not create a geographic pack.
- OSM query's date formatting is stubbed and bbox-only feature selection omits dependencies needed to reconstruct ways/relations.
- DEM adapter executes/status-checks a request but does not decode a height raster and feed it into a geodetic `.10d` tile.
- Generic tile clipping/seam stitching for vectors against DEM/render tiles, scale-based style evaluation, vector-to-asset conversion, and HMC/browser streaming need end-to-end proofs.
- Licences and source provenance must survive feature transformation and mixed-source packaging, with map attribution metadata accessible to the UI.

The game's profile fixture and acceptance tests belong in game-demo; ingestion, geometry-to-mesh transforms, style evaluation engine, format handling, and tile streaming upgrades belong generically in QualiaDB. Keep QualiaDB read-only for this task; the missing items are tracked as QG-20/QG-21 for its development agent.

## Test gates

1. **Source preservation:** round-trip OSM nodes, ways, multipolygon relations, tags, stable IDs and provenance. Rebuilding from the same source/config produces identical derived IDs and digests.
2. **Geometry:** invalid rings, holes, self-crossings, missing way nodes, clipping at tile bounds, antimeridian fixtures, and overlapping road/water/building features produce clear repair/error receipts.
3. **Spatial alignment:** independently validate CRS conversion, metre distances, local ENU anchoring, elevation datum, river/road draping and tile seams.
4. **Style determinism:** render every profile over one shared vector+DEM test locality. Verify source IDs/coordinates/route graph do not change across profiles; profile changes affect only derived presentation.
5. **LOD and resources:** test atlas/corridor/ground views, stable route connectivity, bounded memory, mesh counts, vertex budgets, streaming unload, and a low-detail fallback.
6. **Game applicability:** inspect a real mapped community facility, create a separate fictional project overlay, propose a project that depends on mapped access/routes, travel to a distinct mapped ground, save/reload and replay without a network request.
7. **Rights and attribution:** test per-layer licence manifest, OSM attribution UI, DEM/provider attribution, separable raw/derived source data, and the applicable licence review for every distributable pack.

## Data/source notes

- OpenStreetMap copyright/licence and attribution: https://www.openstreetmap.org/copyright
- Geoscience Australia Digital Elevation Data and ELVIS discovery: https://www.ga.gov.au/scientific-topics/national-location-information/digital-elevation-data
- Never assume the same DEM resolution, vertical datum, terms, or coverage across Australia; inspect the selected ELVIS product's metadata and licence for the specific region.
