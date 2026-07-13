# Direct and typedef-backed atomic enum conformance

The 2026-07-13 conformance run closed the direct/typedef-backed enum branch of Cust's C11 `_Atomic(type-name)` coverage.

## C11 differential results

- GCC and Clang accept `_Atomic(enum State)`, `_Atomic(State)`, and aliases of atomic enum types in ordinary declarations, aggregate fields, function parameters, `sizeof`, and `_Alignof`.
- Both reject `_Atomic(ConstState)` when `ConstState` aliases `const enum State`, and reject `_Atomic(AtomicState)` when `AtomicState` aliases `_Atomic(enum State)`.
- An inner unqualified enum alias may shadow an outer qualified enum alias and then be used in `_Atomic(Alias)`.
- GCC accepts an atomic enum compound literal such as `((_Atomic(State)){READY})`, but Clang rejects it as an illegal initializer type. Do not put that form in the cross-compiler oracle corpus.

## Cust coverage pattern

- Cust lowers enums to its deterministic integer storage, so the existing scope-aware typedef qualification metadata covers qualified and already-atomic enum aliases without an enum-specific runtime path.
- Exact negative tests should assert the alias token location across object, field, parameter, `sizeof`, and `_Alignof` routes.
- Warning-free native fixtures may read atomic enum objects directly and pass them by value with the repository's C11 flags.
- Keep type-query checks ABI-independent by comparing equivalent atomic enum spellings, for example `sizeof(_Atomic(enum State)) == sizeof(AtomicState)` and `_Alignof(_Atomic(State)) == _Alignof(AtomicState)`, rather than assuming atomic and non-atomic enum layouts match.

## Focused verification

```bash
cargo test --test interpreter atomic_enum -- --nocapture
cargo test --test c_compat -- --nocapture
```
