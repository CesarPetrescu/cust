# String literal pointer negative arithmetic

Date: 2026-06-27

Coverage-only conformance note for pointer arithmetic diagnostics across distinct string literal storage roots.

## Decision

Cust already treats string literals as read-only array storage with per-literal identity metadata. Pointer subtraction or relational ordering is valid only within the same array storage root. Separately spelled string literals must therefore be treated like different arrays.

## Fixtures

- `tests/fixtures/invalid/string_literal_pointer_difference_different_literals.c`
  - `char *left = "cat"; char *right = "dog"; return right - left;`
  - Expected: `cannot subtract pointers to different arrays`
- `tests/fixtures/invalid/string_literal_pointer_ordering_different_literals.c`
  - `char *left = "cat"; char *right = "dog"; return left < right;`
  - Expected: `cannot compare pointers to different arrays`

## Pitfall

Focused coverage may pass immediately. That is valid for this conformance-closure slice: record it as coverage over existing string-literal pointer metadata rather than inventing production code changes.
