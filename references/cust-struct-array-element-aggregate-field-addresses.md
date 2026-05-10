# Struct-array element aggregate-field addresses

Date: 2026-05-10

## Feature

Cust now supports taking the address of aggregate-valued fields selected from ordinary struct-array elements, for example:

```c
struct Segment segments[2];
struct Point *start = &segments[1].start;
start->x = start->x + 1;
```

The same run also locked in the already-supported direct embedded aggregate-array syntax:

```c
struct Point *start = &drawing.segments[1].start;
struct Point *nested = &box.drawing.segments[0].end;
```

## Implementation notes

- Parser lowering for `&segments[i].field` already routes through `Expr::AddressOfStructElementField`.
- The runtime gap was in `Interpreter::find_struct_element_field_pointer`: scalar fields returned `PointerValue::StructField`, but aggregate-valued `StructFieldValue::Struct` fields still reported `struct field '<field>' requires field access`.
- The fix is to allow both `StructFieldValue::Scalar` and `StructFieldValue::Struct` to return `PointerValue::StructField` with the struct-array element index preserved.
- Existing struct-pointer field resolution already treats `PointerValue::StructField` as either scalar-field or aggregate-field pointer depending on the selected field value, so `->` reads/writes and helper mutation work without a new pointer target.

## Coverage

- `tests/fixtures/valid/struct_array_element_aggregate_field_addresses.c`
- `tests/fixtures/compat/valid/struct_array_element_aggregate_field_addresses.c`
- `tests/fixtures/valid/struct_field_array_element_aggregate_field_addresses.c`
- `tests/fixtures/compat/valid/struct_field_array_element_aggregate_field_addresses.c`
- Focused RED test: `cargo test --test interpreter supports_addresses_of_struct_array_element_aggregate_fields -- --nocapture` initially failed with `struct field 'start' requires field access`.
- Focused GREEN plus compiler oracle: run the two focused interpreter tests and `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`.

## Pitfalls

- C compatibility fixtures compare native process exit codes, so keep return values in `[0, 255]`.
- Do not add a new pointer target for ordinary struct-array element aggregate fields; preserving `PointerValue::StructField { element_index: Some(i), ... }` keeps existing aliasing and metadata behavior.
