# `sizeof` aggregate assignment-expression metadata

Date: 2026-06-30

Cust supports aggregate-valued assignment expressions such as `left = right` and `number = replacement`, but `sizeof((left = right))` must remain non-evaluating while still reporting the assigned aggregate type's size.

## Root cause

`Interpreter::sizeof_expr()` routed `Expr::Assign { name, .. }` through `sizeof_assignment_result(name)`. That helper knew scalar and pointer assignment result sizes but rejected `Value::Struct` with `struct variable '<name>' assignment is not supported`, even though evaluated aggregate assignment expressions are supported elsewhere.

## Fix pattern

For `Value::Struct { type_name, .. }`, `sizeof_assignment_result()` should look up `type_name` in `self.struct_types` and return the deterministic aggregate size. Keep arrays/aggregate arrays rejected through the existing assignment diagnostics.

This also fixes comma-expression RHS forms such as `sizeof((marker = marker + 1, left = right))` because `Expr::Comma(_, right)` delegates to RHS `sizeof_expr(right)` without evaluating the left operand.

## Fixtures

- Interpreter: `tests/fixtures/valid/sizeof_aggregate_assignment_expressions.c`
- Native oracle: `tests/fixtures/compat/valid/sizeof_aggregate_assignment_expressions.c`

The fixture verifies:

- `sizeof((left = right)) == sizeof(struct Pair)`
- `sizeof((number = replacement)) == sizeof(union Number)`
- assignment side effects do not run inside `sizeof`
- comma-left side effects remain unevaluated when the RHS is an aggregate assignment expression
