# Rolling Commons: Technical Architecture

## Architectural position

Rolling Commons is a WASM-first application. Rust owns deterministic simulation
and all authoritative state transitions. QualiaDB provides the semantic graph,
query, logical validation, Q42 persistence, local inference surfaces, geometry
and rendering integration. The browser UI presents an attractive game, a 3D
world, and optional inspection tools.

The exact QualiaDB feature profile must be verified in an early browser spike.
QualiaDB is pre-1.0; APIs and binary formats must be pinned to a tested release
or commit for every game build.

## Authority boundary

```text
Browser UI + renderer input
          │ player choice / script request / agent proposal
          ▼
Capability gateway ──► schema + policy + intent validation
          │ accepted command only
          ▼
Rust/WASM simulation reducer ◄── QualiaDB graph query + N3Logic/SHACL
          │ deterministic event and state delta
          ▼
Q42 world/event ledger ──► UI, renderer, replay, explanation view

Graph-bounded LLMs and VibeScript may propose commands. They never bypass the
gateway or invoke the reducer directly.
```

## Runtime components

| Component | Responsibility | Authority |
|---|---|---|
| Web UI shell | Menus, HUD, activity selection, explanation and evidence panels | Presentation only |
| `webizen-render` integration | 3D world, camera, selection, animation, lighting, overlays | Presentation only |
| Rust/WASM game core | Turn/tick loop, resources, travel, projects, event reduction, replay | Authoritative |
| Qualia graph | World facts, relationships, requirements, provenance, query results | Authoritative facts |
| N3Logic + SHACL | Rules, permissions, validation, explanations | Authoritative validation |
| Q42 volumes | Versioned local world, content, and event persistence | Durable state |
| `.10d` assets | Dense geometry/visual payloads for places, vehicles, facilities, props | Render payload |
| VibeScript host | Bounded authored scenario and behaviour content | Proposal only |
| Local LLM orchestrator | Grounded dialogue, event prose, candidate actions | Proposal only |

## Presentation profiles

The simulation exposes a stable, presentation-neutral projection API: semantic
entity IDs, names/localised labels, state facets, available actions, inspection
data, spatial anchors, and event/replay deltas. A presentation profile consumes
that projection and maps it into a visual/navigation language. It must not own
or reinterpret authoritative game state.

```text
Q42 world + reducer + rules
          │ stable world projection
          ├── storybook-3d profile      → 3D scene, camera, animation, HUD
          ├── illustrated-adventure     → scenes, hotspots, dialogue layout
          └── rpg-environment profile   → map, entity/status panels, controls
```

A profile declares its supported content/renderer version, required asset
representations, accessibility settings, input mappings, localisation resources,
and performance tier. It may supply presentation-only assets and mappings, but
every selectable object/action must resolve to the same semantic ID and command
schema across profiles. Save files persist gameplay state and selected-profile
preference separately so a save remains portable.

The first profile should use the renderer path verified in Phase 0. The
illustrated-adventure view is a valuable low-spec and accessibility fallback; it
can use semantic hotspots and artwork without requiring 3D GPU features.

## Semantic model

The initial ontology is intentionally small and game-specific. It should extend
only when an implemented rule requires it.

| Domain | Key entities | Key relationships |
|---|---|---|
| Person | Player, NPC, role, skill, need, preference | hasSkill, holdsRole, needs, mayUse |
| Life situation | Household/party, relationship, dependant/care arrangement, travel group | memberOf, travelsWith, caresFor, sharesResourceWith |
| Community participation | Community, participant, interest, project idea, contribution, endorsement | participatesIn, isResidentOf, proposes, endorses, helpsWith |
| Structural context | Housing transition, tenure obligation, recurring cost, local tie, infrastructure gap | hasObligation, createsPressure, hasLocalTie, hasInfrastructureGap |
| Mobile home | Shelter, vehicle, subsystem, upgrade, condition | installedOn, requires, improves, unsafeWhen |
| Place | Town, ground network, node/site, facility, zone, route | locatedAt, hasNode, serves, contains, connectedTo |
| Resource | Natural resource, land, water, shade, solar exposure, material, ecosystem asset | availableAt, hasCondition, isRenewable, requiresStewardship |
| Project | Blueprint, task, material, contribution, outcome | requires, contributesTo, produces, maintainedBy |
| Capacity | Energy, water, tools, space, connectivity, compute | supplies, consumes, reserves, availableAt, transferredVia |
| Governance | Committee, agreement, permission, policy, responsibility | permits, prohibits, accountableTo, expiresAt |
| Economic custody | Wallet, treasury, fund, account entry, spending approval | heldBy, controls, funds, debits, credits, requiresApproval |
| Service boundary | Participation profile, exceptional arrangement, support scope, alternate configuration | hasGroundCapabilityMatch, hasExceptionPath, requiresDifferentConfiguration, hasServiceBoundary |
| Event | Turn, action, validation result, provenance receipt | causedBy, derivedFrom, supersedes, occurredAt |

The core ontology uses regional-neutral concepts such as `Dwelling`,
`ShelterCapability`, `MobilityCapability`, `PlaceStewardship`, `CommunityGround`,
and `SharedFacility`. An Australian pack may specialise a dwelling as a tent,
camper, caravan, van, bus, or vehicle conversion. A future European pack may
specialise it as a heritage dwelling/project within a château, castle, farmstead,
or other shared estate. UI labels, upgrade trees, asset types, rules, climate
assumptions, tenure models, and local policy belong to a declared region profile
and content pack, not to global Rust enums or hard-coded strings.

Australian packs include demountable/container-style homes as a `Dwelling`
specialisation, distinct from both mobile vehicles and permanent buildings. A
demountable home records its footprint/visual asset, transport/placement state,
site connection needs, capacity, accessibility, condition, maintenance, and
household/party suitability. It can transition between temporary and established
scenario states through explicit project, agreement, and service-capacity facts.

For example, a workshop activity is enabled by facts and rules rather than a
hard-coded UI condition: the facility must be open and safe; the player needs
permission; required tools/materials must exist; and a qualified mentor may be
needed for some upgrades.

### Community-ground topology

`CommunityGround` is a semantic network, not a synonym for one building or one
land parcel. It contains one or more `GroundNode` sites linked by routes,
utility/resource connections, stewardship arrangements, and capacity exchanges.
Nodes may contain natural-resource areas, existing buildings, temporary
facilities, or constructed infrastructure.

| Topology | Example | Gameplay consequence |
|---|---|---|
| Integrated hub | Workshop, kitchen, solar, water, garden, data node in one site | Short travel and shared maintenance; one site failure can affect more services. |
| Distributed grounds | Workshop, garden, energy/storage, and data node on separate sites | Uses existing community assets; requires routes, coordination, and transfer capacity. |
| Federated local partners | Facilities are hosted by different community organisations | Expands capability with explicit agreements and availability constraints. |
| Existing civic/recreation node | Showground, sporting ground, hall, or shared venue with existing utilities/facilities | Reuses assets such as lighting, water, buildings, parking, kitchens, and fields; availability is schedule/agreement-bound. |
| Early-stage network | Natural resources, borrowed rooms/tools, temporary shelter and mobile services | Low capital start with incremental projects and practical limits. |

Each node records access, ownership/stewardship, availability, resource condition,
infrastructure level, connections, dependencies, maintenance, and provenance.
Existing civic/recreation nodes additionally record their facilities, utility
loads, lighting, schedule, host activities, and relevant agreement/consent scope.
The renderer can show either a single hub or a town-scale network while retaining
the same semantic IDs and activity rules.

### Communities and participation

A `Community` is the social network associated with one or more community
grounds, partner nodes, or a wider locality. It is not a residency register or
a governance body. A `CommunityParticipation` record links a person, party, or
partner organisation to that community with a player-controlled participation
mode—such as neighbour, visitor, contributor, collaborator, supporter, or
observer—and an optional consented visibility scope. It is independent of
`residesAt`/dwelling location, ground access agreements, committee membership,
and treasury authority. A person can therefore help a project while living
elsewhere, have no ground access, or participate without taking a governance
role.

`ProjectIdea` is a lightweight, revisable proposal: it has an originator,
intent, optional target/community, requested help, and privacy/visibility
scope, but has no resource, access, budget, or approval effect. An idea may be
endorsed, discussed, or receive offers of skills, time, materials, hosting, or
funding. Only an explicit transition into the existing validated `Project`
workflow can reserve capacity, move wallet/treasury value, or change the world.
Contributions record what was offered or completed and its consent/provenance;
helping does not confer committee membership, project ownership, ground access,
or treasury spending authority.

### Disruption and recovery role

A `GroundNode` may have a scenario-specific `ResilienceRole`: ordinary community
use, preparedness resource, temporary-support node, recovery hub, or unavailable.
Its role is derived from stated scenario conditions, hazard context, site access,
facility condition, capacity, stewardship/access agreement, and rules—not from a
generic map label. A disruption changes available actions and resource priorities
but preserves event provenance and player/household dignity.

The model can simulate a need for shelter, communications, power, water,
amenities, transport, care/time planning, and recovery projects. It must not
represent a live warning service, evacuation instruction, emergency assessment,
or assertion that any real location is safe.

### Participation and service boundary

`ParticipationProfile` records a scenario's transparent mobility, stewardship,
agreement, and operational-capacity requirements. It is evaluated alongside
player-selected structural pressures, obligations, local ties, and the ground's
actual capabilities—not as a classification of a person. It produces standard
participation, an exceptional/support arrangement, or a different-configuration
result for that specific ground. `ServiceBoundary` records what the ground does
not claim to provide and the available fictional alternative configuration.
Neither record may derive a result
from a health condition, disability, trauma, body size, criminal record, or raw
real-world licence status. See [Community-Ground Scope and Service Boundaries](10-community-ground-scope.md).

`SafetyScenario` is a versioned content record, not live sensor/triage logic. It
links a fictional hazard context to region, source/version, assumptions,
communication/access constraints, resource availability, available simulated
actions, learning outcome, and uncertainty. The UI must show its source and
simulation status. Agents may explain only source-grounded pack content and are
not permitted to interpret a player's real symptoms or produce emergency advice.
See [Wellbeing, health boundaries, and reflection](09-wellbeing-and-health-boundaries.md).

### Maker and improvement system

An in-game `Project` moves through explicit states: `idea`, `surveyed`,
`designed`, `validated`, `resourced`, `inProgress`, `commissioned`,
`maintained`, `paused`, or `retired`. An improvement can target a vehicle,
dwelling, facility, ground node, route, utility connection, or shared resource.

```text
player/community intent
  → choose or compose a game component/blueprint
  → semantic design proposal (parts, location, capacity, dependencies)
  → logic/shape/safety validation and alternate pathways
  → acquire, repair, reuse, fabricate, learn, or collaborate
  → staged construction events
  → commissioned asset + capacity/maintenance graph updates
  → renderer shows the built result and project provenance
```

Blueprints and components are data-driven semantic assets. They declare compatible
targets, materials, tools, capabilities, footprint/visual variants, effects,
maintenance, and disallowed combinations. The initial system uses curated,
game-safe component libraries; later content authoring can compose only bounded
parts/predicates and requires validation before a new blueprint affects a save.

GIS observations can inform a **site proposal**—for example access, distance,
terrain class, or renewable potential—but they never authoritatively approve a
real construction. Game logic treats them as dated, uncertain scenario evidence.

### Trading posts and fulfilment

`TradeBoard`, `Listing`, `TradeItem`, `Fulfilment`, `ReceivingPoint`, and
`TransactionRecord` are semantic entities linked to inventory and projects. A
listing can be new, second-hand, surplus, swap, donation, loan, or service. It
declares condition, compatibility, location, availability, value terms, and
provenance. Fulfilment is a separately validated simulated path: collection,
community transfer, freight, or abstract international shipping.

A remote or overseas item cannot arrive merely because it is selected. The
reducer requires a valid receiving point, storage/access capacity, route/time
result, agreement where applicable, and eventual inspection/acceptance before
the item enters inventory or affects a project. See
[Trading Posts, Parts, and Logistics](08-trading-posts-and-logistics.md).

### Wallets, committees, and treasuries

Every `Person` has one private game `Wallet`; a party may also have an explicitly
consented shared wallet. Wallets hold only pack-defined scenario value and
exchange-credit balances. They are not real payment accounts and their owner is
the only default viewer and spending authority.

A `Committee` is a governance record with members, roles, term/appointment
evidence, and an explicit decision policy. A committee, rather than an
individual person, owns each community `Treasury`. A treasury can contain
earmarked funds (for example operations, solar maintenance, or a project), but
cannot be treated as a member's personal balance. `TreasuryEntry` records a
balanced debit/credit posting, source command, purpose, counterparty role,
approval evidence, and event/provenance reference. Balances are projections of
entries; the reducer never mutates a balance directly.

Commands that move scenario value name a source wallet or treasury and a
destination wallet, treasury, project, or listing settlement. Spending from a
treasury requires the committee policy to validate the amount, fund purpose,
and authorised approver(s); contributions into it do not grant unilateral
withdrawal rights. The projection exposes a person's own wallet and only the
treasury detail permitted by its visibility policy, while the event ledger
retains the complete auditable simulation record.

`VehicleOrDwelling` additionally carries a pack-defined `MobilityState` and
readiness evidence separate from ownership. `CamperYard` is a specialised
`GroundNode` with bays, sheds, tools, storage, service listings, booking, and
fee/value terms. The reducer applies only simulated scenario-road/site-access
rules after a validated readiness project; it never models or asserts real
registration, inspection, or legal roadworthiness.

## Identity, life situation, and capability model

Model a person as three separate, linked records so the game does not confuse
who somebody is with what they currently can do or what they need.

| Record | Examples | Mechanical boundary |
|---|---|---|
| Self-described identity | Name/presentation, age or life stage, gender, heritage/cultural information, languages, accessibility preferences | Optional local profile data. Never a hidden modifier to worth, capability ceiling, access, or probability. |
| Life situation | Solo traveller, friends, partner, household, children/dependants by age band, shared dwelling, support/care arrangement | Explicitly selected planning constraints/preferences: seats, beds, time, safety, care, consent, and resource needs. Never a penalty for existing. |
| Capability and authority | Knowledge area, self-assessed level, practice history, qualification/credential where relevant, teaching willingness | Can meet task requirements only when rules explicitly require it; self-assessment and formal evidence remain distinct. |

The semantic model needs `Person`, `Party`/`Household`, `Membership`,
`Community`, `CommunityParticipation`, `ProjectIdea`, `Contribution`,
`CareArrangement`, `LanguageCapability`, `KnowledgeArea`, `SkillAssessment`,
`Qualification`, `TeachingOffer`, `LearningPath`, and `TaskRequirement`.
Membership has a role, consent/status, start/end, and sharing preferences. Child
characters use game age bands/life stages; the game never requires real names,
birth dates, or personal records.

Task rules query a capability requirement, not identity. For example,
`InstallSolar` may require an expert mentor, a qualified installer, a supervised
learning path, or an approved external service. A group may meet the requirement
through any consenting member or community offer. The explanation response must
identify the required capability and available pathways, never imply that an
identity attribute caused the result.

All personal profile facts are private local graph data by default. They are
excluded from public town packs, evidence exports, agent context, and future
network sharing unless the player explicitly scopes and consents to a specific
use.

`LifeChapter` is a separate private, versioned graph record linked to confirmed
person/party/capability references. Its optional scenario hooks validate against
a fixed schema and capability surface; they cannot emit arbitrary ontology or
rule changes. See [narrative onboarding](05-narrative-onboarding.md).

`WellbeingPreference` is an optional private record for accessibility, calm-mode,
pace, content-warning, and player-chosen care/time constraints. It is separate
from simulation authority and is excluded from agents/exports by default. Rules
may offer alternative task paths from an explicit preference but cannot derive a
diagnosis, condition, treatment, safety assessment, or personal health outcome.
See [Wellbeing, health boundaries, and reflection](09-wellbeing-and-health-boundaries.md).

`SupportScenario` is a separate, player-confirmed private graph record for an
abstract support/boundary/recovery pathway. It can expose transparent simulated
effects on time, resources, trust, routine, access, or alternative task paths.
It cannot model a diagnosis, assign blame, score body weight, prescribe care,
store a real safety plan, or make violence/abuse/substance use into player powers.

## State and persistence

Separate data by mutability and authority:

- **Content Q42:** curated, versioned town packs, ontology terms, standard
  assets, activity definitions, and validated rules.
- **World Q42:** the current scenario state: people, assets, capacities,
  agreements, and projects.
- **Event Q42:** append-oriented turn outcomes, validation receipts, and replay
  checkpoints.
- **`.10d` assets:** dense terrain/geometry/visual data. Q42 manifests reference
  them by content hash and describe units, frame, semantics, provenance, and
  permissions.

Saves must include game build/version, content-pack versions and digests,
ontology/rule version, random seed, and turn number. A save is rejected or
migrated explicitly when incompatible; it is never silently reinterpreted.

## Simulation model

Start with deterministic turns, not a 60 Hz authoritative world simulation.
The renderer may animate at display rate, but game outcomes resolve through a
repeatable sequence:

1. Receive one typed command.
2. Read required graph facts and current state.
3. Validate schema, policy, SHACL shape, and N3Logic rules.
4. Compute the deterministic result from state plus recorded seed.
5. Produce an event and graph delta with source/provenance links.
6. Commit event and state only after all validation succeeds.
7. Reproject state to UI and renderer.

This provides offline repeatability, testable replays, explainable failure, and
a clean foundation for a later real-time visual layer.

## Logic-driven gameplay rules

The rule system is a versioned content layer over the semantic model. N3Logic
derives facts and pathways; SHACL validates entity/action integrity; deontic and
temporal logic express permissions, obligations, and timing. The reducer consumes
their validated result and records the applicable rule version with each event.

Rules are authored against a fixed ontology vocabulary and scope. They may
derive that an activity is allowed, held for review, prohibited, due, or offers
an alternative pathway. They cannot directly mutate state, access data outside
their declared graph scope, or create privileged capabilities. See
[Logic-driven rules and authoring](06-logic-rules-and-authoring.md).

## Engineering and economic simulation

Typed QualiaDB computation consumes scenario assumptions and Q42-linked facts to
produce unit-bearing, provenance-bearing energy, resource, and economic results.
The game records model/rule versions, assumptions, ranges, and uncertainty before
the reducer applies a result. Engineering/economic modules never bypass the
command gateway and do not claim real-world approval or advice. See
[Engineering and community economics](07-engineering-and-economics.md).

## Renderer and spatial assets

The first map is a stylised 3D/isometric neighbourhood, not a photorealistic
digital twin. Rendered objects need stable semantic IDs. Picking a vehicle,
building, solar canopy, or data container must resolve to its semantic manifest
and relevant world facts.

Asset pipeline:

```text
licensed source / original art
  → optimise and validate mesh/materials
  → package dense `.10d` asset
  → create Q42 manifest and content hash
  → map semantic entity to renderer instance
  → validate asset provenance and bounds in CI
```

An early spike must confirm browser support and practical bundle/memory budgets
for the selected `webizen-render` profile, `.10d` loading, picking, and fallback
rendering. A simple geometry/canvas fallback remains acceptable for development;
the core game must not depend on an unverified browser GPU feature.

Maker output uses semantic design/provenance records linked to original or
parameterised game visual assets. It does not turn arbitrary player geometry into
an executable or unsafe asset. A new built object receives a stable semantic ID,
state/provenance history, and an approved visual representation before it can
appear in the scene.

The art bible must specify original palettes, silhouettes, UI components,
typography, animation language, audio, dialogue, and interaction conventions.
Named games can inform high-level design discussion, but their protected assets,
characters, text, plots, UI layouts, and distinctive visual expression are never
used as implementation targets.

## Regionalisation and localisation

Every scenario declares a `regionProfile` and locale. A profile supplies:

- dwelling, mobility, tenure, and stewardship archetypes;
- climate/season, distance, travel, resource, and safety assumptions;
- jurisdiction-scoped policy/rule packs, with clear game-fiction boundaries;
- unit, currency, date/time, language, and accessibility presentation settings;
- cultural/historical content guidance and review metadata; and
- permitted data sources and relevant licence/privacy controls.

The game core must never infer legal rights, cultural permission, or a
real-world condition merely from `regionProfile`. A profile selects explicit
content and validated rules. Region-specific rule packs need isolated fixtures,
so adding a European heritage pack cannot alter Australian scenario results.

## Graph-bounded NPC agents

NPCs operate through a constrained protocol:

1. The game resolves an NPC role, objective, allowed capabilities, and a scoped
   read-only graph view.
2. A retrieval layer supplies only relevant, allowed, provenance-linked facts.
3. The LLM generates dialogue and a typed candidate action from a fixed schema.
4. The capability gateway validates intent, authority, deontic constraints,
   output limits, provenance/grounding, and action shape.
5. The deterministic reducer accepts or rejects the candidate action.
6. The result and its justification are stored as an event receipt.

Agent context uses only the minimum profile facets needed for the current task,
such as a preferred language or an explicitly modelled access requirement.
Heritage, gender, children/dependants, relationship status, and unneeded profile
fields are excluded by default. The UI must show the context categories sent to
an agent and allow the player to disable agent use entirely.

Health, trauma, wellbeing, care, and clinical fields are always excluded from
agent context in v0.1; no agent is authorised to provide health, therapeutic, or
crisis guidance.

Agents may only refer to an approved, player-visible support pathway when the
player has scoped that context. They must use non-stigmatising language and
cannot pressure disclosure, confrontation, consumption, treatment, or contact.

Generated dialogue must declare uncertainty or abstain when the graph cannot
support a claim. NPCs cannot invent facts, access unrelated private player
state, alter prices/rules, or commit changes without the validated command path.

LLM support is optional. Template-based dialogue and deterministic scenario
events are the baseline, so the game remains playable with no model, network,
or WebGPU inference capability.

## VibeScript role

VibeScript is the authored-content layer for scenarios, events, and bounded
agent behaviour. It calls an explicit game capability surface such as
`World.query`, `Project.propose`, `Narrative.offer`, and `Schedule.create`.
It does not receive raw storage access, arbitrary browser APIs, arbitrary
networking, or direct reducer access.

Rust implements core mechanics and deterministic, performance-critical loops.
Vibe scripts express content and orchestration. The project will benchmark the
actual selected WASM host path before making performance claims relative to
JavaScript or V8.

VibeScript can request an approved rule evaluation or propose a bounded scenario
hook, but it cannot evade N3/SHACL/deontic validation. Logic governs what a
scripted activity may do; scripts govern presentation and authored flow.

## Security and trust model

- Treat every content pack, GIS import, script, LLM response, and network input
  as untrusted until validated.
- Capability grants are narrow, revocable, and visible in developer/evidence
  mode.
- Personal play data remains local by default. Sharing is explicit and uses a
  separate future cooperation design.
- Public data is attributed and licence-checked; sensitive, individual-level,
  or restricted data is excluded from game packs.
- Mod packages are signed/digested and versioned before they can affect a save.

## Browser technical spikes (mandatory before production build)

1. Build/load a minimal Qualia core WASM profile in a blank browser app.
2. Create/query/persist a small Q42 world with the selected browser storage
   strategy and reload it without data loss.
3. Validate one SHACL/N3 rule and return an explanation payload to the UI.
4. Load one `.10d` asset, render it, select it, and resolve its Q42 manifest.
5. Run a minimal Vibe script through the intended browser host capability set.
6. Run one bounded local-inference call if the selected profile supports it;
   otherwise prove the no-LLM fallback.
7. Measure WASM size, first-load time, memory, frame time, and offline reload.
