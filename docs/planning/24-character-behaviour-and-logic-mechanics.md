# Character, community, project, and QualiaDB logic mechanics

## Direction

Maslows Challenge should make character and community outcomes emerge from
what people do, what they know, what they have agreed to, the resources they
control, and the systems surrounding them. A protagonist is not assigned a
single “good/bad” value. The game records actions, commitments, skills,
relationships, material circumstances, and consequences as distinct things.

The experience must make the QualiaDB logic capabilities do real game work.
Each capability should be invoked in an authored scenario, leave a verifiable
receipt, and contribute to a legible player-facing outcome. A logic showcase
panel by itself is not a game mechanic. Conversely, no logic name is claimed as
integrated just because a Rust module, demo page, or export exists upstream.

## Current implementation and integration audit

Audit date: 2026-10-05.

In this checkout, `rolling_commons_shell::GameSession::propose` validates the
catalogued action gate with QualiaDB SHACL, applies declared triple edits, and
appends a compact action event. Its Vibe cell can read a small projection and
select a catalogued action; it cannot query or update a general actor-scoped
Q42 graph. The checked-in generated JS binding includes direct SHACL,
deontic, epistemic, and `clinical_risk` exports, but the game UI does not yet
invoke these as character mechanics. Other requested capabilities are present
in local QualiaDB modules/examples, but were not found as a complete game-host
surface in this package audit. This is a binding/integration audit, not a claim
that the upstream implementations are absent or certified.

Two semantics need special care before they become gameplay: the inspected
CRDT resolver is Last-Write-Wins for ordinary graph facts, so it must not
resolve disputed testimony or consent; and the diffusion module's enqueue
helper alone is not proof of a game-relevant diffusion pass. Verify the
feature-gated execution, data meaning, repeatability, and WASM availability
before connecting it to any outcome.

## Character model

Keep these facets separate in the character graph:

| Facet | What it means in play | What it must not become |
|---|---|---|
| Competencies | Demonstrated or learned, task-specific capability such as repair, growing, organising, listening, or accounting | A measure of intelligence, human worth, or eligibility for basic needs |
| Approaches | Contextual strengths such as patience, curiosity, improvisation, careful planning, reciprocity, courage, and boundary-setting | A permanent moral alignment or a universal “best build” |
| Commitments | Values or promises a character has explicitly chosen, can revise, and can explain | A hidden rule that punishes a player for changing their mind |
| Practices | A sourced timeline of actions: kept terms, shared credit, asked consent, repaired a mistake, or declined unsafe work | A decontextualised virtue score or personality diagnosis |
| Relationships | Directional, context-specific ties: trust, reliance, outstanding obligations, boundaries, and shared history | One global popularity value controlling access to rights or essential services |
| Current capacity | Available time, money, project resources, and player-selected pace/access preferences | A diagnosis, inferred health status, or proxy for deservingness |

Strengths should create options and trade-offs, not deterministic personalities.
An observant character may notice overlooked evidence but can still be wrong; a
patient organiser can build agreement but spend scarce time. Skills grow from
authored practice and feedback, not from grinding the same action.

## People, communities, and projects are distinct scopes

These scopes participate in one simulation but are not interchangeable:

| Scope | Authoritative facts | Important boundaries |
|---|---|---|
| **Person** | Chosen commitments, demonstrated skills, controlled resources, offers/acceptances, bounded relationships, work and contribution receipts | One person's belief or commitment is not automatically a community position; person-specific facts remain private unless shared with permission |
| **Community** | Voluntary membership/representation, charter and procedures, collective assets, proposals/decisions, shared capability, distributions, dissent, and community-held obligations | A community is not a single person with one belief. Preserve subgroup positions, non-members, dissent, consent, and unequal impact; no aggregate virtue or human-worth score |
| **Project** | Versioned charter/scope, sponsors, affected parties, deliverables, alternatives, budget, resources, schedule, contracts, milestones, risks/assumptions, evidence, ownership/stewardship, and operating/maintenance plan | A project has a bounded purpose and lifecycle. Approval by one sponsor or community does not imply consent by every contributor or affected group |

Cross-scope relations should be explicit, time-bound, and provenance-bearing:
`memberOf`, `represents`, `contributesTo`, `affectedBy`, `stewards`,
`funds`, `dependsOn`, `agreedUnder`, `evidenceFor`, `benefits`, and
`bearsCost`. A project can span two communities while keeping the obligations
and effects attributable to each. A collective asset must identify its actual
steward and governing agreement; merely being nearby does not transfer
ownership or decision rights.

Evaluate effects as a linked set of views, not one score:

- **People:** money/time gained or spent, skill learned/shared, consent and
  commitments kept or disputed, boundaries, relationships, and the character's
  own stated priorities.
- **Communities:** who can access the service, distribution of cost and benefit,
  capability retained locally, representation and dissent, collective assets,
  and dependencies on outside help.
- **Projects:** feasibility, alternatives, schedule, budget, contributors,
  deliverable quality, maintenance burden, ownership, and long-term operating
  cost.

Changes propagate only along declared causal and temporal links. A person's
choice can alter project delivery and a community's service access, but the
community does not inherit that person's private circumstances. A project's
short-term success cannot silently erase a community obligation or an
individual's claim.

## Consequences are multi-part and visible

Every consequential choice should show its likely effects across separate
dimensions before commit and report observed effects afterward:

1. **Immediate material result:** coins, supplies, access, time, and project
   progress.
2. **Agreement and rights result:** who consented, who owes what, who received
   payment or credit, and whether an obligation was kept, breached, disputed,
   or excused.
3. **Relationship result:** which people may rely on the actor, set a boundary,
   renegotiate, or decline future work, and on what evidence.
4. **Community result:** distribution of benefit and burden, service access,
   shared capacity, maintenance, and work left for others.
5. **Personal story result:** a character's stated commitment may feel affirmed
   or come into tension with their choices. The game presents the tension as a
   question or remembered event; it does not diagnose the character or assign
   an imposed shame/success score.
6. **Delayed result:** explicitly scheduled obligations, trust consequences,
   maintenance needs, later cooperation, appeal, repair, or restitution.

Predictions must distinguish known, likely, disputed, and unknown effects.
Show material costs and uncertainty; do not promise that fairness always pays
financially or that an exploitative act always causes immediate loss. Let the
fictional institutions and people respond through consistent rules and agency.

The game must also make space for doing the right thing at a real cost. A
character may keep a fair agreement and lose money or a day of work; share
credit while a rival gains short-term status; help a neighbour while delaying
their own project; or choose rest, a boundary, paid help, a smaller plan, or
renegotiation instead. Self-sacrifice is not the only valid route, and unpaid
labour is not the default proof of kindness.

## Example vertical slice: the relay repair contract

At the Saltwind relay, residents have supplied salvaged parts and repair time.
The player receives a contract that states the agreed payment, credit, work
window, safety requirements, and who owns the relay afterward. The player can:

| Approach | Immediate result | Later possibilities |
|---|---|---|
| Pay the agreed crew and name contributors | Fewer coins remain for the next upgrade; repair is slower while the crew completes the work window | The agreement is fulfilled, contributors can vouch for the repair, and future maintenance can be planned together |
| Renegotiate the scope and payment before work | The project is smaller or later; parties choose whether the revised terms work for them | A smaller reliable repair can preserve resources and consent; declined terms remain a legitimate outcome |
| Contribute personal time and share the credit | The actor loses a workday and delays their own objective | Skills and reciprocity may grow, but fatigue/capacity is not a moral penalty and the actor can ask others to share the load |
| Claim the crew's work as the actor's and retain the fee | More coins or status arrive now | The source evidence can contradict the claim; obligations, relationships, project reliability, and future cooperation may change; workers can dispute, decline, or seek restitution |
| Defer or leave the contract | No immediate financial gain or breach if terms permit withdrawal | The relay remains unavailable longer; another crew or a smaller project may be chosen |

No branch is reduced to a red “evil” button or a morality meter. The feedback
names who gained, who paid, what evidence is available, and which rules caused
the outcome. The affected workers are participants with their own knowledge,
agreements, goals, and boundaries, not passive props whose only function is to
reward or punish the player.

## Multi-community project slice: shared water and communications

Extend the relay into a joint Kestrel–Saltwind water and communications
programme. Each community keeps its own charter, members, collective assets,
resource limits, decision process, and disagreements. The project has a shared
purpose but also named work packages: the Kestrel tank, the Saltwind crossing,
the relay link, and an agreed maintenance rota. Some residents contribute
skills or parts; others are affected by access, noise, work scheduling, and
future fees. The player can propose a joint project, split or stage it, select
another design, or decline participation.

The project board must expose at least:

1. **A charter and alternatives:** desired service level, who benefits, who is
   affected, what is out of scope, and alternative designs, including a smaller
   local-only option.
2. **Independent community processes:** proposals, representation, quorum or
   other declared decision rules, dissent and abstentions, and any limits on
   who can commit a community-held resource.
3. **A full project plan:** work breakdown and dependency graph, resources,
   budget, contracts/pay/credit, schedules, milestones, risks, commissioning,
   ownership, and ongoing maintenance.
4. **Cross-scope impacts:** who contributes, pays, gains access, waits, holds
   maintenance responsibility, or can appeal, with sources for every claim.
5. **Multiple viable approaches:** a fast but expensive design, a slower staged
   design, a locally maintainable design, and a renegotiated/jointly funded
   design. The engine reports constraints and consequences; the player and
   communities make the decision through the declared authority rules.

A player might personally receive a fast contract payment for the relay while
one community absorbs unpaid work and the other gains most of the service.
That is a plausible immediate financial success for the actor and project, but
it creates distinct questions: were the work and credit terms lawful under the
fictional agreement; which members knew or consented; how does the allocation
compare with each community's stated service goals; what happens when the
maintenance rota begins; and which later project depends on this one? The game
must show the effects at their actual scope and let the people and communities
respond, contest evidence, revise terms, set boundaries, and seek restitution.
No outcome is a universal moral rating.

## Logic-to-mechanic map

These are design assignments, not certification that every capability is
already bound into this game's selected WASM package. The local QualiaDB tree
contains corresponding source modules or examples for many entries; the game
binding, `wasm-full` feature availability, semantics, and browser receipts must
be checked for each one before calling it demonstrated.

| QualiaDB capability | Mechanic in the relay story | What the player can inspect | Guardrail and proof |
|---|---|---|---|
| **N3** | Express authored facts, relationships, norms, evidence links, and inference rules in the game's semantic graph | The event and inferred facts in the Why panel | Run the authored rule through QualiaDB's N3 path; retain rule version and source event IDs |
| **SHACL** | Validate a proposed contract/action graph: parties, resource quantities, consent evidence, credit, timing, and destination are well-formed | Specific missing/invalid fields and a repair path | The authoritative Qualia validator rejects malformed cases; do not describe JavaScript checks as SHACL |
| **Deontic** | Evaluate permissions, prohibitions, and obligations attached to consent, payment, attribution, safety, and expiry | The relevant obligation/permission, status, defeater, and responsible party | Only the rules/reducer decide whether an action commits; show when a duty is conditional, disputed, expired, or excused |
| **Epistemic** | Keep separate what the actor, worker, committee, and town know, believe, or have not been told | Perspective-limited reports and certainty labels | Never turn a belief into a fact or expose another character's private context without a permission path |
| **Paraconsistent** | Preserve conflicting accounts such as “the repair was solo work” and “three workers supplied the hours” | Both claims, who made them, and their evidence | Contradiction is inspectable and local; it must not erase unrelated facts or silently choose the more powerful speaker |
| **Argumentation** | Let parties support or attack claims using contract terms, work records, witness reports, and provenance | The reasons supporting each live position | An accepted argument is not automatically legal truth; provide a review, response, and appeal path |
| **Dialectical reasoning** | Stage a council discussion: proposal, counter-position, then a revisable synthesis or unresolved result | The premises kept, challenged, or left open in the synthesis | Do not force artificial consensus; dissent and no decision remain valid outcomes |
| **ASP** | Enumerate feasible crew, schedule, budget, and repair-scope plans under explicit constraints | Several feasible plans and which constraints exclude others | ASP returns candidate sets; it must not rank a person's moral value or hide the cost function as a “best” answer |
| **Linear logic** | Consume finite parts, funding commitments, and bounded work slots exactly once when a project commits | Resource receipts and any unspent remainder | People are never consumable tokens. Rest, care, consent, and human relationships are not linear inventory |
| **Description Logic** | Classify task requirements and skills through an explicit ontology: e.g. a trained relay repairer is a kind of electrical maintainer | Which declared capability satisfies which task requirement | Classification applies to evidenced task capability, never inferred identity or eligibility for basic services |
| **LTL** | Check a sequence of commitments over time: eventually pay an accepted invoice; keep the relay safe until handover; always record a safety stop | The event sequence and the point where a condition held or failed | Use the game tick/event trace and versioned formula; explain contrary-to-duty/recovery conditions rather than silently resetting history |
| **Allen interval algebra** | Schedule overlapping shifts, break windows, contract deadlines, tide windows, and maintenance slots | Which intervals meet, overlap, precede, or conflict | Resolve times against the same deterministic game clock; do not encode a person's health as an inferred availability interval |
| **Diffusion** | Explore how repair knowledge, maintenance practice, or a substantiated account spreads through a consented neighbour network | Which source and network paths carried the information and how the field changed | Do not model rumours as facts or manipulate NPCs through an invisible influence score. First verify the Qualia diffusion API's actual semantics and browser/GPU path |
| **Neuro-symbolic sieve** | Constrain any optional adviser/NPC proposal to allowed graph terms and action forms, then pass the proposal through SHACL, deontic, and reducer gates | A bounded proposal plus accepted/rejected token/action and validation receipt | The sieve narrows generation; it does not authorize actions or become a second rules engine. Core play remains complete with AI disabled |
| **CRDT** | Merge offline edits to a shared project board, independent work receipts, or local notes across collaborators | Provenance, clock, merge result, and concurrent versions | Never LWW-overwrite disputed consent, testimony, obligations, or self-authored records. Preserve events and route contradictions to paraconsistent review |
| **CogAI** | Retrieve relevant past experiences for an NPC's dialogue: who kept an agreement, taught a repair, or ignored a request for credit | The memory source, scope, activation, and any decay | Memories are bounded and attributable; no secret personality diagnosis or unauthorized health/private-memory retrieval |
| **Clinical modalities** | Optional, sensitivity-reviewed service-planning vignette with synthetic fictional inputs; use the clinical module only for the specific reviewed simulation task | A clearly fictional result, source/assumption notes, and “not a real care recommendation” boundary | Never diagnose the player, infer a condition, rank worth, grant/deny rights, or give clinical advice. Clinical engine output cannot become a character virtue/consequence stat |

**VibeScript** is the authored scenario and presentation surface around these
calls: it declares the scene, bounded queries, proposal shapes, dialogue cues,
and receipt presentation. VibeScript cannot call storage or reducers directly,
mutate Q42 bytes, or bypass the Qualia rule path. JavaScript is limited to host
input/render glue already sanctioned by the Qualia WASM interface.

## Apply the same logic at community and project scope

Community and project reasoning must be first-class mechanic paths. These are
not two extra labels on the protagonist's sheet: communities have their own
decision procedures and contested collective facts, while projects have
versioned plans, contracts, schedules, resources, deliverables, and operation
after handover.

| Capability | Community mechanic | Project mechanic |
|---|---|---|
| N3 | Link membership, representation, charter, proposals, decisions, shared assets, and dissent with provenance | Link charter, alternatives, contracts, dependencies, milestones, evidence, assets, and operations |
| SHACL | Validate charter versions, eligible decision process, represented scope, declared quorum, and collective-resource authority | Validate scope, stakeholders, budget, schedules, contracts, deliverables, risk/assumption references, and maintenance owner |
| Deontic | Evaluate which body may decide, obligations owed by members/representatives, and limits on collective authority | Evaluate sponsor, contractor, contributor, and steward permissions/obligations over project phases |
| Epistemic | Distinguish an individual's view, a group's reported position, and common knowledge following a valid disclosure/process | Track which project parties know a change, risk, cost, or requirement and what remains unverified |
| Paraconsistent | Keep conflicting community/subgroup claims active without inventing unanimity | Preserve contradictory field reports, acceptance claims, and impact assessments for review |
| Argumentation | Let members support/challenge a proposal with relevant evidence while preserving minority views | Compare project alternatives and challenge cost, schedule, benefit, and harm claims |
| Dialectical | Run a proposal, counter-proposal, and revisable synthesis through the community's declared forum | Use design/change reviews to retain, amend, defer, or reject project requirements |
| ASP | Enumerate feasible coalitions/representatives/resource agreements under declared rules | Enumerate feasible scope, team, procurement, schedule, and budget plans; expose the constraints for each |
| Linear logic | Spend a community-held contribution or grant commitment once, with a receipt and remaining balance | Allocate bounded material/funding/work-slot resources to tasks once; never model people as consumables |
| Description Logic | Classify declared community roles and task capabilities without inferring identity or membership | Classify deliverables, required capabilities, compatible components, and project work packages |
| LTL | Check continuing collective duties, review points, sunset/renewal conditions, and promises to report back | Check milestone order, eventual payment/handover, safety invariants, and continuing maintenance duties |
| Allen intervals | Coordinate meeting, consultation, service, and shared-resource windows across subgroups | Resolve overlapping work shifts, dependencies, deadlines, weather/tide windows, and maintenance periods |
| Diffusion | Trace how a verified practice or report travels between consenting members and communities | Trace how a method, defect, or lesson moves through a project's dependency/supply network |
| Neuro-symbolic sieve | Bound proposals to the community's declared vocabulary, permissions, and policy; require full rule validation | Bound generated plans/change requests to project types, constraints, and permitted actions before proposal |
| CRDT | Merge offline notices, draft proposals, and independent participation receipts while retaining authorship | Merge local work-board updates and non-authoritative draft plans, then check invariants and dependencies |
| CogAI | Retrieve scoped institutional memories about past decisions, commitments, and outcomes for later deliberation | Retrieve attributable project lessons, maintenance history, and unresolved issues relevant to the current phase |
| Clinical modalities | In an opt-in synthetic service scenario, reason about reviewed service/process data at an aggregate planning boundary | Check a reviewed fictional service project's protocol/workflow constraints; never produce a real person's diagnosis or treatment advice |

### Cross-scope acceptance conditions

- A member action can create a person-scoped event, satisfy or breach a
  contract, change one project's delivery, and affect one community-held asset.
  The Logic Receipt must show each link and keep all four results distinct.
- A project may serve multiple communities with unequal benefits and burdens.
  Report per-community access/distribution and project-wide delivery together;
  never collapse them into one “community success” value.
- A community vote records the declared process, eligible scope,
  representation, abstentions, dissent, and result. A voting result cannot
  fabricate an individual's consent to contribute labour or money.
- A project plan result is a set of feasible options with declared constraints
  and assumptions. It is not an autonomous recommendation or permission to
  spend collective resources.
- Contradictory reports, concurrent offline edits, knowledge propagation,
  memory retrieval, and project impact estimates must retain source, time,
  scope, and uncertainty. No downstream modality may overwrite the source
  event or convert a disputed claim into verified fact.
- Community membership, poverty, disability, health, trauma, culture, or a
  fictional character's perceived virtue must never determine human worth,
  basic rights, or who is entitled to benefit from a shared service.

## Authoritative event and explanation path

Every committed action affecting a person, community, or project should be
represented as a versioned, provenance-bearing game event with at least:

```text
event ID and deterministic game tick
actor, action, explicit scope(s), affected parties, and affected
  community/project/resource IDs
agreement/permission reference and consent/withdrawal evidence where relevant
declared costs, payment, credit, work interval, and resulting transfer
source claims and epistemic status (asserted, believed, disputed, verified)
rule, ontology, and content-pack versions
causal links to immediate and scheduled follow-up events
```

The commit path is:

```text
player or bounded Vibe proposal
  → graph/schema validation (SHACL)
  → N3 inference and applicable deontic checks
  → QualiaDB's authoritative deterministic game reducer
  → append event + provenance; consume committed linear resources once
  → evaluate temporal, interval, causal, argument, community, project, social,
    and memory views
  → render immediate cost, delayed commitments, disagreements, and why
```

Derived logic views never edit the authoritative event history. An LLM,
CogAI retrieval, diffusion pass, argumentation result, CRDT merge, or ASP option
set can propose or interpret; only a validated reducer event changes game
state. A **Logic Receipt** should record the invoked Qualia capability and
version, source facts/events, rule/formula digest, bounded result, and resulting
event ID. Player view gives a plain-language “why”; developer view exposes the
machine receipt.

## Required verification and upstream gates

1. Audit the **actual selected full WASM package** for each API, semantics,
   feature flag, and memory/runtime bound. A module, API page, or Rust test is
   not a browser integration receipt.
2. Record each capability as `integrated`, `source-only`, `blocked upstream`,
   or `deferred` in the living QualiaDB capability ledger. Do not label the
   complete list as in-game until every row has a real playable invocation.
3. Build one cross-community water-and-relay project slice whose replay
   includes a fair-cost path, an exploitative short-term-gain path, a
   renegotiation path, separate community decisions, contradictory evidence,
   cross-project dependencies, delayed consequence, and restitution/repair.
   Verify deterministic replay and visible person/community/project impacts.
4. Each logic has positive, negative, edge, malformed, budget, and replay
   fixtures appropriate to its semantics. The event trace must prove which
   capability changed which derived view; screenshots alone are insufficient.
5. Clinical content needs qualified clinical and lived-experience review before
   it ships. Until then its slot is a synthetic engine fixture behind an
   explicit opt-in technical scenario, not a player-health mechanic.
6. If any capability is missing from the full WASM or its actual behavior fails
   the fixture, add a generic QualiaDB upstream task and block only that
   dependent mechanic. Do not replace it with JavaScript, another engine, or a
   private game-only evaluator.

The current character feature uses fixed-choice backgrounds, four story arcs,
and a few traversal traits. It does **not** yet implement this behavior ledger,
relationship consequences, multi-perspective reasoning, or logic receipts.
The earlier [character and first-person plan](23-playable-characters-and-first-person.md)
remains the content and first-person scope; this document defines the deeper
simulation direction.
