# Rolling Commons: Place Data and Content Packs

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
  scenario/*.vibe               bounded scenario scripts
  presentation/*                text, icons, audio, accessible labels
```

The final package layout will follow verified QualiaDB packaging conventions;
this structure is an implementation target, not a new platform format.

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

1. **Australian core pack:** original fictional Australian place; no external
   data required. It proves regional mechanics and provides reliable tests.
2. **Australian data-shaped demonstration pack:** one small public-data-derived locality,
   with a minimal curated layer set and full provenance/evidence view.
3. **Regional pack kit:** validated authoring templates, region profiles, import tooling,
   licence/privacy checklists, and content review gates.
4. **International packs:** separately reviewed regional packs that reuse core
   semantic concepts while bringing their own archetypes, locale, policy
   assumptions, assets, and data-source rules.
5. **Network packs:** explicit shared agreements and capability exchange between
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
