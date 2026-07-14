# Qualified anonymous aggregate atomic array aliases

The 2026-07-14 autonomous run closed fixed-array typedef aliases derived from qualified anonymous aggregate atomic value aliases:

```c
typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef const AtomicAnonValue ConstAtomicAnonValue;
typedef ConstAtomicAnonValue ConstAtomicAnonArray[2];
```

## Root cause and implementation

- `parse_const_qualified_decl_type()` correctly preserved alias-carried qualification and returned `DeclType::Array` for `ConstAtomicAnonArray`.
- `parse_params()` classified an array typedef parameter as pointer-like for C parameter adjustment, but its generic alias-pointer branch put `leading_const` into `Param::is_const` and derived `points_to_const` only from `DeclType::Pointer`.
- That made the adjusted parameter slot incorrectly const while its pointee was incorrectly mutable. Calls with const array aliases therefore failed with `cannot discard const qualifier from pointer target`.
- The fix special-cases `DeclType::Array` parameter aliases as `(is_const = false, points_to_const = leading_const)`. This matches C array-parameter adjustment: element qualification belongs to the pointee, while the adjusted pointer slot remains assignable.
- The general `const_typedef_aliases.c` fixture now also covers this behavior with a `typedef const int ConstScores[3]` prototype/definition, call-time const preservation, reads, and pointer-slot reassignment inside the callee.

## Coverage

The dedicated fixture covers const/volatile anonymous struct and union atomic element aliases across globals, static/automatic locals, aggregate fields, named definitions paired with unnamed prototypes, array decay, pointer views/arithmetic, lexical shadowing, `sizeof`, and `_Alignof`. Exact negative tests retain const element-write, mutable-pointer decay/argument, and multidimensional-array-alias diagnostics.

GCC and Clang both compile the fixture under `-std=c11 -Wall -Wextra -Werror` and return 255, matching Cust. Clang's `-Wdefault-const-init-var-unsafe` rejects uninitialized automatic const arrays under `-Werror`; use zero-initialized static local const arrays in shared warning-free fixtures.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_qualified_array_aliases -- --nocapture
cargo test --test interpreter supports_const_typedef_aliases -- --nocapture
cargo test --test c_compat -- --nocapture
```
