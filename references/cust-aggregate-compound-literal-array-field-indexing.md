# Aggregate compound-literal array-field indexing/address-of

Run: 2026-05-09 autonomous maintenance

## Feature

Cust now supports direct postfix indexing and element address-of on array-valued fields of aggregate compound literals:

- Scalar array field indexing: `((struct Packet){{1, 2}}).values[1]`
- Scalar array field element address-of: `&((struct Packet){{1, 2}}).values[1]`
- Aggregate-array field indexing followed by field access: `((struct Line){{{1, 2}}}).points[0].x`
- Aggregate-array field element address-of: `&((struct Line){{{1, 2}}}).points[0]`

## Implementation notes

Parser postfix indexing routes `Expr::AggregateFieldGet` through the existing pointer machinery by lowering `field[index]` to a dereference of pointer arithmetic over the field-decay expression. This keeps scalar-array and aggregate-array fields on aggregate compound literals aligned with existing pointer decay paths and preserves `&*` identity lowering for address-of element expressions.

`eval_pointer_arithmetic` now special-cases literal integer offsets before probing both operands as pointers. Without this, `pointer + 0` could misclassify the `0` literal as a null pointer and report `cannot add two pointers`.

## Tests

- `tests/fixtures/valid/aggregate_compound_literal_array_field_indexing.c`
- `tests/fixtures/compat/valid/aggregate_compound_literal_array_field_indexing.c`
- `tests/fixtures/invalid/aggregate_compound_literal_array_field_element_const_discard.c`
- `tests/interpreter.rs` direct valid and invalid regressions
- `tests/c_compat.rs` compiler-oracle fixture registration

## Pitfalls

- Do not evaluate compound-literal initializers in non-evaluating `sizeof` paths; this feature is runtime postfix indexing/address-of only.
- Keep native compiler-oracle fixtures warning-free under `-Wall -Wextra -Werror`.
- Pointer arithmetic involving literal `0` must not be treated as pointer + null pointer; classify integer literal offsets before pointer probing.
