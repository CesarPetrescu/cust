# Post-wrapper re-forwarding for captured-field adjusted parameters

Date: 2026-07-20

## Scope

Pointers from captured named, anonymous, and union-containing aggregate-compound-literal array fields may cross an initial one/two-hop mutable or const helper, a conditional/comma post-wrapper, and then another one/two-hop helper before or after `field + 1`, `1 + field`, or `&field[1]`. Every function boundary copies the pointer slot only; hidden containing-root identity, recursive field path, absolute nonzero base, concrete pointee type, and qualification must remain unchanged when the final argument undergoes C array-parameter adjustment.

## Coverage result

`tests/fuzz_safety.rs` adds 5,184 deterministic scalar/named-aggregate two-writer/const-reader cases. The exhaustive matrix crosses three named/anonymous/union paths, four same-element/same-field-distinct/cross-field/separate-root relationships, three inner wrappers, three post wrappers, three offsets, one/two-hop initial helpers, one/two-hop re-forwarding helpers, and re-forwarding before/after offset formation. Separate inner/post marker sets prove short-circuit and comma evaluation; six captured roots remain stable and copied parameter slots remain local.

Fourteen panic-guarded checks preserve exact inner and absolute outer bounds, recursive const-discard/write, separate-root subtraction, concrete aggregate pointee mismatch, and out-of-scope hidden-root diagnostics. The first generated run reached every behavioral case and failed only on an incorrect expected matrix count (10,368 instead of 5,184); correcting the independent coverage oracle produced GREEN without production changes.

The warning-free fixture `adjusted_aggregate_parameter_post_wrapper_reforwarded_compound_literal_field_routes.c` covers mutable and const one/two-hop re-forwarding on both sides of offset formation and returns 45 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_wrapper_reforwarded_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter post_wrapper_reforwarded_compound_literal_field_routes -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate.
