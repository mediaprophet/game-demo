# Maslows Challenge: Place Data and Content Packs

## Purpose

Place data makes a scenario locally meaningful without turning the game into a
surveillance system or claiming to be an operational planning tool. A town pack
uses public, aggregate, licensed data to shape terrain, services, constraints,
and opportunities. It never models actual residents as game NPCs.

## Pack structure

```text
town-pack/
  manifest.json                 identity, version, licence, digests
  graph/town.q42                semantic entities and spatial relationships
  graph/rules.q42 or rules.n3   validated local rules and constraints
  assets/*.10d                  terrain, buildings, facilities, visual props
  data/metadata.json            source, date, transformation, uncertainty
  data/models/*                 reviewed deterministic/statistical analysis inputs/methods
  scenario/*.vibe               bounded scenario scripts
  presentation/*                text, icons, audio, accessible labels
```

This directory layout is an **authoring view**. The distributable pack should
use a verified QualiaDB HMC profile with intact Q42, `.10d`, HCF/Vibe, and
other relevant entries. The final manifest and archive layout follow the
canonical HMC producer/reader chosen in QG-01; this sketch is not a new format.
All acquisition, spatial transformation, geometry generation, `.10d` packaging,
and pack validation must use QualiaDB ecosystem capabilities. Missing adapters
or browser surfaces are upstream work under the
[development contract](13-qualiadb-only-development-contract.md), not private
game-specific importers or mesh generators.
See the [upstream task register](14-qualiadb-format-and-tooling-upstream-tasks.md)
for the pack, asset, save, and terrain integration gates.

## Australia first, world-ready packs

The first content and data-shaped packs target Australia. They may contain
Australian travel, shelter, climate, distance, community-ground, and
infrastructure themes, but must label fictional mechanics as game mechanics and
must not represent legal, planning, heritage, disaster, or welfare advice.

Each pack carries a `regionProfile` that composes universal game concepts with
regional expression. It defines locale, units, currencies, seasons, archetypes,
authoring/review metadata, source policies, and explicit rule-pack dependencies.
It does not fork the game core.

This makes later regional work additive. A European heritage/community pack can
provide châteaux, castles, farmsteads, or other historic/shared spaces as
dwelling and stewardship archetypes; assets, restoration projects, maintenance
constraints, governance, and narratives become its pack content. It must not
assume all European places share one heritage model, and it requires appropriate
local historical/cultural review before publication.

## Data categories

| Category | Game use | Minimum provenance |
|---|---|---|
| Boundaries, terrain, roads, paths | Map, travel and accessibility | Source, licence, version/date, coordinate reference system |
| Land use and public facilities | Locate opportunities and services | Source, licence, collection date, feature transformation |
| Civic/recreational grounds | Existing showgrounds, sporting grounds, halls, fields, amenities, lighting, parking, utilities | Source, feature condition, operator/stewardship assumption, schedule/access uncertainty |
| Transport and basic service access | Travel, route and project constraints | Source, freshness, coverage/uncertainty |
| Aggregated demographic indicators | Scenario framing and aggregate needs | Geography level, period, aggregation method, suppression/privacy note |
| Aggregate housing and infrastructure conditions | Fictionalised tenure pressure, local ties, and capability-gap framing | Geography level, period, source/licence, aggregation method, uncertainty, non-personal interpretation |
| Climate/disaster/resilience indicators | Seasonal conditions and project priorities | Source, period, model/uncertainty, non-advice disclaimer |
| Renewable potential and infrastructure | Energy project options | Source/method, assumptions, resolution, uncertainty |
| Hazard/resilience context | Scenario wildfire, flood, weather, access, or service-disruption conditions | Source/date/coverage, uncertainty, explicit game-scenario transformation, non-emergency label |
| Natural resources and existing assets | Water, land, shade, solar exposure, buildings, utilities, materials | Source, stewardship/condition, seasonal limits, licence/authority where relevant |
| Community-authored content | Projects, stories, agreements, local assets | Author/authority, consent, licence, review status |

## Privacy and ethical rules

- Use aggregate statistics only at a geography and population threshold that
  prevents singling out individuals or households.
- Do not ingest names, exact homes, personal journeys, protected attributes,
  case records, emergency-service records, or individually identifying data.
- Do not infer a person’s need, vulnerability, identity, or behaviour from a
  location or demographic statistic.
- Do not ingest or infer individual rents, debt, contracts, platform charges,
  tenancy status, benefit status, or housing eligibility. These remain fictional
  player-selected scenario facts, never public-data conclusions.

## Statistical scenario models

An approved pack may transform eligible aggregate public data into a bounded
`ScenarioAnalysisModel` used by QualiaDB computation. Deterministic logic,
algebra, constraints, and other scientific calculations, alongside probability
and statistical analysis, derive scenario conditions. Every model declares
whether it is deterministic, probabilistic, or hybrid and why that method fits
its game question. Modelled conditions can include demand, availability, travel
delay, resource capacity, dependencies, or a pack-defined barrier. The model
must not profile, rank, predict, or infer an individual, household, address, or
real community's need, behaviour, rights, safety, or eligibility.

The pack stores the input snapshot/digest, source and licence, geography and
aggregation level, suppression threshold, variables, transformation,
deterministic rules/calculation method, uncertainty where applicable,
applicability scope, and model version. It also declares which game parameters
it can affect and the causal assumptions required to compare a baseline with a
project-implemented scenario. Browser play uses this reviewed, offline model and
records its deterministic result or seed/sample in the event ledger; it never
queries a live feed or silently refreshes model inputs.

Further content boundaries:

- Keep real town data separate from fictional characters and narrative events.
- Clearly label every public-data scenario as a game interpretation, not advice
  or an official model of a community.
- Community-contributed material requires clear authority, consent, licence,
  attribution, amendment, and removal processes before publication.
- Player-created identity, family, language, skill, or circumstance profiles are
  not town-pack data. They stay local and are excluded from pack publication,
  analytics, evidence exports, and agent context unless a player makes a
  specific, revocable choice to include the minimum necessary field.

## Ingestion pipeline

```text
discover source
  → record licence and permissible use
  → acquire immutable source snapshot
  → validate schema, CRS, geometry, and aggregate/privacy boundary
  → transform into a documented game layer
  → create semantic Q42 entities and spatial relationships
  → generate/attach `.10d` visual assets where needed
  → run QA, provenance, licence, and scenario tests
  → publish a versioned pack manifest
```

No live public API is needed during play. A pack has a declared snapshot date;
updates are deliberate new versions that players can inspect and choose to
adopt. This protects offline play, replay determinism, and evidence quality.

## Online GIS acquisition boundary

The Australia-wide spatial hierarchy, OpenStreetMap layer responsibilities,
travel graph, ODbL handling, tile hierarchy, and QualiaDB dependency gates are
specified in the [Australia world atlas and OSM pack plan](29-australia-world-atlas-and-osm-pack.md).
That plan also records the current implementation gap: the existing three
fictional areas share one scene and do not yet constitute an Australian atlas
or an inter-region travel system.

The pack builder may retrieve appropriately licensed GIS/public data from online
sources during an explicit authoring/import step. It records the source URL or
identifier, licence, retrieval date, version/coverage, checksum where possible,
transformation, uncertainty, and import tool version before producing a local
snapshot. Browser play consumes the local pack, not an unreviewed live feed.

Online data is one evidence layer, not the world editor. Players can place,
repair, design, and build **game assets and improvements** over a scenario map.
Their proposals stay in the game world graph with separate player/project
provenance; they never overwrite source GIS records or imply approval to alter a
real location.

The supplied 3D/LLM architecture note suggests HTTP geodata converted to meshes
on demand in WASM. Treat this as a later QualiaDB capability investigation, not
the v0.1 ingestion plan. Before considering it, prove upstream source policy,
snapshot/replay semantics, coordinate and topology handling, tile seams,
bounded WASM/GPU memory, and offline behaviour. The first game scenes remain
reviewed, versioned, offline pack products.

## Player-made improvements

Town packs expose buildable game-space opportunities through semantic anchors:
vehicle/dwelling targets, facilities, ground nodes, routes, resources, and
capacity gaps. Players can use curated components and blueprints to create
repair/improvement projects, then evolve them through design, validation,
resourcing, construction, commissioning, and maintenance.

The pack declares compatible game anchors and component categories; logic
determines available simulated pathways. Every player-made result has a distinct
semantic ID, design version, contributor record, material/tool/skill pathway,
and visual mapping. It is labelled as a game-world proposal or outcome rather
than source GIS or real-world engineering advice.

## Spatial semantics

The graph must retain enough spatial meaning for questions that change play:

- distance/walking or vehicle access to services;
- whether a facility fits a site and is reachable;
- relationships between energy, water, workshop, storage, data, and community
  capacity;
- connections and mutual-aid routes to other community grounds;
- whether community-ground capability is centralised in one hub or distributed
  over accessible nodes, including travel and utility/resource-transfer limits;
- natural-resource condition, stewardship, seasonality, and safe/sustainable use;
- existing civic/recreation facilities, their schedules, utility/lighting loads,
  and explicit agreement/access constraints;
- scenario hazard/resilience context, changing access/condition, and the limits
  of any ground's temporary-support or recovery role;
- provenance and confidence of every spatial claim.

GeoSPARQL/QISP is the preferred query path where supported by the chosen WASM
profile. Queries must be tested against simple, documented geometries first.
The game uses stylised presentation and must not imply survey-grade precision.

## Content-authoring levels

1. **Australian real-place foundation:** a selected Australian locality with a
   versioned geographic snapshot and visible source/provenance. It grounds the
   playable community-development loop in places, routes, terrain, and facilities
   players can recognize. Fictional scenarios and characters may be layered over
   those public geographic facts; they must not be presented as facts about local
   residents or organisations.
2. **Additional Australian local packs:** more localities with distinct sourced
   geography, terrain, public facilities, routes, data coverage, and local style
   profiles. Each distinguishes atlas-only, data-shaped, and fully playable
   areas.
3. **Regional pack kit:** validated authoring templates, region profiles,
   QualiaDB ingestion/rendering tools, licence/privacy checklists, and content
   review gates. It allows players to start from their own community when data
   coverage and licensing support it.
4. **Fictional mechanics fixtures:** small authored places remain useful for
   deterministic tests, safe onboarding, and scenarios where no appropriate
   geographic pack exists. They are clearly labelled as fictional and do not
   substitute for the real-place gameplay goal.
5. **International packs:** separately reviewed regional packs reuse core
   semantic concepts while bringing their own archetypes, locale, policy
   assumptions, assets, and data-source rules.
6. **Network packs:** explicit shared agreements and capability exchange between
   independently authored communities.

## Acceptance criteria for the first data-shaped pack

- Every source feature/layer has a licence and provenance record.
- All demographic values are aggregate and pass documented disclosure checks.
- The pack loads offline, with its Q42 graph and visual assets resolving by
  versioned manifest.
- At least three gameplay decisions use spatial/semantic facts from the pack.
- The pack can model both a compact hub and a distributed community-ground
  network using the same node/resource/capacity schema.
- Evidence view can explain every displayed non-fictional statistic or layer.
- Scenario copy is reviewed so it does not make claims about identifiable people
  or present civic data as professional advice.
