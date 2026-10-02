# Rolling Commons

A browser-first WASM game built alongside [dev.civics.au](https://dev.civics.au)
and QualiaDB. It uses the local-first,
semantic, spatial, collaborative technology developed for QualiaDB to make the
civics concepts and practical considerations explored by dev.civics.au tangible
through computer-game-based gamification. This is deliberately distinct from
real-world gamification: the game explores fictional, bounded scenarios and
does not score, rank, incentivise, or govern real people or communities.

Development is constrained to the **QualiaDB/Webizen ecosystem**. Missing game
capabilities are added to QualiaDB and verified there before this project uses
them; the game does not introduce a separate engine or substitute technical
stack. See the [QualiaDB-only development contract](docs/planning/13-qualiadb-only-development-contract.md).

## What the game is about

Rolling Commons is an original game about the pursuit of making human rights
meaningful: whether people have the dignity, fairness, tenure, material means,
and lawful remedies needed to live with peace amid real-world harms, violence,
material pressures, institutional barriers, and the daily challenges to
human-rights principles. This is distinct from seeking “peace” or world peace
by controlling others. It is not a shooting game and does not turn violence into an
entertaining player power. Instead, it explores how people can build resilient
shared grounds: workshops, renewable energy, water, connectivity, local data
infrastructure, repair capacity, and shared projects. A mobile home is one
possible starting point, not a required or universal life path; content packs
can express different dwelling, place, and community contexts.

The game models societies and communities as arrangements that either do or do
not provide the practical mechanics for people to live with peace: secure enough
tenure, access to necessities and shared capacity, fair agreements, accountable
governance, and pathways to remedy harmful acts. Those arrangements express
values and preferences, shape what success means in a scenario, and affect how a
player proceeds. A player may begin with very little, including a tent, or with
other resources and ties; no starting condition is a measure of worth.

The game treats the capacity to practise civics as something people make
together. It explores how resources, place, maintenance, skills, access
agreements, governance, care, ecological limits, and the systems that interfere
with dignity shape a community project. It does not present real-world
financial, legal, engineering, medical, emergency, or service-eligibility advice.

## How it works

The authoritative game world is a local QualiaDB semantic graph plus a
QualiaDB-provided deterministic Rust/WASM simulation capability. Players inspect places and resources, travel,
learn, repair, exchange materials, propose projects, offer help, and contribute
to projects. Each action passes through explicit schema, permission, safety, and
rule validation before it can change the world. The game can therefore explain
why an action succeeded, was held, or needs another pathway; the event ledger
also makes a scenario replayable.

Community grounds are networks of sites and partners, not simply places where
everyone lives. People can participate without living at a ground, having site
access, or serving on a committee. They can bring forward project ideas, endorse
them, and offer skills, time, materials, hosting, or funds. Ideas become
operational projects only through the game's explicit validation and agreement
flow.

Each person has a private, scenario-only wallet. Community funds are held in
committee-owned treasuries, with earmarks, approval policies, and an auditable
posting ledger. Contributing to a project or treasury does not itself grant
committee membership, site access, or authority to spend shared funds.

The planning set is the current source of truth:

- [Decision register](docs/planning/00-decision-register.md)
- [Product vision](docs/planning/01-product-vision.md)
- [First-playable spec and open questions](docs/planning/12-first-playable-spec-and-open-questions.md)
- [QualiaDB-only development contract](docs/planning/13-qualiadb-only-development-contract.md)
- [QualiaDB format and tooling upstream tasks](docs/planning/14-qualiadb-format-and-tooling-upstream-tasks.md)
- [Living QualiaDB capability ledger](docs/planning/15-living-qualiadb-capability-ledger.md)
- [Technical architecture](docs/planning/02-technical-architecture.md)
- [GIS, public-data, and content-pack plan](docs/planning/03-place-data-and-content.md)
- [Narrative onboarding and life chapters](docs/planning/05-narrative-onboarding.md)
- [Logic-driven rules and authoring](docs/planning/06-logic-rules-and-authoring.md)
- [Engineering and community economics](docs/planning/07-engineering-and-economics.md)
- [Trading posts, parts, and logistics](docs/planning/08-trading-posts-and-logistics.md)
- [Wellbeing, health boundaries, and reflection](docs/planning/09-wellbeing-and-health-boundaries.md)
- [Community-ground scope and service boundaries](docs/planning/10-community-ground-scope.md)
- [Structural economics and SDG-aligned infrastructure](docs/planning/11-structural-economics-and-sdg-infrastructure.md)
- [Implementation plan](docs/planning/04-implementation-plan.md)

For a quick orientation, read the product vision, then the first-playable spec.
The latter marks proposed gameplay details and the decisions still needed before
implementation. The decision register separates accepted direction from open
questions; the remaining documents provide domain and delivery detail.
