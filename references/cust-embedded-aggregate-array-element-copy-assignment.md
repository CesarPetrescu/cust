# Embedded aggregate-array element copy assignment

2026-06-21 autonomous run notes.

## Scope

Closed the parity gap for assigning whole struct/union values into aggregate-array fields embedded in structs:

- `line.points[1] = replacement;`
- `struct Point returned = (line.points[1] = replacement);`
- `slot->points[0] = (struct Point){11, 12};`

The feature deep-copies same-type aggregate RHS values into the selected embedded aggregate-array element and returns a by-value copy from assignment-expression contexts.

## Implementation notes

- Direct field syntax parses as `Expr::StructArraySet`; that node is shared with scalar array fields, so aggregate routing must first detect `direct_struct_aggregate_array_field_type(name, fields)?.is_some()`.
- Direct embedded aggregate-array writes use `eval_struct_field_array_element_assignment_expr`, which:
  - evaluates the RHS with `eval_struct_expr`,
  - checks root variable mutability,
  - uses `checked_struct_aggregate_array_field_index` for bounds/diagnostics,
  - rejects const aggregate-array fields,
  - mutates the embedded element through `struct_field_array_element_fields_mut`,
  - enforces same aggregate type and const-field copy-assignment diagnostics,
  - deep-clones fields with `StructFieldValue::deep_clone_fields`.
- Struct-pointer field syntax (`slot->points[i] = aggregate`) already lowers through `AddressOfStructPtrArrayField` to `Expr::DerefSet`; discard-context handling must detect aggregate pointer targets by evaluating the pointer and checking `find_struct_pointer_fields`, not only by looking for `Expr::Var` pointer variables.

## Tests

- Valid interpreter fixture: `tests/fixtures/valid/embedded_aggregate_array_element_assignment.c`
- Native compiler-oracle fixture: `tests/fixtures/compat/valid/embedded_aggregate_array_element_assignment.c`
- Invalid mismatch fixture: `tests/fixtures/invalid/embedded_aggregate_array_element_assignment_type_mismatch.c`
- Focused tests:
  - `cargo test --test interpreter supports_embedded_aggregate_array_element_copy_assignment -- --nocapture`
  - `cargo test --test interpreter rejects_embedded_aggregate_array_element_assignment_type_mismatch -- --nocapture`
  - `cargo test --test c_compat -- --nocapture`

## Pitfalls

- Running `cargo test --test interpreter embedded_aggregate_array_element_assignment -- --nocapture` filtered to only the invalid test because the valid test name contains `embedded_aggregate_array_element_copy_assignment`; use exact focused substrings or run both commands.
- Do not route all `StructArraySet` nodes through aggregate assignment; scalar array fields still need the existing scalar `eval_struct_array_set` path.
