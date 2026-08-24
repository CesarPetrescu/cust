# Direct `double` typedef aliases

## Completed surface

- Scalar and const aliases preserve exact double values through local/global/static declarations, arithmetic, casts, `_Generic`, `sizeof`, and `_Alignof`.
- Fixed one-dimensional array aliases support objects, aggregate fields, decay, adjusted parameters, and element mutation while retaining bounds and qualification.
- Scalar aliases work in direct function parameters/returns and aggregate scalar fields.
- One-level pointer aliases preserve owner, lexical lifetime, pointee const, and independently const-qualified pointer slots across declarations, fields, calls, and returns.
- `tests/fixtures/compat/valid/direct_double_typedef_aliases.c` is registered in `tests/c_compat.rs` and is warning-free under GCC and Clang C11 `-Wall -Wextra -Werror`.

## Parser pitfalls

1. Reject array aliases followed by an explicit `*` before calling `decl_type_to_pointee()`. Otherwise `Row *` silently collapses from pointer-to-array to scalar pointer in function-return and aggregate-field routes.
2. Reapply direct-double boundaries after alias expansion. `_Atomic(RealPtr)` and `((Row){...})` must not bypass the existing atomic-pointer and double-array-compound-literal restrictions.
3. For an already-pointer `DeclType`, top-level `const` qualifies the pointer slot. Function return and cast rvalues must use only `points_to_const` for pointee qualification. Direct `const double *` is a separate scalar-base-plus-star route and remains pointee-const.
4. Keep exact boundaries for deeper pointers, arrays of pointers, pointer-to-row and multidimensional forms, union-backed double pointer storage, expired local owners, and bounded raw-memory intrinsics.

## Verification

```bash
cargo test --test interpreter direct_double_typedef -- --nocapture
cargo test --test c_compat -- --nocapture
gcc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/direct_double_typedef_aliases.c -o /tmp/cust-double-typedef-gcc
clang -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/direct_double_typedef_aliases.c -o /tmp/cust-double-typedef-clang
```

Then run the canonical local and Docker gate from `docs/plans/autonomous-agent.md`.