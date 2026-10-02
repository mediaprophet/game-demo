# ADR 0002: Session model — N3 world document, SHACL gates, event-sourced replay

Status: Accepted — verified in the browser slice 2026-10-02 (10/10 step
playthrough + byte-identical replay, see `web/game.html?selftest`).
Date: 2026-10-02

## Context

QualiaDB's wasm surface is stateless compute: it validates, derives, and
parses, but exports no mutable graph store (`GraphDatabase.sparql` on the
Vibe `LocalHost` is an honest stub returning an empty list). The game needs
world state that changes over a session, explains refused actions, and
replays deterministically.

## Decision

- The **world state is an N3 document** held by the host — one `s p o .`
  triple per line, raw `rc:`/`ev:` lexical tokens (the lexical form the
  engine's N3/SHACL path hashes and matches on).
- **Action gating is SHACL ShapeSpecs evaluated by the engine**
  (`validate_shacl_json_wasm`). The violation report (`focus_node`,
  `message`, `result_path`, `severity`) is the player-facing "Why?" payload.
- **Action effects are declared data** in `fixtures/actions.json`: `add`
  triple lines plus `removeN`/`removeAll` subject/predicate(/object)
  patterns. Resources are countable token triples (`rc:coin`,
  `rc:labourToken`) so the host performs set edits only — no game-side
  arithmetic or rule evaluation.
- **The log is event-sourced**: each accepted action appends `ev:eN`
  triples inside the same document. Replay = fresh seed + re-validate and
  re-apply each taped action id; byte-equality of the resulting document is
  the determinism receipt.
- Consent/approval is modeled as gated actions on `rc:CommunityProject`
  (`requestApproval` requires real installed parts; `switchPowerOn`
  requires parts AND endorsement) — contribution never confers authority.
- Saves are the document itself through `OpfsVfs`; `GameSession::new`
  recovers the tape and sequence counter from the loaded `ev:` lines.

## Consequences

- The slice is playable offline with no LLM and no GPU.
- "Blocked spends nothing" is structural: gates run before any edit.
- An imperfect/unresolved end state is just a shorter tape — replay still
  works.
- If QualiaDB later ships a wasm graph-store capability, the document model
  can migrate underneath the same action catalogue without changing gates
  or fixtures.
