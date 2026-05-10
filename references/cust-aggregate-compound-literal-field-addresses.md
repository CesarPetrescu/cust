# Aggregate compound-literal scalar-field address-of

2026-05-10 autonomous run: Cust now supports taking the address of scalar fields selected from aggregate compound literals, for example `&((struct Point){4, 8}).x`, nested paths such as `&((struct Box){{2, 3}, 4}).inner.y`, and union scalar fields such as `&((union Number){7}).value`.

Implementation notes:

- Parser lowering adds `Expr::AddressOfAggregateField` when `&` targets an `AggregateFieldGet`.
- Runtime evaluates only aggregate compound literals as addressable field sources, creates the same hidden current-scope aggregate storage used by `&(struct T){...}`, then returns a `PointerValue::StructField` to the selected scalar field.
- Pointer type/const inference handles `AddressOfAggregateField` so pointer declarations/arguments see the selected field's pointee type and const metadata.
- C compiler-oracle fixture uses only warning-free scalar field addresses and mutates through the resulting pointer; no native layout or `sizeof(struct)` assumptions are made.

Focused verification:

```bash
cargo test --test interpreter supports_addresses_of_aggregate_compound_literal_scalar_fields -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
