# Array compound-literal pointer negative arithmetic

Date: 2026-06-27

## Context

Cust supports one-dimensional scalar array compound literals such as `(int[]){1, 2}` as pointer-valued expressions backed by interpreter-owned temporary array storage. Pointer subtraction and relational ordering are intentionally limited to pointers into the same supported array storage root.

## Coverage added

The autonomous coverage run added invalid interpreter fixtures for arithmetic between two separately evaluated array compound literals:

- `tests/fixtures/invalid/array_compound_literal_pointer_difference_different_literals.c`
- `tests/fixtures/invalid/array_compound_literal_pointer_ordering_different_literals.c`

Expected diagnostics:

- subtraction: `cannot subtract pointers to different arrays`
- ordering: `cannot compare pointers to different arrays`

Focused test command:

```bash
cargo test --test interpreter different_array_compound_literals -- --nocapture
```

## Implementation decision

No production-code change was needed. Focused coverage passed immediately because existing array-compound-literal storage metadata allocates distinct hidden array roots for each literal expression, and the existing pointer arithmetic/ordering checks already reject cross-root operations.

Do not add native compiler-oracle fixtures for these invalid programs: subtracting or relationally comparing pointers into different arrays is undefined/unsupported behavior in the C subset and is best locked in as a Cust diagnostic fixture.
