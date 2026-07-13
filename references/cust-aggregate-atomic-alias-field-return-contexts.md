# Aggregate atomic aliases in fields and returns

The 2026-07-13 conformance run extended qualified aggregate atomic-alias coverage from function parameters to aggregate fields and function return declarations/definitions.

- GCC and Clang reject `_Atomic(ConstPoint)` and `_Atomic(AtomicPoint)` when the alias denotes a top-level-qualified or already-atomic aggregate.
- Cust's existing scope-aware typedef qualifier metadata already reports `qualified _Atomic types are not supported` at the alias token in all of these routes.
- `_Atomic(View)` remains supported when `View` is an unqualified pointer alias to a const aggregate pointee; pointers to atomic aggregate aliases also remain supported.
- GCC rejects atomic-qualified function return types under `-Werror=ignored-qualifiers`, and Clang has stricter diagnostics around atomic aggregate initialization/member access. Keep supported return behavior interpreter-only under Cust's deterministic no-op atomic model.
- Native compiler-oracle fixtures should use aggregate-field declarations and ABI-independent relationships such as `sizeof(field) == sizeof(base_type)` without directly reading atomic aggregate members.

Focused verification:

```bash
cargo test --test interpreter aggregate_atomic_aliases_in_fields_and_returns -- --nocapture
cargo test --test interpreter atomic_aggregate_alias_declaration_boundaries -- --nocapture
cargo test --test c_compat -- --nocapture
```