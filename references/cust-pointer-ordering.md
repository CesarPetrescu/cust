# Cust same-array pointer ordering

2026-05-10 autonomous run.

## Scope

Cust supports relational pointer comparisons (`<`, `<=`, `>`, `>=`) only for pointers into the same supported array-backed storage:

- scalar arrays and string-literal storage (`ArrayBase` / `ArrayElement`)
- aggregate arrays (`StructElement` pointers)
- embedded aggregate-array field pointers are routed through the same pointer-difference metadata if exercised by future fixtures

Scalar-object pointers and null pointers remain outside this scoped milestone and continue to report `pointer ordering comparisons are not supported`.

## Implementation notes

- `Interpreter::pointer_ordering` reuses `pointer_difference` so ordering and subtraction share same-storage checks and safe interpreter-owned pointer identities.
- `pointer_difference` errors are remapped for relational operators:
  - `cannot subtract pointers to different arrays` -> `cannot compare pointers to different arrays`
  - scalar/null pointer arithmetic errors -> existing unsupported ordering diagnostic
- The ordinary scalar comparison branch remains unchanged for non-pointer operands.

## Tests

- Valid interpreter fixture: `tests/fixtures/valid/pointer_ordering.c`
- Native oracle fixture: `tests/fixtures/compat/valid/pointer_ordering.c`
- Invalid fixture: `tests/fixtures/invalid/pointer_ordering_different_arrays.c`
- Existing scalar-pointer invalid fixture remains: `tests/fixtures/invalid/pointer_ordering_comparison.c`

## Native `-Werror` pitfall

GCC/cc with `-Wall -Wextra -Werror` rejects obvious self-comparisons such as `p <= p` as `-Werror=tautological-compare`. Use aliases (`int *same = p; p <= same`) in compiler-oracle fixtures when equality-in-ordering coverage is needed.
