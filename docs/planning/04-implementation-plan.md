# Maslows Challenge: Implementation Plan

## Delivery strategy

Build a playable, verifiable vertical slice before broad content, networking, or
advanced agent work. Each phase has an exit gate; later phases do not hide an
unproven core dependency.

The project owner's subsequent RTS direction is specified in the
[RTS and AAA-quality uplift blueprint](17-rts-aaa-uplift-blueprint.md). Its
Gates A-E supersede the earlier text-first presentation sequence for future
production. The phase tasks below remain a system inventory and source of
domain requirements, not a claim that the current browser slice is already an
RTS. When implementing an overlapping task, use the RTS control, simulation,
asset, quality, and QualiaDB dependency gates in that blueprint.

The [first-playable spec](12-first-playable-spec-and-open-questions.md) defines
the proposed player journey and the unresolved gameplay decisions. The phase
lists below include foundation fixtures, future-facing contracts, and research
spikes; they are not all promises of player-visible v0.1 content. Before Phase 1
expands, resolve D-029–D-034 and label each fixture **player-visible**,
**demonstration-only**, or **deferred** in a release matrix. This keeps the
first-playable loop small enough to test while preserving the longer plan.

The [QualiaDB-only development contract](13-qualiadb-only-development-contract.md)
is a dependency gate for every phase. If a required surface is missing from the
selected QualiaDB browser profile, add and verify it in QualiaDB before using
it here. A prototype or release cannot satisfy a gate with a private game-side
replacement.
This check repeats whenever implementation or playtesting reveals a new need;
the [capability ledger](15-living-qualiadb-capability-ledger.md) stays open, and
a blocked game task resumes after its QualiaDB fix is pinned and tested.

The [upstream task register](14-qualiadb-format-and-tooling-upstream-tasks.md)
names the format and host work needed for `.10d`, HMC/HCF, VibeScript, Q42, and
P64 integration. Its QG identifiers are dependencies, not parallel substitutes.

| Gate | Minimum evidence | Decision enabled |
|---|---|---|
| Interaction prototype | A player can inspect, choose between two routes, encounter an explained block, recover, improve one asset, and contribute to one shared project. | Whether the core loop is worth building out. |
| Technical vertical slice | The same journey runs offline through the validated WASM/graph/reducer path, persists, and replays; semantic selection and one bounded authored-content example work. | Whether the architecture supports the game as designed. |
| Public v0.1 demonstration | The selected presentation is readable and accessible on the declared baseline; required pack/data, privacy, licence, performance, and save gates pass. | Whether the slice is ready to share beyond the team. |

## Phase 0 — Repository and capability baseline

**Goal:** establish a reproducible project and prove the target browser/WASM
integration surface.

Tasks:

- Select the Webizen/QualiaDB application surface and pin the QualiaDB revision/release.
- Create a capability ledger for every required game surface: owning QualiaDB
  crate/API, target profile, pinned commit, verification test, status, and
  dependent game feature. Upstream partial/missing capabilities before use.
- Resolve QG-01's canonical HMC producer/reader and format version. Inspect the
  existing core bundle and semantic-library ZIP implementations and the HCF
  draft divergence; do not claim interchange without round-trip tests.
- Scope QG-03's `.10d`/Q42/Vibe/Webizen animation contract before committing to
  character animation or rigged asset production.
- Verify QG-04's VibeScript REPL/game host capabilities against the live catalog
  and QG-05's pack/save compatibility path.
- Prove one agent-authored original prop can travel through QualiaDB geometry,
  validation, `.10d` packaging, semantic manifest, browser rendering, and
  picking without an external modelling suite.
- Define supported browser baseline and development toolchain.
- Define a versioned presentation-profile contract and a presentation-neutral
  world-projection API before building the first UI.
- Create CI commands for format, lint, unit tests, WASM build, and asset checks.
- Implement the seven mandatory browser technical spikes in the architecture
  document.
- Record real size, load, memory, persistence, and rendering measurements.
- Create an Architecture Decision Record log for choices and rejected options.
- Define the versioned `regionProfile` contract and prove that an Australian
  fixture is selected through content configuration rather than hard-coded core
  logic.
- Define the private identity/life-situation schema, consent scopes, data
  minimisation rules, and agent-context exclusion tests.
- Define the private, versioned `LifeChapter` schema, field scopes, bounded
  scenario hooks, revision/deletion flow, and no-LLM onboarding path.
- Define the private `WellbeingPreference` schema, agent/export exclusion,
  calm-mode contract, and no-diagnosis/no-treatment boundaries.
- Define the versioned `SafetyScenario` pack contract: authoritative source,
  regional context, assumptions, uncertainty, content warning, simulated actions,
  and no-live-triage boundary.
- Define the private `SupportScenario` contract for transparent consequences,
  boundaries, nonviolent alternatives, user control, and no-stigma/no-score
  constraints.
- Define `ParticipationProfile` and `ServiceBoundary` contracts, including
  mobility-centred operational requirements, exceptions, scope limits, alternate
  fictional configurations, no-sensitive-proxy rules, and no real-service-
  eligibility claims.
- Define `HousingTransition`, `TenureObligation`, `AffordabilityPressure`,
  `LocalTie`, `InfrastructureGap`, and SDG-aligned outcome contracts; prohibit
  person-deficit scores and real-world entitlement or market claims.
- Define the initial predicate vocabulary, rule scopes, versioning policy, and
  N3/SHACL/deontic fixture format.
- Define typed-quantity, unit-validation, model-version, assumption, uncertainty,
  and provenance contracts for engineering and economic scenario calculations.
- Define `ScenarioAnalysisModel` contracts for approved aggregate public-data
  inputs, privacy/aggregation thresholds, deterministic, probabilistic, and
  hybrid methods, result or seed/sample recording, uncertainty, and declared
  game effects.

**Exit gate:** a browser page loads a small QualiaDB-backed WASM game shell,
reads/writes one test world, evaluates a graph rule, renders/selects a test
object, and reloads offline. The capability ledger links each demonstrated
function to a pinned, tested QualiaDB surface. Missing capabilities are added
upstream. A blocked required feature stays open until its capability is
integrated; Phase 0 does not close later discovery of gaps.

## Phase 1 — Authoritative semantic simulation

**Goal:** make one complete deterministic turn playable without 3D polish.

Tasks:

- Define the minimal game ontology and namespaces.
- Create Australian seed content for player, dwelling/shelter, vehicle, site,
  facilities, skills, activities, projects, capacities, and events.
- Implement `CommunityGround` network, `GroundNode`, natural-resource,
  facility/utility, route, transfer, and stewardship fixtures; prove both a
  compact hub and distributed topology use the same reducer/rule model.
- Implement a community-participation fixture separate from residency, ground
  access, and committee membership: an off-site participant proposes an idea,
  another participant offers help, and the idea enters a project only after
  ordinary validation; prove neither participant gains treasury authority.
- Add a showground or sporting-ground fixture with existing lighting/utilities,
  scheduled activities, and an explicit simulated access agreement.
- Add a demountable/container-home fixture with placement, shared-service,
  household-suitability, maintenance, and temporary-to-established project states.
- Implement the curated component/blueprint schema and project state machine for
  one vehicle/dwelling repair and one ground-node improvement.
- Implement the trade catalogue, boards, listing condition/compatibility, remote
  fulfilment, receiving point, inspection/acceptance, and transaction fixtures.
- Implement private person wallets, committee-owned treasuries, balanced value
  postings, earmarked funds, and policy-validated treasury spending; include
  rejected overdraft, missing-approval, and contributor-without-withdrawal
  fixtures with player-readable explanations.
- Implement a camper-yard node, mobility-state/readiness evidence, bay booking,
  repair project, paid/supervised service listing, and simulated road/site-access
  gate for one acquired unready vehicle.
- Implement person, party/household, life-stage, language, skills,
  qualifications, teaching offers, and task-requirement fixtures.
- Implement one player-selected housing/household transition with continuing
  obligations and local ties, plus a ground capability gap that can be reduced
  by a shared infrastructure project.
- Implement `RightsCondition`, `HarmOrBarrier`, `Safeguard`, and `RemedyPath`
  fixtures. Cover secure-enough tenure, access to a necessity, a fair-agreement
  dispute, an accountable response, and an alternate/support arrangement;
  persist their evidence, consent, capacity, timing, and event provenance.
- Keep universal dwelling, mobility, stewardship, and facility concepts distinct
  from Australian vehicle/camping specialisations.
- Implement typed commands: inspect, travel, repair, learn, use facility,
  propose an idea/project, endorse or offer help, contribute, raise a concern,
  request review, offer/accept/decline a remedy path, and end turn.
- Implement validation order: schema → permissions → SHACL → N3Logic → reducer.
- Implement curated rules for vehicle safety, workshop access, mentoring, and a
  shared-solar project, each with positive/negative/edge fixtures and a
  player-readable explanation trace.
- Prove a player-made improvement reaches commissioned state only through
  validated design, staged contributions, and a maintenance pathway.
- Implement a deterministic solar/battery/critical-load fixture and a separate
  community-ground/project operating ledger with explainable inputs and outputs.
- Implement a shared asset-condition/maintenance/fault fixture covering one
  vehicle/camper item, one energy asset, and one community-ground facility.
- Add one shared EV-charging utility fixture with booking/access, load priority,
  energy allocation, maintenance, and a deferred-charge explanation path.
- Add a vermiculture fixture: vegetable/organic-scrap supply, worm-farm capacity
  and care, amendment inventory, and distribution to one garden node.
- Define a cultivation-method contract and seed one soil/raised-bed node plus one
  abstract hydroponic node with distinct simulated inputs and maintenance paths.
- Define food-project provenance and evidence/claim-scope contracts, with a hard
  rule that scenario logic cannot make personalised or therapeutic food claims.
- Persist events/checkpoints and implement replay from a fixed seed.
- Build the first **Why?** explanation payload and tests for accepted/rejected
  actions.
- Build a dimensional living-with-peace projection for tenure, necessities,
  safety/participation, fair agreements, accountability, and remedy access.
  It must show evidence, unresolved barriers, and available actions without a
  composite score, hardship ranking, or claim about real-world legal status.
- Prove a task can be completed through learning, mentorship, a qualified
  service, or a consenting party/community member without identity-based gating.
- Prove the same material starting point has different transparent outcomes
  when a scenario does or does not provide a stated safeguard/remedy mechanism;
  prove an unavailable or inadequate remedy remains visible and does not force
  harm, consent, or control of another person to progress.

**Exit gate:** a scripted scenario replays to the same graph/event result on two
fresh runs; invalid actions are rejected with an understandable evidence chain.

## Phase 2 — First player journey and UI

**Goal:** make the vertical slice engaging in a conventional browser UI.

Tasks:

- Create activity, inventory/vehicle, time, needs, project, and timeline panels.
- Create the Trading Post board flow with New, Second-hand, Community Exchange,
  and Services sections; include collection/travel and remote-receiving choices.
- Create an inclusive character and life-situation builder with optional fields,
  custom labels, age bands for children/dependants, group membership, and clear
  local/privacy controls.
- Implement guided LifeChapter onboarding with skip/custom/edit/delete controls,
  then test that confirmed hooks remain deterministic and explainable.
- Implement calm mode, content-warning/skip controls, optional pace/rest-time
  planning, and private wellbeing-preference editing/deletion.
- Implement one reviewed fictional Australian remote/hazard scenario with
  source/provenance/evidence view, communication/access constraints, and an
  alternative neutral planning path.
- Implement consequence-and-support fixtures: one deferred/alternative project,
  one community-support path, and one private boundary/transition scenario with
  a neutral exit and no forced disclosure.
- Implement a mobility-centred participation fixture with standard,
  exceptional/support, and different-configuration results; each must display the rule,
  preserve privacy, and open a non-abandoning fictional alternative.
- Show a structural-economics explanation view for the transition fixture:
  continuing obligations, material constraints, ground capability, project
  contribution, assumptions, and no household ranking.
- Show the living-with-peace projection beside project and community views, with
  a clear distinction between fictional scenario mechanisms and real-world
  rights, legal remedies, or service advice.
- Build accessible keyboard-first navigation and responsive layouts.
- Implement the profile selector/persistence boundary and a minimal illustrated
  semantic-hotspot view for the same seed scenario using a Webizen surface.
- Author the initial three-goal journey and tutorial-free contextual guidance.
- Provide save slots, export/import only where safe, reset, and replay controls.
- Add offline/no-model status indicators and a developer evidence mode.

**Exit gate:** a new player can complete the agreed opening journey and shared-solar project
without reading developer documentation; core controls and explanations meet an
accessibility review.

## Phase 3 — 3D community ground

**Goal:** replace the abstract screen with a readable, performant 3D place.

Tasks:

- Build the stylised terrain and six-location community-ground scene.
- Render the compact hub and a distributed-node variant, including visible route,
  capacity, and utility/resource-connection states.
- Produce an original art bible for the selected storybook-3D or RPG-oriented
  v0.1 presentation direction; references guide only broad mood and interaction
  goals, never imitation.
- Establish `.10d` asset ingestion/validation plus Q42 semantic manifests.
- Load those assets from the selected HMC profile and verify Q42 identity/digest
  links. Add an animated asset only after QG-03 passes its upstream/browser gate.
- Render staged construction and completed variants for the first player-made
  repair and community improvement, linked to their project provenance.
- Render EV-charging availability and its relationship to the community energy
  system, without implying real-world electrical status.
- Link renderer picking to semantic entities and inspection panels.
- Visualise conditions: open/closed, capacity, maintenance, energy state,
  project progress, and accessible/blocked routes.
- Add animation, camera, and a verified Webizen low-spec presentation path.
- Set performance budgets and test on the selected browser baseline.

**Exit gate:** every interactive 3D object resolves to a semantic ID and its
facts; the same key entities/actions are reachable in the alternate profile; the
full first scene meets documented frame, memory, and load budgets.

## Phase 4 — Place data and data-shaped town pack

**Goal:** prove GIS/public-data → Q42 → game-world flow safely.

Tasks:

- Choose one openly licensed, small, aggregate-safe Australian data source set.
- Use or extend QualiaDB's acquisition and pack tooling to record source,
  licence, retrieval date, checksum/version, and transformation before compiling
  an offline pack.
- Implement source ledger, licence gate, CRS/geometry validation, and import
  transforms.
- Build a town-pack compiler producing the manifest, Q42 graph, metadata, and
  visual layer assets.
- Add spatial queries to three meaningful gameplay decisions.
- Implement one reviewed aggregate-public-data scenario analysis model and a
  baseline-versus-project comparison. Include deterministic and probabilistic
  analysis where each is appropriate to the stated question; show method,
  uncertainty, result or seed/sample, causal assumptions, and the validated game
  mechanisms responsible for every simulated shift.
- Model collection and distribution routes between food-scrap sources,
  vermiculture capacity, and garden/partner nodes.
- Connect scenario irradiance/site exposure assumptions to the solar/battery
  model, with explicit unit/source/uncertainty evidence in the UI.
- Add a topology decision comparing centralised and distributed infrastructure
  against access, travel, resilience, maintenance, and available resources.
- Add a reuse-versus-new-build decision for an existing civic/recreation node,
  accounting for facilities, lighting/utility load, schedule, access, and upkeep.
- Add a fictional disruption/recovery scenario that changes ground-node access,
  capacity, and critical-load priorities without presenting live emergency advice.
- Build evidence view, including uncertainty and non-advice labels.
- Complete privacy, disclosure, and copy review against the data-pack policy.

**Exit gate:** the data-shaped pack works offline and every displayed data claim
can be traced to source and transformation metadata.

## Phase 5 — Vibe-authored content

**Goal:** demonstrate safe, high-performance alternatives to embedded general
purpose scripting for scenario content.

Tasks:

- Define a minimal game Vibe capability manifest and typed request/response
  schemas.
- Implement the host adapter with capability checks and deterministic command
  proposal handling.
- Author one scenario chain and one facility/project rule in VibeScript.
- Add compile/diagnostic tests, execution limits, and audit receipts.
- Prove that Vibe requests rule evaluation through the gateway and cannot bypass
  N3/SHACL/deontic validation or mutate world state directly.
- Benchmark the selected WASM host against an equivalent baseline; publish only
  measured performance claims.

**Exit gate:** Vibe content can unlock/propose valid gameplay through approved
capabilities, and cannot access storage or mutate simulation state directly.

## Phase 6 — Graph-bounded NPC agents

**Goal:** add meaningful narrative without weakening game truth or offline play.

Tasks:

- Define agent roles, permitted graph scope, goals, dialogue schema, action
  schema, token/output limits, and refusal behaviour.
- Implement retrieval, grounding/provenance display, and gateway validation.
- Add deterministic template fallback for every agent interaction.
- Integrate one local browser inference profile only if Phase 0 confirms it is
  practical; otherwise retain an optional later profile.
- Red-team hallucinated facts, unauthorised actions, prompt injection in content,
  privacy leakage, and malformed action objects.

**Exit gate:** an NPC interaction visibly demonstrates a bounded proposal that
is accepted or rejected by graph/rule validation, while the same scenario remains
fully playable with the LLM disabled.

## Phase 7 — Pack authoring and connected communities

**Goal:** expand from one curated scenario to controlled, inspectable content.

Tasks:

- Publish content-pack schema, authoring template, validation CLI, and review
  checklist.
- Publish the `regionProfile` schema, localisation contract, and region-isolated
  fixture suite. Prove a non-Australian prototype can load a different dwelling
  archetype without modifying the core reducer.
- Support multiple grounds and explicit mutual-aid/project relationships.
- Design opt-in sharing/cooperation separately from local solo saves.
- Add migration and compatibility policy for game, pack, graph, asset, and script
  versions.

**Exit gate:** a second validated pack interoperates with the first using
documented, explicit capability agreements and passes compatibility tests.

## Workstreams and ownership boundaries

| Workstream | Main outputs | Depends on |
|---|---|---|
| QualiaDB simulation | Published commands, reducer, events, replay, tests in the ecosystem; game-specific schemas and fixtures here | Phase 0 capability ledger |
| Semantic/rules | Ontology, Q42 seeds, SHACL/N3, explanations | QualiaDB simulation surface |
| Rule authoring | Predicate vocabulary, rule packages, fixtures, inspector | Semantic model + gateway |
| Webizen UI | Player flow, accessibility, evidence view | QualiaDB projection/input API |
| Render/assets | Scene, `.10d`, picking, low-spec presentation | Phase 0 spike |
| Presentation | Profile contract, world projections, visual art bible, alternate view | QualiaDB projection/input API |
| Place data | Importer, packs, provenance/privacy gates | Semantic model |
| Engineering/economics | Typed models, units, energy/resource results, ledgers | Semantic model + gateway |
| Vibe | Capability manifest, host, authored scenarios | Gateway |
| Agents | Scoped retrieval, orchestration, fallbacks | Rules + gateway |
| QA/release | CI, benchmarks, save compatibility, licence checks | All workstreams |

## Test plan

- **Unit:** reducer arithmetic, command schemas, resource constraints, Vibe host
  boundary, pack transforms.
- **Property/replay:** same seed and command sequence always produce the same
  event/state digests; invalid input never commits partial state.
- **Semantic:** fixtures for SHACL/N3 eligibility, permissions, contradictions,
  explanations, and provenance.
- **Rules:** vocabulary/scope validation, positive/negative/edge cases, bounded
  execution, rule-version replay, and no identity-based proxy predicates.
- **Rights conditions/remedy:** supported/threatened/unavailable/restored state
  transitions; consent, evidence, response, delay, capacity, and accountable-
  role checks; visible unresolved-barrier paths; no composite peace/hardship
  score; and rejection of real-rights/legal-status claims or coercive progress
  conditions.
- **Engineering/economics:** unit compatibility, deterministic calculations,
  model/assumption provenance, uncertainty display, energy-balance and
  ledger-invariant fixtures, and circular-resource flow/capacity/route fixtures.
- **Scenario analysis models:** source/licence/digest, aggregation and
  suppression thresholds, deterministic/probabilistic/hybrid method/version,
  unit/scope validation, replayable calculated results and seeded sampling,
  uncertainty display, baseline/post-project comparison, and rejection of
  individual profiling, live-data refresh, or unsupported causal claims.
- **Condition/maintenance:** scheduled maintenance, known/unknown condition,
  fault observations, constrained operation, repair/defer/replace pathways, and
  deterministic provenance-bearing failure fixtures.
- **Food content:** ingredient/place/provenance traces, claim-scope/review gates,
  uncertainty display, and fixtures rejecting therapeutic/personalised claims.
- **WASM/browser:** offline reload, storage failure handling, bundle size, memory,
  WebGPU availability/fallback, input/accessibility.
- **Asset:** `.10d` manifest/digest linkage, bounds, unit/frame, semantic ID,
  licence/provenance, and player-made project-to-visual linkage.
- **Trade/logistics:** listing status/condition, compatibility, reservation,
  receiving-point, remote-fulfilment, inspection, and transaction-provenance
  fixtures.
- **Mobility/yards:** unready-vehicle storage, yard capacity/booking, repair and
  service pathways, readiness evidence, and scenario-road/site-access fixtures.
- **Data:** CRS, source attribution, aggregation/disclosure, licence policy,
  snapshot determinism, resource condition/stewardship, node/route topology, and
  clearly labelled scenario hazard/resilience transformation.
- **Agent safety:** context scoping, unsupported claims, action schema rejection,
  prompt injection, profile-data minimisation, privacy, no-LLM equivalence.
- **Narrative onboarding:** per-field confirmation, schema-bound extraction,
  no-sensitive-inference fixtures, template composition, revisions/deletion, and
  absence from default agent/export queries.
- **Wellbeing/privacy:** no-forced-disclosure flow, calm-mode accessibility,
  private reset/deletion, complete agent/export exclusion, and fixtures rejecting
  diagnosis/treatment/crisis outputs.
- **Support/agency:** transparent scenario consequences, nonviolent alternative
  paths, boundary/exit controls, no-stigma language checks, and rejection of
  hidden body/substance/trauma/abuse scoring.
- **Scope boundaries:** standard/exception/different-configuration fixture paths, explicit
  purpose/effect, no sensitive-field proxy, no automated referral, and no
  real-service eligibility/admission claim.
- **Safety scenarios:** source/version/provenance visibility, regional-context
  fixtures, uncertainty/content warnings, no-live-triage boundary, and rejection
  of ungrounded medical/emergency output.
- **Playtest:** clarity, dignity, player choice, readability, and whether the
  QualiaDB features are understandable without a technical briefing.

## Release definition: Demonstration v0.1

The first public demonstration is ready only when it has:

- a reproducible WASM build and offline single-scenario play;
- a complete first-player journey plus one shared community project;
- Q42-backed state/events, graph queries, and validated rules exposed through
  player-readable explanations;
- an interactive 3D ground with semantically linked visual assets;
- a provenance-visible fictional pack; an aggregate public-data pack joins the
  demonstration only after its licence, privacy, and transformation gates pass;
- one safe Vibe-authored scenario; and
- deterministic NPC dialogue and a validated proposal path, with local LLM
  inference only if the Phase 0 browser capability and privacy gates pass.

The release matrix records which supporting fixtures are visible in the public
journey. A technical evaluator may use evidence mode to inspect additional
fixtures, but a player should not need them to finish the session.

## Key risks and responses

| Risk | Response |
|---|---|
| Browser Qualia profile is too large or incomplete | Reproduce the limit, improve the QualiaDB profile or underlying capability, and remeasure; keep the dependent game task blocked until it passes |
| Renderer or `.10d` browser path is impractical | Fix and verify the QualiaDB/Webizen path; keep dependent visuals blocked until the ecosystem proof passes |
| Game feature lacks a QualiaDB capability at any phase | Record the block and upstream owner in the living ledger; implement/test the public surface in QualiaDB, pin it, then resume game integration |
| LLM claims exceed grounded facts | Enforce fixed action schema, visible evidence, gateway validation, and template fallback |
| Public data creates privacy/licence harm | Use the data policy, versioned snapshots, aggregate thresholds, and manual review gate |
| Identity becomes a hidden penalty or stereotype | Separate identity from capability; test task rules and explanations for explicit skill/authority requirements only |
| Family/care details are over-collected or exposed | Use optional age bands, local-only defaults, field-level consent, and strict agent/export exclusion |
| Background stories stereotype, pressure disclosure, or create hidden rules | Make LifeChapters optional/revisable; use player-confirmed hooks only and require sensitivity review plus explanation tests |
| Wellbeing feature becomes diagnostic, therapeutic, or a source of sensitive-data harm | Preference-only v0.1, privacy by default, no agent/export access, sensitivity review, and explicit non-clinical boundary |
| Reality modelling becomes stigma, punishment, or abuse simulation | Person-first review, explicit/inspectable consequences, support/exit alternatives, no violence/coercion mechanics, and no hidden sensitive scoring |
| Mobility-centred scope becomes discriminatory exclusion or is mistaken for formal service triage | Explicit purpose/exception/boundary model, no sensitive proxy, alternative path, independent review for real deployment, and no admission/eligibility claim |
| Housing pressure is represented as personal failure or a real-world allocation rule | Model tenure, recurring obligations, local ties, and capability gaps as player-selected structural context; prohibit person-deficit scores, entitlement claims, and individual data inference |
| Safety learning is removed, generic, or confused with live emergency triage | Region-aware source-linked scenarios, explicit assumptions/uncertainty, source review, and no symptom-based automated advice |
| Australian assumptions leak into global mechanics | Enforce region-neutral core entities, declared profiles, and an isolated non-Australian fixture |
| Community grounds are modelled as a single idealised site | Use the ground-network/node schema and test integrated, distributed, partner-hosted, and early-stage topologies |
| Scripts expand authority accidentally | Capability manifest, no direct storage/reducer access, limits, and tests |
| Pack rule is ambiguous, unsafe, or expensive | Fixed vocabulary/scope, SHACL/policy gate, bounded evaluation, fixtures, and explicit held result |
| GIS data is confused with player construction or real-world approval | Snapshot/authoring boundary, separate project provenance, simulation labels, and no real-world approval claims |
| Trade board is confused with a real marketplace or shipping service | Curated fictional listings, scenario-value/fulfilment boundary, no real payment/seller/shipping claims, and provenance labels |
| Game readiness is confused with actual registration or roadworthiness | Pack-defined simulated policy, no legal/inspection claims, and clear scenario-road/site-access labels |
| Scenario model is mistaken for real engineering or financial advice | Typed assumptions, uncertainty/provenance UI, abstraction limits, and clear non-advice boundary |
| Failures feel arbitrary or punitive | Tie events to visible scenario condition/maintenance evidence, explain uncertainty, and provide multiple repair/support pathways |
| Food content is mistaken for health or therapeutic guidance | Claim-scope/evidence/review gate, no personalisation or efficacy inference, and clear non-advice labels |
| Disaster scenario is mistaken for live emergency guidance or site safety | Fictional/snapshot scenario boundary, uncertainty labels, no warning/evacuation claims, and explicit ground-role constraints |
| Game feels like hardship optimisation | Dignity review, multiple viable starts, cooperation mechanics, and playtests with affected perspectives |

## Immediate next actions

1. Inventory QualiaDB/Webizen game-capable surfaces and open the Phase 0
   capability ledger; pin the candidate revision.
2. Select its UI/application surface and build the browser spike with a
   measurement log. Upstream any gap blocking the test journey.
3. Prove the QualiaDB-only original-prop → `.10d` → browser-selection path.
4. Write the first ontology/seed-content fixture and deterministic replay test.
5. Sketch the first community-ground scene, list original asset needs, and
   select the fictional core town before a public-data locality.
