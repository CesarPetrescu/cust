# Post-promotion const re-forwarding for direct-literal derived inner pointers

Date: 2026-07-21

## Scope

After a direct aggregate-array compound literal decays into an adjusted `struct Item *` parameter at a nonzero base, a derived mutable `int *` or `struct Point *` may be promoted to `const T *`. Passing that promoted inner pointer through one/two-hop const-preserving helpers before a second conditional/comma wrapper, after the wrapper, or after a second nonzero offset must preserve the hidden literal root, adjusted outer base, inner index, concrete pointee type, and const qualification. Function parameters and local pointer variables copy pointer slots; they do not create new storage roots.

## Coverage result

Use an exhaustive matrix over scalar/named-aggregate inner pointees; initial promotion before the first wrapper, after the first wrapper, or after the first offset; every first and second conditional/comma wrapper; every first and second `pointer + 1`, `1 + pointer`, and `&pointer[1]` offset; one/two-hop promotion helpers; second const re-forwarding before the wrapper, after the wrapper, or after the offset; and one/two-hop const-preserving helpers. The 2026-07-21 run covered 5,832 valid identity/read routes plus 36 panic-guarded mutable-rebinding/write diagnostics.

Separate root, first-wrapper, and second-wrapper markers prove one-time evaluation. Adjusted/local pointer-slot reassignment remains local, legal const reads preserve identity, and the warning-free static fixture returns 39 under Cust, GCC, and Clang. Existing Cust behavior passed immediately, so this is deliberate conformance/property closure rather than a production-code fix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_promotion_inner_const_reforwarding_preserves_direct_literal_adjusted_parameter_identity_without_panics -- --nocapture
cargo test --test interpreter direct_literal_derived_inner_pointer_const_reforward -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
```

Follow with the canonical local/Docker gate. The next distinct seam is the same post-promotion inner-pointer matrix over captured named/anonymous/union aggregate-compound-literal array fields, where containing-root and recursive field-path metadata must also survive.
