# Subscript comma expressions

Date: 2026-07-10

Cust now accepts C comma expressions directly inside supported subscript/index expressions, such as `values[i++, 2]`, `text[i++, 1]`, `points[i++, 2].y`, and `cursor[i++, 1].x`.

Implementation notes:

- `parse_index_expr()` keeps the existing contextual invalid-start guard before parsing the first operand.
- The first operand still uses the previous `parse_logical_or()` path so legacy missing-`]` diagnostics like `values[0 = 3;` remain stable.
- A comma after the first operand is parsed into `Expr::Comma(left, rhs)`, with the RHS parsed as an assignment expression and missing-RHS diagnostics matching the ordinary comma operator.
- This avoids confusing subscript comma support with array-designator comma handling; designator indexes still use parser-folded integer constant expressions and intentionally reject comma operators.

Coverage:

- `tests/fixtures/valid/subscript_comma_expressions.c`
- `tests/fixtures/compat/valid/subscript_comma_expressions.c`
- Focused interpreter test `supports_comma_expressions_in_subscript_indices`
- Native compiler-oracle fixture in `tests/c_compat.rs`
