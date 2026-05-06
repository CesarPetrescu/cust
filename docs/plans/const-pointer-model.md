# Const Pointer Model

Last updated: 2026-05-06

Cust supports a deliberately scoped subset of C const-qualified pointer declarations and parameters without adding a full qualifier type lattice.

## Supported syntax

- `const int *p` / `const char *p`: pointer to const scalar elements. The pointer variable may be reassigned, but writes through `*p`, `p[i]`, compound assignment, and increment/decrement through the pointer are rejected.
- `int * const p` / `char * const p`: const pointer slot. The target may be mutated if the pointee is otherwise mutable, but the pointer variable cannot be reassigned, advanced with pointer compound assignment, or incremented/decremented.
- `const int * const p` / `const char * const p`: both the pointer slot and the pointee are const-qualified.
- Function parameters accept the same spelling, so `void read(const int *p)` gets a read-only pointer view while `void write(int * const p)` gets a non-reassignable parameter slot.

## Runtime model

- Parser metadata splits pointer declaration qualifiers into two booleans:
  - `is_const`: the pointer variable/parameter binding is read-only.
  - `points_to_const`: writes through that pointer binding are rejected.
- This is metadata on Cust pointer variables/parameters; pointer targets still use the existing interpreter-owned scalar/array/string/struct storage model.
- Existing const scalar targets and read-only arrays remain enforced at the target storage level, so `int *p = &const_scalar; *p = ...` and string/const-array writes still fail through the older checks.

## Intentional limits

- Pointer typedef aliases keep C typedef behavior for this milestone: `const IntPtr p` is treated as a const pointer slot, not a pointer-to-const alias.
- Cust does not yet perform full assignment compatibility checks for discarding pointee constness between pointer variables. Future work may add exact diagnostics for `int *mutable = const_ptr;` if the roadmap needs stricter qualifier conversions.
- Struct-field const qualifiers and aggregate/nested const fields remain outside the current supported subset.

## Acceptance fixtures

- Valid: `tests/fixtures/valid/const_pointer_qualifiers.c`
- Invalid: `tests/fixtures/invalid/const_pointer_write.c`, `tests/fixtures/invalid/const_pointer_index_write.c`, `tests/fixtures/invalid/const_pointer_reassignment.c`
- Native C oracle: `tests/fixtures/compat/valid/const_pointer_qualifiers.c`
