# Captured aggregate-compound-literal field offsets through adjusted parameters

Date: 2026-07-20

## Scope

Array fields selected from captured named, anonymous, and union-containing aggregate compound literals can be offset before C array-parameter adjustment. `field + 1`, reverse `1 + field`, and `&field[1]` must retain the hidden containing root, selected field path, absolute outer index, embedded field, and inner index.

`tests/fuzz_safety.rs` models 54 scalar/named-aggregate two-writer/`const`-reader cases across three containing paths and three same-element/same-array-distinct/cross-field relationships. Six initializer markers prove each containing literal is captured once; one/two-hop forwarding and copied pointer slots preserve recursive identity.

## Coverage and diagnostics

The generated matrix passed without a production change. Thirteen panic-guarded checks retain exact inner bounds, absolute upper/lower field bounds, recursive const-discard/write behavior, concrete aggregate pointee types, and cross-path subtraction diagnostics.

The warning-free fixture `adjusted_aggregate_parameter_compound_literal_field_offset_routes.c` proves that a callee based at field element one can use `items[-1]` to reach the preceding same-field element. Cust, GCC, and Clang return 34.

Focused commands:

```bash
cargo test --test fuzz_safety generated_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_compound_literal_field_offset_routes_match_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
