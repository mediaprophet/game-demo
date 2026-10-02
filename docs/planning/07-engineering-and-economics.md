# Rolling Commons: Engineering and Community Economics

## Purpose and boundary

Engineering, resource, and economic systems make community-ground decisions
meaningful: what can be built, what it provides, what it consumes, what it costs
to maintain, and how resilient it is over time. QualiaDB's typed computation,
semantic graph, logic, Q42 provenance, and WASM execution should make these
models inspectable rather than opaque score formulas.

All models are game-scale simulations with declared assumptions and uncertainty.
They are not engineering designs, operating instructions, financial advice,
environmental assessment, safety certification, or regulatory approval. Real
construction, energy, fuel, chemical, hydrogen, and waste-processing processes
remain outside the game and require qualified real-world assessment.

## Model contract

Every engineering/economic model input and output has a typed unit, source or
scenario assumption, model version, applicable range, confidence/uncertainty,
and provenance receipt. The system rejects incompatible units and surfaces
missing evidence rather than silently guessing.

Where a game model uses public aggregate data, its deterministic derived facts,
constraints, probability distributions, and statistical ranges are typed,
versioned inputs. QualiaDB may use algebra and scientific computing for
deterministic, probabilistic, and hybrid calculations, and its logic systems for
inference, validation, dependencies, temporal conditions, and permitted game
effects. Every model declares its method; stochastic samples are seeded and
persisted for replay, while deterministic results are recorded directly. These
models inform only the fictional scenario; they never score, predict, or
determine a real person's or community's circumstances, rights, behaviour, or
eligibility.

```text
Q42 facts + typed quantities + scenario assumptions
  → validated QualiaDB computation / rule evaluation
  → result with units, range, version, and provenance
  → deterministic game-state proposal
  → command gateway + reducer commit
  → explainable energy/resource/economic change
```

For a project comparison, the computation evaluates both baseline and declared
post-implementation assumptions. The result presents distributions/ranges,
uncertainty, and the mechanisms credited to the project—not a single promised
benefit. A project can shift a simulated outcome only through a declared,
validated change to capacity, access, cost, maintenance, travel, tenure security,
or a safeguard/remedy pathway.

## Energy and resource systems

The initial simulation connects source, conversion, storage, distribution, load,
condition, and maintenance models.

| System | Game model | Primary gameplay questions |
|---|---|---|
| Solar resource | Irradiance/radiance band, season/weather assumption, site exposure, panel condition | Is this location useful, and how variable is it? |
| Solar + battery | Generation, conversion-loss band, storage state, charge/discharge limits, priority loads | Can the ground meet priority loads through this period? |
| EV charging | Charge-point availability, simulated power/capacity allocation, booking/access, source/storage priority, maintenance | Can travelling members charge without compromising critical community loads? |
| Existing venue utilities | Lighting, kitchens, halls, amenities, water, parking, fields, schedule and maintenance loads | Can a showground/sporting-ground node host a project while respecting existing commitments? |
| Disruption resilience | Temporary shelter, water, communications, power, amenity, transport, and recovery capacity under scenario conditions | Which capabilities are protected or restored first in this scenario? |
| Water/food/biomass | Resource condition, production/harvest cycle, inputs, storage, ecological constraints | What can be sustained, and what stewardship is needed? |
| Vermiculture | Food-scrap collection, feedstock quality, worm-farm capacity/condition, amendment output, garden distribution | Can organic material become a useful, maintained community soil resource? |
| Community food | Soil/raised-bed, hydroponic, or pack-defined cultivation capacity; ingredients, preparation spaces, storage, distribution, cultural context | How can shared food capability strengthen nourishment, culture, and resilience? |
| Circular materials | Collection, sorting, reuse/recycling or abstract processing capacity, residue/upkeep | Is reuse preferable and what care does it need? |
| Advanced conversion plant | Abstract modular process with declared feedstock/energy/water/output/maintenance/safety/permit constraints | Is this scenario investment justified? |
| Mobility fuels | Available energy carriers, storage/transport, compatibility, operating demand | Which mobility plan fits the ground and party? |

Plastic pyrolysis, hydrogen production, synthetic fuels, and similar industrial
chains are high-level, pack-authored process modules only. The game may model
their resource dependencies, environmental trade-offs, costs, maintenance,
governance, and uncertainty, but never provides recipes, parameters, equipment
configurations, or operating procedures. v0.1 starts with solar, battery,
essential loads, and simple renewable/community resource loops.

### EV charging as shared infrastructure

An EV charging point is a `GroundNode` utility, not simply an inventory upgrade.
It has a location, access/booking agreement, simulated connection and available
capacity, load priority, energy-source relationship, maintenance state, and
vehicle compatibility category. It can be installed at an integrated hub or a
distributed mobility node.

The game uses it to make energy allocation visible: charging may draw from solar,
battery storage, or other scenario energy supply, but community rules can protect
critical loads such as communications, water, accessibility, refrigeration, or
community operations. A party may plan a trip around charging availability or
contribute to a charging project. This is a simulation of capacity, scheduling,
and shared stewardship—not guidance for electrical design, installation, or
vehicle charging.

### Vermiculture and local nutrient loops

Vermiculture is an early, tangible circular system for the game. A community
needs a suitable, maintained worm-farm facility; a supply of appropriate
game-modelled vegetable/organic scraps; collection/storage capacity; people or
roles responsible for care; and garden/food-production nodes that can receive
the simulated nutrient-rich soil amendment. The resulting loop is:

```text
household/kitchen/garden scraps
  → collection and quality check
  → worm-farm capacity and maintenance
  → amendment inventory with provenance/condition
  → route, agreement, and distribution to garden/partner nodes
  → soil/resource condition and future food/garden capacity
```

The game can model contamination risk, season/condition bands, labour, transport,
storage, access, and fair distribution as scenario factors. It does not offer
real-world vermiculture, sanitation, fertiliser, or food-safety instructions.

### Community food and evidence-sensitive food concepts

The nutrient loop can support community gardens, kitchens, preservation spaces,
ingredient exchanges, and shared meals. A food project can trace a simulated
ingredient from its growing/resource context through harvest, storage,
preparation, distribution, and community benefit. This provides meaningful links
between soil/resource stewardship, food sovereignty, skills, culture, and local
resilience.

Each food-production node declares a cultivation method. Soil/raised-bed systems
can consume space, simulated soil condition, water, amendments, labour, and
seasonal capacity. Hydroponic systems can instead model infrastructure, water,
energy, nutrient solution, monitoring/maintenance, and controlled growing
capacity. Region/content packs may add other methods only through the same
typed-input, uncertainty, provenance, and stewardship contract. The comparison
is a game trade-off model, not growing, nutrient, sanitation, or food-safety
instruction.

Some packs may describe foods as functional, traditional, or associated with
wellbeing. Such descriptions are evidence-sensitive content, not a health system:
they require explicit claim scope, source/provenance, cultural context,
uncertainty, and review status. The game never diagnoses, recommends treatment,
calculates a dose, gives nutrition/medical advice, or personalises food claims
for a player's age, pregnancy, health, medication, heritage, or other profile.
Where evidence is incomplete or contested, the game presents that uncertainty or
omits the claim.

## Engineering predicates and rules

Core predicates include `hasRatedCapacity`, `hasStateOfCharge`,
`requiresEnergy`, `hasEnergyOutput`, `hasResourceInput`, `hasConversionModel`,
`hasMaintenancePlan`, `hasCondition`, `hasUncertainty`, `hasPriority`,
`mayOperate`, and `requiresStewardship`.

### Condition, maintenance, and failure

Every significant game asset—vehicle, demountable home, battery, solar system,
EV point, tool, worm farm, hydroponic node, venue utility, or data container—has
a visible condition and maintenance record. It may be `unknown`, `serviceable`,
`needsAttention`, `degraded`, `restricted`, `failed`, `underRepair`, or `retired`.
The record links inspection observations, expected maintenance, service history,
usage/age bands, current constraints, and a provenance-bearing fault event.

```text
condition observation or scheduled maintenance due
  → inspect / diagnose a simulated fault
  → choose stabilise, defer, repair, replace, seek mentorship, or book service
  → resource/time/capacity and rule validation
  → repair outcome, remaining uncertainty, and updated maintenance plan
```

Failures are scenario-driven and bounded by declared condition, maintenance,
usage, environmental assumptions, and optional narrative events. They must not
be hidden dice punishment or claim to predict real reliability. A player always
has an explanation, can inspect what is known/unknown, and is offered legitimate
paths such as a temporary workaround, a parts search, a loan, a shared facility,
or professional assistance.

EV-charging predicates include `providesCharging`, `hasChargeAvailability`,
`hasLoadPriority`, `isCompatibleWithVehicleCategory`, `requiresBooking`, and
`hasMaintenanceState`. Rules may reserve game capacity, protect critical loads,
or explain a deferred charge; they cannot assess real electrical safety or
compatibility.

Existing venue predicates include `hasExistingFacility`, `hasLightingLoad`,
`hostsScheduledActivity`, `hasAccessAgreement`, and `hasAvailableTimeWindow`.
Rules can account for simulated lights and other venue loads before allocating
community energy/capacity, and hold a proposal when a schedule or agreement
conflicts. They never represent an operator's actual permission or real utility
capacity.

Condition predicates include `hasCondition`, `requiresMaintenance`,
`hasMaintenanceDueAt`, `hasFaultObservation`, `hasFailureMode`,
`hasOperationalConstraint`, `isUnderRepair`, and `hasServiceHistory`. Rules can
hold a simulated activity, reduce available game capacity, protect a critical
system, or open repair pathways. They cannot determine real equipment safety,
reliability, or maintenance requirements.

Resilience predicates include `hasResilienceRole`, `hasHazardContext`,
`hasTemporaryCapacity`, `hasCurrentAccess`, `hasCriticalLoad`, and
`requiresRecoveryProject`. They support scenario planning and transparent
resource-priority trade-offs; they do not represent real hazard assessment or
emergency operations.

Vermiculture adds `producesScrap`, `acceptsFeedstock`, `hasFeedstockQuality`,
`hasProcessingCapacity`, `producesAmendment`, `requiresCare`, `mayDistributeTo`,
and `improvesResourceCondition`. The rules can hold a collection or distribution
request when capacity, quality, care, route, consent, or garden suitability is
missing, and explain alternative game pathways.

Food-content predicates include `hasIngredient`, `grownAt`, `preparedAt`,
`hasStorageCondition`, `hasCulturalContext`, `hasEvidenceSource`,
`hasClaimScope`, and `hasReviewStatus`. Logic may determine whether a game food
project has sufficient provenance to display a contextual description; it must
not derive health efficacy, safety for an individual, or treatment eligibility.

Cultivation predicates include `usesCultivationMethod`, `requiresWater`,
`requiresNutrientInput`, `requiresEnergy`, `hasGrowingCapacity`,
`hasResourceCondition`, and `requiresMaintenance`. Rules can derive simulated
capacity and hold projects when declared inputs or care pathways are absent;
they do not assert real crop yield, food safety, or system safety.

Rules can derive that a load is covered, a battery reserve is protected, a
project needs maintenance, a resource use is held for review, or a process is
not eligible in the current scenario. They cannot claim physical safety or grant
real-world permission. Every result explains inputs, assumptions, and the rule
or model version used.

## Community economics

Economics is more than a player wallet. Each person has a private wallet, and
each committee-controlled community fund has a treasury; the game tracks their
separate, related ledgers alongside the ground and specific project:

| Ledger | Tracks | Example decisions |
|---|---|---|
| Person/party wallet | Pack-defined scenario value/exchange credit, time, shared resources, commitments, chosen affordability goals | Repair now, learn first, travel, share a resource, defer a purchase |
| Committee treasury | Operating funds, earmarked reserves, materials, volunteer/paid time, maintenance and service capacity | Expand solar, repair a tool, fund a mentor, build reserve capacity |
| Project | Capital inputs, operating inputs, maintenance/replacement plan, contributions, outputs, benefits | Compare repair with staged infrastructure improvement |
| Ecological/community account | Resource condition, regeneration, waste/residue burden, access, resilience, shared benefit | Avoid a short-term gain that exhausts a resource or creates unmaintained burden |

### Custody, approval, and audit

A wallet is owned by one person (or by an explicitly consented party) and is
private by default. A treasury is owned by a `Committee`, never by its
treasurer: the treasurer role prepares or records a transaction, while the
committee's declared approval policy authorises spending. Policies can require
one or more current role-holders, a spending cap, a vote/consent threshold, and
an earmarked-fund match. A member may contribute to a treasury without gaining
authority to withdraw from it.

Community participation and project help are distinct from this custody model.
An off-site neighbour, supporter, or partner may suggest a project, offer work
or materials, or contribute value without residing at a ground or joining its
committee. Such participation is never evidence of treasury authority; only a
current committee role and its approval policy can authorise a treasury spend.

All value movement is a balanced, immutable posting between named accounts or
an explicitly modelled external scenario source/sink. Each posting records
amount/unit, purpose, fund, initiating command, approvers, counterparty role,
timestamp, and event provenance. Reducers derive wallet and treasury balances
from those postings, reject an overdraft or unauthorised spend before commit,
and preserve a clear explanation of a held proposal. This is game bookkeeping,
not banking, payment processing, tax, fundraising, or financial advice.

### Tenure, obligations, and material pressure

The person/party ledger may record player-selected continuing obligations such
as a lease or shared mortgage abstraction, deposits, recurring utilities,
service/platform costs, school/work/local ties, and shared assets. These facts
explain why another dwelling, vehicle, or community-ground arrangement may be
needed even while prior obligations continue. They are structural scenario
conditions, not a score of deservingness, prudence, or human value.

Costs and obligations are fictional or pack-configured parameters with visible
assumptions. The game does not quote markets, interpret contracts, advise on
finance or law, determine entitlement, or imply that a relationship change
erases a mortgage, debt, or care responsibility. Infrastructure projects record
which capability gap they reduce and which SDG-aligned outcomes they may support;
they do not create an official certification or allocation claim.

Costs, prices, wages, grants, loans, exchanges, volunteer contributions, and
shared-benefit measures are scenario parameters with sources/assumptions. The
game does not claim real market prices or advise on financial decisions. It shows
cash flow, reserves, operating/maintenance burden, dependencies, benefit
distribution, and uncertainty—not one currency score for community wellbeing.

## Player experience

- A load-priority view shows what solar/battery capacity can support and what
  must wait.
- A project canvas compares components, skills, inputs, stages, maintenance,
  expected capacities, and scenario uncertainty.
- A wallet view shows the player's own funds, commitments, and pending transfers;
  a treasury view shows committee-approved contributions, earmarks, reserves,
  and spending with consent-aware visibility.
- A condition board shows what needs attention, why it matters in the scenario,
  what is known versus uncertain, and repair/maintenance alternatives.
- A circular-resource view follows scraps through a worm farm to amendment and
  garden capacity, including care, collection, and distribution constraints.
- A food provenance view links game ingredients, place, preparation, cultural
  context, and evidence/claim-status labels without making personal health claims.
- An evidence view shows the model, assumptions, units, pack-data source, rule
  result, and provenance behind an outcome.

## v0.1 and expansion proof

v0.1 implements one transparent solar/battery/critical-load model, personal
wallets, and one committee treasury for the community-ground/project ledger. It
includes typed quantities,
unit-validation, a scenario irradiance assumption, storage/maintenance logic,
economic provenance, balanced-posting/approval checks, and deterministic tests.

Later packs may add biomass/oil, circular-material, hydrogen, synthetic-fuel, or
other conversion chains only after explicit scope, safety, environmental,
data-quality, model-validation, and authoring review. They remain abstract game
modules and preserve the same typed/provenance/rule contract.

Community food capability may be introduced after v0.1 with the same review
discipline. Functional/traditional/medicinal-food language requires an additional
evidence, culture, legal/claims, and safety review before it can appear in a
published pack.
