# Broader integer constant expressions for enums and switch cases

Date: 2026-05-11

Cust now uses a small parser-side integer-constant-expression evaluator for enum initializer values and `switch case` labels. This keeps constant expressions non-evaluating and independent of runtime variables while supporting the operators needed by current fixtures.

## Supported parser-side operators

The evaluator handles, in C-like precedence order:

- Parentheses
- Unary `+`, `-`, `~`, and `!`
- Multiplicative `*`, `/`, and `%` with the existing `division by zero` diagnostic
- Additive `+` and `-`
- Shifts `<<` and `>>` with a targeted non-negative shift-count check
- Bitwise `&`, `^`, and `|`
- Identifiers that resolve to earlier constants in the same enum declaration or to visible enum constants from parser enum-constant scopes

This is intentionally narrower than full C integer constant expressions: logical `&&`/`||`, comparisons, equality, `sizeof`, `_Alignof`, casts, and conditional expressions are not part of this parser helper yet.

## Implementation notes

- `parse_enum_constant_value` and `parse_switch_case_value` route through the shared `parse_integer_constant_expr` helper chain.
- The helper returns both the folded `i64` and first token location so duplicate `switch case` diagnostics can still point at the label expression.
- Keep this path parser-only; do not evaluate normal Cust expressions or read runtime variables while parsing enum/case labels.

## Fixture notes

- `tests/fixtures/valid/switch_enum_case_labels.c` and its compiler-oracle twin cover parenthesized enum constants, multiplicative/additive enum initializers, shift/bitwise case labels, and unary logical-not case labels.
- The fixture avoids native ABI assumptions and returns a stable exit code under the existing C compiler-oracle harness.
