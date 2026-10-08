# Maslows Challenge: First-Playable Spec and Open Questions

## Status and use

This is a **proposed design brief**, not an approved feature list. It translates
the [product vision](01-product-vision.md) into a small play experience that can
be prototyped and tested. The [decision register](00-decision-register.md)
remains authoritative for accepted direction; the open items below need a
decision before they become implementation contracts.

All technical implementation follows the
[QualiaDB-only development contract](13-qualiadb-only-development-contract.md).
Any missing capability in this proposed journey becomes upstream QualiaDB work
before it can be used by the game.

The first playable should answer two questions: **Is the journey enjoyable and
understandable without a technical explanation?** and **do graph facts, rules,
provenance, and replay make a visible difference to a player's choices?** A
feature that answers neither question belongs in a later slice.

## One-session experience to prototype

**Premise.** In a real Australian locality represented by a dated, attributed
geographic pack, the player arrives with a chosen simple shelter and one
practical goal: establish a workable place to stay while helping a community
project. The geographic base (terrain, waterways, roads, buildings, and public
facilities) comes from declared sources; characters, dialogue, and scenario
events are fictional unless individually sourced and consented. Game-built
changes are visibly separate from the real mapped ground. This is a scenario
premise, not a universal starting condition. A short setup offers a few
authored starts and a skip path; private identity or life-history disclosure
is never required.

**Initial situation.** The workshop has tools and willing contributors, but its
available power is too limited for a repair task. A shared solar and storage
improvement is possible if the group can agree on access, source suitable parts,
find the needed capability, and reserve time for upkeep. The player's own
shelter or vehicle has one inspectable, repairable limitation. At least two
routes forward must be visible from the start.

**Example sequence to playtest** (illustrative, subject to D-029–D-034):

1. Inspect the shelter and ground. See what is usable, what is uncertain, and
   which facts support those descriptions.
2. Choose a near-term action: use an available facility, learn from a mentor,
   obtain a part, or ask a steward about an agreement. Travel and time have
   understandable costs; inspection and asking for information remain safe ways
   to recover from uncertainty.
3. Attempt a task with a missing requirement. The **Why?** view names the
   requirement and offers at least two feasible paths, such as a mentor and a
   service or a substitute part. A rejected action spends no resources.
4. Repair or improve one personal asset through a visible staged project. The
   result changes a capability the player can use, not just a completion badge.
5. Propose or join the shared-power project. An off-site contributor can help;
   access and treasury authority remain separate from contribution.
6. Source a part through a fictional trading board, fulfil and inspect it, then
   complete a validated build stage. A community decision or agreement is shown
   as an actual requirement with an accountable role and a review path.
7. End the session with a usable workshop capacity, an upkeep responsibility,
   and a choice of what to do next. The timeline can replay how that outcome
   arose. An unresolved barrier remains visible if the player took another
   route; the game does not require a perfect outcome.

This sequence is a **playtest script**, not a mandatory order of actions. The
prototype should permit changing the order, deferring a project, and returning
later without a dead end. The authored story provides purpose and characters;
the rules explain the consequences.

## Smallest useful system contract

| System | First-playable behaviour | Observable proof |
|---|---|---|
| Turn and time | One explicit command advances the simulation when it has a cost; inspection and UI navigation do not. The exact clock scale is open. | Player can predict whether a choice consumes time; replay gives the same result. |
| Needs and resources | Track only quantities that create an interesting choice in this story, with units, source, and a visible safe range where applicable. | No hidden meter causes an unexplained loss; missing data says “unknown.” |
| Actions | Each action shows intent, requirements, possible cost, and likely outcome before confirmation. | A failed precondition returns a reason and possible alternatives without a partial commit. |
| Projects | A project has a goal, staged work, dependencies, contributors, approval, commissioning, and upkeep. | Personal repair and shared power use the same state model but different permissions. |
| Participation | People can help without residence, site access, or spending power. | Contribution never silently grants membership or treasury authority. |
| Trade | Listing, reservation, fulfilment, inspection, acceptance, and settlement are distinct steps. | A remote item cannot be installed before arrival and acceptance. |
| Evidence | **Why?** shows player-facing facts first and offers technical provenance on demand. | A player can tell what blocked a choice without knowing Q42 or logic notation. |
| Recovery | Every required objective has an alternate path or an explicit defer-and-return path. | Playtesters do not need to restart after a reasonable mistake. |
| Save | Local save, reload, reset, and deterministic replay. | Completing the same command sequence from the same seed yields the same state and event digest. |

Amounts, durations, capacities, prices, and difficulty curves should be authored
as a **versioned scenario tuning sheet**, with ranges and short reasons. The
planning docs intentionally do not invent real engineering, market, or legal
figures. Early playtests can use clearly fictional values; their purpose is to
check whether trade-offs are legible and fun.

## Outcomes and player feedback

The session needs a clear **local resolution**: the player has a viable next
place/action, and the shared project either changes workshop capability or has
an explained, recoverable reason for delay. This is a story and systems outcome,
not a score for dignity, wellbeing, poverty, or human rights. The broader
rights-condition view should show separate conditions, evidence, barriers, and
remedy paths; it must never collapse them into a “peace” rating.

Feedback should distinguish **known**, **uncertain**, **not yet permitted**, and
**impossible in this scenario**. Each blocked action should identify the next
useful choice. If a system cannot produce that explanation, the prototype should
not quietly present its result as authoritative.

## Release cut line

The mechanics may be tested with a fictional fixture, but the player-facing
community-development slice requires one real-place geographic pack with
source/licence evidence, an attributed map, terrain, waterways, roads and
buildings/facilities, plus a separate game-scenario overlay. The **v0.1
technical demonstration** must prove the verified browser QualiaDB profile,
semantic asset selection, offline persistence, a bounded VibeScript example,
local projects anchored to the geographic pack, and travel between mapped
places with recorded time/resource consequences. Local LLM inference, a second
full presentation profile, multiplayer, and broader simulation domains remain
extensions subject to their respective gates.

The detailed [implementation plan](04-implementation-plan.md) contains fixtures
that may be built in parallel or behind a developer flag. Their appearance in a
phase task list does not make every fixture part of the first player journey.
The release checklist must name which fixtures are player-visible, which are
technical demonstrations, and which remain research.

## Open specification gaps

These gaps are ordered by how strongly they affect a first playable. “Decide
with” names the smallest evidence that can settle the question, not a deadline.

| ID | Priority | Question to decide | Decide with / resulting artifact |
|---|---|---|---|
| D-029 | First | Who is the primary first-playable audience: players seeking an enjoyable life/community game, technical evaluators, or both in separate modes? | Five short concept interviews and a written audience statement; adjust tutorial and evidence-view defaults accordingly. |
| D-030 | First | What are the exact first-session objective, completion condition, and approximate session length? | Paper or clickable playtest of the example sequence; record intended and observed duration, confusion, and choice points. |
| D-031 | First | What is the smallest set of tracked resources, turn costs, and failure/recovery rules? | A versioned tuning sheet and a scripted playthrough that can recover from two common mistakes. |
| D-032 | First | Which presentation profile is the primary v0.1 experience, and what is the minimum alternate/low-spec view? | Browser spike plus two low-fidelity interaction mockups tested for readability and keyboard use. |
| D-033 | First | Which systems are required in the public v0.1 journey, and which are hidden fixtures or later content? | A release matrix mapping each planned fixture to player-visible, demonstration-only, or deferred. |
| D-034 | First | How do project consent, committee approval, review, and an unresolved dispute appear in ordinary play? | One scripted proposal, one rejection/review, and one alternate path with player-readable explanations. |
| D-035 | Next | What is the content scale and production budget for the first town (locations, characters, items, dialogue, art, audio)? | Scene/content inventory and one complete sample location; revise effort estimate after production. |
| D-036 | Next | Which accessibility baseline, supported inputs, browsers, devices, storage limits, and offline installation path define v0.1? | Phase 0 measurements and an accessibility review of the first interactive prototype. |
| D-037 | Next | What content review process covers lived experience, Aboriginal and Torres Strait Islander context, rights claims, hazards, and cultural representation? | Named review roles, scope, consent/compensation plan, and a pack-level sign-off checklist before related content ships. |
| D-038 | Next | How are saves migrated, backed up, deleted, and recovered from interrupted writes or storage eviction? | Versioned save contract, corruption/eviction fixture, and a player-facing recovery flow. |

The open questions should not be resolved by silently adding mechanics or making
claims about real people or places. Accepted answers belong in the decision
register, with dependent specs updated in the same change.
