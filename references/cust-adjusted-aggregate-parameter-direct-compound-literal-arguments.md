# Direct aggregate-array compound literals through adjusted parameters

Date: 2026-07-20

## Scope

A mutable `(struct Item[N]){...}` or const-array-typedef compound literal used directly as a function argument decays while binding the C-adjusted aggregate parameter. The callee may derive scalar or named-aggregate pointers from embedded arrays; those pointers must retain the hidden literal root, outer element index, embedded field, and inner index through direct/reverse address forms, one/two-hop forwarding, and copied pointer slots.

`tests/fuzz_safety.rs` models 48 scalar/named-aggregate two-writer/`const`-reader cases. Three balanced relationships cover the same element, a distinct element in the same embedded array, and a separate direct literal root. Per-root initializer markers prove each argument literal is captured once, while callee parameter-slot reassignment remains local. Eleven panic-guarded checks retain exact inner/outer bounds, const-discard, const-write, concrete aggregate pointee mismatch, and separate-root subtraction diagnostics.

## Coverage result and native-oracle pitfall

The generated matrix and fixture passed immediately, so this package is deliberate conformance/property closure rather than a production-code change. Existing aggregate-array literal and adjusted-parameter metadata already preserve recursive hidden-root identity.

Do not increment the same marker object in two separate function arguments: C does not sequence argument evaluation, and Clang rejects the resulting `++marker` pair under `-Werror,-Wunsequenced`. Use distinct `left_marker` and `right_marker` objects and check both after the call. Cross-root subtraction remains interpreter-only because native C does not define subtraction between unrelated arrays.

The fixture `adjusted_aggregate_parameter_direct_compound_literal_alias_routes.c` passes mutable and const-typedef literals directly, uses only defined identity/mutation routes, and returns 72 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_direct_compound_literal_alias_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
