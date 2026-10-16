# Maslows Challenge — Interactive SDG Simulation & Systems Runtime

A browser-first WASM application built alongside [dev.civics.au](https://dev.civics.au)
and QualiaDB. It uses the local-first, semantic, spatial, collaborative technology
developed for QualiaDB to make the civics concepts, practical considerations, and
Sustainable Development Goals (SDGs) tangible through embodied systems exploration.
This is deliberately distinct from real-world gamification: the environment explores
bounded scenarios and does not score, rank, incentivise, or govern real people or
communities.

While presented as an accessible exploratory scenario, the underlying QualiaDB
engine is an epistemic spatial runtime that unifies five distinct modes of inquiry:
- **Fiction & Narrative:** Speculative worldbuilding, protagonist paths, traits, and consequential story branches.
- **Non-Fiction & Field Sciences:** Authentic empirical datasets (Atlas of Living Australia / GBIF biodiversity observations, Australian ecological surveys, true geographic coordinates).
- **Simulation & Digital Twins:** Physical engineering dynamics (radiator thermal dissipation, solar generation curves, fluid transport, off-grid battery balances).
- **Hypothesis & Counterfactual Modeling:** Rigorous modal logic ("what-if" inquiries, deontic permissions/obligations, LTL temporal contracts, paraconsistent quarantine).
- **Civics & Cooperative Governance:** Bilateral micro-commons, stewardship of shared resources, and mutual agreements without scoring or ranking.

Development is constrained to the **QualiaDB/Webizen ecosystem**. Missing capabilities
are added to QualiaDB and verified there before this project uses them; the shell does
not introduce a separate engine or substitute technical stack. See the
[QualiaDB-only development contract](docs/planning/13-qualiadb-only-development-contract.md).
The browser shell builds `qualia-core-db` with `wasm-full` and uses `QualiaPortal` for
volumetric 3D rendering. WebCivics is a separate profile; it is not the engine or its licence.

The [Talbingo camping drive prototype](web/talbingo.html) contains six distinct,
stylized campsite scenes, a visible camper, site-specific road bends and events,
an outbound drive, on-foot landmark exploration and camp choice, and a return drive.
Serve `web/` locally and open `talbingo.html` to play it. Its road and camp layouts
are illustrative and must not be used for navigation. The
[Talbingo campground gameplay plan](docs/planning/31-talbingo-campgrounds-and-drivable-site-maps.md)
covers the later source-accurate place pack. The main campaign remains the fictional
Kestrel/Saltwind fixture.

## The game now

The **Walk** view includes an in-game character roster and creator. Protagonists
have personal backgrounds, story paths, field traits, and separate starting
regions and saved positions. Their chapters follow the shared campaign. See the
[playable character and first-person plan](docs/planning/23-playable-characters-and-first-person.md)
for the current slice, its limits, and the QualiaDB capabilities needed to
expand it. The [character behaviour and logic mechanics plan](docs/planning/24-character-behaviour-and-logic-mechanics.md)
sets the next simulation direction: linked but distinct consequences for
people, community governance, and whole-project delivery, evaluated through
the QualiaDB logic suite.

Maslows Challenge opens with a practical choice: spend the town's twelve coins
to move quickly, or spend workdays salvaging and repairing so cash remains
for later. Rest restores one day's labour. Powering the workshop unlocks one
paid repair order, so the player can recover funds for the final project.
The workshop can also reconnect the communications mast by spending a crew
shift or three coins. That link unlocks one remote repair contract worth four
coins. The mast changes visibly when commissioned, and the contract joins the
validated action and replay record.
Saved version-four campaign worlds acquire the offline mast when loaded, so
existing progress can continue with the new orders.
Kestrel Flats leads across the canal to Saltwind Reach. The player repairs a
crossing, commissions a pump and plants an orchard. The canal rises after day
four: a quick bridge repair is no longer possible, so a late player must
spend an extra crew shift on bracing or pay a specialist crew two coins. The
water level and bridge bracing are authored Qualia `.10d` scene states. The
orchard then offers a one-time choice: reserve its harvest to reduce the cost
of a commons gathering, or sell it for three coins toward the town's next
project. Qualia validates the choice and the orchard visibly changes to packed
produce crates. The player then chooses an ending:
open a local commons with a harvest gathering, or outfit a trade boat for
neighbouring settlements. The two endings have different requirements and
consequences in the saved, replayable world. The interface shows a field
dispatch, project progress, relevant orders, day count and a result screen.
Selecting a coloured world marker now focuses its site and opens contextual
orders there. The in-game dispatch highlights its next suggested order and can
locate that order's site. QualiaDB rules validate every order from either path.
Accepted and blocked orders focus the affected site, keeping visible model
changes and rule explanations beside the player's decision.
Completing either ending opens an in-game campaign record with the outcome,
project count, replay verification, and a new-campaign control.

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
Generated Storybook, Earthlight and Community Grounds HMC packs each contain
578 distinct `.10d` assets across 40 world-state, touring-rig and Australian
ecology-profile combinations. Opening-scene fixtures contain 477–487 assets,
depending on the selected touring rig. Packs include four touring rigs and
profile-selected Australian fauna and flora; the broad sample corridor remains
illustrative until its regional ecology and geography are sourced and reviewed.
The connected scene uses
QualiaPortal's camera target and daylight sky APIs. State variants live in
the game-owned catalog. Rounded assets use Qualia's computational
geometry authoring and parametric CAD; all scene assets retain source
provenance. Layered terrain, rounded meadows and shrubs, canal banks and
ripples, orchard rows, and market and building accents give the two territories
clearer shapes and colour at strategy zoom. The workshop, hall and barn have
separately coloured roofs and structural trim, with a rounded flue and
curved barrel canopies over the two market stalls. The camp is a rounded
home-on-wheels pod with chassis, wheels, hitch, awning and a shelter-upgrade
annex, parked in a bay under a long tilted solar shade structure with
slope-mounted workshop panels, while residents gather on the deck beside
their home. These remain composition
blockouts in material; finished materials, animation and canonical production HMC tooling
remain upstream gates.

The in-game System panel offers three visual styles. Storybook keeps the
bright original palette; Earthlight supplies a warmer, less saturated `.10d`
surface reading and Qualia sky; Community Grounds applies the muted sage,
olive, warm tan, terracotta and cream palette sampled from the civics.au
community-ground concept illustration, with object-family colours and soft
warm daylight. The game exports a separate HMC for each style and
remembers the player's presentation preference independently of campaign saves.
The selectable style system is a foundation for the fuller animated-film art
direction described in [the visual plan](docs/planning/22-animated-film-visual-direction.md).

The botanical pass adds pale, branching gum trees with blue-green crowns and
hanging foliage, flowering wattle, a herb border, planted vegetable leaves and
produce, and a distinct lavender patch. The planting is authored in the game
repository and compiled and rendered by Qualia; it remains stylized geometry
rather than botanically detailed foliage.

The touring drive now loads a checked VibeScript module from
`web/fixtures/drive-simulation.vibe`. That module owns steering and speed
response, dawn/dusk and night hazard range, safe and emergency stops, thermal
load, oil/coolant heat, engine damage, following-traffic buildup, and mirror
visibility. The browser supplies control intent and samples terrain, road and
selected-place fauna; it presents the state returned by the WASM-hosted
VibeScript runtime. Existing UI action IDs are also wrapped as VibeScript
command cells before the QualiaDB SHACL action gate applies them.

The Australian planting set now includes full-height genus-level eucalypt and
Acacia silhouettes beside the existing smaller species groups. Their placement
remains part of the illustrative profile, not evidence that all plants occur
together at one real locality. The workshop shell is now assembled from its
rear, side and front walls around a usable interior, with a framed window,
entry step, ventilation, service panel, conduit and downpipe rather than a
single solid block.

The four-tile build passes both campaign endings and replay
self-tests. The full Qualia WebGPU canvas now renders on the published game;
[QG-12](docs/planning/19-qualiadb-upstream-gate-work-orders.md) records the
earlier renderer gate and its reproduction.

The in-game Qualia HUD offers Map, Survey, Walk, Direct Drive and Activities
modes. Walk uses WASD and drag-look; Direct Drive adds throttle, reverse/braking
and steering for the selected campervan, 4WD/caravan, car/camper-trailer or
4WD/roof-top-tent rig. Its HUD button or **V** key switches between first
person and an aerial route-follow view on the local demo road. Its Kestrel–Saltwind route and goat/horse road-hazard
actors are local illustrative fixtures, not a real-world route or
geolocated occurrence feed. The first ecology profile is the broad Noosa–
Melbourne coast corridor with inland coverage toward the South Australian
border; regional data remains separately scoped. See the [ecology data
contract](docs/art/australian-ecology-data-contract.md) and [Australia route
plan](docs/planning/29-australia-world-atlas-and-osm-pack.md).
Campervan, caravan and 4WD cabin shells use rounded Qualia parametric geometry;
the roof-top tent has a gabled silhouette, with glazing, chassis, wheels and
roof loads authored as separate assets.

The illustrative plant collection now has bottlebrush, banksia, grass-tree,
saltbush and kangaroo-paw models. Place ecology profiles select stable flora
and fauna keys; the corridor showcase does not claim these groups co-occur at
one locality.

Direct Drive shows separate road-hazard and simulated engine-temperature
alerts, rising/cooling oil and coolant gauges, ambient temperature, road grade,
surface, engine load and engine condition. Towing rigs build a traffic queue;
the simulated rear mirror is partly blocked by the trailer and reveals more of
the queue around the authored bends. Sustained full load in hot weather or on
climbs can overheat and damage the simulated engine. Ten repeatable review
cameras and a 30-second, read-only showcase route are defined in
`web/fixtures/graphics/review-cameras.json`. The route tours the hub, workshop,
garden, touring rig, water crossing, highlands, local aerial drive, the fictional
place graph, resident choice mode and workshop repair mode; Escape or **End showcase**
returns to the original camera without changing the campaign.
`?graphics-view=<view-id>` uses the seed world, disables save/load and campaign edits,
and loads a named read-only review state.

The Activities panel contains versioned resident-conversation, fruit-harvest,
workshop-repair, shared-meal and fictional housekeeping-practice modules.
Choices use the existing Qualia campaign gates and event tape, so their state
replays with the rest of the game. The scenario contract and fixture are in
`docs/schemas/community-scenario-module.v1.schema.json` and
`web/fixtures/scenarios/community-grounds-v1.json`; the housekeeping vignette
does not represent a real employer or venue.

The Places HUD page reads a versioned, normalized place graph and opens the
campaign's distinct site targets. Its Kestrel/Saltwind entries are explicitly
fictional and schematic. The game-side package contract is
`docs/schemas/community-place-pack.v1.schema.json`; its offline structural and
render-artifact integrity validator is
`scripts/validate-community-place-pack.mjs`. The current fixture contains no
Australian boundary or route data. Once Observatory exports are ready, reviewed
QualiaDB adapters can produce packs for any coverage and connect locality,
town/site, route, terrain, ecology and render-tile data without freezing the
producer's RDF/Q42 or enhancement syntax.
Validate a package with `node scripts/validate-community-place-pack.mjs <unpacked-directory-or-place-pack.json>`.

An illustrated Australian loader appears on initial online entry and while
switching visual styles. It reports preparation/download/validation stages,
shows actual pack-byte progress when available, respects reduced motion, and
keeps a retry path visible after fatal startup failure. Observatory's future
Q42/RDF, Qualia Logic and VibeScript exports have a provisional manifest and
integrity-validation scaffold; no unfinished source syntax is interpreted.

The terrain mesh comes from Qualia's geospatial heightfield builder. Camera
position and ground height are presentation state and do not change the
campaign or replay.

Check out QualiaDB beside this repository at the commit in [`qualia.ref`](qualia.ref).
Build with `scripts/build-game.ps1` to produce the full Qualia WASM package in
`web/pkg/`, the game's `.10d` asset snapshots in `assets/generated/`, and a
Qualia QBDL/HMC pack in `web/assets/maslows-challenge-scenes.hmc`.
Both outputs live in this game repository. The game owns its asset recipes,
scene states and exported containers; QualiaDB owns the generic compiler and
renderer. Serve `web/` with a local HTTP server and open its root URL. `web/game.html?selftest`
runs the scripted action, scene, VibeScript, and replay checks; `web/spike.html`
retains the earlier capability spike.

The [game Pages workflow](.github/workflows/game-pages.yml) checks out the
pinned full QualiaDB engine as a sibling, builds the WASM game shell, exports
and verifies the game-owned `.10d`/HMC pack, and deploys `web/` from `main`.
GitHub Pages must use **GitHub Actions** as its build source for that deployment.

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

- [Community Grounds graphics overhaul — comprehensive implementation handoff](docs/planning/35-community-grounds-graphics-overhaul.md), using the supplied Community Grounds illustration as the primary visual target
- [Australian ecology and roadside fauna data contract](docs/art/australian-ecology-data-contract.md), including the first Noosa–Melbourne/inland South Australian corridor profile and the public-data import path
- [Decision register](docs/planning/00-decision-register.md)
- [Product vision](docs/planning/01-product-vision.md)
- [First-playable spec and open questions](docs/planning/12-first-playable-spec-and-open-questions.md)
- [QualiaDB-only development contract](docs/planning/13-qualiadb-only-development-contract.md)
- [QualiaDB format and tooling upstream tasks](docs/planning/14-qualiadb-format-and-tooling-upstream-tasks.md)
- [Living QualiaDB capability ledger](docs/planning/15-living-qualiadb-capability-ledger.md)
- [RTS and AAA-quality uplift blueprint](docs/planning/17-rts-aaa-uplift-blueprint.md)
- [Asset production catalog](docs/planning/18-asset-production-catalog.md)
- [QualiaDB upstream gate work orders](docs/planning/19-qualiadb-upstream-gate-work-orders.md)
- [AAA graphics and asset engine gap register](docs/planning/31-aaa-graphics-engine-gap-register.md)
- [Current game build and QualiaDB needs](docs/planning/20-current-game-build-and-qualiadb-needs.md)
- [QualiaDB refactor build gates](docs/planning/21-refactor-build-gates.md)
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
