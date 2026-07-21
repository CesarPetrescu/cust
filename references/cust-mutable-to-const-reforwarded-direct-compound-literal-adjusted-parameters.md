# Mutable-to-const re-forwarding for direct aggregate-array literal adjusted parameters

Date: 2026-07-21

## Scope

A mutable direct aggregate-array compound literal may be promoted to `const struct Item *` either before a post conditional/comma wrapper or afterward through the re-forwarding helper. The const helper may run before or after `literal + 1`, `1 + literal`, or `&literal[1]`. Qualification changes write permissions, not the interpreter-owned hidden root, absolute nonzero base, or concrete scalar/named-aggregate inner-pointer identity. C array-parameter adjustment and local pointer-slot reassignment likewise copy only the pointer slot.

## Coverage result

Use an exhaustive matrix over promotion stage, inner and post wrapper, nonzero-offset spelling, one/two-hop initial helper, one/two-hop const re-forwarding helper, and re-forwarding before/after offset formation. The 2026-07-21 run covered 432 generated identity/read cases. Selected and comma markers prove one-time hidden-root evaluation; const scalar and aggregate inner pointers preserve identity; adjusted and locally reassigned pointer slots remain copies; and four panic-guarded tests retain exact mutable-rebinding and write diagnostics.

Existing Cust behavior passed immediately, so this was deliberate conformance/property closure rather than a production-code fix. The warning-free fixture returned 37 under Cust, GCC, and Clang.

Focused commands:

```bash
cargo test --test fuzz_safety generated_mutable_to_const_reforwarded_direct_aggregate_array_literal_adjusted_parameters_preserve_identity_without_panics -- --nocapture
cargo test --test interpreter mutable_to_const_reforwarded_direct_compound_literal_routes -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate. The next analogous seam is mutable-to-const promotion over captured aggregate-compound-literal array fields, which must additionally preserve containing hidden-root and recursive field-path metadata.
