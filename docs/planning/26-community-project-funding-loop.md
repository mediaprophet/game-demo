# Community project funding, harm, and power

## Design rule

Do not make community work fair, safe, affordable, or successful by default. The
game is about the real gap between useful work and the people or institutions
that control money, credit, access, and remedies. A good outcome for residents
can coexist with exploitation of its contributor. A harmed contributor can
also have harmed someone else. Neither fact cancels the other.

Keep evidence, allegation, responsibility, material conditions, institutional
decisions, and consequences as separate world facts. Do not compress them into
one virtue, trust, or reputation meter. Lawfulness, fairness, financial return,
relationship damage, and community benefit are distinct dimensions.

## Opening situation

The playable character is unhoused and camping at Kestrel Flats. They initiated
an earlier water project because no one was addressing the unmet need. A later
funding gap left unsafe access hours and harmed Mara. The character accepts
responsibility for taking the initiative and for the consequences of the work,
and has attempted repair without funding. That does not establish that they
controlled or could fund maintenance, or that they alone caused the harm. A
committee chair's claim about their responsibility is separately recorded as
disputed. Their standing is contested; neither homelessness nor initiative
makes them virtuous by default.

The committee is now using this character's solar plans while excluding them
from the project decision. The committee chair has a conflict of interest in
that reuse. The previous harm and the present appropriation are independent
events with different people affected. The character can record what is
verifiable, request a police review, or make the dispute public. The assigned
officer discloses a relationship with the chair and closes the review without
action. Public disclosure can cost a supporter after committee leaders label
the dispute disruptive. Neither route guarantees restored credit, payment, or
a legal remedy.

When grant money becomes available, the chair invokes the disputed history to
undermine the initiator's credibility with the funder. The funder gives the
committee the money and control while omitting the initiator. The committee can
still choose how much credit, pay, access, and maintenance to provide. A useful
outcome for residents does not undo the credibility attack.

## Playable community project loop

1. **Record the opening history.** Spend a workday documenting the current
   reuse of the plans. The event records the prior maintenance harm separately
   from the current dispute. Remedy-seeking and public disclosure remain
   optional and have different costs.
2. **Build a mandate.** Hear residents' concerns about access and maintenance.
   Convene a committee only after a resident supports the proposal. A broader
   mandate helps, but residents are not interchangeable approval tokens.
3. **Prepare the project.** Pay two personal coins and a workday to scope the
   work, then one coin and another workday to apply for a grant. Record the
   preparation as unpaid and self-funded.
4. **Wait for the decision.** The grant decision arrives after rest. The
   authored slice awards it with two resident endorsements and declines it
   with one. The threshold is visible and deterministic; it is a prototype,
   not a claim about how real funders behave.
5. **Respond to failure or choose a manager.** After a declined or lost grant,
   the player can spend time organizing five community coins, or spend three
   personal coins and another day. The latter leaves earlier costs unpaid.
   With a funded pool, fair hiring pays a local installer, records the
   contributor, and keeps maintenance and access terms. Insider contracting
   pays the chair's associate and can still deliver a community service, while
   denying credit and leaving maintenance unfunded. Grant mismanagement
   consumes the grant and requests an audit; it does not deliver service.
6. **Deliver a physical service.** Procurement readiness gates acquisition,
   installation, approval, and power-on. Only a powered workshop gives the
   community the service. A contract award, payment, or useful plan is not
   treated as a completed project.

The funded routes are deliberately asymmetric:

- **Compete as the originator.** Pay three personal coins and a day for
  registration, insurance, and tax requirements. Selection is not guaranteed.
  If selected, the creator is paid three committee coins and teaches local
  workers. Their earlier preparation and registration remain sunk costs.
- **Use the committee's installer.** The current plan can support a routine
  installation and the committee can fund maintenance, but the creator's
  tacit integration knowledge remains with them unless it is separately
  licensed and taught.
- **Accept government capture.** The department announces the work as an
  existing program, takes five grant coins and control, and omits the initiator.
  The initiator can spend two more coins and a day challenging this. The
  authored slice dismisses that challenge after rest and leaves control with
  the department.
- **Delegate to a political backer.** The department pays a connected
  contractor; the committee chair expects future political support. The route
  records a visible equipment/signage outcome, no transferred integration
  knowledge, and no funded maintenance. The map may look complete while the
  project's technical capability remains a shell.

These are pathways with different causal records, not equivalent ways to reach
one success score. The world separately tracks personal and committee/public
money, who receives a contract, project control, authorship, resident benefit,
technical capability, knowledge transfer, maintenance, reputation, and
relationship damage. These separate facts are available to VibeScript queries
and replay so later systems can infer a project's actual state without
equating a sign, grant, or powered light with a working service.

### Knowledge, artifacts, and technical capacity

The project may copy plans, take credit, gain the budget, or shift its legal
and administrative control. Those actions do not transfer the creator's
knowledge. The creator retains integration know-how until they are paid to
operate, license, or teach it. Other actors can produce a visibly installed
shell from documents and money without being able to maintain it, improve it,
or cooperate across jurisdictions.

For a later Australian AI facility scenario, apply the same model to distinct
capabilities: a building and signage; compute procurement; locally controlled
data and models; skilled local operations; interjurisdictional cooperation;
services usable locally and abroad; maintenance and independent improvement.
A captured grant may produce the building or an imported service while local
technical capacity stays absent. A foreign provider can be the only working
option in that branch, with faster service but external dependence and no
automatic domestic capability transfer. Track those as real tradeoffs and
specific outcomes, not a single “facility delivered” milestone.

## Character and institution mechanics

- Show material conditions such as housing, cash, food, health, unpaid time,
  and safe work separately from character traits and reputation.
- Track specific actions and their effects: who did the work, who controlled
  the budget, who received money or credit, who bore the risk, who benefited,
  and who was harmed.
- Separate possession of documents, authority over a budget/project, public
  authorship, and actual expertise. Copying plans cannot set a knowledge
  transfer fact. Require paid work, a licence, collaboration, or teaching to
  change who can operate and improve the system.
- Preserve uncertainty and provenance. A documented payment, an allegation,
  an officer's disclosed conflict, and a committee's account are not the same
  kind of fact. Give characters ways to challenge, correct, or corroborate a
  record, even when the challenge fails.
- Make remedy paths costly and contingent. Records can take unpaid time;
  police can be conflicted; committees can capture useful work; legal help can
  be unavailable; disclosure can bring retaliation or support. Do not make
  doing the fair thing a guaranteed route to money, safety, or vindication.
- Allow exploitative and unlawful strategies as authored choices, with their
  short-term gains and separate downstream consequences. Do not equate
  conduct with identity, and do not imply that experiencing harm excuses
  harming other people. Add these choices as scenario-specific mechanisms
  after defining their actual leverage, evidence, affected parties, and repair
  paths; avoid a generic “good/bad” option.
- Let a project benefit residents despite a contributor's poor treatment, and
  let a contributor's responsible repair attempt coexist with unresolved
  accusations. Community outcomes, personal outcomes, and institutional
  accountability can diverge.

## Current data and rule locations

- `web/fixtures/seed.ttl` holds the unhoused/camping status, the character's
  initiative on an unmet need, prior maintenance lapse, responsibility for
  undertaking the work and its effects, disputed blame, attempted but unfunded
  repair, current plans reuse, contested standing, technical know-how held by
  the creator, and the agency/political-contractor network.
- `web/fixtures/actions.json` declares action gates, costs, provenance facts,
  committee decisions, delayed funding outcomes, police review, and the
  community-funding/self-funding fallback.
- `crates/rolling-commons-shell/src/lib.rs` evaluates gates through the
  QualiaDB SHACL bridge and applies delayed transitions to replayable world
  events. Game policy stays in the game repository.
- `web/game.html` presents the dispatch, ledger, consequences, and available
  actions. It should not imply that an action succeeded beyond the world facts
  the rule engine recorded.

## Next mechanics to define

1. Add an independent/community review path with its own eligibility, costs,
   evidence standard, conflict disclosure, delays, and possible adverse or
   partial outcomes.
2. Make restitution and accountability playable for the character's earlier
   maintenance lapse. Repair should take time and resources, may not restore
   reputation, and should not erase Mara's harm.
3. Add a contributor-credit and payment dispute after appropriation, including
   evidence strength, no-cost and paid assistance, retaliation risk, lost-work
   opportunity cost, and remedies that can fail or arrive too late.
4. Add carefully scoped choices for coercive, deceptive, or unlawful conduct
   by players, committees, contractors, and officials. Record immediate
   benefits and who is harmed, plus later financial, legal, relational, and
   community effects. Include repair without guaranteeing forgiveness.
5. Expand the authored grant threshold into visible competition for finite
   funds, eligibility rules, committee factionalism, conflicts of interest,
   audit capacity, and maintenance over time.
6. Model people, communities, and projects with independent resources,
   priorities, boundaries, and histories. Do not assume any person is
   permanently agreeable, trustworthy, or beyond repair.

## QualiaDB boundary

The current slice uses the existing QualiaDB gate/transition and replay
capabilities. It makes no QualiaDB source changes. If implementation reveals a
missing general-purpose capability, record that as an upstream requirement
with a reusable non-game example before proposing an engine change.
