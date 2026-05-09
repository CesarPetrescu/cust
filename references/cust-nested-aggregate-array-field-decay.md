# Nested embedded aggregate-array field decay fixture coverage

2026-05-09 autonomous run.

## Feature locked in

The existing embedded aggregate-array field pointer machinery now has explicit regression coverage for nested paths through struct pointers:

```c
struct Box { struct Line inner; };
adjust_points(box->inner.points);
struct Point *p = &box->inner.points[2];
p = box->inner.points + 1;
```

This confirms that nested `StructFieldValue::StructArray` paths reuse `PointerValue::StructFieldElement` correctly for pointer decay, element address-of, bounded pointer arithmetic, and pointer-indexed field mutation.

## Implementation notes

No production code was needed for the nested aggregate-array path: the prior `StructPtrArrayGet`/`StructFieldElement` lowering already handled nested `box->inner.points` paths. The run added valid/invalid/compiler-oracle fixtures so future changes cannot regress this parity.

## Tests

- `tests/fixtures/valid/nested_aggregate_array_field_decay.c`
- `tests/fixtures/invalid/nested_aggregate_array_field_const_discard.c`
- `tests/fixtures/compat/valid/nested_aggregate_array_field_decay.c`
- Focused: `cargo test --test interpreter nested_aggregate_array_field -- --nocapture`
- Oracle: `cargo test --test c_compat -- --nocapture`
