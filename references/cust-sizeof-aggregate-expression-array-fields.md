# sizeof aggregate-expression array fields

2026-05-09 autonomous run.

## Feature

Cust now reports full array-object sizes for array fields selected from aggregate-valued expressions in `sizeof`, without evaluating the aggregate expression:

- `sizeof(make_line(10).points)` for embedded aggregate-array fields.
- `sizeof((line = make_line(20)).numbers)` for assignment-result aggregate expressions.
- `sizeof((1 ? line : make_line(30)).values)` for scalar array fields selected from aggregate conditionals.
- `sizeof(make_box(40).line.points)` for nested aggregate-valued expression field paths.

## Implementation note

The prior `sizeof_aggregate_field_type` path already handled scalar array fields (`StructFieldType::Array`) and scalar/nested/pointer fields. The missing case was `StructFieldType::StructArray` when it was the final selected field. Add a final-field arm that looks up the embedded aggregate element type in `struct_types`, uses its deterministic Cust size, and multiplies by the declared field length. This keeps `sizeof` metadata-only and avoids evaluating calls, assignments, or unselected conditional branches.

## Tests

- Interpreter fixture: `tests/fixtures/valid/sizeof_aggregate_expression_array_fields.c` checks deterministic Cust sizes and marker variables proving `sizeof` did not evaluate operands.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/sizeof_aggregate_expression_array_fields.c` compares ABI-independent array-length ratios rather than native byte sizes.

Focused commands:

```bash
cargo test --test interpreter supports_sizeof_array_fields_on_aggregate_valued_expressions -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
