# Rolling Commons: Logic-Driven Rules and Authoring

## Purpose

Rules are a primary gameplay surface. QualiaDB's graph, N3Logic, SHACL, and
deontic/other logic capabilities should let authors and communities create
distinctive rules from explicit ontological predicates—not hide all behaviour in
Rust conditionals.

The rule layer answers both **what is permitted/possible** and **why**. Rust
still owns deterministic state reduction: a rule can derive eligibility,
obligation, prohibition, recommendation, or explanation; it cannot perform an
unreviewed write or bypass the command gateway.

## Predicate-first model

Every rule is built from a documented vocabulary. Initial examples include:

| Domain | Example predicates |
|---|---|
| Capability | `hasSkill`, `hasQualification`, `offersTeaching`, `requiresCapability` |
| Safety | `isSafe`, `requiresInspection`, `hasAccessibleRoute`, `requiresSupervision` |
| Place and resources | `locatedAt`, `isOpen`, `hasAvailableCapacity`, `supplies`, `consumes` |
| Ground topology | `hasNode`, `connectedTo`, `transferredVia`, `hasResource`, `requiresStewardship` |
| Cooperation | `hasConsent`, `isMemberOf`, `mayUse`, `isAccountableFor`, `sharesWith` |
| Community participation | `participatesIn`, `isResidentOf`, `proposes`, `endorses`, `offersHelp`, `helpsWith`, `hasParticipationScope` |
| Projects | `requiresMaterial`, `requiresTool`, `contributesTo`, `unlocksCapability` |
| Making and improvement | `targets`, `hasComponent`, `compatibleWith`, `requiresValidation`, `hasProjectState` |
| Energy and economics | `hasRatedCapacity`, `requiresEnergy`, `hasEnergyOutput`, `hasStateOfCharge`, `hasPriority`, `hasMaintenancePlan` |
| Shared mobility charging | `providesCharging`, `hasChargeAvailability`, `hasLoadPriority`, `requiresBooking`, `isCompatibleWithVehicleCategory` |
| Existing venue use | `hasExistingFacility`, `hasLightingLoad`, `hostsScheduledActivity`, `hasAccessAgreement`, `hasAvailableTimeWindow` |
| Condition and maintenance | `hasCondition`, `requiresMaintenance`, `hasMaintenanceDueAt`, `hasFaultObservation`, `hasFailureMode`, `isUnderRepair` |
| Trade and fulfilment | `listedOn`, `hasListingType`, `compatibleWith`, `hasFulfilmentOption`, `requiresReceivingPoint`, `requiresInspection`, `acceptedBy` |
| Mobility readiness and yards | `hasMobilityState`, `requiresRepair`, `mayStoreAt`, `mayUseYard`, `hasYardCapacity`, `permitsScenarioRoadUse` |
| Dwelling placement | `hasDwellingType`, `requiresPlacementSite`, `hasServiceConnectionNeed`, `isSuitableForParty` |
| Resilience/disruption | `hasResilienceRole`, `hasHazardContext`, `hasCurrentAccess`, `hasTemporaryCapacity`, `requiresRecoveryProject` |
| Safety scenarios | `hasSafetyScenario`, `hasSourceVersion`, `hasCommunicationConstraint`, `hasResourceAvailability`, `hasScenarioAction`, `hasUncertainty` |
| Support and boundaries | `hasSupportScenario`, `hasSupportPath`, `hasBoundary`, `hasTrustedContactOption`, `hasPrivateScope`, `hasAlternativePath` |
| Participation/service scope | `hasGroundCapabilityMatch`, `requiresMobilityResponsibility`, `hasExceptionPath`, `requiresDifferentConfiguration`, `hasServiceBoundary` |
| Structural economics/infrastructure | `hasTenureObligation`, `hasRecurringCost`, `hasLocalTie`, `createsAffordabilityPressure`, `hasInfrastructureGap`, `canContributeToCapability`, `hasSDGAlignedOutcome` |
| Rights conditions and remedy | `hasRightsCondition`, `hasTenureSecurity`, `hasAccessToNecessity`, `createsBarrier`, `maySeekRemedy`, `requiresResponse`, `hasAccountableRole`, `hasSafeguard`, `hasRemedyPath`, `remainsUnresolved` |
| Economic custody | `hasWallet`, `hasTreasury`, `ownedBy`, `controlledBy`, `hasCommitteeRole`, `requiresApproval`, `approvesSpend`, `earmarkedFor`, `debits`, `credits` |
| Circular food/resource loops | `producesScrap`, `acceptsFeedstock`, `hasFeedstockQuality`, `producesAmendment`, `mayDistributeTo`, `improvesResourceCondition` |
| Food provenance and claims | `hasIngredient`, `grownAt`, `preparedAt`, `hasEvidenceSource`, `hasClaimScope`, `hasReviewStatus` |
| Cultivation systems | `usesCultivationMethod`, `requiresWater`, `requiresNutrientInput`, `hasGrowingCapacity`, `hasResourceCondition` |
| Time and provenance | `validDuring`, `occurredAt`, `derivedFrom`, `supersedes` |

Rules must query capabilities, consent, safety, and explicit task requirements;
they must not use identity, heritage, gender, family composition, or other
sensitive profile information as covert proxies for eligibility.

Economic rules additionally validate that a source wallet/treasury has a
sufficient projected balance and that any treasury spend satisfies its current
committee role, approval threshold, spending cap, and earmark policy. They may
approve, hold, or explain a simulated transfer; only the reducer creates the
balanced postings after all checks pass. No rule may infer spending authority
from a contribution, identity, or informal relationship.

Community-participation rules may surface an idea, endorsement, or offer of
help according to its visibility/consent scope. They must not derive residency,
ground access, committee membership, project ownership, voting rights, or
treasury authority from community participation or a contribution. A proposal
only gains operational effect after the normal project, agreement, and resource
validation path succeeds.

Rights-condition rules derive only scenario-state results. They can identify a
threatened or supported condition, open a declared remedy pathway, hold a
request that lacks consent/evidence/capacity, require an accountable response,
and explain an unresolved barrier. They cannot declare a real legal entitlement,
liability, service eligibility, safety assessment, or outcome. They must not
make a person endure harm, surrender consent, or control another person as a
precondition for progress; no composite “peace”, worthiness, or hardship score
is permitted.

## Rule types

| Type | QualiaDB-oriented mechanism | Example |
|---|---|---|
| Integrity constraint | SHACL | A solar project must name a site, capacity, and responsible steward. |
| Derived fact | N3Logic / graph query | A ground becomes energy-capable when generation, storage, and safe maintenance are present. |
| Permission/prohibition | Deontic logic plus capability policy | A party may reserve a workshop bay only with consent, opening hours, and free capacity. |
| Temporal condition | Temporal logic | A maintenance obligation is due after an inspection interval. |
| Scenario rule | Versioned N3 + bounded scenario hook | A study group can form when a mentor, space, and interested learners coexist. |
| Presentation hint | Derived projection, never authority | Display a facility as “ready,” “at capacity,” or “needs attention.” |

## Rule lifecycle

```text
ontology terms and rule intent
  → structured rule builder / advanced N3 editor / optional LLM draft
  → parse and vocabulary validation
  → SHACL shape + policy/capability review
  → fixture, counterexample, determinism, and explanation tests
  → player/author review and explicit activation
  → versioned pack or private rule set
  → command-time evaluation and provenance-bearing result
```

An LLM can explain a rule or draft a proposal from natural language, but it must
target a fixed schema/vocabulary. It cannot invent privileged predicates, alter
the core ontology, activate a rule, or write state. The author sees the
generated rule, affected predicates, example cases, expected outcomes, and
warnings before activation.

## Authoring levels

1. **Player settings:** safe toggles and constrained preferences, such as
   choosing a calmer project cadence. No arbitrary logic.
2. **Scenario/pack author:** curated rule templates with editable predicates and
   parameters—e.g. workshop booking, training, energy sharing, stewardship.
3. **Advanced author:** N3/SHACL/deontic rule source, complete test fixtures,
   declared permissions, version metadata, and explanatory labels.
4. **Community governance:** a future, explicitly consented workflow for rule
   proposals, review, adoption, supersession, and provenance. It is not part of
   solo v0.1.

All rules declare scope: private scenario, content pack, region profile, or
future community agreement. A rule can only reference predicates and actions
allowed by its scope.

## Example: multiple valid pathways

The game should express pathways rather than binary personal judgement:

```text
Task: install a community solar subsystem

Allowed if the site is safe, capacity is available, and one of:
  - a qualified installer is responsible; or
  - an expert mentor supervises a learning team; or
  - an approved external service is booked.

Then derive: project may proceed, with the chosen pathway and evidence.
```

The explanation panel can say what is missing and offer legitimate next steps:
find a mentor, complete training, arrange a service, improve site safety, or
choose another project. It never implies an identity trait prevented progress.

The same predicates support local variation: a workshop booking may be allowed
at a central hub, or at a distributed workshop node only if the party can reach
it and required power/tool/material capacity is available there or can be
transferred under an agreement. A natural resource may contribute to a project
only when its condition, stewardship, season, and safe-use rules permit it.

Maker rules validate a game blueprint before it changes simulated capacity. For
example, a solar canopy proposal must target a compatible node, use compatible
components, fit its game footprint, satisfy simulated resource/capability/safety
requirements, and identify a maintenance pathway. A rule result offers missing
requirements and alternate game pathways; it is never real-world certification
or construction approval.

Food rules can validate ingredient/provenance and community-project conditions,
or gate display of an evidence-sensitive contextual statement on its source,
scope, uncertainty, and review status. They cannot infer that a food treats a
condition, is safe for a particular person, or should be consumed by anyone.

Disruption rules may derive a simulated node's temporary-support/recovery role
from explicit scenario facts, access, capacity, condition, and agreement scope.
They can prioritise critical loads or hold a project when evidence is incomplete.
They never issue real warnings, evacuation instructions, or safety assertions
about a real place.

Safety-scenario rules may select source-grounded fictional actions and explain
missing communication, access, companion, equipment, or resource assumptions.
They cannot use player symptoms to diagnose/triage, identify real animals or
conditions, or generate unsourced medical/emergency instructions.

Support rules may expose player-confirmed, nonviolent alternatives such as a
different task pace, separate resources, community support, a mentor, or a
neutral exit from a scenario. They must not infer substance use/trauma/abuse,
assign fault, restrict dignity/access, or create a hidden behaviour score.

Participation rules may apply only the declared simulated requirements of a
mobility-centred ground and expose a documented exceptional/support path or an
different-configuration result for that ground. They must not infer or proxy a sensitive
identity/health/licensing field, decide real-service eligibility, or convert a
scope boundary into abandonment.

## Guardrails

- Rules are bounded for time, recursion, query cost, output size, and referenced
  graph scope.
- Rule packages are versioned, digested, provenance-bearing, and test-gated.
- Rule changes cannot retroactively rewrite event history; they take effect from
  a declared point and prior results retain their rule version.
- Contradictions, incomplete evidence, and ambiguous terms produce an explicit
  held/needs-review result rather than a silent approval.
- Deontic rules can express permissions/prohibitions/obligations but never
  override player consent, privacy scope, safety constraints, or core capability
  policy.

## v0.1 implementation and proof

v0.1 provides a small, curated predicate vocabulary; N3/SHACL-backed rules for
workshop access, mentoring, vehicle safety, and shared solar; rule version and
provenance; test fixtures; and a player-readable **Why?** panel. A developer
rule inspector may expose source and fixtures. General community rule editing,
LLM drafting, and governance workflows are later gated features.

Acceptance tests must prove that every rule has valid vocabulary, a declared
scope, at least one positive/negative/edge fixture, deterministic results,
bounded execution, and an explanation trace. Changing a rule must either migrate
explicitly or preserve/replay the prior rule version.
