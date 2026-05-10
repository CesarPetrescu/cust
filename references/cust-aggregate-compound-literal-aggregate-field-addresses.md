# Aggregate compound-literal aggregate-field addresses

Date: 2026-05-10

## Feature

Cust supports taking the address of an aggregate-valued field selected from an aggregate compound literal, for example:

```c
struct Point { int x; int y; };
struct Box { struct Point inner; int tail; };

struct Point *inner = &((struct Box){{5, 7}, 9}).inner;
inner->x += 3;
```

The address operation materializes the aggregate compound literal in hidden current-scope storage and returns a `PointerValue::StructField` whose path points at the embedded aggregate field. Struct-pointer field reads/writes through `->` then route through `find_struct_pointer_fields` / `_mut`, which now recognize `PointerValue::StructField` targets when the selected field is itself a `StructFieldValue::Struct`.

## Coverage

- Interpreter fixture: `tests/fixtures/valid/aggregate_compound_literal_aggregate_field_addresses.c`
- Compiler-oracle fixture: `tests/fixtures/compat/valid/aggregate_compound_literal_aggregate_field_addresses.c`
- Focused test: `cargo test --test interpreter supports_addresses_of_aggregate_compound_literal_aggregate_fields -- --nocapture`
- Oracle test: `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`

## Pitfalls

- `PointerValue::StructField` was previously scalar-field-only in struct-pointer contexts and produced `pointer does not reference a struct`; update both immutable and mutable struct-pointer field resolution paths when enabling aggregate field pointers.
- Keep pointer-to-pointer forms out of scope: address-of pointer fields remains rejected by the existing `pointer field '<field>' cannot be addressed in this pointer milestone` diagnostic.
- Native fixtures should avoid ABI/layout assertions and should exercise behavior through ordinary struct pointer field access and exit codes.
