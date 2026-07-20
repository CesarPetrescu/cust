# Post-forward wrappers for direct aggregate-array literal adjusted parameters

Date: 2026-07-20

## Scope

A direct aggregate-array compound literal may be selected by an inner conditional/comma expression, copied through a one- or two-hop `struct Item *` / `const struct Item *` helper, then selected again by a post-forward conditional/comma pointer expression either before or after `+ 1`, reverse `1 +`, or `&pointer[1]`. Every layer must preserve the selected hidden root, absolute nonzero base, concrete aggregate pointee type, and qualification when the final pointer is adjusted into an array parameter.

## Coverage result

`tests/fuzz_safety.rs` adds 648 deterministic scalar/named-aggregate two-writer/const-reader alias cases. The full matrix crosses two pointee families, three same-element/same-array-distinct/separate-root relationships, three inner wrappers, three post-forward wrappers, three offsets, before/after-offset post-wrapper placement, and one/two-hop outer helpers. Separate inner and post markers prove selected branches execute once, unselected branches do not execute, and comma left operands execute once. Inner forwarding and copied parameter slots remain local.

A separate 108-case const-array-typedef matrix covers every inner-wrapper/post-wrapper/offset/placement/helper-depth route. Fourteen panic-guarded checks retain exact inner and absolute outer bounds, const-discard/write behavior, separate-root subtraction, concrete aggregate pointee mismatch, and out-of-scope hidden-root diagnostics. Existing Cust behavior passed immediately, so this is deliberate conformance coverage rather than a production fix.

The warning-free fixture `adjusted_aggregate_parameter_post_forward_wrapped_direct_compound_literal_routes.c` covers representative mutable and const one/two-hop routes with wrappers on both sides of offsets and returns 49 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_forward_wrapped_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter post_forward_wrapped_direct_compound_literal_routes -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate.
