# Structural harm, legal access, and documentary evidence ontology

## Purpose

This ontology is for stories about people initiating useful work under unequal conditions: unpaid effort, scarce funds, limited legal help, disputed credit, control changing hands, and consequences that may compound into debt, loss of housing, damaged relationships, severe distress, or death. It must allow a community project to help people while its originator is harmed or excluded. It must also allow an exploitative path to produce immediate money or political advantage while reducing technical capability, trust, maintenance, or future options.

The ontology is grounded in the lived structural realities the game is intended to expose and the human-rights and rule-of-law purposes it is intended to advance. Unequal access to remedies, the conversion of unpaid initiative into controlled assets or funding, and the gap between formal rights and practical capability are foundational model concerns, not optional allegations to be validated before the game may represent them. The model must still allow variation: no single pathway describes every committee, agency, service, or dispute. Particular scenes can be fictional, reported, alleged, adjudicated, or statistically observed; those evidence states qualify a specific event or number, not the underlying ontology.

The existence of the problem is the reason for this project, not a hypothesis that the project must first prove in order to be allowed to exist. Lived experience, documented events, and established structural mechanisms can establish that a problem is real. Later statistical work should measure prevalence, distribution, trends, and impact, and strengthen particular empirical claims; missing aggregate statistics must never be presented as evidence that the underlying problem does not exist. This distinction matters when explaining why the game itself merits support: show the problem and its mechanisms now, then add well-defined population evidence as that research becomes available.

The project also has a history of work on W3C Verifiable Credentials, Verifiable Claims, Web Payments, and the Read-Write Web/Solid. That history should inform the model's treatment of people as agents with rights, relationships, contributions, consent, and control over their data and work—not merely as identifiers in wallets. Preserve the project's own records and chronology as primary provenance for this design history. The game should show how systems that recognise identity without recognising contribution, context, authorship, recourse, or human consequences can fail to make rights effective. Specific claims about standards decisions, company motives, or why proposals did not advance belong to a sourced project-history narrative, not an unsupported blanket assertion.

The project owner's lived experience of doing this work unpaid while living in a camper, under an uncertain future, is part of the game's design motivation. A scenario can be fictional, based on lived experience, or a mixture; whether it is fictional or factual does not affect whether the simulation runs or whether its causal mechanics can be tested. Courts and relevant real-world processes determine evidentiary facts in actual cases. The game models pathways and outcomes; it does not adjudicate testimony or police what stories may be told.

Quantitative claims such as a particular average legal-aid amount, coverage rate, or outcome rate need definitions and sources before appearing as real-world statistics. That evidence requirement does not reduce the structural problem to an unverified claim: it makes numerical descriptions precise enough to support the work.

## Modelling rules

1. **Keep distinct things distinct.** Knowledge is not a transferable object that can be stolen. Record separately: authored artifacts or records; attribution and claims; permission or licence; project governance and control; money and contracts; and practical or tacit know-how held by people. A copy of a plan does not imply transfer of its creator's understanding.
2. **Represent action and response as events.** Contributions, funding decisions, legal inquiries, public claims, retaliation, housing changes, and remedies have actors, times or intervals, targets, costs, provenance, and outcomes. Record an attempted action separately from a granted remedy.
3. **Make causality inspectable and defeasible.** Use `contributedTo`, `preceded`, `associatedWith`, or a sourced causal claim as appropriate. Do not silently turn temporal order or correlation into proven causation. Allow conflicting claims to coexist without deleting either.
4. **Preserve unequal access and uncertainty.** Eligibility, nominal assistance, actual service, affordability, delay, capacity, outcome, and appeal are separate variables. Model barriers and unmet need as structural states; source time- and jurisdiction-specific numerical values rather than encoding them as timeless Australian rules.
5. **Track distributions, not just totals.** Attribute money, unpaid hours, risk, credit, authority, capability, and losses to the people, organisations, and places that receive or bear them.
6. **Treat severe distress with dignity.** It can be a serious downstream outcome in a person's history and in community-level evidence. It is never a reward, spectacle, bargaining tactic, or mechanically optimal route. Insurance is a distinct contract and beneficiary outcome; it does not compensate for death or make it an acceptable strategy. Do not imply that a person dies by suicide to provide for family unless a particular, carefully sourced account supports that interpretation and can be represented ethically.

## Core entities

- `Person`: a person with changing resources, obligations, abilities, relationships, housing episodes, and preferences.
- `Community`, `Project`, `Organisation`, `Committee`, `Agency`, `Funder`, `Contractor`, `LegalService`, `CourtOrTribunal`, `PoliceService`, `Insurer`, `Household`, and `Place`.
- `Initiative`: a proposal or body of work with an initiator, identified need, evolving scope, contributors, artifacts, governance, and versions.
- `Contribution`: paid or unpaid labour, money, equipment, care, expertise, access, or risk. Include contributor, recipient, amount or hours where known, valuation method, consent, terms, recognition, and evidence.
- `Artifact`: plans, specifications, code, research, designs, records, or other authored material. Keep artifact custody, access, copying, attribution, licence, and ownership claims separate from tacit know-how.
- `Decision` and `Claim`: a decision has a decision-maker, authority, process, reasons, affected parties, and review route. A claim has an asserter, subject, proposition, evidence, status, contesting claims, and adjudication state.
- `StoryContribution` and `NarrativeTransformation`: an account as the storyteller declares it, its provenance and privacy choices, the transformation requested, and the resulting narrative with any uncertainty or dispute made legible.
- `CommunityChoice` and `CommunityPosition`: a time- and place-specific collective decision plus distinct positions, affected groups, dissent, decision process, and limits of representation. Do not infer unanimity from a leader, survey, vote, or character's statement.
- `FundingArrangement`: source, amount or in-kind value, eligibility, application work, decision, recipient, conditions, control, spending, audit, repayment, and maintenance obligations.
- `AccessEpisode`: a person's time-bounded access to a service or remedy, with jurisdiction, matter type, eligibility, means test, scope, provider, funding basis, delay, work delivered, unmet need, and outcome.
- `HousingEpisode`, `FinancialStrainEpisode`, `HealthOrDistressEpisode`, `SafetyEvent`, `InsurancePolicy`, `Death`, and `Bereavement`: use only the detail the story or evidence supports, with privacy and dignity safeguards.
- `EvidenceItem` and `StatisticalObservation`: evidence has source, author, date, jurisdiction, population, method, limitations, permissions, and reliability assessment. An observation has a defined measure, numerator/denominator or estimate, period, population, geography, uncertainty, and source.

## Relations and event vocabulary

### Consequential investment choice

Gameplay is the measurement environment for choices about socioeconomic resources. Each accepted action is a `ResourceInvestmentDecision`, including actions that spend time, shift control, provide free work, withhold maintenance, or do nothing while an opportunity expires. Capture the observed before/after world state and the action tape so a saved playthrough can be replayed and assessed. At minimum record:

- the decision-maker and day/sequence;
- resource type, source, quantity, and direction (personal or shared money, workday/time, skill or knowledge, physical asset, care, social support, authority, risk);
- intended purpose and WordNet-backed vocabulary tags;
- immediate costs and who carries them;
- available alternatives and the opportunity each foregone route represents;
- who controls the resulting funds, artifact, service, or decision;
- intended and observed beneficiaries, contribution, payment, credit, and exposure;
- immediate state changes plus later housing/basic-needs, service, project capability, maintenance, accountability, and community-economy outcomes.

Use `web/fixtures/investment-taxonomy.json` as the first game crosswalk from WordNet lemmas and parts of speech to project-specific categories. The current source is the user's local Open English WordNet 2025+ copy at `C:/github/ontology/english-wordnet-2025-plus`. Its `.q42` header reports volume version 3; its embedded Q42LEX reports format version 4 and 1,732,603 lexical entries. This is distinct from the older Princeton WordNet 3.1 `.q42` in QualiaDB, whose lexicon is empty. The game consumes a small crosswalk and does not copy either full ontology volume. Keep WordNet's lexical relationships distinct from the game's socioeconomic definitions and causal rules.

The source graph is available for actual sense resolution, but the current game WASM package does not expose the QualiaDB Q42 range-query path needed to query subject/predicate patterns efficiently and resolve each candidate to its synset and gloss. The QualiaDB Rust `Q42RangeVolume` already plans object lookups through BIDX and field lookups through FIDX/PIDX; the missing integration is a bounded browser/WASM interface with Q42LEX v4 resolution and paged results. Track this as a generic QualiaDB capability task, not a game-only query implementation. Until that interface is available, keep each vocabulary entry explicitly marked as a lemma/POS candidate and do not present an unverified sense assignment as resolved.

The starting categories cover investment/allocation, contribution/work, cost/opportunity cost, benefit/return, risk/exposure, capability/maintenance, and control/governance. They are analytical descriptors, not virtue, criminality, social worth, or a pre-judgment of the community's preference. Gameplay provides observations for evaluating and refining the taxonomy. Compare routes from equivalent initial conditions and inspect distributions over people, projects, and communities; do not claim a synthetic playthrough is a public statistic or empirical finding. Public data mapping later tests prevalence and calibrates parameters. Participant stories later add different conditions and decisions without changing the measurement schema.

### Initiative, contribution, and capture

`initiatedBy`, `respondsToNeed`, `contributedTo`, `contributionReceivedBy`, `selfFunded`, `unpaidWork`, `artifactCreatedBy`, `artifactUsedBy`, `artifactCopiedBy`, `creditedTo`, `creditWithheldFrom`, `permissionGrantedBy`, `licenceAppliesTo`, `governedBy`, `controlledBy`, `fundsControlledBy`, `contractAwardedTo`, `knowHowHeldBy`, `knowHowSharedWith`, `trainingDeliveredBy`, `capabilityAvailableAt`, `maintenanceProvidedBy`, `benefits`, `bearsCost`, `retaliatedAgainst`, `remedySought`, and `remedyOutcome`.

A later organisation can use an artifact, take governance control, receive a grant, omit credit, and hire a different contractor; model each transition. The resulting service may deliver a public benefit and still exclude the initiator. It may also have lower practical capability if no one transfers or develops the required know-how. Conversely, the initiator may freely teach a local crew without pay, with costs to their time and finances and benefits to local capability. Neither route is collapsed into an assumption that knowledge was stolen.

### Economic and legal access

For each legal-access episode, distinguish at least:

- jurisdiction and date;
- issue/matter classification (for example, personal, family, criminal, administrative, civil, commercial, intellectual-property, or mixed);
- the applicable program/provider and its rules at that time;
- eligibility, means test, priorities, exclusions, and application result;
- service requested (information, advice, document assistance, representation, mediation, or review);
- service actually delivered, provider capacity, wait, geographic access, and out-of-pocket cost;
- funding model (public grant, capped allocation, pro bono, community-funded, self-funded, or none);
- case-level expenditure or value **only where an auditable source defines what the figure means**;
- remedy sought, result, duration, and residual harm.

Commercial disputes falling outside practical legal-aid coverage, and personal legal needs receiving only minimal assistance, are structural conditions the game must be capable of modelling. The exact Australian eligibility rule and the reported “about $80 per personal case” figure are jurisdiction- and period-specific empirical details to source. The latter requires its denominator, period, jurisdiction, mean/median or allocation status, included services and costs, and whether it means actual expenditure or an estimate before it is presented numerically. Preserve the underlying experience of a formal right without usable help; source the particular rule or amount accurately. A person without funded representation may seek advice elsewhere, act without representation, use a tribunal or community route, abandon a claim, or face delay; each route has its own availability, cost, and consequences.

Model the pre-funding work too: identifying a suitable program, forming a committee, securing endorsement, writing applications, meeting procurement rules, and carrying the costs while waiting. Funding may be declined, delayed, conditional, captured by another controller, mismanaged, or used well. A grant paid to an organisation does not mean the originator received funds or could afford a legal response.

### Harm, housing, health, and family

Represent a sequence as linked events rather than a single moral score: contribution or dispute → income/time/opportunity loss → debt or reduced resources → housing pressure or displacement → health, safety, relationship, or family impacts. In authored scenarios, the causal chain is executable game logic whether fictional or based on lived events. Multiple causes and alternative pathways remain possible. A project can improve community services while its initiator's housing worsens; a person can face reputational harm without a formal finding; a dispute can remain unresolved. Quantitative and documentary modes can attach source evidence later without changing the fictional simulation's behavior.

Homelessness, insecure housing, camping, or a move are time-bounded episodes with dates, places where safe to record, conditions, and uncertainty. Distinguish housing status from character, culpability, or worth. Severe distress and suicide-related outcomes must be represented as human loss and public-health concerns, not as character failure, deserved consequence, plot twist, or a means of protecting dependants financially. Family protection, insurance eligibility, exclusions, beneficiary, delay, and payment are separate facts and may be uncertain or absent.

### Institutional integrity and contested evidence

Use the logic already available in QualiaDB to keep obligations, knowledge states, temporal intervals, contradictions, arguments, and provenance explicit. Examples: a deontic duty to maintain can coexist with a recorded funding gap; two actors may make incompatible claims about authorship; an officer's conflict disclosure is evidence of a relationship, not proof of an unlawful decision; a rejected appeal is a failed process outcome, not proof that the original claim was false. Preserve source statements and counter-evidence. Do not award a single hidden virtue score for “good” or “bad” people.

### Cumulative causality and international representation

The game must model how repeated, coordinated choices can make lawful mitigation practically unavailable even when lawful rights and procedures formally exist. Represent the chain as linked decisions and changing capacities, not as one vague “corruption” flag:

```text
initiative and unpaid contribution
  -> artifacts, evidence, or funding opportunity become valuable
  -> control/credit/procurement shifts to better-resourced actors
  -> originator loses income, access, standing, time, or practical ability to challenge
  -> legal, administrative, or public-interest remedies are delayed, conflicted, unaffordable, or ineffective
  -> local capability, trust, and legitimate representation weaken
  -> cross-border opportunities flow to actors who still have capital, networks, and institutional access
  -> communities may become more dependent on intermediaries they cannot effectively scrutinise
```

Each arrow is a defeasible causal mechanism with event records and competing explanations. A story can show how a sophisticated pattern (fragmented decisions, proxy actors, control of records or funds, procurement barriers, retaliation, conflicts, delay) disables a lawful actor's response. It must record which actors made which decisions, what they knew or were obliged to do, which protections were available in theory, and why those protections did not work in practice. The result can compound prior decisions by other people across years, without falsely attributing every downstream condition to the affected person's character or one isolated choice.

At community and international scales, model `Opportunity`, `InternationalRepresentation`, `Representative`, `InstitutionalCapacity`, `LegitimacyAssessment`, and `CrossBorderRelationship`. Track who can submit bids, speak for a project, obtain finance, enter agreements, provide technical delivery, meet compliance, and maintain the resulting service. Distinguish practical capacity from legitimate authority and from a polished announcement. A captured or hollowed-out local capability can leave a community with fewer good-faith partners and greater dependence on external providers. The game may explore a further risk that poorly accountable intermediaries with financial or political reach dominate international representation, including pathways associated with rights abuse, trafficking, or money laundering. The system must let people state such concerns without an AI evidence gate. The simulation can run the scenario without adjudicating the real-world truth of a claim; only a documentary mode must keep its source status clear, and actual legal findings remain for courts and other competent processes.

Basic dignity is part of productive and civic capacity, not a cosmetic reward after a project succeeds. Model a safe place to camp, sleep, wash, access a hot shower, get water, store belongings, and maintain health as separate services with access rules, opening hours, cost, safety, accessibility, transport, privacy, and reliability. A shelter upgrade is not housing security; a water tank is not washing access; a project announcement is not a usable service. Show how missing facilities consume time and money, impede work and participation, and can make formal economic or international opportunities inaccessible. Also show competing investment choices: funds used for procurement or a committee may mean no shower this week; preserving someone's basic needs may delay a bid but retain their ability to contribute. No need should be automatically traded away as a hidden penalty or resolved by a single benevolent button.

The central comparative test is not whether a criminally controlled route can produce some immediate income or service. It is whether a properly functioning system can produce a better, durable outcome while protecting people and their rights. Test at least three paths from the same starting conditions: (a) an exploitative or criminally controlled arrangement that can provide immediate access or money but leaves power, safety, and future capacity exposed; (b) a nominally lawful route where a person raises corruption but receives no investigative resources, so the matter is not properly examined and the person bears delay, cost, exposure, and possible retaliation; and (c) a properly resourced, independent, rights-respecting route with basic-needs support, qualified investigation, protected evidence handling, legal assistance, transparent procedure, and a real path to community delivery. The third path must be materially better on safety, access to justice, service quality, local capability, and sustained public benefit. That is the success criterion for the software and the institutions it represents—not an AI moral verdict on which people deserve help.

Do not fund the response by making the complainant pay to investigate the alleged corruption. Investigation capacity, independence, and protection are responsibilities of the system being modelled. Where those resources are absent, show the resulting lack of investigation and the ongoing harm explicitly. Where they are supplied, let an independent process reach a fair outcome; “better” means people are protected, facts can be examined, decisions can be challenged, and the community can get a reliable service. It does not mean guaranteeing that every allegation is upheld or that one contributor wins every dispute. A fictional test case can exercise the whole chain without claiming to be evidence about an actual accused person.

The game should allow interventions at several points: preserve a dated record, build independent community governance, share capability under agreed terms, obtain fair procurement, disclose conflicts, seek review, tell and contextualise an account, build lawful cross-jurisdictional partnerships, or decide a route is too costly. Each route has realistic constraints and may fail. A successful community outcome does not erase personal harm; a failed remedy does not prove consent; and an effective intervention can improve future capability without repairing everything already lost.

### Confidential evidence and privacy-preserving articulation

### Story mode and factual adjudication

Every scenario can run as fiction, a participant-authored story, or a documentary mode. The choice of mode does not disable simulation or change the scenario mechanics. In fiction mode, the author can specify events and outcomes directly. In story mode, the narrator can express what happened as the contributor describes it and turn the described choices into playable events. Documentary mode can attach sources and jurisdiction-specific records when that is useful. The game does not demand documentation as a precondition for play and does not decide which testimony is true; evidentiary findings about actual disputes belong to courts and other competent processes. QualiaDB can preserve source and claim relations where the chosen mode requires them, while the game tests causal behavior regardless of mode.

The planned “tell your story” feature must let a person or community declare what happened in their own terms and work through the issue in a game narrative. The AI must not be a permission gate, censor, or arbiter that decides whether a story about reality may be told. Its role is to help express, structure, and contextualise; it must not silently rewrite the storyteller's account into a safer institutional version or suppress it because the account is contested, politically inconvenient, or critical of powerful actors. The storyteller may choose what to submit, the intended audience, and optional privacy transformations. Offer redaction, pseudonyms, composites, restricted storage, and a no-retention path, and explain their trade-offs; do not require anonymisation as the price of telling a story. Keep storage and reuse behavior clear, minimise collected data, and give the storyteller control of their own submitted material. Do not make disclosure a gate to gameplay, services, or credibility.

Narrative transformations should preserve the storyteller's account and distinguish their testimony from added narration, reconstruction, and other people's accounts. The system may offer a draft, optional questions, provenance labels, and a visible account of what it transformed; the storyteller can use, change, or reject that draft. This review is a tool for the storyteller, not an AI approval step. The game must not claim that generated prose is a verbatim account or independent corroboration. Where accounts conflict, present their sources and differences without letting the AI decide which person is allowed to speak.

### Community preference, economic survival, and rights

The simulation must allow people in different places to describe or choose arrangements that an outside institution would classify as informal, illicit, or criminally operated—including choices perceived locally as the most viable economic path under the available alternatives. That choice belongs to the people concerned, not to an AI narrator, distant civilian evaluator, donor, or single self-appointed representative. Model expressed preference, decision process, who participated, who dissented or could not participate, immediate livelihood and security, distribution of gains and risk, coercion, future options, and consequences for rights and safety as separate facts. Local preference must not be assumed to be unanimous, permanent, freely chosen, or harmless; nor should outsiders' disapproval automatically erase the community's agency.

Show the actual trade-offs and causal consequences over time: income, food, shelter, washing, public goods, debt, violence or coercion where present, exclusion, environmental effects, state response, international access, and options to change course. Do not treat criminal governance as automatically optimal because it pays now, or automatically inferior because an outside authority names it. Distinguish a community's collective choice from the conduct of powerful actors who impose it or exploit those with less power. Preserve each affected person's rights and account of harm, including minority and dissenting voices, without substituting an AI moral score for deliberation. If a UN or other international framework is later used, show its actual instrument, interpretation, jurisdiction, and the people's own engagement with it; do not present one model narrator as speaking for the UN or settling a contested local decision.

## Statistical evidence plan (later task)

When research begins, build a source register before inserting numeric values. For every measure, record: exact definition; source and release; jurisdiction; period; population and exclusions; collection method; numerator, denominator, and units; disaggregation; missingness; revisions; uncertainty; and known limitations. Separate administrative counts from survey estimates, budget allocations from expenditure, expenditure from service value, and a case sample from a population rate.

Potential evidence families to investigate include legal-aid service rules and annual reports; public budgets and audited expenditure; court and tribunal data; homelessness and housing-service statistics; income, debt, and financial-stress surveys; grant/procurement records; workplace and volunteer contribution research; and public-health suicide data. These are a later plan for quantifying and contextualising the problem, not a prerequisite for recognising that it exists. Match each source to the population and period the story depicts. Do not combine unlike data into a spurious causal estimate.

## Ongoing development sequence

This is a continuing programme, not a one-shot requirements exercise. The QualiaDB ontology, logic capabilities, game mechanics, public-data mappings, and community-authored stories are developed in successive loops. Later evidence enriches and tests the model; it does not retroactively grant permission for the game to describe the problem.

1. **Keep defining the domain and the engine.** Continue the work already invested in QualiaDB: structural ontology, causal and temporal relations, duties and rights, conflicting claims, arguments, capability, evidence provenance, and community/project/person state. Implement missing pieces as general QualiaDB capabilities when the game exposes a real gap. Keep the QualiaDB repository read-only for this game task unless explicitly authorised; record generic upstream requirements here or in the agreed upstream backlog.
2. **Build the playable causal model now.** Add interlocking pathways with resources, time, dependencies, competing actors, partial remedies, delayed effects, and replayable provenance. The game's journal should show which decisions changed what capacity and who bore the costs. A community benefit may coexist with personal dispossession; a contributor may share knowledge freely; attempts at lawful remedy may be blocked or fail.
3. **Map public datasets when the ontology is ready to receive them.** The intended Australian sources include data.gov.au and ABS releases, alongside relevant legal-aid, housing, health, grants, procurement, and justice datasets. For each dataset create a versioned mapping profile: publisher, dataset/release, licence, geography, time range, definitions, units, population, privacy limits, and transformation into QualiaDB observations. Preserve the source measure as published and map it to game concepts without pretending that a proxy measure is an exact match. Public-data ingestion can quantify reach, reveal regional differences, and calibrate scenarios; it should not overwrite testimony or collapse local differences into a single national average.
4. **Add community story contribution when that feature is built.** Let people narrate their realities directly and turn those accounts into playable material. The AI assists with structure and scenario mechanics; it does not decide whether an account may be declared. The scenario runs whether it is fictional, first-person, composite, or based on later-sourced public data. Courts and real-world authorities—not this simulator—decide legal findings. Offer privacy choices and clear storage behavior, and preserve narrator agency without requiring private correspondence or public proof.
5. **Use an iterative feedback loop.** Community ideas and story contributions may propose new institutions, choices, costs, remedies, and outcomes. Convert recurring needs into ontology additions and generic QualiaDB capability requests, then into game mechanics. Compare under-resourced lawful processes, exploitative alternatives, and properly resourced systems against the same human outcomes; revise until a rights-respecting path can reliably deliver a better result. Preserve disagreement and avoid treating one community, country, or international framework as the universal template.

The phases can overlap. The ordering prevents premature statistical mapping onto unstable concepts; it does not delay the game's representation of current lived and structural realities until a later phase is complete.

Fictional scenes should label invented cases and mechanics as dramatization. Documentary or investigative records should preserve source, date, scope, uncertainty, and corrections where available; lack of a public citation is not an AI-controlled bar on narration. Protect personal data and private sources, while recognising the storyteller's right to decide what they say about their own experience.

## Minimum fields for an evidence-backed claim

```text
claim_id
subject
proposition
jurisdiction
valid_time
asserted_time
asserter
status: fictional | reported | alleged | corroborated | adjudicated | statistical_estimate
source_ids[]
method_or_basis
counterclaims[]
uncertainty_and_limits
privacy_and_consent
```

A causal edge should add `mechanism`, `temporal_order`, `alternatives_considered`, `evidence_strength`, and `who disputes it`. A statistical observation should add `measure_definition`, `population`, `period`, `geography`, `numerator`, `denominator`, `estimate`, `uncertainty`, and `revision_date`.

## Implementation slices

1. Add typed records for structural barriers, rights, duties, capabilities, remedies, and evidence. These categories are foundational and should be queryable in QualiaDB. Separately type the provenance of an individual scene, assertion, finding, and statistic.
2. Add a playable, time- and jurisdiction-scoped legal-access pathway: research eligibility, request advice, obtain limited help, seek community/pro-bono support, proceed alone, or stop. Make cost, delay, availability, and outcomes explicit and unequal.
3. Add event-based household economics so unpaid work, self-funding, delay, lost income, debt, and housing pressure can accumulate and be interrupted by several plausible routes.
4. Add attribution, artifact access, governance, grant control, procurement, and practical capability as separate dimensions. Include voluntary unpaid teaching and paid teaching as different options.
5. Add community, family, institutional and personal outcomes without making one compensate automatically for another; allow multiple community positions and materially different local priorities.
6. Design story contribution around the storyteller's freedom to narrate, data minimisation, clear retention choices, optional privacy transformations, transparent AI-assisted changes, and no AI approval gate. Protect private source material without making public proof a condition for speaking.
7. Add source ingestion only after a reviewed source register exists; preserve provenance and conflicts in QualiaDB records.

## Known game gaps to close

- The current Kestrel campaign can patch shelter and restore shared water, but it has no playable, reliable hot-shower or washing-access service. Add this as a basic community facility with site access, water, heat, safety/privacy, a realistic build or service-access pathway, operating capacity, and maintenance—not as a decorative icon.
- The current corruption branch can be documented and sent to conflicted police, who close it without action. The game still needs the comparative positive route: an independently resourced investigation with protected participation, competent review, and an actionable community remedy. The complainant should not have to self-fund the investigation. A fair process can still reach an adverse or inconclusive finding while materially improving safety and access to justice.
- The first WordNet crosswalk is present and its categories are used in the in-game decision journal. Resolve ambiguous vocabulary to actual WordNet 3.1 synsets from the Q42 graph and retain those IDs and version metadata in the mapping. Keep the full 127 MB volume upstream rather than copying it into the game repository.
- The comparative test harness should replay the same starting state through under-resourced lawful response, exploitative/criminal capture, and properly resourced rights-respecting alternatives. Report measured differences in basic needs, investigation, safety, payment, control, capability, maintenance, and sustained SDG service delivery.

## Current boundary

The current character and events are authored fiction, while the structural model is intended to express real barriers and consequences that motivate the project. Research for exact Australian legal-aid rules and population statistics is future work to deepen and measure the account; it does not decide whether the problem exists or whether a fictional scenario runs correctly. The game and its funding case should articulate the problem now through the lived pattern and its causal mechanisms. This document is an ontology and implementation specification for game-demo and a generic capability brief for QualiaDB. If the engine lacks reusable support for provenance, temporal/causal relations, contested claims, rights/duties, access episodes, investment measurement, or statistical observations, record and implement those as upstream QualiaDB capabilities—not game-only workarounds.
