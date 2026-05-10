# Embedded aggregate-array element aggregate-field addresses

Date: 2026-05-10

## Summary

Cust now supports taking the address of aggregate-valued fields through pointers that already point into embedded aggregate-array fields. Example:

```c
struct Segment *second = drawing.segments + 1;
struct Point *start = &second->start;
start->x = start->x + 10;
```

The feature extends the scalar-field address path introduced for `&p->x` when `p` is a `PointerValue::StructFieldElement`.

## Implementation notes

- `&second->start` lowers through the existing `Expr::AddressOfStructPtrField` path.
- `find_struct_pointer_field_pointer` may produce `PointerValue::StructFieldElementField` for both scalar and aggregate-valued fields selected from an embedded aggregate-array element.
- `find_struct_pointer_fields` and `find_struct_pointer_fields_mut` must treat aggregate-valued `PointerValue::StructFieldElementField` as a struct target so subsequent `->` reads/writes route back to the original containing struct storage.
- The helper traversal for nested struct fields intentionally returns an owned type name plus borrowed field map so type-name display metadata does not tie the returned field-map lifetime to a temporary local `String` from `struct_field_array_element_fields*`.

## Coverage

- Interpreter fixture: `tests/fixtures/valid/struct_field_element_aggregate_field_addresses.c`
- C compiler-oracle fixture: `tests/fixtures/compat/valid/struct_field_element_aggregate_field_addresses.c`
- Focused test: `cargo test --test interpreter supports_addresses_of_aggregate_fields_through_embedded_aggregate_array_pointers -- --nocapture`
- Compat oracle: `cargo test --test c_compat -- --nocapture`

## Pitfalls

- Returning references from helper functions that also return type-name metadata can accidentally borrow a local `String` produced by `struct_field_array_element_fields*`; return an owned `String` for the type name and only borrow the field map.
- Update both immutable and mutable struct-pointer field resolution paths; otherwise reads through `start->x` may pass while writes such as `start->x = ...` fail.
