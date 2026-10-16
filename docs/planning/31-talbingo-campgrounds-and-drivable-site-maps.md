# Talbingo: six distinct campground gameplay maps and drivable journeys

Status: implementation handoff plan, 15 October 2026. This supersedes the earlier conversation-only locator plan. The requested deliverable is **inside Maslows Challenge**, not a web directions map.

## Playable prototype completed 16 October 2026

The user chose playable stylized campsite maps first. `web/talbingo.html` now runs
six authored scenes from `crates/rolling-commons-shell/src/talbingo_scenes.rs`
through Qualia `.10d` compilation and `GamePortal`. A compact camper uses
Qualia's skeletal scene actor root, and `web/fixtures/talbingo-drive.vibe`
supplies the six illustrative road bends, surface labels and vehicle response.
Each map has a different landmark, native flora and fauna ecology, and camp decision.

Vegetation and wildlife are specifically authored for the Kosciuszko/Talbingo sub-regions:
- **The Pines (Blowering shore)**: Tall Ribbon Gums (*Eucalyptus viminalis*), Golden Wattle shrubs (*Acacia pycnantha*) with bright golden blossoms, foreshore turf, and grazing Eastern Grey Kangaroos.
- **Humes Crossing (Blowering inlet)**: Lakeside gums, wattle understory, inlet timber crossing deck, and Swamp Wallaby habitats.
- **Jounama Creek (Montane corridor)**: Montane river gums, Black Sallee (*Eucalyptus stellulata*), crimson River Bottlebrush (*Callistemon sieberi*) along granite boulders and creek gravel, and resident Bare-nosed Wombats (*Vombatus ursinus*).
- **Yarrangobilly Village (River flats)**: River gums, bottlebrush, historic riverbank hut, Laughing Kookaburras (*Dacelo novaeguineae*), and wild deer.
- **O'Hares (Steep Talbingo Reservoir ridge)**: Towering Alpine Ash (*Eucalyptus delegatensis*) framing steep mountain slopes, high lookout deck, and basking Lace Monitors (*Varanus varius* / Goanna).
- **Three Mile Dam (Kiandra high plains, 1400m)**: Contorted subalpine Snow Gums (*Eucalyptus pauciflora* subsp. *niphophila*) with mottled cream/olive/grey bark, dense alpine snowgrass (*Poa costiniana*) / sphagnum hummocks, and free-roaming Kiandra Brumbies.

The browser UI supports driving out and back, roadside wildlife encounters, an on-foot character moving by Vibe
rules and Qualia actor poses, landmark proximity/inspection, camp camera orbit
and zoom, and touch/keyboard controls. `scripts/check-talbingo.mjs` verifies all
six scenes, 30 visual/simulation road samples, 12 simulated route traversals,
six reachable on-foot landmarks, one full browser round trip including
walking, wrong and correct camp choices, and a mobile control layout check.
Screenshots are in `docs/qa/talbingo-*.png`.

This is **not** the source-accurate place pack defined below. It has no OSM
road geometry, surveyed campground layout, real-world route distance, save or
replay, or live access status. The page labels its road and layouts as
illustrative. Continue the geographic pack work below without relabeling the
prototype as navigation data. The immediate game-quality follow-up is
obstacle-aware local traversal and deeper interactions at each campsite; the
current walk has map bounds and illustrative water-edge blocking but no full
collision, pathfinding, inventory, save, or replay.

## Player outcome

From an in-game Talbingo area map, the player can choose any of six real campground destinations, inspect access and vehicle suitability, drive an outbound route, arrive in a **different playable local map for that campground**, explore there, and drive a return route or choose another reachable site. Save/load and replay preserve the selected destination, route progress, arrival and local activity state. The existing Kestrel/Saltwind world remains a labelled fictional campaign; Talbingo is a separate geographically sourced scenario pack.

Scope is the six destinations already named in this request: The Pines, Humes Crossing, Jounama Creek, Yarrangobilly Village, O'Hares and Three Mile Dam. “All” means these six for this delivery. Add other sites only through a later pack revision with the same provenance and acceptance gates.

## Source-backed identity and distinct map design

These are **design briefs**, not claims that real-world features have already been modeled. Each site must be bounded by a reviewed geographic extract and must have its own scene ID, origin, bounds, road entry, terrain/water layers, asset manifest and local interactions. Never relabel one scene six times.

| Site ID / coordinate anchor (lat, lon) | Source-backed character | Required local gameplay distinction |
|---|---|---|
| `au:nsw:talbingo:the-pines` / -35.447498, 148.285004 | Blowering Reservoir; NPWS describes sealed, 2WD access and a boat ramp | Gum-tree lake edge; launch/parking space, shoreline exploration and water-level-aware approach. The campground name is not evidence for pine vegetation. |
| `au:nsw:talbingo:humes-crossing` / -35.473610, 148.276108 | Blowering shore adjacent to Snowy Mountains Highway; sealed, 2WD access | Highway-side arrival, broad lakeside ground, boat-ramp and family-camp layout |
| `au:nsw:talbingo:jounama-creek` / -35.565764, 148.332163 | Turnoff opposite Talbingo township, then about 500 m of unsealed access trail; 2WD listed | Creek crossing/approach, grassy creekside pitches, short unsealed handling segment |
| `au:nsw:talbingo:yarrangobilly-village` / -35.652370, 148.462520 | Yarrangobilly River beside Snowy Mountains Highway; bridge works affect part of campground through February 2027 | River-flat ground, highway bridge/river arrival, heritage context only where sourced; current works represented as dated information, not a permanent map obstacle |
| `au:nsw:talbingo:ohares` / -35.822144, 148.365479 | Elliott Way at southern Talbingo Reservoir; sealed, 2WD access and boat ramp | Longer mountain-road arrival, reservoir-edge ground and launch area; verify full approach before authoring route |
| `au:nsw:talbingo:three-mile-dam` / -35.888331, 148.448881 | Kiandra/Link Road, 5.8 km from turnoff; seasonal vehicle gate from June to October long weekends | High-country dam, exposed terrain, Link Road approach and a seasonal park/foot-access state |

Coordinate anchors above came from ExplorOz/Wildwalks and are **candidate anchors**, not surveyed boundaries or route centerlines. Verify each against NSW NPWS maps and OSM feature IDs before use as player spawn or routing endpoints. Do not infer campground extent from the point alone.

Primary visitor sources:

- https://www.nationalparks.nsw.gov.au/camping-and-accommodation/campgrounds/the-pines-campground/visitor-info
- https://www.nationalparks.nsw.gov.au/camping-and-accommodation/campgrounds/humes-crossing-campground/visitor-info
- https://www.nationalparks.nsw.gov.au/camping-and-accommodation/campgrounds/jounama-creek-campground/visitor-info
- https://www.nationalparks.nsw.gov.au/camping-and-accommodation/campgrounds/yarrangobilly-village-campground/visitor-info
- https://www.nationalparks.nsw.gov.au/camping-and-accommodation/campgrounds/ohares-campground/visitor-info
- https://www.nationalparks.nsw.gov.au/camping-and-accommodation/campgrounds/three-mile-dam-campground/visitor-info

Each has a corresponding `/local-alerts` page. Refresh alert data at pack build/review time and show a live outbound link in game; never treat a dated notice as live closure state. Preserve NPWS map PDFs as review references, with dates and rights recorded. OSM roads/water/land-use and separately licensed elevation/ecology are candidate geographic layers; acquire a dated bounded snapshot and retain raw source identity, tags, licence, SHA-256 and transforms. Do not scrape Google Maps geometry or bulk download OSM raster tiles. Display `© OpenStreetMap contributors` wherever OSM geometry is shown.

## Current game architecture and hard boundaries

- `web/game.html` currently has a schematic Australia trip overlay, a fictional `PLACE_PACKAGE`, `map-place:` navigation that only binds to fictional `territoryId` values, and a Direct Drive controller whose route ends at the local demo road. The drive VibeScript is `web/fixtures/drive-simulation.vibe`.
- The place contract is `docs/schemas/community-place-pack.v1.schema.json`; the only loaded fixture is `web/fixtures/places/fictional-community-v1.json`, whose route graph and render tiles are empty.
- `docs/planning/29-australia-world-atlas-and-osm-pack.md` requires real-place route packs, individual site packs, bounded render tiles and map-matched driving; it expressly forbids renaming fictional scenery as real places.
- QualiaDB supplies the spatial/render/pack/simulation substrate. The game must not add a second map parser, geometry compiler, pathfinder, renderer, save store or vehicle simulation. Read `C:/github/qualiaDB/AGENTS.md` before upstream changes.
- QualiaDB has bounded Overpass and Terrarium adapters and OSM feature-to-`.10d` compilation, but the game does not yet have an offline geographic scene loader, route-network product or multi-site transition flow. Those are blocking platform tasks, not UI polish.

## Data and pack contracts

Create `web/fixtures/places/talbingo-campgrounds-v1.json` only after source review, validated against the existing place-pack schema. It needs a Talbingo locality/area node, six site nodes with unique `sceneId`, directed outbound/return route edges and node anchors, each source record, and references to six separate render artifacts/tiles. If the schema cannot encode active route context, seasonal access, per-site scene manifests or local activity bindings without abusing `extensions`, version the schema and validator. Do not set `spatialStatus: source_geometry` without a real `geometryRef` and packaged geometry.

Each site scene pack needs:

1. Stable site/scene ID; WGS84 extent and local ENU origin; declared CRS and vertical datum.
2. Road entry and safe spawn/parking geometry with a route-to-scene seam.
3. Terrain, water and significant road/path geometry derived from source layers; missing coverage explicitly flagged.
4. A distinct, authored arrangement of camp pads, vegetation, facilities and activity anchors, grounded in the site brief and source coverage. Unsupported buildings/facilities remain omitted or marked illustrative.
5. Qualia `.10d` assets and HMC manifest with provenance, licence, hashes, budgets and LODs.
6. At least one site-specific interaction and a reversible exit/return drive trigger. Gameplay outcomes use the shared Qualia action/event ledger.

## Driving contract

- Build a **Talbingo hub ↔ each site** road network with source-matched geometry and directed edges. Preserve alternatives and access tags. Two directions can share geometry in reverse only where the road is bidirectional; do not assume that a one-way park road permits the return path.
- Route selection must display route name, source date, surface, access constraints, approximate distance and supported vehicle rigs. Travel time derives from a versioned simulation profile, not a live Google estimate. Require explicit player choice to start each drive.
- Direct Drive loads the selected route and streams nearby road/terrain tiles. Vehicle steering, speed, thermal load and hazards remain in the existing Qualia/VibeScript drive path. The player sees first-person and optional aerial view, progress/remaining distance, arrival, off-road correction and a safe way to stop/leave. No teleport when switching scenes.
- Route ends commit an arrival receipt, save the visited site, unload the old route tiles and load the destination site pack. Return driving reverses the journey only after checking its directed edge and current access state.
- Seasonal restrictions (especially Three Mile Dam gates) are data with effective windows and explicit uncertainty. They cannot silently block the player based on a stale web alert; show the dated source and let the scenario use a documented simulation date.
- If an accurate mapped drive cannot be built for one destination, keep that destination visible as **atlas-only** and state why. Do not substitute a straight line or the fictional demo road.

## Implementation order and concrete work

### 0. Preserve a clean baseline

Run `git status`, the place-pack validator, existing game tests and WASM smoke test. Keep unrelated changes. Record QualiaDB revision and current HMC/API versions. Mark the earlier conversation locator as a reference only.

### 1. Acquire and audit geographic inputs

For all six sites and their drive corridors, acquire bounded dated OSM vector extract, official NPWS location/maps/access pages, elevation tiles and any ecology layer used. Store immutable raw snapshots plus SHA-256/provenance in an authorised data directory or external source cache with a reproducible retrieval manifest. Validate coordinate order, extents, OSM licence/attribution, access tags, road topology and missing segments. Produce a coverage report before mesh work.

### 2. Close QualiaDB upstream gates

Implement/test generic offline geographic pack ingestion, bounded spatial queries, route graph construction/map matching, route tile streaming and scene transition loading in QualiaDB. Reuse its existing Overpass, DEM, geometry and HMC APIs. Add native and WASM fixtures with malformed data, budget exhaustion, missing road connections, one-way roads, missing DEM and tampered pack hashes. Update the upstream capability ledger and pin the tested Qualia revision here.

### 3. Produce six independent site packs and route pack

Author the six local scene recipes and compile through Qualia into separate HMC resident scene IDs; verify different extents, geometry digests, visual landmarks, and entry/exit anchors. Package outbound and return road segments for each route, including any shared highway segments once and site-specific spurs separately. Build a Talbingo hub/garage entry map. Do not embed a whole regional map at full detail in every site pack.

### 4. Integrate in `game-demo`

Add a scenario/pack selector alongside the fictional campaign, load and validate the Talbingo place pack, extend atlas selection to show the six sites and route choices, and bind `sceneId` to site-specific HMC loading. Extend Direct Drive setup/tick/camera input with selected route context through Qualia APIs. Add visible source/fiction/coverage labels and OSM attribution. Save route and site state in the canonical save; do not create browser-only shadow state.

### 5. Gameplay, QA and release

For each site, test Talbingo→site drive, arrival, local exploration/interaction, site→Talbingo drive, save mid-drive, reload, replay and switching rigs. Exercise trailer handling on unsealed or constrained approaches and winter-gate state. Verify distinct scene IDs, geometry hashes and screenshots for all six. Profile memory/frame time at route/site transitions and avoid loading all six site packs concurrently. Run `scripts/validate-community-place-pack.mjs`, `scripts/test-game-wasm.mjs`, `scripts/verify-game-release.mjs`, relevant Cargo tests and a browser playthrough.

## Definition of done

- Six selectable destinations and **six visibly different playable local maps**, each with source-linked geography, separate pack/scene ID, unique entry/exit and at least one interaction.
- Twelve drivable journeys: outbound and return for every site, on mapped road geometry with correct direction/access handling; the shared road network may reuse segments.
- First-person and aerial drive views, rig-dependent handling, route progress and deterministic arrival receipts work for every journey.
- Save/load/replay retains site and route state, including mid-drive recovery, without returning to fictional Kestrel scenery.
- Source/rights/attribution/uncertainty are visible and packaged; current road/park conditions are linked, not claimed to be permanently accurate.
- All platform, pack, browser and gameplay checks above pass with recorded Qualia and game revisions.

## Handoff checklist

The next agent should read this file, `README.md`, `docs/planning/29-australia-world-atlas-and-osm-pack.md`, the place-pack schema/validator, `web/game.html` drive and `map-place:` sections, `web/fixtures/drive-simulation.vibe`, and `qualiaDB/AGENTS.md`. Start at step 0, update this document with source snapshot IDs and gate receipts, and mark each site/route complete only after an in-browser playthrough. Do not use the prior conversation HTML locator as an in-game implementation.
