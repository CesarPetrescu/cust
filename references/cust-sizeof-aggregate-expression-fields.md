# sizeof fields on aggregate-valued expressions

2026-05-09 autonomous run.

## Feature slice

Cust now supports `sizeof` on fields selected from aggregate-valued expressions, without evaluating the aggregate expression operand:

- aggregate assignment expressions: `sizeof((left = right).x)`
- aggregate-valued conditionals: `sizeof((cond ? left : right).tag)`
- aggregate-valued comma expressions: `sizeof((side_effect(), right).x)`
- aggregate-returning calls: `sizeof(make_point(7).tag)`
- union-valued assignment/conditional/call shapes: `sizeof((number = make_number(8)).value)` and `sizeof((cond ? number : make_number(9)).tag)`

The regression fixture intentionally verifies non-evaluation by placing assignments/function calls inside the `sizeof` operand and checking that marker state and left-hand aggregates remain unchanged.

## Implementation notes

- `Interpreter::sizeof_expr` now routes `Expr::AggregateFieldGet` / `AggregateFieldSet` / `AggregateFieldCompoundSet` through `aggregate_expr_type_name()` instead of requiring the aggregate root to be a raw `Expr::AggregateLiteral`.
- `aggregate_expr_type_name()` infers aggregate type metadata syntactically/runtime-metadata-only for the existing aggregate expression forms already handled by `eval_struct_expr`: literals, variables, aggregate assignments, pointer dereference assignments, calls, conditionals, comma expressions, aggregate field gets, array gets over aggregate arrays/pointers, and dereferences over aggregate pointers.
- `aggregate_field_type_name()` walks nested aggregate fields when the selected field itself is a struct/union, so `sizeof(expr.nested.x)` can continue using `sizeof_aggregate_field_type()` for the final field.

## Tests

- Interpreter fixture: `tests/fixtures/valid/sizeof_aggregate_expression_fields.c`
- Compiler-oracle fixture: `tests/fixtures/compat/valid/sizeof_aggregate_expression_fields.c`
- Focused commands:
  - `cargo test --test interpreter supports_sizeof_fields_on_aggregate_valued_expressions -- --nocapture`
  - `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`

## Pitfalls

- Do not call `eval_struct_expr()` from `sizeof` metadata inference; it would execute assignment/comma/call side effects in a non-evaluating context.
- Native C fixture must avoid asserting exact `sizeof(int)` values. Use `sizeof(field) == sizeof(type)` boolean checks so the exit code stays ABI-independent.
