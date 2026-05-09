# Cust nested struct array-field decay

Date: 2026-05-09

Implemented nested scalar array-field decay and element address-of const propagation for direct struct variables, struct-array elements, and struct pointers when the array field is behind one or more nested struct fields.

## Covered forms

- `one.inner.values` decays to an `int *`/`char *` pointer in pointer contexts.
- `&one.inner.values[i]` creates an interpreter-owned array-element pointer aliasing embedded field storage.
- `boxes[i].inner.values` and `&boxes[i].inner.values[j]` work for struct-array elements, including const struct-array conversion checks.
- `ptr->inner.values` and `&ptr->inner.values[i]` work for struct pointers.
- Const root structs such as `const struct Box box` and const struct arrays such as `const struct Box boxes[1]` now cause nested array-field decay to be treated as a pointer to const, so mutable pointer parameters reject it with `cannot discard const qualifier from pointer target`.

## Implementation notes

- Valid nested decay already routed through `nested_field_value`; the missing behavior was const inference for direct nested array-field decay in pointer conversion checks.
- Added `nested_field_path_is_const` and `direct_struct_array_field_points_to_const` in `src/lib.rs`.
- `pointer_expr_points_to_const(Expr::StructGet)` now combines direct array-field const inference with existing pointer-field `points_to_const` inference, avoiding regressions for pointer fields where a const pointer slot (`int * const p`) should not imply a const pointee.
- The compiler-oracle fixture avoids native ABI-size assumptions and returns only an exit-code behavior comparison.

## Verification used

```bash
cargo test --test interpreter nested_struct_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
```
