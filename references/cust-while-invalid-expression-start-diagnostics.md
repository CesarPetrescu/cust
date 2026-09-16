---
title: Contextual invalid expression starts in C `while` conditions
---

# Contextual invalid nonempty `while` condition starts

## Scope

A `while` condition must start with a Cust expression. Binary-only, assignment,
and comparison operators cannot begin one, so they should identify the `while`
context instead of falling through to the generic primary-expression diagnostic.

## Implementation

Keep the refinement local to `Parser::parse_while()`. After the existing
`reject_missing_control_condition_expr("while")` preserves structural,
EOF, and keyword-specific diagnostics, call
`reject_invalid_while_condition_expr()` before `parse_expr()`. It rejects
binary-only operators, every assignment operator, and comparison/shift
operators with the source-located form:

```text
expected expression after while, found <Token>
```

Do not reject legal unary starts (`+`, `-`, `++`, `--`, `!`, `~`, `*`, `&`),
parenthesized expressions, literals, identifiers, or `_Generic`. Keeping this
guard separate from the shared control-condition helper prevents this bounded
`while` diagnostic closure from changing `if` or `do-while` behavior.

## Regression shape

Use one exact matrix covering representative binary (`/`, `&&`), assignment
(`=`, `%=`), equality (`==`), and relational (`<`) starts with line/column
assertions. In the same focused test, retain unary, grouped, and scalar
conditions. Run:

```bash
cargo test --test interpreter rejects_invalid_nonempty_while_condition_starts_with_context -- --nocapture
cargo test --test interpreter missing_control_flow_condition_expressions -- --nocapture
cargo test --test interpreter keyword_start_return_and_control_expressions -- --nocapture
```

## Boundary

This is diagnostic closure only: it does not broaden Cust's expression grammar
or change the existing structural/keyword control-condition diagnostics.
