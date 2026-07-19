# Nested adjusted-aggregate-parameter alias mutations

Date: 2026-07-19

## Scope

`tests/fuzz_safety.rs` extends the adjusted-aggregate-parameter two-writer/`const`-reader model across five outer-array path families: root arrays, direct aggregate fields, nested named fields, nested anonymous fields, and union members. Scalar `int *` and named `struct Point *` embedded-array elements use balanced direct/reverse addresses and one/two-hop forwarding.

The independent model identifies storage by the complete outer owner/path plus outer aggregate index, embedded field kind, and inner index. It applies replacement and compound writes in source order, checks intermediate/final reader observations, and verifies that reassigned callee parameter slots do not change caller pointers.

## Generator pitfall

Coverage counters are executable requirements. Adding storage variants to the shared selector initially left anonymous and union routes below the required floor and also disturbed the prior identity model's root/field count. Keep the earlier identity selector stable and use a dedicated nested-storage selector for the alias-mutation model. Assert a minimum count for every path family alongside the existing direct/reverse and one/two-hop balance.

A coverage-counter-only RED is a generator defect, not evidence of an interpreter bug. Fix route distribution before drawing runtime conclusions.

## Diagnostics and oracle boundaries

Twenty-three targeted panic-guarded checks cover nested named/anonymous/union routes for inner bounds, outer bounds, recursive const ancestors, concrete aggregate pointee mismatches, and cross-path pointer subtraction. Cross-path subtraction stays interpreter-only because unrelated-array subtraction is undefined in native C.

The warning-free fixture `adjusted_aggregate_parameter_nested_path_alias_mutation_routes.c` uses only defined same-root pointer identity/mutation and isolated storage observations. Cust, GCC, and Clang return 32 under `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_adjusted_parameter_alias_mutations_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety generated_nested_adjusted_parameter_diagnostics_remain_exact_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_nested_path_alias_mutation_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
