# 05 — World content, six distinct sites, and provenance

Status: implementation plan, 17 October 2026. No content pack, source verification, or runtime change is delivered by this document. Read [the current audit](00-current-build-audit.md), [product plan](01-product-and-gameplay.md), and [architecture plan](03-architecture-state-and-platform.md) first. The binding [QualiaDB-only contract](../13-qualiadb-only-development-contract.md) applies to authoring tools as well as the shipped game.

## Outcome and release boundary

Build six memorable, internally consistent illustrative camping scenes. Each needs its own arrival, layout, landmarks, exploration route, environmental reading, and meaningful camp decision. A player should be able to distinguish any pair from their terrain and composition before reading a title. Share an asset library, but do not share a whole campsite layout with renamed signage.

The first release remains an **illustrative Talbingo-inspired game**. Its coordinates, roads, site layouts, hazards, conditions, and travel times must not be presented as navigation or visitor advice. Use local scene units until the scale contract is verified; afterwards label authored metres as simulated distance. A separate, optional geographic pack can graduate to source-backed geography only through DATA-08 and DATA-09. Its optional status is a product sequencing decision, not permission to replace required QualiaDB capabilities with game-side code.

Real campground names do not establish that a depicted deck, hut, lookout, animal, tree species, rock formation, or camping spot exists at that place. Art can retain an invented feature if the pack explicitly identifies it as authored. Unsupported factual labels must be removed or rewritten before release; copying a number from an older plan is not verification.

## What is present in the inspected build

The following statements describe source inspected at game revision `b2bfe7cdc5e75e7423c855ded420fb0bb24a2d3e`. They are not new browser test results.

| Existing surface | Evidence and practical limit |
|---|---|
| Six authored scene variants | `crates/rolling-commons-shell/src/talbingo_scenes.rs` selects assets with `site: u8`, compiles them through `asset_compile::compile_asset`, and returns assets and texture receipts. The common road, camp spur, gate, fire circle, picnic table, tents, and seven-tree pattern make layouts closely related. |
| Procedural terrain | `talbingo_terrain.rs::elevation` uses trigonometric, exponential, and polynomial functions; `build_terrain_asset` samples a 48 × 36 grid. Comments describe surveyed/authentic geography, but this function consumes no survey or DEM source. Correct that mismatch. |
| Inline place information | `web/talbingo.html::SITES` carries coordinates, elevation, `distKm`, geological ages/units, ecology text, landmark positions, and choice answers. Displayed GPS is interpolated from hub to destination during travel. Values differ from candidate anchors in [plan 31](../31-talbingo-campgrounds-and-drivable-site-maps.md); this audit does not decide which coordinates are correct. |
| Vegetation and fauna | `talbingo_vegetation.rs` and `fauna.rs` create original visual recipes. Species-level comments and UI text are not accompanied by Talbingo-specific reviewed source records. Treat those assignments as unverified illustrative selections. |
| General place-pack contract | `docs/schemas/community-place-pack.v1.schema.json` already supports maturity, spatial status, source records, places, route graph, artifacts, tiles, and data layers. `web/fixtures/places/fictional-community-v1.json` is an existing fixture. This does not mean Talbingo currently loads such a pack. |
| Ecology review helper | `scripts/import-ala-ecology.mjs` aggregates a supplied ALA CSV and metadata into coordinate-free review data. It does not approve a species, place animals, or prove road risk. Review its ownership against the QualiaDB contract before extending its parser or processing logic. |
| Original asset provenance | `asset_compile.rs` attaches original source provenance with `All rights reserved (licence not assigned)`. That records a current declaration; it is not a reviewed distribution licence for every future contribution. |

Do not treat historic plans' “verified” status as evidence for the current Talbingo browser path. Reproduce the narrow feature at the pinned QualiaDB revision and record its receipts.

## Proposed content architecture

Everything below is a proposed authoring contract. Names are planning names, not already-exported QualiaDB APIs. JSON may be used as a declarative authoring input only; authoritative facts, accepted actions, package encoding, geometry generation, and persistence remain QualiaDB-owned.

Keep three independent layers:

1. **Place context:** name, provenance, spatial status, reviewed geographic features when available, and evidence status. Immutable within a released pack.
2. **Illustrative scene content:** layout, spawn points, authored geometry parameters, semantic interaction IDs, cameras, art profile, and encounter settings. Clearly distinguish authored values from source values.
3. **Session state:** inspected clues, placed camp equipment, completed tasks, accepted encounter outcomes, and journal entries. Store through the architecture chapter's QualiaDB session and replay contract; never write progress back into the immutable pack.

Proposed authoring files:

```text
web/fixtures/places/talbingo-illustrative-v1.json       # existing place-pack v1 shape
assets/talbingo/scenes/the-pines.v1.json              # proposed scene schema below
assets/talbingo/scenes/humes-crossing.v1.json
assets/talbingo/scenes/jounama-creek.v1.json
assets/talbingo/scenes/yarrangobilly-village.v1.json
assets/talbingo/scenes/ohares.v1.json
assets/talbingo/scenes/three-mile-dam.v1.json
assets/talbingo/content/claims.v1.json
assets/talbingo/content/credits.v1.json
assets/ecology/profiles/talbingo-illustrative-v1.json
docs/schemas/talbingo-scene.v1.schema.json             # proposed authoring validator
docs/schemas/talbingo-claims.v1.schema.json            # proposed authoring validator
docs/art/talbingo-source-review.md                    # proposed human review log
```

Do not create the source-accurate pack under the illustrative pack ID. Reserve `talbingo-geographic-v1` for the reviewed geographic path. Do not silently change a saved illustrative scene's world coordinate frame when a geographic pack becomes available.

### Scene document fields and invariants

| Proposed field | Type and rule | Consumer |
|---|---|---|
| `schemaVersion`, `sceneId`, `placeId`, `contentVersion` | Fixed version; stable string IDs; reject unknown versions. Use a stable slug, not the dropdown index, in saves and semantic IDs. | Pack validation, session |
| `maturity`, `spatialStatus` | First pack uses `illustrative` and `schematic`. A top-level value cannot promote individual unreviewed claims. | UI disclosure, source inspector |
| `units`, `localFrame`, `bounds` | Explicit scale/axis convention; finite bounded local AABB; runtime render normalization is separate. Do not invent a CRS for an illustrative scene. | Authoring compiler, traversal, cameras |
| `seed` | Fixed integer interpreted by a versioned QualiaDB generator; recorded in build receipt. Never seed required interactions from wall-clock time. | Asset placement, cosmetic variation |
| `entrySpawn`, `parkingPose`, `walkSpawn`, `exitTrigger` | Stable IDs, finite positions/orientations, intended surface IDs. They must be inside scene bounds and usable with authored collision. | Travel and local traversal |
| `terrainRef`, `surfaceRefs`, `waterRefs`, `obstacleRefs` | References to compiled scene data and the same semantic surfaces used by collision. No parallel mesh-derived gameplay truth in JavaScript. | QualiaDB geometry and queries |
| `landmarks` | Stable ID, display key, inspect text key, asset/entity reference, reachable interaction area, and evidence references. Max 12 per initial scene is a provisional authoring bound. | Interaction and objective UI |
| `activityRefs`, `clueRefs`, `encounterProfileRef` | Reference supported scenario content and rules. No embedded executable JavaScript or arbitrary shader code. | QualiaDB rules/Vibe host |
| `floraProfileRef`, `faunaProfileRef` | Select reviewed or explicitly illustrative render keys independently of species names. Unknown taxa remain unknown. | Ecology selection, asset compiler |
| `cameraProfileRef`, `artProfileRef`, `audioProfileRef` | Presentation references admitted by the engine capability ledger. No audio profile implies support for its browser playback. | Presentation bridges |
| `sourceRefs`, `claimRefs`, `assetRefs` | Every reference resolves within the pack or its immutable dependencies; dependencies are digest-bound. | Provenance and packaging |

Use a versioned named entry under existing place-pack `extensions` only if the existing validator can validate that entry and readers reject incompatible versions. Otherwise add a deliberate schema revision with compatibility fixtures. Do not add ad hoc top-level keys to `community-place-pack.v1.schema.json`; it has `additionalProperties: false`.

### Claim and source status

Proposed claim records need `claimId`, `subjectId`, `field`, `value`, `unit` where relevant, `status`, `sourceRefs`, `reviewedAt`, `reviewer`, `uncertainty`, and `displayKey`. Statuses:

- `authored`: a declared game design choice; cannot be displayed as a measured fact.
- `source_reviewed`: the specific source supports this specific claim at a recorded scale/date.
- `estimated`: derivation and uncertainty are explicit; never silently shown as measured.
- `unverified`: retain for internal review; do not publish precise public-facing claims.
- `contested`: show the disagreement only with reviewed context; do not collapse it to one number.

Source records reuse the existing place-pack identity and digest fields. Supporting metadata must additionally record the exact dataset/product edition, retrieval date, licence/attribution, spatial extent, horizontal and vertical reference system where relevant, source resolution, permitted uses, transformation receipt IDs, and review status. These additions need a versioned contract; they are not assumed to exist in v1.

For existing `licenceStatus` and `gameUseStatus`, preserve their actual allowed values. A reviewed distribution gate should admit only a source whose intended use is approved and whose applicable conditions are satisfied. `unknown`, `review_required`, and `reference_only` must not be converted to `approved` merely because a file can be downloaded.

## Six illustrative scene briefs

All terrain, structures, activities, and ecological arrangements in these briefs are **proposed game fiction**, pending any independent source review. They deliberately do not assert real facilities or current conditions. Integrate their objectives with the common gameplay loop in chapter 01; do not implement a separate quest engine.

Each brief supplies: one unmistakable silhouette; one arrival reveal; a 2–4 minute optional exploration loop; three visible clues; at least two defensible choices with a trade-off; one non-combat encounter; and a short outcome that changes the camp and journal. Times are provisional playtest targets, not real travel durations.

### The Pines — open shore and a sheltered clearing

- **Composition:** a broad horizontal water view, an irregular shoreline, and two separated tree groups frame an open camp clearing. The main landmark is the contrast between open shore and inland shelter, not a named real-world tree species. The view must remain readable without a blanket haze.
- **Arrival:** reveal the water after a gentle bend; present parking and a walking loop as separate spaces. The player leaves the vehicle in a clearly authored parking area and sees the clearing before choosing a pitch.
- **Exploration:** short loop from parking to shore viewpoint to sheltered ground, with an optional accessible shortcut. Three clues: exposed ground/wind indicator, a visibly damp low patch, and a durable established surface.
- **Decision:** compare a scenic exposed pitch with a more sheltered durable pitch, using the fictional scenario's stated wind and ground conditions. Do not claim either is a real permitted campsite. Let the player inspect both before committing and relocate after feedback.
- **Encounter and change:** a distant illustrative grazing animal makes the player pause without requiring approach or feeding. Completion places selected camp equipment, records the reasoning, and reveals a return view from the clearing.
- **Acceptance:** water/clearing/parking identifiable from overview and walk camera; no interaction requires entering the water or approaching wildlife; pitch choice affects at least one visible and one journal state.

### Humes Crossing — inlet edge and route choice

- **Composition:** a branching inlet creates two differently shaped banks. A short invented timber deck may remain as an explicitly authored landmark, never a claim about an actual crossing at the campground.
- **Arrival:** a more enclosed entry opens onto the inlet. A turning/parking area creates a different driving finish from The Pines.
- **Exploration:** one direct path and one longer dry-side loop converge at the landmark. Three clues: a marked inspection point, visible deck/surface wear, and an alternative route with its own distance/effort trade-off.
- **Decision:** inspect the fictional crossing and decide between the short crossing permitted by scenario rules and the longer dry route. If the designed condition closes the crossing, clearly block it through collision and rules; do not make the answer depend only on colour.
- **Encounter and change:** a quiet observation stop on the far side with an optional ecological note. Completion updates an in-game route marker and the journal; it does not purport to repair a real structure.
- **Acceptance:** both routes are reachable or explicitly unavailable, collision agrees with the condition, and the inlet silhouette is distinct in a six-scene contact sheet.

### Jounama Creek — intimate creek corridor

- **Composition:** a meandering narrow waterway, asymmetric banks, several varied boulder shapes, and patches of low growth produce a close, shaded scene. Rock types stay generic until sourced.
- **Arrival:** the authored road changes surface before the parking spur; use the accepted surface state for both handling and sound.
- **Exploration:** a bank-side path exposes three clues: damp margin, a flatter durable bench, and a clear buffer around the water. The return path passes behind the boulders instead of duplicating the approach.
- **Decision:** choose a camp footprint whose size, slope, and distance from the authored sensitive margin satisfy the stated scenario constraints. Trade a longer equipment carry for better conditions.
- **Encounter and change:** an illustrative small fauna encounter can be observed from the path. Completing a cleanup/packing activity removes authored litter or consolidates gear through accepted state changes, with no unsupported claims about actual site condition.
- **Acceptance:** all required clues can be inspected from dry traversable ground; creek and bank materials remain distinct at low quality; collision/placement reject a camp on the fictional wet margin.

### Yarrangobilly Village — river flat and story stop

- **Composition:** a wide river curve, a small authored shelter or interpretation point, and an open flat create a social/story scene. Retain the current hut only as a fictional structure unless a source and rights review support a real depiction.
- **Arrival:** frame the story landmark across the flat before showing the pitch choices. Keep the road clearly separate from pedestrian space.
- **Exploration:** a loop joins the interpretation point, river viewpoint, and existing durable camp surface. Three clues: the scenario's ground condition, a visible access path that must stay clear, and a sheltered communal spot.
- **Decision:** arrange camp while preserving shared access, then choose one optional conversation or journal observation. Authored fictional characters may explain the game objective; they must not impersonate a real local representative.
- **Encounter and change:** a non-combat social encounter or distant bird observation replaces an obligatory identical roadside animal stop. Completion produces a visibly organized camp and a journal page.
- **Acceptance:** content distinguishes fictional story from cited history; no invented Indigenous knowledge, heritage fact, or cultural association is published as factual. A generic descriptive page remains possible until reviewed content exists.

### O'Hares — steep backdrop and sheltered stop

- **Composition:** steep distant slopes contrast with a believable bounded play terrace; long reservoir views create depth. The existing invented ridge platform is a candidate game landmark, not evidence of a real lookout.
- **Arrival:** use a different curve rhythm and a constrained-looking but traversable parking finish. Keep the driving challenge within the shared vehicle model rather than custom per-site physics.
- **Exploration:** a gentle terrace path and optional steeper scenic branch lead to an authored viewpoint. Three clues: slope, exposure, and a clear safe interaction area behind the authored boundary.
- **Decision:** choose a parking/camp arrangement that preserves turning space and limits fictional wind exposure. The return walk shows the vehicle from another angle to make spatial orientation clear.
- **Encounter and change:** a distant reptile or generic fauna observation is optional and explicitly illustrative until the locality profile is reviewed. Completion adds a viewpoint note and prepares the return leg.
- **Acceptance:** no required action occurs on an unstable slope or inaccessible prop; the boundary is communicated by terrain and a redundant UI cue; camera collision does not reveal a terrain edge.

### Three Mile Dam — open grassland and sensitive margin

- **Composition:** low irregular tree forms, sparse open grass/tussocks, a rounded water body, and distant layered landforms. Exact tree, grass, moss, and animal taxa remain review-gated.
- **Arrival:** longer sightlines and an exposed curve sequence create a different journey rhythm. A fictional gate may support a clearly dated scenario state, without implying the real road is open or closed.
- **Exploration:** a dry rise and visually different soft margin offer three clues: ground firmness, exposure, and an established route. Keep a dry observation path around the designed sensitive area.
- **Decision:** choose ground that protects the authored soft margin and then secure equipment for the scenario's wind. Show the trade-off between shelter and carrying distance.
- **Encounter and change:** if horse imagery is retained, label the encounter illustrative and review conservation/management context before publishing factual interpretation. Do not make chasing, feeding, or approaching wildlife a progress requirement.
- **Acceptance:** open composition differs clearly from Jounama; sensitive ground has matching placement/traversal rules; narrative explains the decision without pretending to model real habitat condition.

## Implementation work packages

### DATA-01 — Remove unsupported factual presentation and inventory claims

**Priority:** P0. **Depends on:** audit; coordinate labels with chapter 02. **Files:** existing `web/talbingo.html`, `talbingo_terrain.rs`, `talbingo_vegetation.rs`; proposed `assets/talbingo/content/claims.v1.json` and `docs/art/talbingo-source-review.md`.

1. Inventory every player-visible place, species, geological, distance, elevation, access, heritage, and facility claim. Link each to its source location and current evidence status.
2. Mark formula terrain, interpolated GPS, route lengths, and invented props as authored. Replace unsupported live/survey/geology badges with an illustrative scenario label. Do not substitute older candidate coordinates.
3. Remove precision that the evidence cannot support. Use a broad display name or explicit fictional description where exact taxonomy/geology is unverified.
4. Record the review queue for useful future facts; preserve source history in the audit without keeping misleading runtime copy.

**Acceptance:** no displayed or documented surveyed/live claim lacks its specific approved source; GPS interpolation is no longer called GPS; all six scenes visibly state their illustrative nature. A content test searches the release copy for retired labels and confirms the approved replacements, without trying to infer truth from text alone.

### DATA-02 — Establish stable scene and claim contracts

**Priority:** P0. **Depends on:** DATA-01 and architecture's versioned content boundary. **Files:** proposed schemas and authoring paths above; existing `docs/schemas/community-place-pack.v1.schema.json`, `scripts/validate-community-place-pack.mjs`, `talbingo_scenes.rs`, and the future thin content loader.

1. Draft schemas from the field tables; decide explicit local axes and units with traversal owners.
2. Map dropdown indices 0–5 to stable scene IDs once at the UI boundary. Never persist the index as identity.
3. Validate finite coordinates, bounds, unique IDs, source references, allowed content operations, digest references, and scene-to-place links. Validation tooling must use QualiaDB-owned processing where required by the contract.
4. Create one representative illustrative fixture and invalid fixtures for missing references, incompatible versions, out-of-bounds spawn, unreviewed factual claim, and duplicate landmark identity.
5. Specify compatibility behavior: reject incompatible packs with a readable explanation; do not partially load or silently reinterpret units.

**Acceptance:** one validated fixture compiles through the existing asset pipeline and resolves all content IDs; invalid fixtures fail with a useful file/field reason; proposed fields are never assumed to be exported APIs.

### DATA-03 — Build the shared world-authoring kit

**Priority:** P1. **Depends on:** DATA-02; ART-02/03/04; shared spatial-query gate in chapter 04. **Files:** `talbingo_scenes.rs`, `talbingo_terrain.rs`, `talbingo_vegetation.rs`, `asset_catalog.rs`; proposed scene fixtures.

1. Separate shared prop recipes from per-site composition. Use explicit recipe references, scale, orientation, material family, semantic role, and provenance.
2. Author terrain, route, water, obstacle, pitch, and interaction data in one scene frame. Query the same compiled surfaces for placement and traversal.
3. Provide an authoring review output showing bounds, entry/exit, obstacles, water, interaction regions, and camera positions through supported QualiaDB debug presentation. Do not add a parallel 3D editor or collision solver.
4. Freeze the illustrative seed and variant version so visual rebuilds do not move important objects unpredictably.

**Acceptance:** one scene can move a landmark without separately editing hard-coded UI, interaction, and collision coordinates; every required interaction has a traversable approach; all source-generated positions have reproducible receipts.

### DATA-04 — Produce and playtest six differentiated scenes

**Priority:** P1. **Depends on:** DATA-03; gameplay activity contract; ART-01 through ART-06. **Files:** six proposed scene fixtures; `talbingo_scenes.rs`; authored rules under the chapter 01/03 convention; proposed `docs/qa/talbingo-renewal/content-review.md`.

1. Complete The Pines as a vertical slice first: road arrival, parking, clue loop, choice, visible result, return.
2. Test it at walk and overview scale before detailed dressing. Then implement Humes/Jounama as the route/water variation pair, Yarrangobilly/O'Hares as story/relief variations, and Three Mile Dam as the open composition.
3. Use the briefs as constraints, not fixed world coordinates. Review traversable space before adding detail.
4. Capture one overview and one landmark view per scene, and ask reviewers to identify scenes with names hidden. Record confusion pairs and revise composition.
5. Play each site's complete loop with keyboard, controller, and touch through the same authoritative scenario state.

**Acceptance:** all six have distinct layout, encounter pacing, clue placement, and visible outcome; no site merely renames the same binary quiz; no required clue is hidden behind an inaccessible mesh. Provisional target: at least four of five reviewers distinguish all six overview images; a miss opens a composition task rather than being dismissed as taste.

### DATA-05 — Review ecology before promoting species claims

**Priority:** P1 for honest illustrative labels; P2 for source-backed ecology. **Depends on:** DATA-01/02. **Files:** `assets/ecology/profiles/`, `talbingo_vegetation.rs`, `fauna.rs`, `taxon_bindings.rs`, `docs/art/australian-ecology-data-contract.md`, and proposed source review log.

1. Keep model identity separate from asserted taxon and locality. Reuse a generic render key without implying verified species presence.
2. For a future review, select bounded source products and record taxonomy, observation age, spatial uncertainty, aggregation, reporting bias, and sensitive-record treatment. An occurrence does not establish abundance, habitat condition, a particular campsite placement, or road risk.
3. Have a reviewer approve the locality profile and the specific interpretive text. Store rejected/unresolved selections rather than silently mapping them to another species.
4. Make native/introduced or management statements only when the selected authoritative source supports them. Do not infer habitat suitability from a matching green colour or OSM land-use tag.
5. Keep exact sensitive coordinates out of runtime packs. Any aggregate still needs rights/privacy review before distribution.

**Acceptance:** every published species-at-place claim resolves to reviewed evidence and uncertainty; authored encounters state that status; rendering a species does not automatically approve its educational label. Pipeline tests reject unresolved factual selections but allow explicit generic illustrative assets.

### DATA-06 — Produce contextual writing, credits, and localisation

**Priority:** P1. **Depends on:** DATA-02/04/05; chapter 02's text/accessibility contract. **Files:** proposed `assets/talbingo/content/`, reviewed ecology profiles, existing asset provenance compiler and HMC manifest path.

1. Move the six descriptions, clues, interactions, outcomes, and journal pages out of inline `SITES` into stable localisation keys.
2. Give each required decision enough visible evidence to be understandable without specialist Australian ecology knowledge. Reward observing, comparing, and revising, not memorizing an unsupported “safe” answer.
3. Separate gameplay explanation, fictional story, and reviewed place notes in the content metadata and UI.
4. Keep a credits entry for every external dataset, recording, font, texture, and contributor, with permitted uses and required attribution. Resolve original asset distribution terms before a public release.
5. Treat sensitive heritage/cultural content as a commissioned review task with appropriate sources and permissions. Until that exists, use neutral description rather than invented tradition or authority.

**Acceptance:** text expansion and translation do not change logic; all outcomes explain consequences; every packaged external asset has traceable credit and allowed distribution use; the release contains no unattributed copied interpretation.

### DATA-07 — Compile deterministic offline content packs

**Priority:** P1. **Depends on:** DATA-02/03 and the chapter 03 pack/session gates. **Files:** existing `scripts/export-game-assets.mjs`, `asset_compile.rs`, `pack_validation.rs`, `web/assets/`; proposed Talbingo package manifest and build receipts.

1. Move Talbingo release asset compilation from site-switch time to the supported QualiaDB build/export path. Retain runtime recipe compilation only as a declared development mode.
2. Compile Q42 references, `.10d` geometry/material/rig data, HCF text, and HMC distribution through their existing or newly verified public capabilities.
3. Emit a manifest of content/schema/engine versions, source digests, transformation receipts, asset counts, bounds, and budgets. Asset filenames are outputs, not the source of semantic identity.
4. Rebuild from identical input twice; compare canonical content/geometry identities using QualiaDB-supported receipts. Exclude documented nonsemantic timestamps from the comparison through the supported canonicalization contract.
5. Load every site offline and verify missing/corrupt packs fail atomically with the prior playable scene preserved.

**Acceptance:** release switching does not re-author thousands of leaves in the browser; offline provenance/credits remain readable; pack/version mismatch cannot change a saved scene silently; chapter 07 residency budgets and chapter 08 corruption tests pass.

### DATA-08 — Optional source-backed geography feasibility gate

**Priority:** P2, explicit separate expansion. **Depends on:** DATA-01/02/07; QualiaDB geospatial capability assessment. **Files:** existing [plans 29](../29-australia-world-atlas-and-osm-pack.md), [30](../30-procedural-geospatial-rendering-pipeline.md), and [31](../31-talbingo-campgrounds-and-drivable-site-maps.md); proposed `docs/art/talbingo-geographic-feasibility.md`.

1. Read the current source and tests in QualiaDB's geospatial/terrain/OSM adapters. Classify bounded acquisition, CRS conversion, DEM decoding, topology, route connectivity, tiling, licence preservation, and offline loading individually as verified, partial, missing, or unverified.
2. Revisit the provider links in plan 31 at implementation time. Verify each site's identity, anchors, boundaries, access information, and data rights; do not reuse its dated notices as current facts.
3. Propose a bounded data snapshot with an explicit horizontal/vertical reference system, resolution, coverage, rights, and review owner. Do not fetch uncontrolled data during ordinary play.
4. Produce one tiny source-backed terrain/road/water fixture through QualiaDB; independently check a known distance, axis orientation, height datum, missing-data case, and seam. Keep source coordinates separate from render exaggeration.
5. Record upstream work orders for unsupported stages. Do not add OSM parsing, map routing, raster decoding, or coordinate math to the game as a shortcut.

**Acceptance:** a written go/no-go report includes precise supported capabilities, gaps, rights conditions, storage estimates, and one reproducible fixture. “Go” does not relabel the illustrative pack. “No-go” leaves it available and honestly labelled.

### DATA-09 — Optional geographic pack production and freshness

**Priority:** P2. **Depends on:** successful DATA-08 and all required upstream proofs. **Files:** proposed `web/fixtures/places/talbingo-campgrounds-v1.json` per earlier plan, six separate source-backed scene manifests, source snapshots outside redistributable assets unless allowed, release attribution pages.

1. Snapshot roads, water, elevation, place/facility context, ecology, and access independently; record source IDs and transformation receipts at feature level.
2. Resolve road endpoints and site boundaries using reviewed data. Derive route distances through QualiaDB routing/geometry; retain missing/uncertain values instead of fabricating them.
3. Compile six independent site packs and directed outbound/return connections. Topology, collision, and display consume the same source-derived geometry version.
4. Add an explicit freshness model: `observedAt`, `reviewedAt`, `validUntil` where supplied, `unknown`/`stale` status, and a provider link. A snapshot must never claim live closure or visitor suitability. Reading a current link is the player's external action, not a guarantee by the game.
5. Review redistribution, attribution, derivative data obligations, sensitive data, and third-party media rights for the actual snapshots. Requirements are determined during review, not assumed from the provider name.
6. Save pack identity/version with the session. Offer a clear choice to start a new geographic scenario; never teleport an existing illustrative save into source geography.

**Acceptance:** coordinate/distance/terrain source accuracy is independently reviewed within declared error tolerances and source resolution; all six remain distinct; offline attribution and dated status are visible; unsupported access or ecology information remains unknown. Release documentation names exactly what was validated and what remains stylized.

## Review evidence and handoff

For every accepted site, retain: source/claim inventory, pack and engine revisions, authored local frame, overhead navigation overlay, two standard screenshots, one full-loop recording, interaction reachability results, content review notes, and licence/credit receipt. Store the evidence under `docs/qa/talbingo-renewal/` with the release record in [chapter 08](08-testing-and-release.md).

The useful definition of done is a player-readable, repeatable scene whose geometry, interactions, copy, and provenance agree. More precise numbers or more named species are not improvements until the evidence supports them.
