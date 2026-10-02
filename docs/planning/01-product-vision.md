# Rolling Commons: Product Vision

## Purpose

**Rolling Commons** is an original, browser-first WASM game that makes the
capabilities of QualiaDB visible through play. It is about the pursuit of making
human rights meaningful: whether people have the dignity, fairness, tenure,
material means, and lawful remedies needed to live with peace amid harms,
violence, material pressures, institutional barriers, and the daily challenges
to human-rights principles. This is distinct from seeking “peace” or world peace
by controlling others. It is not a shooting game and does not make violence an
entertaining player power; it instead explores how people participate in the
growth of resilient, connected communities.

The game represents societies and communities as arrangements that either do or
do not provide the mechanics needed for someone to live with peace: secure enough
tenure, access to necessities and shared capacity, fair agreements, accountable
governance, and pathways to remedy harmful acts. Those arrangements reveal
values and preferences, shape scenario success, and affect play without ranking
people by their starting circumstances. A player may begin with very little,
including a tent, or with other resources and ties; neither is a measure of
human worth.

The game draws on elements of games its author played decades ago: the
life-development aspects of *Jones in the Fast Lane*, and the game-like repair
of vehicles—campers, caravans, and related equipment—reminiscent of features in
*Street Rod*. Those influences inform broad interaction goals only. Rolling
Commons has its own setting, writing, characters, art, and rules.

## Player promise

Starting with a situation such as a tent, an unreliable car, or a basic van, a
player can create a safe, maintainable home-on-wheels and contribute to a
community ground. The player gains agency through repair, learning, cooperation,
and useful infrastructure rather than through combat, extraction, or a simple
wealth race.

The game must make these questions enjoyable to answer:

- What can I safely do next, and why?
- What would make this vehicle a viable home?
- What can this community build together that nobody could build alone?
- Which nearby communities can share skills, energy, designs, and capacity?

## Player-defined people and circumstances

Before play, people can create a character and life situation that feels like
their own: name/presentation, age or life stage, gender, heritage/cultural
information, languages, areas of expertise, skills, and what support or
accessibility they want represented. Every identity field is optional,
self-described, locally stored by default, and editable. The game provides
respectful defaults without forcing disclosure or a fixed taxonomy.

Players also define who is travelling or living together: alone, with friends,
a partner, a household, children, or another chosen travelling group. Group
members have their own agency and needs; they are never inventory, passive
bonuses, or plot devices. A player can model children using an age band/life
stage rather than a real birth date, and can choose how much family detail the
scenario represents.

Players can also choose an optional **LifeChapter** through guided choices,
branching, templates, free writing, or a mix. It remains editable and removable;
see [Narrative onboarding and LifeChapters](05-narrative-onboarding.md).

Identity is not an attribute score. Gender, heritage, language, age, and family
composition must never secretly reduce a person's worth, authority, skill
ceiling, access to shelter, or chance of success. They can shape chosen dialogue
language, accessibility, cultural context, care/time planning, relationships,
and player-authored story—but only through explicit, inspectable mechanics.

Players can also select private wellbeing/accessibility preferences—such as calm
presentation, pace, content warnings, rest/time planning, or a low-sensory
environment—without disclosing a diagnosis or personal history. The game can
offer fictional planning alternatives but is not therapy or health advice; see
[Wellbeing, health boundaries, and reflection](09-wellbeing-and-health-boundaries.md).

### Skills, learning, and mutual aid

The game models skills as a visible capability graph: a player or group member
can self-identify an area of knowledge and a level such as **new to it**,
**developing**, **practised**, or **expert**. Where a task needs evidence or
formal authority, that is recorded separately from self-assessed experience.

Projects can therefore be solved in several dignified ways: learn through a
course or mentorship, invite/contract a qualified person, collaborate with a
travelling companion, borrow community capability, or adapt the plan. The game
should reward teaching, documentation, and sharing a skill—not only possessing
one already.

## Design principles

1. **Dignity before optimisation.** Starting conditions describe a real need,
   never a failure state or a spectacle of hardship. Every start has multiple
   viable paths forward.
2. **Reality with care.** Explicit actions, resource limits, maintenance,
   relationships, stress, and circumstances have understandable consequences;
   the game responds with accountability, repair, boundaries, and support—not
   shame, violence, coercion, or a claim that people are reducible to a score.
3. **The graph decides; the LLM describes.** Rules and simulation state are
   authoritative. Generated language may explain or propose; it never changes
   the world by itself.
4. **A place is evidence-bearing.** Real-place content is aggregated,
   provenance-bearing, licensed, and inspectable. No real person becomes a game
   entity.
5. **Cooperation is a primary mechanic.** Shared projects are not decoration;
   they create meaningful capability that improves individual and network play.
6. **Inspectable complexity.** A player can open an explanation panel to see
   the facts, requirements, rule result, and event history behind an outcome.
7. **Local-first by default.** A complete core scenario runs offline in the
   browser once installed. Networked and LLM features are optional enhancements.

## Regional focus and portable design

The first release is designed for **Australian places and play**. Its first
journeys foreground long-distance travel, tent and vehicle-supported living,
caravans/vans/buses, repair spaces, solar and storage, variable service access,
and community grounds. Australian public-data and fictional/data-shaped packs
are the first content target.

This focus must not become a hard-coded universal life path. The core game is
about making a viable dwelling and contributing to shared civic capability. A
region chooses how that is expressed. For example, a future European pack might
centre the adaptive reuse and shared stewardship of a château, castle, farmstead,
or other heritage/community estate rather than vehicle-based living. Other packs
can centre apartments, post-disaster repair, rural compounds, or different
locally meaningful forms of dwelling and common space.

Every regional pack must be original, respectful of local context, and authored
with appropriate historical, cultural, legal, accessibility, and community
review. The game must not reduce a continent or culture to a single aesthetic.

## Game pillars

### Presentation profiles: one world, several ways to inhabit it

Presentation is deliberately separate from simulation. A player can select a
profile without changing the semantic world, rules, progression, save, place
pack, or agent permissions. Profiles may change camera, navigation, visual
language, animation, sound, UI composition, dialogue delivery, and level of
spatial detail.

Initial profile directions are:

| Direction | Player experience | Original treatment requirement |
|---|---|---|
| Storybook 3D adventure | Warm third-person or isometric exploration, expressive characters, tactile repair/build scenes | May evoke a friendly folklore-adventure feeling, but uses original art direction, world, interfaces, characters, and writing. |
| Illustrated point-and-click | Composed scenes, readable object interactions, rich dialogue, playful visual metaphor | May use the clarity and comic timing associated with classic adventure games, but must not imitate any particular game's art, puzzles, UI, text, or characters. |
| RPG-oriented environment | Spatial party/community view, map exploration, configurable HUD, project/status readability | Uses original systems and visual language; it is not required to adopt combat as a primary interaction. |

The v0.1 slice ships one polished profile and validates the profile contract with
one deliberately small alternate view. Multiple full art productions are not a
prerequisite for the first playable release.

### 1. A viable dwelling, made by the player and community

In the Australian v0.1, vehicles are practical systems rather than score
containers. A vehicle or shelter has condition, safety, maintainability, power,
water, storage, thermal comfort, accessibility, and legal/site suitability. A
repair bay visualises and installs upgrades: tyres, drivetrain work, insulation,
solar, batteries, water, communications, furniture, and storage.

The Australian progression can move from tent to car-supported living,
camper/van, caravan, bus, or a carefully maintained custom conversion. Different
paths are valid; a larger vehicle is not inherently a better outcome. Future
regional packs map this same dwelling capability model to their own assets,
upgrade paths, custodianship, and constraints.

Demountable/container-style homes are another important Australian dwelling path.
They can support a person or household that has local ties but needs a stable,
movable, incrementally improvable home after a relationship/household change,
disaster, or other housing loss. In-game decisions include suitable site access,
transport/placement, shared services, accessibility, thermal comfort, space for
children/dependants where modelled, maintenance, stewardship, and a pathway from
temporary to more stable living. No dwelling type is treated as a lesser life.

### 2. Community grounds

Community grounds are shared, buildable places. Their possible facilities
include a repair workshop, kitchen, learning space, energy canopy and battery
bank, water system, gardens, communications node, and containerised local data
centre. Facilities supply capabilities, carry maintenance needs, and unlock
projects for people and neighbouring communities.

A community ground is not required to be a single parcel of land. It may be a
compact hub with all facilities together, a distributed network of sites across
a town, or an early-stage arrangement that shares existing local spaces. One
community might begin with a creek/water source, open land, a donated shed, and
borrowed tools; another might have a workshop in one location, gardens elsewhere,
solar/storage at a public building, and a data/communications container near a
network connection. The game represents these choices as different, valid
topologies with real trade-offs in access, travel, resilience, maintenance, and
shared capacity.

Existing showgrounds, sporting grounds, and similar civic/recreational spaces
can be useful ground nodes. They may already offer fields, halls, sheds, kitchens,
toilets, parking, water, lighting, power, storage, access routes, and community
schedules. In play, these are not assumed to be freely available: use depends on
pack-defined stewardship/access agreements, opening times, existing activities,
capacity, maintenance, and community consent.

Depending on a scenario's site/hazard context and explicit agreements, grounds
can also serve community preparedness, temporary support, or recovery roles
during/after a bushfire, flood, or other disruption. Their usefulness depends on
simulated access, condition, water, power, communications, amenities, shelter
capacity, transport, existing commitments, and changing hazard conditions—not a
generic claim that any ground is safe or suitable.

Community grounds are voluntary mobility-centred settings, not universal housing
or care provision. They are a material infrastructure configuration within a
wider housing, care, and community network. Where a ground cannot meet the
current configuration of obligations, local ties, and practical needs, the game
makes that limit explicit and opens another arrangement—not abandonment or a
judgement of the person. See [Structural Economics and SDG-Aligned
Infrastructure](11-structural-economics-and-sdg-infrastructure.md) and
[Community-Ground Scope and Service Boundaries](10-community-ground-scope.md).

### 3. Make, repair, and improve

Players and communities can do more than select a finished facility. They can
diagnose an asset, choose a repair, assemble a component from materials, adapt a
vehicle or dwelling, and plan/build an improvement for a ground node. A project
has a visible design, components, required capabilities, resource inputs, safety
checks, contributors, construction stages, maintenance needs, and resulting
capacity.

The maker loop supports original, game-scale parts and improvements—such as a
storage module, insulation fit-out, water capture, solar canopy, accessible ramp,
tool bench, garden bed, communications relay, or data-container fit-out. It
rewards repair, reuse, teaching, documentation, and collaborative making. It is
a simulation and learning experience, not engineering, electrical, structural,
planning, or safety instruction for real-world construction.

The Trading Post gives that loop a social and geographic story: discover new and
second-hand parts, vehicles, campers/vans/buses/caravans/4WDs, camping equipment,
solar blankets/panels, batteries, and services on different community boards;
then collect, exchange, repair, or arrange simulated fulfilment to a suitable
receiving point. See [Trading Posts, Parts, and Logistics](08-trading-posts-and-logistics.md).

### 4. Connected commons

Communities exchange designs, skills, materials, compute/data services, mutual
aid, and project outcomes. The game represents cooperation as explicit,
inspectable relationships rather than an anonymous global resource pool.

### 5. Place-aware scenarios

Fictional or public-data-grounded town packs give places different needs and
opportunities: transport access, services, land use, weather exposure, housing
pressure, renewable potential, and nearby community capacity. Packs use only
appropriately licensed, aggregate, non-sensitive information.

## Core play loop

```text
Need or opportunity
  → inspect site, vehicle, skills, resources, and relationships
  → choose an activity or propose a shared project
  → repair, learn, travel, negotiate, build, or maintain
  → validate constraints and apply a deterministic simulation turn
  → record provenance-bearing outcome
  → unlock capacity for the person, ground, or network
```

## First playable scenario

The vertical slice is a single small community ground in a fictionalised,
data-shaped Australian town. It contains a tent site, repair bay, shared solar
canopy, community room, data-container pad, and six nearby locations.

The player works with four NPC roles:

| Role | Purpose |
|---|---|
| Mechanic | Repairs, safety checks, vehicle suitability |
| Mentor | Training and skills pathways |
| Site steward | Space, shared resources, and project governance |
| Travelling neighbour | Mutual aid and cross-community connections |

The slice proves three goals: obtain safe shelter, complete a basic
certification, and secure a viable work/contribution pathway. It also proves one
shared build: extend the solar and battery system so the workshop gains useful
new capability.

These goals describe the current vision, not yet a fixed win condition or
session-length target. The [first-playable spec](12-first-playable-spec-and-open-questions.md)
provides an example journey and identifies the choices still needed to make the
opening session testable. The player must be able to defer a goal, recover from
a blocked action, and understand what changed without consulting developer docs.

## Visible QualiaDB demonstration moments

- Selecting a building displays its `.10d` visual asset and Q42-linked purpose,
  capacity, maintenance state, and project history.
- Attempting an activity opens **Why?**, showing the relevant graph facts,
  constraints, and validated rule result.
- An NPC proposes a project or dialogue response from bounded graph context;
  the UI shows what was permitted, accepted, or rejected.
- The timeline replays prior turns from the local event ledger.
- Switching to evidence view overlays GIS layers and identifies the version,
  licence, source, transformation, and uncertainty of town-pack data.
- A player can inspect, edit, or remove confirmed LifeChapter facts that made a
  relevant opportunity or planning consideration appear.

## Non-goals for the first release

- Nationwide or global simulation coverage.
- Real-time multiplayer, cryptocurrency, or a social network.
- Replacing professional housing, disaster, financial, legal, engineering, or
  welfare advice.
- Mod execution with arbitrary filesystem, network, or JavaScript/V8 access.
- Treating local demographic data as an identity or surveillance system.

## Measures of success

The project succeeds when a player can enjoy the vertical slice offline and a
technical evaluator can independently observe that the game uses a WASM
simulation, semantic graph/query/logic, Q42 persistence, semantic visual assets,
a bounded NPC proposal path, VibeScript, and content provenance in meaningful
gameplay rather than as passive integrations. Local model inference and public
place-data evidence join that demonstration only after their capability,
privacy, and licence gates pass.
