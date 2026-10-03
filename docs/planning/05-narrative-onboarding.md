# Maslows Challenge: Narrative Onboarding and Life Chapters

## Purpose

A **LifeChapter** is optional player-authored context for why a person or group
is travelling, rebuilding, studying, caring, seeking community, or changing
direction. It contains only player-confirmed scenario facts, goals, preferences,
and explicitly modelled planning constraints. It is separate from identity,
skills, town data, and authoritative game simulation.

It must help the game offer relevant, humane possibilities without diagnosing a
player, forcing disclosure, or trapping them in a label. Players can skip it,
play a wholly fictional character, revise it, or delete it at any time.

## Four equal entry paths

| Path | Experience | Canonical result |
|---|---|---|
| Guided form | Direct fields for goals, party, skills, dwelling, support, and privacy | Player-entered scenario seed |
| Branching journey | Short, revisable choices explaining possible paths and trade-offs | Player-confirmed scenario seed |
| Free writing | “Tell us only what you want the game to know” | Private text plus reviewable proposed seed |
| Starting template | Select and customise a broad life-chapter direction | Player-confirmed scenario seed |

The guided form is sufficient. No player requires an LLM, free-writing ability,
or sensitive disclosure to begin.

## Natural-language to scenario seed

Natural-language onboarding is assistive authoring, not an autonomous profile
engine.

```text
private player text
  → local/approved bounded LLM extraction
  → proposed structured seed + uncertainty markers
  → player edits, removes, accepts, or rejects each proposed item
  → schema validation
  → explicit local commit to a LifeChapter graph
```

The LLM receives only text selected by the player and a constrained schema. It
may propose goals, time constraints, companions, preferred language, skills,
support needs, or dwelling preferences. It must not infer heritage, gender,
health, pregnancy, relationship safety, legal status, finances, children, or
other sensitive facts the player did not state. Uncertain extraction appears as
a question, never as a committed fact.

The output cannot create ontology terms, edit rules, create resources, or bypass
consent. Original text is retained only if the player elects to retain it;
otherwise the save holds confirmed structured facts only. A no-LLM form path is
always available.

## Scenario directions

Life chapters are composable circumstances, not mutually exclusive labels. A
student can be a parent; a traveller can be a skilled contributor; a person in
a career transition can travel with friends.

| Direction | Player-chosen game-relevant choices | Never assumed |
|---|---|---|
| Rest, recovery, or career transition | Pace, budget horizon, privacy, skills to share, connection goals | Diagnosis, wealth, family status |
| Study or apprenticeship | Learning goal, schedule, budget, location preference, existing skills | Academic success, income, housing eligibility |
| Backpacking or long-distance travel | Companions, route flexibility, language confidence, vehicle/shelter preference | Nationality, visa/legal status, risk tolerance |
| Relationship or household transition | Preferred support, membership, separate/shared resources, safe-contact preference | Cause of separation, abuse, custody, reconciliation goal |
| Pregnancy, parenting, or care planning | Player-chosen continuity/care goal, age bands, space/time/support requirements | Medical state, reproductive decision, clinical advice |
| International travel or pilgrimage | Purpose, languages, cultural/faith context if volunteered, reflection goals | Religion, nationality, legal status, access permission |
| Disaster or major life disruption | Stability, recovered capabilities, support preferences, rebuilding goals | Trauma, fault, insurance/legal outcome |
| Community project or service | Expertise, teaching willingness, project ambition, collaboration preference | Unpaid labour obligation or leadership role |
| Custom chapter | Any player-authored mix of hopes, constraints, and tone | A need to fit a template |

Sensitive subjects use non-judgmental language and “prefer not to say.” The game
offers fictional planning choices, never medical, legal, financial, immigration,
or crisis advice; it never pressures someone to narrate a painful event.

## Canonical model and mechanics

```text
LifeChapter
  id, version, status, createdAt, revisedAt
  selectedTemplates[]
  playerNarrative?                 optional private source text
  goals[], hopes[], constraints[], preferences[]
  partyReferences[], capabilityReferences[]
  scenarioHooks[]                  validated, optional, transparent
  provenance[]                     authoring method, confirmation/revision
```

Scenario hooks may introduce relevant opportunities, tutorial wording, optional
contacts, or planning requirements. They cannot impose irreversible events,
alter core rules, or manufacture facts. Each declares its source and can be
disabled.

This enables practical, non-punitive variation: a learner sees courses and
mentors; a party gets planning for seats, beds, time, shared resources, and
consent; a household can select age-band-appropriate space/support planning; a
visitor can choose language-aware dialogue; a skilled person can teach or lead a
project. Every visible effect explains which confirmed fact or preference
enabled it.

## Privacy and v0.1 acceptance

LifeChapter data is private/local by default and excluded from town packs,
telemetry, exports, sharing, and agent context. Field scopes distinguish
private, gameplay-only, player-visible, and future explicit share permission.
Templates require sensitivity review; no template rewards trauma disclosure or
penalises privacy.

v0.1 ships a guided form, three customisable templates—learner, solo/party
traveller, community contributor—and custom free text without LLM
interpretation. Branching and LLM proposal flows remain feature-flagged until
tests prove per-field confirmation, no-sensitive-inference, schema boundaries,
revision/deletion, template composition, deterministic hooks, and exclusion from
default agent/export queries.
