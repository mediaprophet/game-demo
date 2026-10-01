# Rolling Commons

A browser-first WASM game built alongside [dev.civics.au](https://dev.civics.au)
and QualiaDB. It uses the local-first,
semantic, spatial, collaborative technology developed for QualiaDB to make the
civics concepts and practical considerations explored by dev.civics.au tangible
through computer-game-based gamification. This is deliberately distinct from
real-world gamification: the game explores fictional, bounded scenarios and
does not score, rank, incentivise, or govern real people or communities.

## What the game is about

Rolling Commons is an original, peaceful simulation about rebuilding a viable
life while helping create resilient shared grounds: workshops, renewable energy,
water, connectivity, local data infrastructure, repair capacity, and shared
projects. A mobile home is one possible starting point, not a required or
universal life path; content packs can express different dwelling, place, and
community contexts.

The game treats civic capability as something people make together. It explores
how resources, place, maintenance, skills, access agreements, governance, care,
and ecological limits shape a community project. It does not present real-world
financial, legal, engineering, medical, emergency, or service-eligibility advice.

## How it works

The authoritative game world is a local QualiaDB semantic graph plus a
deterministic Rust/WASM simulation. Players inspect places and resources, travel,
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
