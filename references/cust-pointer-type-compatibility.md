# Cust pointer type compatibility

Date: 2026-05-07

## Work package

Cust previously tracked declared pointer pointee types (`int *`, `char *`, `struct T *`, `union T *`) but did not validate the runtime pointer target shape at conversion boundaries. That allowed unsupported C forms such as passing `char *` to `int *` or `union Number *` to `struct Point *`; later reads/writes could mutate the wrong storage or fail with misleading field diagnostics.

## RED tests

Added exact invalid-fixture regressions in `tests/interpreter.rs`:

- `tests/fixtures/invalid/scalar_pointer_type_mismatch.c`
  - `write_int(&c)` originally returned `200` because a `char *` argument was accepted by an `int *` parameter.
- `tests/fixtures/invalid/aggregate_pointer_type_mismatch.c`
  - `set_point(&n)` originally failed later as `struct 'Number' has no field 'x'` instead of rejecting the pointer conversion.
- `tests/fixtures/invalid/pointer_assignment_type_mismatch.c`
  - assignment of `struct Size *` into a `struct Point *` slot is now an explicit conversion error.

Focused RED command:

```bash
cargo test --test interpreter pointer_type_mismatch -- --nocapture
```

## Implementation notes

- Added `pointer_value_type()` to infer the actual pointee type from interpreter-owned pointer targets:
  - scalar scope targets -> declared scalar `CType`
  - scalar arrays/string storage -> array element `CType`
  - struct/union variables and array elements -> aggregate type name
  - struct fields -> final field scalar/array/aggregate/pointer metadata
  - null -> no concrete type, accepted for any pointer target
- Added `ensure_pointer_type_matches(expected, pointer)` and `pointee_label()` for diagnostics such as:
  - `cannot convert pointer to char to pointer to int`
  - `cannot convert pointer to union 'Number' to pointer to struct 'Point'`
- Applied checks at pointer parameter binding, pointer declaration/static initialization, pointer statement/expression assignment, and pointer-field brace initialization, alongside the existing const-preservation checks.

## Pitfalls

- `PointeeType::Struct(String)` intentionally covers both structs and unions; diagnostics must consult `struct_types[type_name].kind.keyword()` to say `struct` vs `union`.
- Preserve null pointer compatibility: `int *p = 0;`, `struct Point *p = 0;`, and reassignment to `0` must not require a concrete source pointee type.
- Run focused test filters separately when names do not share a substring; Cargo test filters are substring-only.
