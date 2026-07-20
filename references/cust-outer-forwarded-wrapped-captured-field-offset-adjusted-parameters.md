# Outer-pointer forwarding for wrapped captured field offsets

Date: 2026-07-20

## Scope

Conditional/comma-selected array fields from captured named, anonymous, and union-containing aggregate compound literals may flow through one- or two-hop `struct Item *` / `const struct Item *` return helpers either before or after `field + 1`, reverse `1 + field`, and `&field[1]`. Function boundaries must preserve the hidden containing root, recursive field path, absolute nonzero base, concrete aggregate pointee type, and qualification before C array-parameter adjustment copies the resulting pointer slot.

## Coverage result

`tests/fuzz_safety.rs` adds 144 deterministic scalar/named-aggregate two-writer/const-reader cases. The matrix balances three containing paths, four same-element/same-field-distinct/cross-field/separate-root relationships, three wrappers, three offsets, and four one/two-hop-before/after outer-forwarding routes. Inner scalar/aggregate pointer forwarding remains balanced, selected/unselected/comma markers stay exact, six captured roots execute once, and copied parameter slots remain local.

Fourteen panic-guarded checks retain exact inner and absolute outer bounds, recursive const-discard/write behavior, separate-root subtraction, concrete aggregate pointee mismatch, and out-of-scope hidden-root diagnostics through outer helpers. Existing Cust behavior passed immediately, so this is deliberate conformance coverage rather than a production fix.

The warning-free fixture `adjusted_aggregate_parameter_wrapped_compound_literal_field_outer_forwarding_routes.c` covers named, anonymous, union-containing, cross-field, and const roots and returns 44 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_outer_forwarded_wrapped_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_wrapped_compound_literal_field_outer_forwarding_routes_match_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate.
