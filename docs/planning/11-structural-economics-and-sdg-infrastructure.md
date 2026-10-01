# Rolling Commons: Structural Economics and SDG-Aligned Infrastructure

## Premise

Housing insecurity, constrained mobility, and interrupted life plans are not
character flaws. They can follow from rents, shared mortgages, debt and contract
obligations, deposits, utility and service costs, extractive platform charges,
relationship change, caregiving, disaster, discrimination, or a lack of local
infrastructure. A household can retain obligations even when its relationships
or living arrangements must change. The game models those material realities
without converting them into shame, a poverty score, or an eligibility test.

The player is never reduced to a label such as “homeless”, “non-compliant”, or
“unfit”. They define their circumstances, ties, aims, and boundaries. Where a
community ground cannot meet a situation, that is a visible limit of an
infrastructure configuration—not a statement about the person.

## Material capability model

A community ground is one node in a wider local network. Its meaningful value is
the capability it can steward with people: safe shelter and dwellings, water and
sanitation, energy, accessible transport and repairs, communications and local
data, food and soil systems, shared work space, governance, and disaster
resilience. Nodes may be co-located, distributed through a town, or supplied by
partner organisations.

| Domain | Scenario capability | Example game question |
|---|---|---|
| Shelter and tenure | temporary dwellings, secure storage, receiving address, local connections | Can the party bridge a transition while obligations and ties are still real? |
| Water, sanitation, energy | potable water, amenities, solar, batteries, EV charging, backup power | What upgrades make the ground useful during ordinary life or disruption? |
| Mobility and making | camper yard, workshop, parts, road-readiness, shared tools | Can a repair or shared skill turn an unusable vehicle or dwelling into a viable option? |
| Food, ecology, healthful living | soils, hydroponics, vermiculture, kitchens, crops, cooling | What inputs, labour, storage, and stewardship make the system durable? |
| Connection and governance | local data node, connectivity, trading post, agreements, project records | Who can collaborate, and how are costs, consent, and maintenance made visible? |

## SDG-aligned outcomes, not a scorecard

The game may tag projects with *possible* SDG-aligned outcomes: poverty and
basic-needs resilience (SDG 1), health-supporting environments (3), water and
sanitation (6), clean energy (7), decent local work and skills (8), resilient
infrastructure (9), reduced inequality (10), sustainable communities (11),
circular materials (12), climate resilience (13), accountable governance (16),
and partnerships (17). These tags describe a project’s stated contribution and
evidence, uncertainty, and trade-offs. They are not UN certification, a funding
claim, a measure of individual deservingness, or an automated allocation tool.

Each capability record carries source, date, geography, assumptions, confidence,
and the community decision that approved it. Packs can use public aggregate data
to show a town’s infrastructure gaps, but must not infer a resident’s debt,
health, housing status, or entitlement.

## Tenure and recurring-obligation scenarios

The scenario ontology separates people from the systems acting on their choices:

```text
Household ──hasLocalTie──▶ Place
Household ──undergoes──▶ HousingTransition
HousingTransition ──hasObligation──▶ TenureObligation
TenureObligation ──creates──▶ AffordabilityPressure
CommunityGround ──hasCapability──▶ InfrastructureCapability
Project ──reduces / improves──▶ InfrastructureGap
```

Useful predicates include `hasSharedAsset`, `hasContractualObligation`,
`hasRecurringCost`, `hasServiceDependency`, `hasLocalTie`,
`hasInfrastructureGap`, `canContributeToCapability`, and
`hasSDGAlignedOutcome`. Scenario values are fictional or pack-configured; they
are not rent quotes, financial advice, legal advice, or a claim about an actual
person’s circumstances.

A relationship transition can therefore retain a shared mortgage, lease, bills,
children’s routines, work, school, or community ties while creating an immediate
need for another place to sleep or regroup. The gameplay is about options,
cooperation, repair, negotiation, and infrastructure—not a fantasy that an
obligation disappears because a relationship has changed.

## Boundaries and governance

- No homelessness, poverty, compliance, criminality, or human-value score.
- No real-world housing, service, credit, insurance, benefit, or admission
  decision; the game cannot determine who deserves support.
- No hidden use of identity, disability, health, age, trauma, or licence status
  as a proxy for access or worth.
- Community-ground requirements describe operational capacity and can be
  challenged, revised, or supplemented through transparent fictional governance.
- A capability mismatch opens an alternative arrangement; it never means a
  player is abandoned, morally failed, or stripped of local ties.

## v0.1 proof

Ship one fictional Australian transition in which a household has continuing
tenure obligations and local ties, one ground with explicit capability gaps, and
two player-led infrastructure projects (for example solar-plus-battery and a
camper-yard repair bay). The result screen must explain material changes,
assumptions, maintenance burden, shared benefit, and SDG-aligned tags without
ranking the household. Tests must prove no person-deficit score, no real-service
eligibility claim, and no sensitive-field proxy.
