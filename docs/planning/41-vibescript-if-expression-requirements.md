# VibeScript: value-producing `if` expressions

## Status and scope

**Required upstream capability; not implemented by this document.** QualiaDB is read-only for the game-demo work. This file records the language change for the QualiaDB/VibeScript maintainer; it does not patch the engine or claim that the game now starts.

Reference revision: `355a836a77989f51486829479ebd417eb38c12de` (`game-demo/qualia.ref`).

The immediate missing feature is **an `if` that produces a value wherever an expression is allowed**, including `else if` chains and record fields. Statement-form `if` already exists and must remain compatible.

## Confirmed startup failure

The local game reports:

```text
Uncaught VibeScript character module: E001@352..354: expected expression
```

`web/fixtures/character-arcs.vibe:11` contains:

```vibe
let base = if running { 5.0 } else { 2.6 };
```

`CharacterSimulation::new` calls `vibe::load_program` on this source and returns the parse diagnostic. The constructor is called during `web/game.html` startup, so initialization stops before the game can render.

Source inspection confirms the language gap, rather than merely an old WASM package:

- `qualiaDB/crates/vibe/src/parse/stmt.rs:139–165` parses `if` as a statement.
- `qualiaDB/crates/vibe/src/parse/expr.rs:433–681` has no conditional-expression parser; an `if` in expression position reaches `expected expression`.
- `qualiaDB/crates/vibe/src/ast.rs:397–468` has no conditional-expression variant in `ExprKind`.
- `qualiaDB/crates/vibe/src/eval/stmt.rs:90–103` evaluates statement-form conditionals; `eval/expr.rs` has no matching expression case.

Rebuilding the same pinned engine alone will not add this feature.

## Syntax needed by the game

### Binding and assignment values

```vibe
let base = if running { 5.0 } else { 2.6 };
action = if st.supporters > 0.0 { "formCommittee" } else { "canvassMara" };
```

### Record-field values

```vibe
return {
    title: if is_commons { "The commons are open" } else { "The canal is connected" },
    action: if st.labour { "startGarden" } else { "endTurn" }
};
```

### Chained and nested conditions

```vibe
let action = if st.labour {
    "connectMastLabour"
} else if st.coins >= 3.0 {
    "connectMastCoins"
} else {
    "endTurn"
};
```

These forms must also compose in function arguments, list elements, returns, parentheses, and ordinary operators. Existing record literals must not be confused with the braces delimiting a conditional branch.

## Proposed minimal semantic contract

1. Add an explicit conditional expression node, for example `ExprKind::IfElse { cond, then_value, else_value }`, with an accurate source span. An `else if` chain can be represented as an `IfElse` in `else_value`.
2. For the initial feature, each branch contains **one result expression**, without a trailing semicolon. This is sufficient for the current game fixtures; general statement blocks with trailing values are a separate extension.
3. Require a final `else` in expression position. Do not silently invent `null` or another result when it is missing. Statement-form `if` may continue to omit `else`.
4. Evaluate the condition exactly once. Evaluate **only the selected branch**. Do not implement this as an ordinary eager helper function whose arguments evaluate both branches.
5. Preserve existing statement-conditional truthiness semantics (`Value::is_truthy`) unless the language owner explicitly versions a stricter Boolean-only rule. The game's examples already use Boolean conditions.
6. Return the selected branch's value unchanged. Apply existing assignment, return, and type-checking rules; do not introduce implicit string/number conversions. Where static result typing is available, incompatible branches must receive a clear diagnostic or follow an explicitly documented existing dynamic-value rule.
7. Keep branch-local expression evaluation under existing effect/capability restrictions. Static effect analysis must inspect **both** possible branches even though runtime execution selects one.
8. Preserve evaluation-step, allocation, code-size, and nesting limits. Charge execution through existing budget mechanisms; unselected branches must not execute host calls or consume runtime execution steps as though selected.
9. Specify precedence and postfix behavior. Parenthesized conditional results must work with member access, indexing, calls, and arithmetic without changing existing operator precedence or statement parsing.

## Upstream implementation surfaces

This needs more than accepting the token in the parser.

| Surface | Required work |
|---|---|
| `crates/vibe/src/ast.rs` | Represent the value-producing conditional and its source span. |
| `crates/vibe/src/parse/expr.rs` | Parse branch expressions, mandatory final `else`, chained conditions, nesting, and delimiters. Preserve `parse/stmt.rs` statement behavior. |
| `crates/vibe/src/eval/expr.rs` | Execute the condition and selected result lazily with normal budgeting and diagnostics. |
| `crates/vibe/src/check.rs` | Traverse condition and both branch values for effects, capabilities, and existing validation rules. |
| `crates/vibe/src/bytecode/compiler.rs` | Compile branch jumps so exactly one result reaches the merge point, with consistent stack depth and normal code-size bounds. Keep interpreter and bytecode semantics aligned. |
| `crates/vibe/src/cbor_ast/{encode,decode}.rs` | Add a stable expression tag and round-trip support; preserve existing tags and reject unsupported forms clearly. |
| `crates/vibe/src/decompiler.rs` | Emit parseable conditional-expression syntax with necessary parentheses. |
| Other expression walkers | Audit `ast_query.rs`, `reactive_cell.rs`, `vocab/shake.rs`, `projectional.rs`, `metamodel/node_kinds.rs`, and reflection for exhaustiveness and dependency traversal. These files reference `ExprKind`; inspect each before changing it. |
| Language documentation and release capability metadata | Specify the syntax/semantics and identify the first engine release supporting it. |

## Concrete game integration sites

- `web/fixtures/character-arcs.vibe:11`: walking/running base speed.
- `web/fixtures/campaign-rules.vibe`: record fields at lines 8–9, 24–25, 34–35, 111–112, 131–133, 143–144, 153–154, 174, and 185; action assignments around lines 50–80; bindings at lines 103, 118, 170, and 181.
- `crates/rolling-commons-shell/src/character_simulation.rs:14–19`: constructor/loading boundary that reports the current failure.
- `web/game.html:774–780`: character and campaign constructors invoked during startup.

Line numbers describe the inspected working tree and may move.

## Completion criteria

- The original character and campaign fixtures load unmodified through the public Vibe API, natively and in the shipped WASM build.
- `walk_speed("trailwise", false)` returns `3.25`; `walk_speed("steadfast", true)` returns `5.75`; ordinary walking/running retain `2.6`/`5.0`.
- Conditional record fields and chained campaign decisions return the same selected strings as the corresponding existing statement-form logic.
- An unselected branch containing an undefined value, division error, or permitted observable host call is not executed. Static capability restrictions still apply to either branch.
- Missing `else`, empty branches, and malformed delimiters produce targeted diagnostics at the offending span.
- AST serialization/decompilation preserve the expression; interpreter and bytecode results agree for supported branch value types.
- Existing statement-form conditionals and resource-budget enforcement remain intact.
- After the upstream change is approved and released, update `qualia.ref`, rebuild the game WASM, align release manifests/packs with its actual digest, and confirm `game.html` reaches a rendered playable scene without the constructor error.

## Interim game-side option

If local startup must be restored before the engine feature is released, mechanically rewrite the affected fixtures into existing statement-form conditionals:

```vibe
let mut base = 2.6;
if running { base = 5.0; }
```

For record fields, compute named values using statements before constructing the record. Preserve all branch ordering and campaign decisions. This is a compatibility workaround, **not** implementation of `if` expressions in VibeScript.

## Separate issues, not part of this language addition

The EONT and IMPO mini-header size mismatches discussed during model migration are QualiaDB container-codec defects, not VibeScript syntax features. They do not explain `E001@352..354` and should remain separate upstream work items. The model release hash alignment was completed separately; it does not resolve this parser failure.
