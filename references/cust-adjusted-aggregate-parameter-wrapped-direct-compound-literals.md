# Conditional/comma-wrapped direct aggregate-array literals through adjusted parameters

Date: 2026-07-20

## Scope

A direct aggregate-array compound literal can decay before parameter binding through either arm of a conditional expression or through the right operand of a comma expression. The selected literal must retain one hidden storage root, outer element index, embedded field, inner index, concrete scalar/aggregate pointee type, and qualification after C array-parameter adjustment.

`tests/fuzz_safety.rs` models 36 scalar/named-aggregate two-writer/`const`-reader cases. Three balanced relationships cover the same element, another element in the same embedded array, and a separate wrapped literal root. Conditional-true, conditional-false, and comma routes are balanced; selected/unselected/comma markers prove branch short-circuiting and one-time hidden-root creation. One/two-hop forwarding and copied pointer slots retain identity. Eleven panic-guarded checks preserve exact inner/outer bounds, const-discard/write, aggregate type, and cross-root subtraction diagnostics. Separate const-array-typedef checks cover all three wrappers.

## Result and native-oracle guidance

The behavior passed once the generated harness called the established forwarding helper names and used the actual deterministic one/two-hop route counts. No production change was needed: existing conditional/comma pointer classification, aggregate-array literal storage, and adjusted-parameter binding already compose correctly.

For native fixtures, use distinct marker objects for separate function arguments because C does not sequence argument evaluation. Conditional operands safely prove the unselected literal initializer is not evaluated; the comma left operand safely proves its side effect occurs before the selected literal. Keep cross-root subtraction interpreter-only because native C leaves unrelated-array subtraction undefined.

The warning-free fixture `adjusted_aggregate_parameter_wrapped_direct_compound_literal_alias_routes.c` returns 69 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_wrapped_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_wrapped_direct_compound_literal_alias_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
