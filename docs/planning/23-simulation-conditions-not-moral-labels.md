# Simulation: conditions, not moral labels

Status: design lock. The game is a **simulation environment**, not Earth and not
a jural act. It is accurate about hard outcomes so players can work through
philosophical and practical questions inside the frame — including harms that
often meet indifference in the real world (exposure, poverty, no housing, bad
driving that wrecks instruments or ends a life).

## What the mechanic stores

| Claim | Kind | Not |
| --- | --- | --- |
| Exposure, poverty, no housing | **Conditions** on the scene / dwelling | Proof someone is good or bad |
| Wealth / resources | **Resources** on instruments and grounds | Proof of kindness or villainy |
| Bad drive | **Act** of the driver (fictional participant) | Blame transferred to the other instrument alone |
| Car / van hit | **Instrument** state (impact, write-off) | A who |
| Kindness / harm | Read from **what they do** in the scene | Halo or villain palette from wealth or poverty |

A cell may set conditions and acts. It must not derive good or bad from poverty
or wealth. The story can ask the question; it does not rewrite the mechanic so
one side is spared.

## Custody

Custody follows a **warranting act** only. The subject must be human
(`human`, `NaturalAgent`, or `who`). A company, an agent, a tool, or an
instrument cannot be in a cell.

Poverty, wealth, lack of housing, and exposure never authorize custody and
never paint guilt or innocence. They stay conditions. A cell in the scene is
a look after that act — expensive and appropriate when the act warrants it,
not a default for the poor, and not a reading of wealth as innocence.

The shell allowlist is short (`acts_frame`):

| Act | Frame light | Custody |
| --- | --- | --- |
| `act.violence_against_person` | villain | yes, human only |
| `act.theft_with_force` | villain | yes, human only |
| `act.harm` | villain | no |
| `act.illegal_drug_supply` | villain | yes, human only; never inferred from no housing |
| `act.kindness`, `act.care` | halo | no |

`condition.poverty`, `condition.wealth`, `condition.exposure`, and
`condition.no_housing` are not acts. They do not open a cell and they do not
set halo or villain light. Good and bad still read from what someone does.

### No housing is not a costume

No housing / homelessness is **never** a junkie costume or look. Injury and
no medication are **conditions** on that human: stylised and dignified, not
guilt and not gore (`condition.injury`, `condition.no_medication`).

Illegal drug **supply** is a separate act on the person who supplies
(`act.illegal_drug_supply`). Villain light and custody-relevant acts attach
there, not to no-housing. A cell must not infer a drug act from no housing,
injury, or no medication. The shell refuses that mapping.

### Pathways stay named

Paths into no housing stay **separate** conditions. They are not one homeless
kind and not human garbage. None of them is a crime. A help cell must name
the condition. A single silhouette that hides which path it is is refused,
because that hide also hides what would help (housing, health).

| Condition | Not |
| --- | --- |
| `condition.cannot_afford_child` | A crime, or a preference about abortion or faith |
| `condition.spent_caring_for_relative` | Guilt |
| `condition.lost_business` | A moral label |
| `condition.returned_from_war` | Villain light |
| `condition.unhappy_relationship_mortgage_stuck` | Custody. Mortgage due, nowhere to go, is still a condition |

The shell keeps each id as written. It does not rewrite one of these to a
generic homeless id.

## Presentation

Same original Pixar-like appeal on everyone. A worn van and a polished car do
not tell you who is kind.

**Halo and villain light are allowed**, and they come only from **what someone
does** in the scene — never from poverty, wealth, or the finish of a vehicle.
They sit as light on the frame, not as a costume and not as the car's paint, so
the same van can read either way. The palette shows the act; it does not change
the mechanic and does not store good/bad as a who-kind.

Civics sites ≠ civic government ≠ game.

## Privacy

Never take a name or resident from a public-data snapshot into a wreck or a
moral judgement. Life-ending stays a scene receipt (blurred figure allowed as
look — see [22](22-destructible-kinds-catalog.md)).
