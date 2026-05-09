# Aggregate compound-literal array fields

2026-05-09 autonomous run.

## Feature

Cust now treats array-valued fields read from aggregate compound literals as pointer-capable expressions:

- Scalar array fields, e.g. `((struct Packet){{1, 2, 3}}).values`, continue to decay through the existing `ArrayBase` pointer path.
- Embedded aggregate-array fields, e.g. `((struct Line){{{1, 2}, {3, 4}}}).points`, now decay to `struct Point *` / `union T *` by installing hidden current-scope aggregate-array storage and returning a `PointerValue::StructElement` to element zero.
- Pointer arithmetic, `->` field access, and pointer/array parameter binding then reuse the existing aggregate-array pointer machinery.
- Const aggregate/scalar array fields on aggregate compound literals now report `cannot discard const qualifier from pointer target` when assigned to mutable pointer targets.

## Implementation notes

- `Interpreter::eval_aggregate_literal_field_pointer` handles `StructFieldValue::StructArray` by deep-cloning the field elements into a hidden `Value::StructArray` named `__cust_compound_aggregate_field_array#N` in the current scope.
- This mirrors the existing aggregate-array compound literal storage/lifetime pattern and avoids adding another pointer variant.
- `Interpreter::pointer_expr_points_to_const` now distinguishes aggregate-literal field metadata by field type: scalar/aggregate array fields use the field `is_const` flag; pointer fields continue to use `points_to_const` so `int * const field` does not incorrectly become pointer-to-const.

## Coverage

- Valid interpreter fixture: `tests/fixtures/valid/aggregate_compound_literal_array_fields.c`.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/aggregate_compound_literal_array_fields.c`.
- Invalid const-discard fixtures:
  - `tests/fixtures/invalid/aggregate_compound_literal_array_field_const_discard.c`
  - `tests/fixtures/invalid/aggregate_compound_literal_pointer_field_const_discard.c`

## Pitfalls

- Do not point directly into a temporary `StructFieldValue::StructArray`; it is stored as a `Vec<HashMap<...>>`, not shared pointer storage. Use hidden scope storage so existing `StructElement` pointer reads/writes, arithmetic, and lifetime checks stay consistent.
- For pointer fields on aggregate compound literals, keep using `points_to_const`, not `is_const`, for pointee const conversion. The field's `is_const` means the pointer slot is const, not that the pointee is const.
