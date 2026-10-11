# Civics project data bridge for the game

Date: 2026-10-11 (Australia/Sydney). Status: **source-envelope scaffold added; no civics.au data pack or semantic adapter is yet integrated into the game**.

This register turns the requested civics.au / dev.civics.au grounding into an implementable source contract. It separates real project outputs, public evidence, game-authored mechanics, demonstration data and personal/restricted data. A project page or working browser prototype is a discovery source; it is not automatically a licensed offline export.

## What already exists in the local workspaces

| Workspace | Observed artifact | What it can support | Current limit |
|---|---|---|---|
| `game-demo` | `fixtures/seed.ttl`, `fixtures/actions.json`, campaign event/action definitions | The existing fictional community simulation and its validated, replayable decisions | These are game rules and scenario content, not a real locality or public evidence dataset. |
| `game-demo` | `assets/ecology/profiles/se-corridor-reference.json`, `scripts/import-ala-ecology.mjs` | An Australia-region profile contract and an offline ALA CSV aggregation/report step | The corridor blend is illustrative. No downloaded ALA observation snapshot, locality polygon, dated OSM place pack or route graph is bundled. An occurrence is not a road hazard. |
| `game-demo` | Generated `.10d`/HMC catalogues and game-authored scenario/vehicle/fauna recipes | Reusable visual models, authored states and presentation | They are not source GIS, operating facility facts, or evidence that a service exists at a real place. |
| `qualiaDB` | `data/schemaorg` and generic RDF/JSON/Q42, geospatial, render and HMC capabilities | Shared vocabularies, evidence structures, provenance, geometry, and packaging | A general ontology directory is not civics place evidence and does not itself supply Australian locality content. |
| `www.civics.au` | `articles/*.md`, `articles/*.yaml-ld-q42`, article manifest and website source | Public concept language and semantic article descriptions | The tracked source does not contain the planning/evidence datasets or geographical exports described by the newer dev site. Treat prose as design context, not a machine-readable fact feed. |
| `observatory.civics.au` | Tracked `README.md` | Repository purpose/hand-off context | No tracked data export or evidence pack was found in the current checkout. |
| `humanitarian-equity-framework` | RDF cost/rights/provenance ontologies and enumerator example output | Possible concepts for recording contribution, cost and rights in authored scenarios | Its repository states a custom “All Rights Reserved, Pending Compensatory Implementation” licence. Do not redistribute or derive bundled game content from it until rights are explicitly cleared. |
| `WebCivics-Databox` / public databox demo | Service code/configuration and demonstration flows | Optional authored scenarios about data sharing, consent, access and receipts | The public project describes demo records as synthetic. Never connect the game to a personal pod, credential, payment or deployed community databox. |

The intended source architecture is for `observatory.civics.au` to become the data registry, with related descriptions and project material at `dev.civics.au`. Registry sources are expected to include RDF-based Q42 files with Qualia Logic and VibeScript enhancements, alongside a broad range of public datasets. The producer schema and lifecycle are still being defined. The current local Observatory checkout contained a README but no tracked export contract, so the game must not scrape HTML into a supposed authoritative database or silently reuse live demo state.

## Provisional package scaffold

The game now defines [the v1 envelope schema](../schemas/civics-observatory-game-pack.v1.schema.json) and `scripts/validate-civics-observatory-pack.mjs`. An unpacked future export has a `manifest.json` plus opaque, digest-checked payload files. The envelope records publisher/source identity, snapshot and update cadence, licence and redistribution statement, access and maturity, geography/time coverage, vocabulary/transformation versions, uncertainty, privacy review, prohibited inferences, artifact format and SHA-256. Artifact roles and format labels remain open strings so future Q42/RDF, Qualia Logic, VibeScript, vocabulary, geometry and other public-data payloads can be listed without freezing their syntax prematurely.

Run the scaffold with `node scripts/validate-civics-observatory-pack.mjs <unpacked-package-directory>`. It checks the provisional envelope fields, package-relative paths, duplicate IDs/paths, file containment and payload hashes. It does not parse or execute payloads, authenticate the publisher, establish licence rights, validate a source's claims, or make data ready for runtime. Source approval and game binding remain separate review steps. The manifest schema is a game-side proposal and must be aligned with the Observatory's actual export contract when that producer is ready; no Q42 or enhancement syntax is invented here.

## Game-side place and route interface

The source envelope is intentionally separate from the runtime geography model. After an approved adapter has parsed source-owned artifacts through the appropriate QualiaDB and domain tools, it can emit a normalized place pack described by [the v1 place-pack schema](../schemas/community-place-pack.v1.schema.json). The pack supports an open-ended parent/child place graph from Australia or state/territory through regions, localities, towns/areas and multiple distinct sites; route nodes and edges; source and hazard-profile references; and separately hashed render tiles/artifacts. Place geometry is a reference back to its reviewed source artifact, with CRS retained, rather than an invented game-side polygon.

The game-side package uses `sourceRecords` to retain exact source pack, manifest digest, artifact ID/digest, snapshot, licence/game-use status and transformation IDs. `spatialStatus` distinguishes source geometry from schematic or unavailable geometry. Open-ended `dataLayers` bind terrain, elevation, ecology, climate, settlements, facilities and later domains to their source and render artifacts. Route edges can carry distance, surface, grade, elevation change and rig constraints for trip-planning and vehicle-thermal simulation. These values must be sourced or explicitly marked as authored simulation inputs; animal occurrence data alone must never become a road-hazard probability. Render tiles are implementation chunks and are not place nodes.

The current [fictional place fixture](../../web/fixtures/places/fictional-community-v1.json) is consumed by a Places HUD page that navigates between the existing Kestrel/Saltwind scenes and their separate site targets. It is a graph-shape and UI example only: it has no real Australian boundaries, road graph, source-derived species records or trip route. Run `node scripts/validate-community-place-pack.mjs <unpacked-directory-or-place-pack.json>` to check graph references, cycles, tile links and rendered-file digests for a normalized pack; the existing Observatory validator remains responsible for opaque input-artifact containment and digests. Neither validator interprets source payloads or grants approval.

## Public project products to map

The live project pages currently describe these inputs and products:

| Product | Data maturity and likely game binding | Implementation rule |
|---|---|---|
| [Planning and outcomes](https://dev.civics.au/analysis) | Prepared ABS context at local-government-area scale plus reviewed public information on housing, services, work, education, tourism, business, retirement and safety; proposed to support planning and locally grounded questions | Consume area-level rows only after the exact source files, definitions, date, licence and geographic vintage are pinned. Use them to shape a town's context and scenario choices; never infer a character's circumstances, eligibility, safety or behaviour. Keep community-provided local knowledge separate and consented. |
| [Community site planner](https://dev.civics.au/site-planner) and [modelling lab](https://dev.civics.au/model) | Site, load, amenities and location assumptions flow through financial, solar/battery, water/thermal and report calculations; the public project labels the planning environment as an early proposal and the financial model as preliminary | Represent inputs, assumptions and computed ranges in town/site activities and visible infrastructure states. Version formulas and units. Never present estimates as engineering design, guaranteed savings, measured site condition or a live operating service. |
| Walkabout and rig fit-out | The project describes campervan, camper-trailer and caravan choices with towing, payload, berths, repairs, fit-out and weekly budget | Map authored rig definitions into game loadouts and trips after checking the actual source model and revision. Keep simulation handling and thermal gauges explicitly in-game. |
| [Food & health evidence](https://dev.civics.au/food-health) | The page describes a reviewed linked-data map of 297 normalised foods/ingredients and 200 meal options, with evidence tiers, provenance, cautions and trade-offs | A cooking/food-preparation scenario can reuse reviewed public food identifiers and cautions after a rights/domain review. Never ingest personal health context; don't recommend treatment or label a game choice medically beneficial for a player. Native foods require local and cultural review. |
| [Databox and data-safe design](https://dev.civics.au/databox) | Governed exchange, scoped disclosures and record receipts; the described in-browser reference demo uses synthetic data, while production deployment needs hosting/governance/security work | These can inform authored interpersonal choices about permission and accountability. Keep all actors and records fictional. Do not make online calls to a real databox. |
| Community grounds, cooperative projects and community ledger | Site stewardship, work contributions, project provenance, maintenance and possible fair returns; product maturity varies by page | Bind visual changes to the existing action receipt model. Scenario authors must label authored proposals and assumptions, and must not imply a signed site agreement, employer, job vacancy, access permission or real person's contribution. |

Project webpages are useful source locators and definitions; they are not substitutes for the actual export artifact and its licence. At the time of this inventory, the public site presents several components as concept notes, the planning/financial work as preliminary in places, the food-and-health map as a reviewed dataset, and the databox example as synthetic. Preserve those distinctions in every pack.

## Pack and adapter contract

An accepted project-data input has a stable `source_id` and a versioned adapter record with at least:

- publisher/project, artifact name and stable URL or repository path;
- exact release, snapshot date/checksum and update cadence;
- source licence, redistribution/derivative conditions, access type and attribution text;
- geography and time period, geographic/semantic units, definitions and measurement units;
- transformations, mapping/vocabulary version, code digest and dropped/aggregated fields;
- maturity (`observed`, `prepared_public_context`, `reviewed_evidence`, `preliminary_model`, `concept_reference`, or `synthetic_demo`), coverage, uncertainty and known evidence gaps;
- privacy/sensitivity review, permitted simulation binding and explicit prohibited inferences.

Package a reviewed offline snapshot for deterministic play. Preserve the original project identity and terms in provenance; a normalized game observation links to its source fact rather than replacing it. Store person-owned or restricted records outside shared packs. The game never uploads character journeys, play choices or local saves into a public source service by default.

## Binding data to places and modes

The atlas can show sourced boundaries, route references, area aggregates and uncertainty at their supported scale. A town pack aggregates only data intended for that geography. It then links one or more independent Community Grounds sites to the town graph. Each site's source extent, building/amenity evidence and authored models remain distinct.

Scenario modules declare which sources they can read and which shared game state they can change. A fruit-picking task may use an authored crop, a season profile and a cited regional work context; a housekeeping shift may use an authored hotel/motel/resort site and its shift tasks; a workshop task may consume fictional spare parts and produce an action receipt; a cooking task may draw on reviewed meal/ingredient identifiers and display cautions. A personal/interpersonal story may use area-level context to set its authored question, but all characters and dialogue remain fictional unless there is explicit consent for a separately governed story. These are example bindings, not claims of a real employer, local food service or job vacancy.

Driving loads route and region facts for the current trip segment and selected rig. An exact road/wildlife encounter needs source-backed local profiles and its own evidence/uncertainty review; a public species observation cannot create a collision location or individual animal trajectory. Ambient temperature, grade, road surface, towing/load and engine health can drive the current game simulation, but it cannot claim vehicle telemetry or diagnose a real vehicle.

## Completion gates

1. Record an actual output artifact for each selected project product. If no licensed export exists, document that dependency and use only properly licensed public data or clearly marked synthetic fixtures.
2. Align the provisional envelope with the Observatory export contract. Then build QualiaDB-owned generic RDF/Q42 and enhancement validation/import/pack capabilities where a reusable operation is missing; keep game code to authored mappings, source manifests, scenario bindings and fixture data.
3. Produce one reproducible offline Australian locality pack with public provenance, one or more distinct grounds, a linked route graph, a separate DEM/ecology profile, and no personal data.
4. Demonstrate route selection, a place-scoped simulation decision and at least one interpersonal or work/activity scenario whose outcomes replay from shared receipts.
5. Verify privacy, source terms, location/taxonomy uncertainty, attribution, cached/online loading and offline fallback before any pack is represented as complete.
