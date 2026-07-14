# Qualified anonymous aggregate atomic value aliases

The 2026-07-14 autonomous run closed conformance coverage for top-level qualified aliases derived from anonymous aggregate atomic value aliases:

```c
typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef const AtomicAnonValue ConstAtomicAnonValue;
typedef volatile AtomicAnonValue VolatileAtomicAnonValue;
```

## Existing implementation path

- `const_type_alias_scopes` and the parallel volatile/atomic qualification metadata retain qualification through chained aliases while `TypeAlias::Struct` keeps the original anonymous aggregate identity.
- Qualified aliases work for global/local objects, fixed arrays, aggregate fields, named definitions paired with unnamed prototypes, pointers to qualified values, lexical shadowing, `sizeof`, and `_Alignof`.
- Const-qualified objects and arrays reject direct writes with the root-variable diagnostic; pointers to const-qualified aliases reject writes with `cannot assign through pointer to const` and const-to-mutable conversions with `cannot discard const qualifier from pointer target`.
- Wrapping a const or volatile alias in another `_Atomic(...)` reports `qualified _Atomic types are not supported` at the alias token.
- Focused tests needed only one expected-diagnostic correction (`values[0].field` checks the const root before the aggregate-array write path); no production-code change was needed.

## Native-oracle findings

GCC and Clang accept the shared C11 fixture under `-std=c11 -Wall -Wextra -Werror`; both binaries return 255, matching Cust. Use `sizeof`/`_Alignof` relationships and pointer identity rather than atomic aggregate field reads, and keep all return arithmetic within the process exit-code range.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_qualified_value_aliases -- --nocapture
cargo test --test interpreter rejects_atomic_wrapping_of_qualified_atomic_anonymous_aggregate_value_aliases -- --nocapture
cargo test --test c_compat -- --nocapture
```
