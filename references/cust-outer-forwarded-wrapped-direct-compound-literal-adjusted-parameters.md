# Outer-pointer forwarding for wrapped direct aggregate-array literals

Date: 2026-07-20

## Scope

Conditional/comma-selected direct mutable and const-array-typedef aggregate-array compound literals may flow through one- or two-hop `struct Item *` / `const struct Item *` return helpers before or after `literal + 1`, reverse `1 + literal`, and `&literal[1]`. The returned pointer must retain the hidden literal root, absolute nonzero base, concrete aggregate pointee type, and qualification when C array adjustment copies it into a parameter slot.

## Coverage result

`tests/fuzz_safety.rs` adds 216 deterministic scalar/named-aggregate two-writer/const-reader alias cases. The matrix balances three same-element/same-array-distinct/separate-root relationships, three conditional/comma wrappers, three nonzero offsets, and four one/two-hop-before/after outer-helper routes. Inner scalar/aggregate forwarding remains balanced, selected/unselected/comma markers prove each hidden root executes once, and copied parameter slots remain local.

A separate 36-case const-array-typedef matrix covers every wrapper/offset/outer-helper combination. Fourteen panic-guarded checks retain exact inner and absolute outer bounds, const-discard/write behavior, separate-root subtraction, concrete aggregate pointee mismatch, and out-of-scope hidden-root diagnostics. Existing Cust behavior passed immediately, so this is deliberate conformance coverage rather than a production fix.

The warning-free fixture `adjusted_aggregate_parameter_wrapped_direct_compound_literal_outer_forwarding_routes.c` covers representative mutable and const one/two-hop helpers before and after offsets and returns 49 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_outer_forwarded_wrapped_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_wrapped_direct_compound_literal_outer_forwarding_routes_match_fixture -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate.
