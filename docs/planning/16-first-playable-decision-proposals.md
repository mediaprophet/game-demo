# Rolling Commons: First-Playable Decision Proposals (draft)

Status: **draft proposals for review** — none of these are accepted decisions
until moved into the [decision register](00-decision-register.md). Prepared to
unblock Phase 0/1 work in parallel with the browser spikes.

## D-029 — First-playable audience

**Proposal: keep open; build the prototype audience-neutral.**

The smallest useful system contract already separates player-facing facts from
technical provenance ("Why?" shows facts first, technical detail on demand).
Build that two-layer evidence view as the single default rather than two
audience modes; record audience interviews before v0.1 so the release matrix
can pick defaults with evidence instead of guesses. Prototype work is not
blocked: tutorial and evidence defaults stay configurable.

Evidence still needed: the five concept interviews and written audience
statement from the open-questions table.

## D-030 — First-session objective and completion

**Proposal:** adopt the playtest script in
[the first-playable spec](12-first-playable-spec-and-open-questions.md) as the
objective contract:

- *Objective:* establish a workable place to stay while helping the community
  restore workshop power.
- *Completion condition:* the shared-power project reaches `commissioned`
  **or** ends with an explained, recoverable delay, and the player has at
  least one repaired/improved personal asset.
- *Session length target:* 30–45 minutes of deterministic turns; tune against
  observed playtest duration.
- *Non-goal:* no score, ranking, or "peace" rating; unresolved barriers stay
  visible.

Evidence still needed: paper/clickable playtest to confirm duration and
confusion points before locking.

## D-031 — Minimal tracked resources, turn costs, recovery

**Proposal for the versioned tuning sheet (v0):**

| Tracked quantity | Why it earns a place | Safe range |
|---|---|---|
| Time blocks per day (e.g. 6) | The core trade-off currency; travel + work consume blocks | n/a — fixed per day |
| Personal wallet credits | Enables trade-board choices and contribution decisions | 0..∞ (no debt in v0) |
| Workshop available power (W) | Drives the central project; makes staging legible | stated per fixture |
| Personal shelter/vehicle condition | One inspectable, repairable limitation | 3 named states: usable / degraded / unsafe |
| Project stock: named parts | Makes the trade/fulfilment loop real | count per part |

Deliberately excluded for v0: hunger/energy meters, mood, reputation — none
create a choice in this journey that the five above don't already cover.

Turn costs: inspection, asking, and UI navigation are free; travel, work
stages, and fulfilment consume time blocks. Rejected actions spend nothing.

Recovery: every required objective carries ≥1 alternate path (mentor,
service, substitute part, deferred completion); the "Why?" view must name a
next feasible step for every block.

## D-032 — Presentation profile

**Decided direction:** storybook-3D leads (per project owner, 2026-10-02).

Consequences carried into Phase 0: the D-019 spike (`.10d` →
`webizen-render` browser render + pick + low-spec path) is on the critical
path; the illustrated/2D profile remains the required alternate for
accessibility and devices without adequate WebGPU. The presentation-neutral
world projection is still built first so both profiles consume the same API.

## D-033 — Release matrix (v0.1 cut line)

| Fixture / capability | Class |
|---|---|
| Inspect shelter/ground; Why? evidence view | player-visible |
| Travel + time blocks; facility use; mentor learn | player-visible |
| Personal asset repair project (staged) | player-visible |
| Trade board: list → fulfil → inspect → accept | player-visible |
| Shared-power project: propose, endorse, contribute, approve, commission, upkeep | player-visible |
| Save/reload/reset + deterministic replay | player-visible |
| Private wallet + committee treasury (contribute only, no withdrawal for player) | player-visible |
| Committee consent/approval flow with review path (D-034) | player-visible |
| VibeScript-authored scenario cell | demonstration-only |
| OPFS/backup policy surface, telemetry | demonstration-only |
| Local LLM NPC inference | deferred (D-020 gate) |
| `.10d` animated characters (QG-03) | deferred |
| Real-town data pack (QG-07, licence gates) | deferred |
| Multiplayer, on-demand geography | deferred (D-023, D-041) |

## D-034 — Consent and committee approval in play

**Proposal:** one scripted flow in the shared-power project:

1. Player or off-site contributor proposes a `ProjectIdea` (no resource effect).
2. Steward role converts it to a `Project` draft; validation names access and
   treasury requirements separately.
3. Committee approval is a visible requirement with an accountable role; the
   player sees the pending decision and may keep working on other tasks.
4. One rejection/review path is authored: a substitute-parts proposal is held,
   the "Why?" view names the concern, and a revised submission is possible.
5. Contributing to the project or treasury never grants access or authority;
   the UI states this when a contribution is made.
