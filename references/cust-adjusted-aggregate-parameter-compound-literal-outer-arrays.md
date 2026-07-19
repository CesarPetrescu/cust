# Adjusted aggregate parameters over compound-literal outer arrays

Date: 2026-07-19

## Scope

Aggregate compound literals remain alive for the enclosing block when their addresses are captured once. Outer `struct Item[N]` fields selected through named, anonymous, and union-containing literal roots can then decay into adjusted aggregate parameters without losing the literal root, containing path, outer index, embedded field, or inner index.

`tests/fuzz_safety.rs` models 48 scalar/named-aggregate two-writer/`const`-reader cases over named, anonymous, and union paths. Four balanced relationships cover the same element, a distinct element in the same embedded array, a different outer-array field, and a separate captured literal root. Direct/reverse addresses and one/two-hop forwarding are balanced. Six initializer markers per generated program prove that every hidden root is captured once, while callee parameter-slot reassignment remains local.

## Coverage result and diagnostics

The generated matrix and the warning-free fixture passed immediately, so this package is deliberate conformance/property closure rather than a production-code change. Existing recursive pointer metadata already retained aggregate-literal owner/path identity through C array adjustment.

Targeted panic-guarded cases retain exact inner/outer bounds, const-discard, concrete aggregate pointee mismatch, and separate-root subtraction diagnostics. Cross-root subtraction remains interpreter-only because native C does not define subtraction between unrelated arrays.

The fixture `adjusted_aggregate_parameter_compound_literal_outer_alias_routes.c` uses only defined identity and mutation routes. It captures four literal roots once and returns 67 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_compound_literal_outer_array_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_compound_literal_outer_alias_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
