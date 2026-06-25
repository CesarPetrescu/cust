# Direct enum aggregate fields and pointer-field indexing

Date: 2026-06-25

## Context

A conformance fixture for direct named-enum fields inside supported structs exposed a nearby runtime gap: `job->cursor[1]`, where `cursor` is a pointer-valued aggregate field, parsed through the same `StructPtrArrayGet` AST path used by scalar array fields and then failed with `struct field 'cursor' is not an array`.

## Implementation notes

- Direct enum fields reuse Cust's existing enum-as-int lowering in aggregate field declarations, scalar array fields, pointer fields, const field metadata, and pointer/array parameter binding.
- Parser lowering for `field[index]` remains shared; runtime now distinguishes pointer-valued fields from array-valued fields before indexing.
- For direct struct values, `StructArrayGet` checks whether the selected field is a pointer. Pointer fields are read with `read_direct_struct_pointer_field`, indexed through `checked_pointer_value_index`, then dereferenced.
- For struct pointers, `StructPtrArrayGet` and `AddressOfStructPtrArrayField` check `struct_pointer_pointer_field_type(...)` first. Pointer fields route through `struct_pointer_pointer_field_index_pointer`; actual array fields continue through the existing embedded-array path so array bounds diagnostics are not masked.

## Coverage

- `tests/fixtures/valid/direct_enum_aggregate_fields.c`
- `tests/fixtures/compat/valid/direct_enum_aggregate_fields.c`
- `tests/fixtures/invalid/const_enum_aggregate_field_assignment.c`

Focused commands:

```bash
cargo test --test interpreter direct_enum_aggregate -- --nocapture
cargo test --test interpreter const_enum_aggregate -- --nocapture
cargo test --test interpreter struct_pointer_array_field_decay -- --nocapture
cargo test --test c_compat -- --nocapture
```
