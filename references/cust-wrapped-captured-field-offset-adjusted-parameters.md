# Wrapped captured field offsets through adjusted aggregate parameters

Date: 2026-07-20

## Scope

Conditional and comma expressions may select array fields from captured named, anonymous, and union-containing aggregate compound literals before `field + 1`, reverse `1 + field`, or `&field[1]` is passed to a C-adjusted aggregate parameter. The selected expression must retain containing root, field path, nonzero base, outer/inner indexes, concrete pointee type, and recursive const qualification.

`tests/fuzz_safety.rs` models 72 scalar/named-aggregate two-writer/`const`-reader cases. The matrix balances three containing paths, four same-element/same-field-distinct/cross-field/separate-root relationships, conditional-true/conditional-false/comma wrappers, and all three nonzero-offset forms. Distinct marker objects prove selected/unselected/comma evaluation; six enclosing roots remain captured once, one/two-hop inner-pointer forwarding retains identity, and copied parameter slots remain local.

## TDD result and diagnostics

The focused fixture passed immediately as deliberate conformance closure. The generated model then went RED because its comma renderer incremented the comma marker but not the selected-route marker expected by the shared oracle. This was a generator bug, not an interpreter bug; making the comma route model both events produced GREEN.

Thirteen panic-guarded checks retain exact inner bounds, absolute upper/lower field bounds, recursive const-discard/write behavior, concrete aggregate pointee types, and separate-root subtraction diagnostics. Cross-root subtraction remains interpreter-only because native C does not define subtraction between unrelated arrays.

The warning-free fixture `adjusted_aggregate_parameter_wrapped_compound_literal_field_offset_routes.c` uses separate marker full expressions and returns 44 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_wrapped_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_wrapped_compound_literal_field_offset_routes_match_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate.
