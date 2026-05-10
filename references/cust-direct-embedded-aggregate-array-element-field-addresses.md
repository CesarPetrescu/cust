# Direct embedded aggregate-array element field address-of

Date: 2026-05-10

## Feature

Cust supports direct address-of for scalar fields selected after indexing embedded aggregate-array fields:

```c
struct Line line = {{{1, 2}, {3, 4}, {5, 6}}, 7};
int *x = &line.points[1].x;
*x = 23;
```

Nested containing paths are covered too:

```c
struct Box box = {{{{8, 9}, {10, 11}, {12, 13}}, 14}};
int *y = &box.line.points[2].y;
```

The resulting pointer aliases the original embedded aggregate-array element field and can be passed to helpers or dereferenced for mutation.

## Implementation notes

The parser already lowered `line.points[i].x` reads/writes to `Expr::StructFieldArrayElementGet`. Address-of previously rejected that expression shape as an invalid target. The implementation now rewrites address-of over `StructFieldArrayElementGet` into the already-supported pointer path:

1. Build `AddressOfStructArrayField { name, fields: array_fields, index }` for `&line.points[i]`.
2. Wrap it as `AddressOfStructPtrField { pointer, fields }` for the trailing scalar field path.
3. Runtime evaluation then reuses `find_struct_array_field_pointer` to produce a `PointerValue::StructFieldElement` and `find_struct_pointer_field_pointer` to produce the scalar field pointer target.

This avoids adding a new pointer target and preserves existing owner/path/index metadata used by dereference, aliasing, const, and pointer identity helpers.

## Coverage

- `tests/fixtures/valid/struct_field_array_element_field_addresses.c`
- `tests/fixtures/compat/valid/struct_field_array_element_field_addresses.c`
- Focused interpreter test: `supports_direct_addresses_of_embedded_aggregate_array_element_fields`
- Full C compiler-oracle fixture list includes the compat fixture.

## Pitfalls

- Do not lower this to a host/native address. Keep using Cust's interpreter-owned pointer metadata.
- Avoid a new enum variant unless the existing aggregate-array element pointer plus struct-pointer field-address route cannot express the target.
- Native C fixtures should use mutation results in the exit code so `-Wall -Wextra -Werror` has no unused-value warnings.
