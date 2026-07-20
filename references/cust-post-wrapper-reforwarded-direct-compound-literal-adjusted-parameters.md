# Post-wrapper re-forwarding for direct aggregate-array literal adjusted parameters

Date: 2026-07-21

## Scope

Pointers from direct aggregate-array compound literals may cross an inner conditional/comma wrapper, an initial one/two-hop mutable or const helper, a second conditional/comma pointer wrapper, and another one/two-hop helper before or after `literal + 1`, `1 + literal`, or `&literal[1]`. Every boundary copies only the interpreter-owned pointer slot; the hidden literal root, absolute nonzero base, concrete aggregate pointee type, and qualification must remain unchanged when the final pointer undergoes C array-parameter adjustment.

## Coverage result

Use an exhaustive matrix over scalar/named-aggregate embedded pointee kind, same-element/same-array-distinct/separate-root relationship, inner and post wrapper, nonzero-offset spelling, initial helper depth, re-forward helper depth, and re-forward placement. Keep separate inner/post marker sets and check copied parameter slots separately from pointee mutations. Add a dedicated const-array-typedef matrix over every wrapper/offset/helper/placement route.

The 2026-07-21 Cust run covered 1,296 generated alias cases, 216 const-array-typedef cases, and 14 panic-guarded bounds/const/root/type/lifetime diagnostics. The RED was an independent matrix-count error (2,592 expected versus 1,296 actual), so recompute the Cartesian product before treating count-only failures as interpreter defects. Existing Cust behavior passed every semantic case without production changes. The warning-free fixture returned 49 under Cust, GCC, and Clang.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_wrapper_reforwarded_direct_aggregate_array_literal_adjusted_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test interpreter post_wrapper_reforwarded_direct_compound_literal_routes -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Follow with the recursion-depth regression and canonical local/Docker gate.
