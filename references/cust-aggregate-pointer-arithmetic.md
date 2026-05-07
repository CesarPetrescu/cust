# Cust aggregate pointer arithmetic (2026-05-07)

## Scope completed

Cust now supports bounds-checked pointer arithmetic over interpreter-owned struct/union array element pointers:

- `struct Point *p = &points[0]; p = p + 1; p->x`
- `p += n`, `p -= n`, prefix/postfix `++`/`--` on pointer variables that currently hold `PointerValue::StructElement`
- pointer difference for two struct/union element pointers into the same aggregate array
- deterministic out-of-bounds diagnostics such as `struct array pointer index 2 out of bounds for length 2`

Standalone scalar pointers, standalone struct/union variable pointers, struct field pointers, one-past aggregate pointers, pointer ordering, and aggregate pointer indexing (`p[i].field`) remain unsupported.

## Implementation notes

- Reused the existing `PointerValue::StructElement { scope_id, name, index }` target for both struct-array and union-array element pointers.
- Added `Interpreter::struct_array_pointer_at(scope_id, name, index)` to validate live scope, confirm the owner is still a `Value::StructArray`, check bounds, and return an updated `StructElement` pointer.
- Extended `offset_array_pointer()` to offset `StructElement` pointers while preserving old scalar/null diagnostics for non-array-backed pointers.
- Changed pointer subtraction from a static array-only helper to `self.pointer_difference(...)` so it can validate `StructElement` pointers by scope/name/index as well as existing `ArrayBase`/`ArrayElement` pointers.
- The existing pointer-variable compound assignment and increment/decrement paths already routed through `offset_array_pointer()`, so they picked up aggregate-array element support after the helper learned `StructElement`.

## TDD / verification notes

RED:

```bash
cargo test --test interpreter pointer_arithmetic_for_struct -- --nocapture
cargo test --test interpreter rejects_struct_pointer_arithmetic_out_of_bounds -- --nocapture
```

Both failed with the old `scalar pointer arithmetic is not supported` diagnostic.

GREEN/focused:

```bash
cargo test --test interpreter pointer_arithmetic -- --nocapture
cargo test --test c_compat -- --nocapture
```

The native C compatibility fixture initially used `(int)(last - first)`, which Cust parsed as a cast and rejected (`expected expression, found Int`). Remove the cast in compat fixtures; with `-Wall -Wextra -Werror` the implicit `ptrdiff_t` to `int` return is accepted by the local C oracle because `-Wconversion` is not enabled.

## Fixtures

- `tests/fixtures/valid/aggregate_pointer_arithmetic.c`
- `tests/fixtures/invalid/struct_pointer_arithmetic_out_of_bounds.c`
- `tests/fixtures/compat/valid/aggregate_pointer_arithmetic.c`
