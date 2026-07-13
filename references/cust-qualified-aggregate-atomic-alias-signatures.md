# Qualified aggregate `_Atomic` aliases in function signatures

## C11 differential result

Local `cc -std=c11 -Wall -Wextra -Werror -fsyntax-only` establishes these boundaries:

- `_Atomic(ConstPoint)` is invalid when `ConstPoint` aliases `const struct Point`, including named parameters and unnamed prototype parameters.
- `_Atomic(AtomicPoint)` is invalid when `AtomicPoint` aliases `_Atomic(struct Point)`.
- `_Atomic(View)` is valid when `View` is an unqualified pointer alias whose pointee is `const struct Point`.
- `AtomicPoint *` is a valid pointer parameter spelling, and an inner unqualified aggregate alias can shadow an outer qualified alias before `_Atomic(Alias)` is parsed.

## Cust coverage pattern

- Reuse the scope-aware top-level-qualified typedef metadata used for scalar and pointer-slot aliases; aggregate aliases need no separate parser path.
- Assert invalid named and unnamed parameter forms at the alias token so function lookahead cannot hide the diagnostic.
- Keep the valid compiler-oracle fixture ABI-independent: compare `sizeof(*atomic_aggregate_pointer)` with `sizeof(AtomicAlias)` and avoid field access through an atomic aggregate object.
- A native aggregate atomic object may be left uninitialized when used only in `sizeof` or by address; this avoids Clang rejecting braced initialization of `_Atomic(struct)` while GCC accepts it.

## Focused verification

```bash
cargo test --test interpreter rejects_qualified_aggregate_atomic_aliases_in_function_signatures -- --nocapture
cargo test --test interpreter supports_atomic_aggregate_alias_signature_boundaries -- --nocapture
cargo test --test c_compat -- --nocapture
```
