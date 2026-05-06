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
- Existing const scalar targets and read-only arrays remain enforced at the target storage level, so writes through older aliases to const scalars/arrays still fail even when the pointer expression did not carry explicit `points_to_const` metadata.
- Pointer conversions preserve pointee constness: assigning or passing a pointer-to-const expression to an `int *` / `char *` target reports `cannot discard const qualifier from pointer target`, while assigning mutable pointers to `const int *` / `const char *` targets is allowed.

## Intentional limits

- Pointer typedef aliases keep C typedef behavior for this milestone: `const IntPtr p` is treated as a const pointer slot, not a pointer-to-const alias.
- Const conversion tracking is deliberately conservative for syntax-level pointer expressions such as conditionals and pointer arithmetic; if either possible pointer source is const-qualified, Cust requires a const-qualified destination.
- Struct-field const qualifiers and aggregate/nested const fields remain outside the current supported subset.

## Acceptance fixtures

- Valid: `tests/fixtures/valid/const_pointer_qualifiers.c`, `tests/fixtures/valid/const_pointer_conversions.c`
- Invalid: `tests/fixtures/invalid/const_pointer_write.c`, `tests/fixtures/invalid/const_pointer_index_write.c`, `tests/fixtures/invalid/const_pointer_reassignment.c`, `tests/fixtures/invalid/const_pointer_discard_decl.c`, `tests/fixtures/invalid/const_pointer_discard_assignment.c`, `tests/fixtures/invalid/const_pointer_discard_argument.c`
- Native C oracle: `tests/fixtures/compat/valid/const_pointer_qualifiers.c`, `tests/fixtures/compat/valid/const_pointer_conversions.c`
