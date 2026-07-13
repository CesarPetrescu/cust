# Typedef aliases of anonymous aggregate atomic pointer types

The 2026-07-13 autonomous run closed conformance coverage for aliases such as `typedef _Atomic(struct { int value; } *) AtomicAnonPtr` and `_Atomic(const union { ... } *)` counterparts.

## Existing implementation path

- Parsing the typedef base creates one unique anonymous aggregate identity, lowers the atomic pointer to `DeclType::Pointer`, and copies that identity into every comma-separated `TypeAlias::Pointer` declarator. Reusing an alias therefore preserves compatibility across named/unnamed parameters and prototype/definition pairs.
- `points_to_const` remains pointee metadata for `_Atomic(const union { ... } *)`; writes through an alias-spelled pointer report `cannot assign through pointer to const`.
- A later `const` on the pointer alias (`typedef AtomicAnonPtr const FixedAtomicAnonPtr`) remains pointer-slot const metadata and reports the established `cannot assign to const variable '<name>'` diagnostic on reassignment.
- The typedef registration records `_Atomic` as top-level qualification. Wrapping the alias in another `_Atomic(Alias)` is rejected at the alias token, while an inner ordinary pointer typedef with the same name shadows the outer atomic alias.
- No production-code change was needed; the selected backlog item was deliberate conformance coverage.

## Native-oracle findings

Local GCC and Clang with `-std=c11 -Wall -Wextra -Werror` both accept the warning-free fixture, including comma-separated aliases, alias-spelled function declarations/definitions, const alias slots, size/alignment relationships, and lexical shadowing. Both binaries return 12, matching Cust. Use type-to-type `_Alignof` relationships; Clang rejects `_Alignof(expression)` under these flags as a GNU extension.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_pointer -- --nocapture
cargo test --test c_compat -- --nocapture
```
