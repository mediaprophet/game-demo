# Maslows Challenge, an SDGs game

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
The playable browser shell builds `qualia-core-db` with `wasm-full` and uses
`QualiaPortal` for 3D rendering. WebCivics is a separate profile; it is not
the game's engine or its licence.

## The game now

Maslows Challenge opens with a practical choice: spend the town's twelve coins
to move quickly, or spend workdays salvaging and repairing so cash remains
for later. Rest restores one day's labour. Powering the workshop unlocks one
paid repair order, so the player can recover funds for the final project.
Kestrel Flats leads across the canal to Saltwind Reach. The player repairs a
crossing, commissions a pump and plants an orchard. The canal rises after day
four: a quick bridge repair is no longer possible, so a late player must
spend an extra crew shift on bracing or pay a specialist crew two coins. The
water level and bridge bracing are authored Qualia `.10d` scene states. The
player then chooses an ending:
open a local commons with a harvest gathering, or outfit a trade boat for
neighbouring settlements. The two endings have different requirements and
consequences in the saved, replayable world. The interface shows a field
dispatch, project progress, relevant orders, day count and a result screen.

The Kestrel Flats scene compiles original town geometry through QualiaDB
computational geometry, provenance-bearing `.10d` containers, and the full
QualiaPortal WebGPU path. Players can buy or salvage solar parts, install them,
seek committee endorsement, power the workshop, improve shelter, save and
reload through QualiaDB OPFS, replay actions, and inspect semantic scene nodes.
They can repair the shared water tank with coins or labour, then plant the
garden once water is available.
The page also exposes VibeScript cells for live world snapshot queries,
SHACL-gated order proposals, and a bounded scene asset that compiles through
Qualia computational geometry to a provenance-bearing `.10d` mesh.
Kestrel Flats has 91 always-present original `.10d` scene organs; the new
Saltwind Reach tile adds 24 more plus state variants. The connected scene uses
QualiaPortal's camera target and daylight sky APIs. State variants live in
the game-owned catalog. Rounded assets use Qualia's computational
geometry authoring and parametric CAD; all scene assets retain source
provenance. These are detailed composition blockouts; finished materials,
animation and HMC packs remain upstream gates.

The current two-territory build loads 115 Qualia `.10d` meshes and passes both
campaign endings and replay self-tests. Browser visual acceptance is **blocked**: the full
Qualia WebGPU canvas remains black even with one accepted mesh. The exact
reproduction and generic Qualia renderer fix are recorded in
[QG-12](docs/planning/19-qualiadb-upstream-gate-work-orders.md); this repository
keeps using the full Qualia engine and treats QualiaDB as read-only.

Build with `scripts/build-game.ps1` to produce the full Qualia WASM package in
`web/pkg/`, the game's `.10d` asset snapshots in `assets/generated/`, and a
Qualia QBDL/HMC pack in `web/assets/maslows-challenge-scenes.hmc`.
Both outputs live in this game repository. The game owns its asset recipes,
scene states and exported containers; QualiaDB owns the generic compiler and
renderer. Serve `web/` with a local HTTP server and open its root URL. `web/game.html?selftest`
runs the scripted action, scene, VibeScript, and replay checks; `web/spike.html`
retains the earlier capability spike.

The current action session still holds N3 text in the game shell. A mutable
Q42 graph session, game-scoped VibeScript capabilities, HMC packaging, and
animated `.10d` assets are tracked QualiaDB upstream tasks in the
[capability ledger](docs/planning/15-living-qualiadb-capability-ledger.md).

The production direction is a real-time strategy game with the scale and
control clarity associated with Age of Empires, while retaining Rolling
Commons' original setting and rules. The current level is still early and the
renderer gate prevents its art from being judged in play. See the [RTS and AAA-quality uplift blueprint](docs/planning/17-rts-aaa-uplift-blueprint.md)
for the gameplay, art, interaction, QualiaDB upgrade, and delivery gates.

## What the game is about

Maslows Challenge is an original game about the pursuit of making human rights
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

The target authoritative game world is a local QualiaDB semantic graph plus a
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
- [RTS and AAA-quality uplift blueprint](docs/planning/17-rts-aaa-uplift-blueprint.md)
- [Asset production catalog](docs/planning/18-asset-production-catalog.md)
- [QualiaDB upstream gate work orders](docs/planning/19-qualiadb-upstream-gate-work-orders.md)
- [Current game build and QualiaDB needs](docs/planning/20-current-game-build-and-qualiadb-needs.md)
- QualiaDB platform completion brief (sibling repository:
  `docs/work-in-progress/qualia-capability-demonstration-program.md`)
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
