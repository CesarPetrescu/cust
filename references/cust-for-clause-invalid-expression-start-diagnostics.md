---
title: Contextual invalid expression starts in C `for` clauses
---

# Contextual invalid `for`-clause expression-start diagnostics

## Scope

Cust accepts declarations or expressions in the initializer and expressions in the
nonempty condition and increment clauses of `for (init; condition; increment)`.
A clause delimiter itself is valid only when the corresponding clause is empty.
Malformed nonempty clauses should identify both the clause and the offending token
rather than falling through to generic expression or statement diagnostics.

## Implementation

Keep the grammar decision local to `Parser::parse_for()`. Before parsing a
nonempty clause expression, call
`reject_invalid_for_clause_expression_start("for initializer"|"for condition"|"for increment")`.
The helper reports a source-located:

```text
expected expression after for <clause>, found <Token>
```

It guards tokens that cannot begin a Cust expression: binary/assignment/comparison
operators, closing delimiters, structural punctuation, a repeated semicolon, and
EOF. It must *not* reject legal unary starts such as `+`, `-`, `++`, `--`, `!`,
`~`, `*`, `&`, `sizeof`, `_Generic`, or `(`. Keep the pre-existing
statement-only-control-flow and integer-constant-start checks after this guard so
their more specific diagnostics retain precedence.

## Regression shape

Use exact source/location assertions for all three clauses, covering at least an
assignment operator, a closing bracket, and a delimiter/EOF marker. Preserve
valid empty clauses, a unary-complement condition, declaration initialization,
and assignment/expression clauses. Run:

```bash
cargo test --test interpreter rejects_invalid_for_clause_expression_starts_with_context -- --nocapture
cargo test --test interpreter rejects_statement_only_control_flow_in_for_clauses_with_context -- --nocapture
```

## Boundary

This is diagnostic closure only. It does not expand Cust's expression grammar or
change the existing statement-only control-flow and integer-constant diagnostics.
