# Cust switch invalid-expression-start diagnostics

## Scope

TODO 437 closes malformed, nonempty `switch` controlling expressions whose first token cannot begin a Cust expression. The parser must retain contextual source-located diagnostics rather than falling through to generic primary-expression errors.

## TDD evidence

The focused interpreter regression first showed generic `expected expression, found <Token>` errors for `/`, `=`, `==`, `&&`, `%=`, and `<` immediately after `switch (`. The desired diagnostics are `expected expression after switch, found <Token>` at the same token location.

## Implementation

After `parse_switch()` validates the opening parenthesis and existing structural missing-expression cases, call the shared `reject_invalid_control_condition_expr("switch")` before `parse_expr()`. The helper is deliberately narrow: it rejects binary-only, assignment, equality, relational, shift, and compound-assignment starts without consuming any token. It does not affect valid unary, grouped, scalar, enum, assignment-expression, or `_Generic` switch expressions.

## Coverage

- Exact six-token diagnostic matrix: `/`, `=`, `==`, `&&`, `%=`, `<`.
- Positive interpreter cases for unary, grouped, scalar-variable, and enum conditions.
- Existing switch fallthrough/break/continue and enum-case-label tests remain part of the focused preservation checks.

No compiler-oracle fixture is needed: the work only changes Cust diagnostic context for invalid programs.
