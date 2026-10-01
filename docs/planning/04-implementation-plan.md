# Rolling Commons: Implementation Plan

## Delivery strategy

Build a playable, verifiable vertical slice before broad content, networking, or
advanced agent work. Each phase has an exit gate; later phases do not hide an
unproven core dependency.

## Phase 0 — Repository and capability baseline

**Goal:** establish a reproducible project and prove the target browser/WASM
integration surface.

Tasks:

- Select the Rust web application shell and pin the QualiaDB revision/release.
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

**Exit gate:** a browser page loads a small WASM game shell, reads/writes one
test world, evaluates a graph rule, renders/selects a test object, and reloads
offline. Unsupported QualiaDB features have a documented fallback or are removed
from the first slice.

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
- Keep universal dwelling, mobility, stewardship, and facility concepts distinct
  from Australian vehicle/camping specialisations.
- Implement typed commands: inspect, travel, repair, learn, use facility,
  propose an idea/project, endorse or offer help, contribute, and end turn.
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
- Prove a task can be completed through learning, mentorship, a qualified
  service, or a consenting party/community member without identity-based gating.

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
- Build accessible keyboard-first navigation and responsive layouts.
- Implement the profile selector/persistence boundary and a minimal illustrated
  semantic-hotspot fallback for the same seed scenario.
- Author the initial three-goal journey and tutorial-free contextual guidance.
- Provide save slots, export/import only where safe, reset, and replay controls.
- Add offline/no-model status indicators and a developer evidence mode.

**Exit gate:** a new player can complete the first week and shared-solar project
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
- Render staged construction and completed variants for the first player-made
  repair and community improvement, linked to their project provenance.
- Render EV-charging availability and its relationship to the community energy
  system, without implying real-world electrical status.
- Link renderer picking to semantic entities and inspection panels.
- Visualise conditions: open/closed, capacity, maintenance, energy state,
  project progress, and accessible/blocked routes.
- Add animation, camera, and low-spec renderer fallback.
- Set performance budgets and test on the selected browser baseline.

**Exit gate:** every interactive 3D object resolves to a semantic ID and its
facts; the same key entities/actions are reachable in the alternate profile; the
full first scene meets documented frame, memory, and load budgets.

## Phase 4 — Place data and data-shaped town pack

**Goal:** prove GIS/public-data → Q42 → game-world flow safely.

Tasks:

- Choose one openly licensed, small, aggregate-safe Australian data source set.
- Implement an explicit online-acquisition record (source, licence, retrieval
  date, checksum/version, transformation) that compiles to an offline pack.
- Implement source ledger, licence gate, CRS/geometry validation, and import
  transforms.
- Build a town-pack compiler producing the manifest, Q42 graph, metadata, and
  visual layer assets.
- Add spatial queries to three meaningful gameplay decisions.
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
| Game core | Commands, reducer, events, replay, tests | Phase 0 |
| Semantic/rules | Ontology, Q42 seeds, SHACL/N3, explanations | Game core |
| Rule authoring | Predicate vocabulary, rule packages, fixtures, inspector | Semantic model + gateway |
| Web UI | Player flow, accessibility, evidence view | Game core API |
| Render/assets | Scene, `.10d`, picking, fallback | Phase 0 spike |
| Presentation | Profile contract, world projections, visual art bible, alternate view | Game core API |
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
- **Engineering/economics:** unit compatibility, deterministic calculations,
  model/assumption provenance, uncertainty display, energy-balance and
  ledger-invariant fixtures, and circular-resource flow/capacity/route fixtures.
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
- one licensed, aggregate-safe, provenance-visible place-data pack or a clearly
  labelled fictional-only pack if the data gate is not complete;
- one safe Vibe-authored scenario; and
- an optional, graph-bounded NPC demonstration with deterministic no-LLM
  fallback.

## Key risks and responses

| Risk | Response |
|---|---|
| Browser Qualia profile is too large or incomplete | Prove it in Phase 0; trim to a verified capability profile and defer optional modules |
| Renderer or `.10d` browser path is impractical | Keep stable semantic IDs and a simple renderer fallback; do not block core simulation |
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

1. Decide the UI/application shell and exact QualiaDB revision to pin.
2. Create the Phase 0 browser spike project and measurement log.
3. Write the first ontology/seed-content fixture and deterministic replay test.
4. Sketch the first community-ground scene and create an original placeholder
   asset list.
5. Select the fictional core town before choosing a public-data demonstration
   locality.
