# Embedded aggregate-array element scalar-field address-of

Date: 2026-05-10

## Summary

Cust now supports taking scalar-field addresses through pointers that target elements of embedded aggregate-array fields. Forms such as `struct Point *p = line.points + 1; int *x = &p->x;` and nested containing paths such as `box.line.points + 2` now produce safe scalar field pointers that alias the original containing struct storage.

## Implementation notes

- Added `PointerValue::StructFieldElementField` to represent a scalar/aggregate field selected from a `PointerValue::StructFieldElement` owner. It carries the containing scope/name, optional root struct-array element index, embedded aggregate-array field path, embedded element index, and selected field path.
- `find_struct_pointer_field_pointer` now recognizes `PointerValue::StructFieldElement` roots and maps `&p->field` to the new scalar-field pointer representation instead of reporting `pointer does not reference a struct`.
- Scalar dereference reads and writes route through `read_struct_field_element_field_pointer` / `assign_struct_field_element_field_pointer`, which resolve the embedded array element then apply normal nested-field scalar access and const-field checks.
- Pointer metadata and non-indexability/arithmetic checks classify the new pointer target as scalar-field-like, preserving type checks and preventing accidental array/aggregate pointer behavior.

## Coverage

- `tests/fixtures/valid/struct_field_element_field_addresses.c`
- `tests/fixtures/compat/valid/struct_field_element_field_addresses.c`
- Focused interpreter test: `supports_addresses_of_fields_through_embedded_aggregate_array_pointers`
- C compiler-oracle fixture registered in `tests/c_compat.rs`

## Pitfalls

- Do not collapse this target into `PointerValue::StructField`: `StructField` only has the root optional struct-array `element_index` and cannot identify an element of an embedded aggregate-array field.
- Keep `StructFieldElementField` scalar-pointer-like for pointer arithmetic and indexing; the owning `StructFieldElement` remains the aggregate pointer target for array-style movement.
