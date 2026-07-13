# Derived declarators from anonymous aggregate atomic value aliases

The 2026-07-14 autonomous run closed conformance coverage for pointers and fixed arrays derived from aliases such as `typedef _Atomic(struct { int value; }) AtomicAnonValue` and matching anonymous-union aliases.

## Existing implementation path

- Alias lookup resolves `AtomicAnonValue` to the same `TypeAlias::Struct` internal identity created by the original anonymous aggregate typedef.
- Ordinary declarator parsing then derives one-level aggregate pointers and one-dimensional aggregate arrays from that identity. The same route covers global objects, aggregate fields, named/unnamed parameters, compatible prototypes/definitions, `sizeof`, and `_Alignof`.
- Cust's existing safety guards remain active after alias resolution: a second pointer star reports `pointer-to-pointer declarations are not supported`, an array suffix after a pointer declarator reports `pointer array declarations are not supported`, and a second fixed-array suffix reports `multidimensional array declarations are not supported` at the unsupported token.
- No production-code change was needed. The focused valid and exact-diagnostic tests passed immediately as deliberate conformance coverage.

## Native-oracle findings

Local GCC and Clang with `-std=c11 -Wall -Wextra -Werror` both accept the warning-free fixture. It covers derived pointers and fixed arrays, aggregate fields, an unnamed prototype paired with a named definition, array-parameter decay, pointer arithmetic, `sizeof`, and ABI-independent `_Alignof(T[N]) == _Alignof(T)` relationships. Both binaries return 127, matching Cust.

Avoid reading fields of atomic aggregate objects in the shared oracle fixture. Type relationships and pointer identity exercise the declarator metadata without relying on compiler-specific atomic aggregate access rules.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_value -- --nocapture
cargo test --test c_compat -- --nocapture
```
