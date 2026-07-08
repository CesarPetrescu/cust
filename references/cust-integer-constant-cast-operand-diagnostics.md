# Integer-constant cast operand diagnostics

Date: 2026-07-08

Cust supports scalar casts inside parser-folded integer constant expressions for enum initializers, switch case labels, and fixed-size array lengths. When the cast type has been consumed, missing or malformed operands must be diagnosed at the post-cast operand token instead of falling through to generic integer-constant parsing.

Coverage added in `tests/interpreter.rs`:

- `enum Bad { VALUE = (int) };` -> `expected integer constant after cast, found RBrace`
- `int values[(int)];` -> `expected integer constant after cast, found RBracket`
- `case (int):` -> `expected integer constant after cast, found Colon`
- `enum Bad { VALUE = (int)return };` -> `expected integer constant after cast, found Return`

Implementation note: keep the guard in `parse_integer_constant_cast(...)` immediately after `expect_closing_paren_after("integer constant expression cast type")` and before `parse_integer_constant_unary(...)`. This preserves valid scalar casts while catching delimiter, selector, invalid operand-start, and keyword starts with source locations.
