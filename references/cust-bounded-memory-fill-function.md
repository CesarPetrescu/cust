# Bounded character-storage `memset`

## Scope

Cust recognizes `memset` only with the normalized explicit prototype:

```c
void *memset(void *destination, int value, unsigned long int count);
```

User-defined functions take precedence. Missing declarations remain undefined, and incompatible declarations receive the dedicated unsupported-declaration diagnostic at runtime and beneath `sizeof`.

The first byte model remains intentionally narrow: only interpreter-owned character scalars and arrays are writable. Integer and aggregate object representations are rejected rather than serialized using host ABI layout. Counts are limited to 4,096 character cells.

## Runtime model

1. Evaluate destination, fill value, and count once in source order.
2. Require scalar value/count shapes and a nonnegative bounded count.
3. Preserve the destination pointer's interpreter-owned owner, path, lexical lifetime, and read-only metadata.
4. Reject const targets, expired owners, non-character storage, aggregate-backed character fields, and insufficient capacity through the shared bounded-memory diagnostics.
5. Convert the scalar fill value with `rem_euclid(256)`, matching conversion to `unsigned char` without changing Cust's deterministic character storage representation.
6. Write each selected cell through existing pointer assignment helpers and return the original destination identity as declared `void *`.

Zero count leaves storage unchanged while retaining the established bounded-memory destination validity checks.

## Non-evaluating dispatch

`sizeof(memset(...))` validates the exact declaration, arity, destination qualification, and pointer/value/count shapes without executing arguments. Recursive validation handles nested calls linearly. A recognized exact call also needs an `Ok(POINTER_SIZE)` fallback in scalar-size call dispatch after the recursive metadata pass; without it, direct and nested exact calls fall into the incompatible-declaration guard even though runtime dispatch recognizes the prototype.

## TDD and verification

The initial focused fixture failed with `undefined function 'memset'`. After runtime dispatch was added, direct and 30-level nested `sizeof` tests failed with the unsupported-declaration diagnostic, exposing the missing pointer-size fallback; the focused fix made all nine `memset` tests pass.

Coverage includes low-eight-bit conversion, embedded NULs, pointer offsets, zero count, destination identity, source-order one-time evaluation, const/lifetime/capacity/count/value/type diagnostics, aggregate-backed rejection, exact/incompatible/absent declaration handling, user-definition precedence, and direct/nested non-evaluation. The warning-free compiler-oracle fixture is registered explicitly in `tests/c_compat.rs`.

Local `man 3 memset` (Linux man-pages 6.18; C11/POSIX.1-2008) is the contract source: fill the first `n` bytes with `c` converted to `unsigned char` and return the original destination pointer.
