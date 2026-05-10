# Struct-pointer scalar field address-of

Date: 2026-05-10

## Summary

Cust now supports C-style address-of for scalar fields reached through struct pointers, such as `&point_ptr->x` and nested paths like `&box_ptr->inner.y`. The produced pointer aliases the original struct storage and can be used by helper functions or dereferenced directly.

## Implementation notes

- Parser lowering adds `Expr::AddressOfStructPtrField` for `&` applied to `Expr::StructPtrGet`.
- Runtime evaluation first resolves the struct pointer, then maps supported roots into existing `PointerValue::StructField` targets:
  - `PointerValue::Struct` for ordinary `struct T *p = &value;`
  - `PointerValue::StructElement` for pointers to elements of struct arrays
  - `PointerValue::StructField` for pointers to nested aggregate fields such as `&box_ptr->inner`
- Nested aggregate-field pointer paths append the selected `->` field path to the existing field-pointer path, so `struct Point *inner = &box_ptr->inner; &inner->x` aliases `box.inner.x`.
- Array fields still decay to `ArrayBase` for parity with existing direct struct-field address handling.
- Pointer-valued fields remain rejected with the existing pointer-to-pointer boundary diagnostic: `pointer field '<field>' cannot be addressed in this pointer milestone`.

## Coverage

- `tests/fixtures/valid/struct_pointer_field_addresses.c`
- `tests/fixtures/compat/valid/struct_pointer_field_addresses.c`
- Focused interpreter test: `supports_addresses_of_struct_pointer_scalar_fields`
- C compiler-oracle fixture registered in `tests/c_compat.rs`

## Pitfalls

`PointerValue::StructFieldElement` (embedded aggregate-array element pointers) cannot yet be converted into a scalar field pointer target because `PointerValue::StructField` has no slot for an embedded aggregate-array element index. Add a dedicated representation before supporting forms such as taking `&p->x` when `p` points into an embedded aggregate-array field.
