# Typedef aliases of anonymous aggregate atomic value types

The 2026-07-13 autonomous run closed conformance coverage for aliases such as `typedef _Atomic(struct { int value; }) AtomicAnonValue` and matching anonymous-union atomic aliases.

## Existing implementation path

- Parsing the typedef base creates one unique anonymous aggregate identity and stores it in `TypeAlias::Struct`; every comma-separated alias declarator reuses that same internal type name. Assignment between the aliases and named/unnamed prototype-definition compatibility therefore preserve the intended identity.
- Alias-spelled global/local objects, aggregate fields, by-value parameters, `sizeof`, and `_Alignof` reuse the existing aggregate metadata and deterministic no-op atomic model.
- The typedef registration records `_Atomic` as top-level qualification. Wrapping the alias in another `_Atomic(Alias)` reports `qualified _Atomic types are not supported` at the alias token, while an inner ordinary scalar typedef with the same name shadows the outer atomic alias.
- No production-code change was needed; this was deliberate conformance coverage of an existing parser/type-system seam.

## Native-oracle findings

Local GCC and Clang with `-std=c11 -Wall -Wextra -Werror` both accept the warning-free fixture, including comma-separated aliases, global/local objects, aggregate fields, named definitions paired with unnamed prototypes, same-alias value copies, type-query relationships, and lexical shadowing. Both binaries return 11, matching Cust. Keep `_Alignof` comparisons type-to-type because Clang rejects `_Alignof(expression)` as a GNU extension under these flags.

Avoid `(void)value` for these Cust aggregate parameters: the current scalar-use path diagnoses an aggregate variable used as scalar. A non-evaluating `sizeof(value) == sizeof(Alias)` expression both avoids native unused-parameter warnings and exercises the intended alias metadata.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_value -- --nocapture
cargo test --test c_compat -- --nocapture
```
