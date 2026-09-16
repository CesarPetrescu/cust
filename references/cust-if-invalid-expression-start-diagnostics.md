---
title: Contextual invalid expression starts in C `if` conditions
---

# Contextual invalid nonempty `if` condition starts

## Scope

An `if` condition must start with a Cust expression. Binary-only, assignment,
and comparison operators cannot begin one, so they should identify the `if`
context instead of falling through to the generic primary-expression diagnostic.

## Implementation

After `Parser::parse_if()` preserves structural, EOF, and keyword-specific
condition diagnostics through `reject_missing_control_condition_expr("if")`,
call the shared `reject_invalid_control_condition_expr("if")` before
`parse_expr()`. It rejects binary-only operators, every assignment operator,
and comparison/shift operators with this source-located form:

```text
expected expression after if, found <Token> at line <line>, column <column>
```

Keep the structural/keyword guard first. Do not reject legal unary starts
(`+`, `-`, `++`, `--`, `!`, `~`, `*`, `&`), parenthesized expressions,
literals, identifiers, or `_Generic`.

## Regression shape

`rejects_invalid_nonempty_if_condition_starts_with_context` covers
representative binary (`/`, `&&`), assignment (`=`, `%=`), equality (`==`),
and relational (`<`) starts with exact locations. It also retains legal unary,
grouped, and scalar conditions.

Run:

```bash
cargo test --test interpreter rejects_invalid_nonempty_if_condition_starts_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_nonempty_while_condition_starts_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_nonempty_do_while_condition_starts_with_context -- --nocapture
```

## Boundary

This is diagnostic closure only: it does not broaden Cust's expression grammar
or change existing structural/keyword control-condition diagnostics.
