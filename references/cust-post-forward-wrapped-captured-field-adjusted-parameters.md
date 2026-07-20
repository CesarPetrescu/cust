# Post-forward wrappers for captured aggregate-compound-literal field adjusted parameters

Date: 2026-07-20

## Scope

Array fields selected from captured named, anonymous, and union-containing aggregate compound literals may pass through one- or two-hop `struct Item *` / `const struct Item *` helpers, then through a second conditional/comma pointer wrapper before or after `+ 1`, reverse `1 +`, or indexed address formation. The composed expression must retain the hidden containing root, recursive field path, absolute nonzero base, concrete aggregate pointee type, and qualification when C array adjustment copies the final pointer slot.

## Coverage result

`tests/fuzz_safety.rs` adds 2,592 deterministic scalar/named-aggregate two-writer/const-reader cases. The exhaustive matrix crosses three named/anonymous/union paths, four same-element/same-field-distinct/cross-field/separate-root relationships, three inner wrappers, three post-forward wrappers, three offsets, before/after-offset post-wrapper placement, and one/two-hop outer helpers. Separate inner/post markers prove selected branches and comma left operands execute once while unselected branches do not; six captured roots remain stable and copied parameter slots remain local.

Fourteen panic-guarded checks retain exact inner and absolute outer bounds, recursive const-discard/write behavior, separate-root subtraction, concrete aggregate pointee mismatch, and out-of-scope hidden-root diagnostics. Existing Cust behavior passed immediately, so this is deliberate conformance/property coverage rather than a production fix.

The warning-free fixture `adjusted_aggregate_parameter_post_forward_wrapped_compound_literal_field_routes.c` covers representative mutable and const routes and returns 45 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_forward_wrapped_captured_literal_field_offset_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter post_forward_wrapped_compound_literal_field_routes -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate.
