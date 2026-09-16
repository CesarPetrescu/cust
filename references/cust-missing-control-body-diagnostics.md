# Contextual missing control-body diagnostics

## Scope

Use this pattern when a supported control form accepts either one statement or a block after its header. Cust routes `if`, `while`, `do`, `for`, and `else` through `parse_control_body_after()`. `switch` is different: it requires `{` and already uses `expect_opening_brace_after("switch expression")`.

## Implementation rule

Reject only demonstrated tokens that cannot begin a Cust statement, before falling through to `parse_stmt()`. Keep the error source-located at the next token:

```text
expected statement after <control context>, found <Token>
```

Use the caller's existing context (`if condition`, `while condition`, `do`, `for clauses`, or `else`) so valid syntax still flows through the shared parser. Do not broaden the guard to tokens that have their own useful statement diagnostics (`case`, `default`, `goto`) or to valid declaration/expression/control starts.

## Regression matrix

For each statement-bodied control, assert exact diagnostics for:

- missing body before `RBrace`;
- missing body at EOF; and
- a punctuation/operator start such as `Comma`.

Also retain valid empty, block, expression, nested control, dangling-`else`, and switch block-only behavior. The 2026-09-16 TODO 430 test uses a 15-case exact source-location matrix: four statement-bodied controls × three invalid starts plus three preserved switch opening-brace diagnostics.

## Review checklist

- The guard is shared only by statement-bodied controls.
- `switch` remains block-only and unchanged.
- Existing `else` diagnostics retain their wording.
- Run focused new, valid-control, and dangling-else tests before the canonical gate.
