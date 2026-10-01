# Rolling Commons: Decision Register

This register distinguishes agreed direction from questions that require a
verified technical or product decision. A decision is not implementation proof;
each implementation-affecting decision also needs an automated test or a
recorded capability-spike result.

| ID | Status | Decision / question | Rationale and next evidence |
|---|---|---|---|
| D-001 | Accepted | The game is an original, peaceful life-and-community simulation. | Use inspiration only at the level of genre/mechanics; produce original content and assets. |
| D-002 | Accepted | Browser-first Rust/WASM is the primary runtime. | Demonstrates local-first portability; Phase 0 must establish the exact browser/toolchain baseline. |
| D-003 | Accepted | Rust reducer plus graph/rules own authoritative game state. | Ensures deterministic replay and explainable outcomes. |
| D-004 | Accepted | LLMs propose/narrate; they never mutate state directly. | All agent proposals go through scope, grounding, policy, validation, and reducer gates. |
| D-005 | Accepted | VibeScript is bounded authored content, not unrestricted embedded JavaScript. | Script access is capability-scoped and cannot call storage/reducer/browser APIs directly. |
| D-006 | Accepted | Q42 describes and persists semantic state; `.10d` carries dense 3D visual geometry. | Preserve linked manifests, digests, units/frame, provenance, and semantic IDs. |
| D-007 | Accepted | The first release is offline-capable and fully playable without an LLM. | Eliminates network/model availability as a gameplay dependency. |
| D-008 | Accepted | Place data is snapshot-based, aggregate, provenance-bearing, and licence-gated. | Avoid live-API dependence, privacy harm, and nondeterministic replay. |
| D-009 | Accepted | The first scenario is fictional/data-shaped before a real-town demonstration pack. | Lets game mechanics and safety controls mature without claims about a real place. |
| D-010 | Accepted | Australia is the v0.1 regional focus; the core model remains region-neutral. | Australian dwelling, travel, climate, place-data, and community-ground packs are first. Regional expression belongs in packs and policy profiles, not hard-coded mechanics. |
| D-011 | Accepted | Presentation is a swappable profile, separate from authoritative simulation and place content. | The same semantic world/save can support original storybook 3D, illustrated point-and-click, or RPG-oriented views without changing rules or state. |
| D-012 | Accepted | Player identity, circumstances, household/party, and skills are explicit, optional local-first profiles. | Identity is self-defined; sensitive identity attributes never determine capability, worth, access, or difficulty. Circumstances and care needs affect planning only where the player chooses to model them. |
| D-013 | Accepted | A background story is an optional, editable, private `LifeChapter` composed of player-confirmed facts, hopes, constraints, and narrative tone. | It can be formed through guided choices, a branching journey, free text, or a mix. It is never a diagnosis, a mandatory disclosure, or an automatic penalty system. |
| D-014 | Accepted | Natural-language onboarding may propose a constrained scenario seed, but cannot automatically change ontology/rules or infer sensitive facts. | The player reviews every proposed fact, skill, relationship, need, and mechanic before commit; no-LLM structured path remains complete. |
| D-015 | Accepted | Unique rules are declarative, ontology-predicate-based, versioned content with validation, tests, explanation, and explicit activation. | N3Logic/SHACL/deontic rules constrain and explain commands; Rust remains the deterministic reducer. |
| D-016 | Accepted | Engineering and economics are typed, provenance-bearing scenario models with explicit assumptions, uncertainty, and safety boundaries. | They inform game choices and rules; they are not design, process, financial, environmental, regulatory, or operational advice. |
| D-017 | Open | Which QualiaDB revision and WASM capability profile will be pinned? | Resolve in Phase 0 using a clean browser spike; record build/reproducibility and bundle/memory results. |
| D-018 | Open | Which web UI shell and package/build tooling will host the game? | Compare only choices compatible with pinned QualiaDB WASM surface and offline deployment. |
| D-019 | Open | Is the selected `webizen-render` + `.10d` path practical in target browsers? | Resolve with asset render, picking, memory, frame-time, and fallback spike. |
| D-020 | Open | Does the selected browser profile expose a practical graph-bounded local LLM path? | Prove one governed call; retain templates if it fails bundle/memory/latency budget. |
| D-021 | Open | What exact Australian public datasets and licences will form the first data-shaped pack? | Select only after source, licence, aggregation, privacy, and transformation review. |
| D-022 | Open | What browser baseline and performance budgets define v0.1 support? | Establish from Phase 0 measurements, not assumptions. |
| D-023 | Deferred | Real-time multiplayer/cooperation protocol. | Requires separate consent, identity, conflict, moderation, and security design; not needed for v0.1. |
| D-024 | Deferred | Community pack publishing/distribution model. | Begin with local curated packs; add signing/review/publishing after content tooling exists. |
| D-025 | Accepted | Wellbeing, social, physiological, and hazard scenarios are optional, private, source-linked simulation content with player-selected context. | The game can support rehearsal, reflection, and safety learning, but is not therapy, diagnosis, treatment, live triage, or an emergency service; sensitive data is never inferred, exported, or sent to agents by default. |
| D-026 | Accepted | Simulation outcomes reflect explicit choices, circumstances, maintenance, relationships, and systems, while preserving dignity and nonviolent support pathways. | Substance-use concerns, body/health constraints, trauma/stress, and abuse contexts are never score penalties or moral labels; they can reveal player-selected consequences, boundaries, support, recovery, and alternative plans. |
| D-027 | Accepted | Community grounds are a voluntary, mobility-centred community-living model with explicit participation criteria and exceptional/support pathways; they are not a substitute for formal housing, health, care, or wraparound services. | A scenario can require maintained simulated road-use responsibility for a particular ground, but never treats a licensing/health/support status as a measure of human worth or an entitlement decision. |
| D-028 | Accepted | Housing insecurity, tenure transitions, recurring obligations, local ties, and missing infrastructure are structural scenario conditions—not measures of a person's worth, eligibility, or capability. | Model material pressures and community capability gaps without a homelessness, poverty, compliance, or human-value score. SDG-aligned outcomes describe infrastructure effects, never rank people or decide access. |

## Decision procedure

1. Write a concise proposal with options, impact, compatibility, security,
   performance, licence, and maintenance implications.
2. Run the smallest practical spike or test where the question is technical.
3. Record result, selected option, revision/version, and follow-up work here.
4. Update dependent planning documents and tests in the same change.

No open decision permits a workaround that expands script, model, or data
authority beyond the accepted boundaries above.
