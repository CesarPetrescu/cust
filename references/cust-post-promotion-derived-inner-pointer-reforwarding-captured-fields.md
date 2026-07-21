# Post-promotion const re-forwarding for captured-field derived inner pointers

Date: 2026-07-21

## Scope

After a named, anonymous, or union-containing aggregate compound literal is captured once, one of its aggregate-array fields may decay into an adjusted `struct Item *` parameter at a nonzero base. A mutable `int *` or `struct Point *` derived from that adjusted parameter may then be promoted to `const T *` and passed through another one/two-hop const-preserving helper before a second conditional/comma wrapper, after the wrapper, or after a second nonzero offset. Every step must preserve the containing hidden root, recursive field path, outer base, inner index, concrete pointee type, and const qualification.

## Coverage result

Use an exhaustive matrix over both inner pointee kinds; named, anonymous, and union-containing captured paths; all three initial promotion placements; every first and second conditional/comma wrapper; every first and second `pointer + 1`, `1 + pointer`, and `&pointer[1]` offset; one/two-hop promotion helpers; all three second const re-forwarding placements; and one/two-hop const-preserving helpers. The 2026-07-21 run covered 17,496 valid identity/read routes plus 108 panic-guarded mutable-rebinding/write diagnostics.

Separate six-root, first-wrapper, and second-wrapper marker sets prove stable captured storage and one-time evaluation. Adjusted and local pointer slots remain independently assignable, legal const reads preserve identity, and the warning-free named/anonymous/union fixture returns 58 under Cust, GCC, and Clang. Existing Cust behavior passed immediately, so this is deliberate conformance/property closure rather than a production-code fix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_promotion_inner_const_reforwarding_preserves_captured_field_adjusted_parameter_identity_without_panics -- --nocapture
cargo test --test interpreter captured_field_derived_inner_pointer_const_reforward -- --nocapture
cargo test --test c_compat -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
```

Follow with the canonical local/Docker gate. A distinct next seam is returning these already-promoted derived inner pointers from adjusted-parameter callees to their callers, where the pointer must retain direct/captured compound-literal storage identity and const qualification across the function-return boundary.
