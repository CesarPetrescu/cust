# Outer caller return boundaries for derived inner const pointers

Date: 2026-07-22

## Scope

After an adjusted aggregate-parameter callee derives and promotes an inner scalar or named-aggregate pointer, optionally re-forwards it, and returns it to a caller, that caller may apply a one/two-hop const-preserving helper plus conditional/comma and nonzero-offset composition and then return the result across one/two additional function boundaries. Final use must retain the direct hidden root or captured named/anonymous/union containing root/path, outer base, inner index, concrete pointee type, const qualification, and lexical lifetime provenance.

## Coverage pattern

- Generate 2,592 balanced routes across both pointee kinds, all four direct/captured roots, promotion and callee re-forward placement, one/two initial return boundaries, caller re-forward placement/depth, all caller wrapper/offset routes, and balanced one/two outer return boundaries.
- Keep promotion, callee re-forward, caller wrapper, caller helper-call, root, and outer-slot markers independent. Check values at promoted/base/final indexes 1/2/3 and reassign copied pointer slots only.
- Preserve eleven exact const-discard/write, inner-bounds, cross-root, concrete-type, and out-of-scope lifetime checks with the outer return boundary inserted.
- Add a warning-free fixture covering direct/captured scalar and aggregate roots plus one/two initial and outer return boundaries. Cust, GCC, and Clang return 20.

The focused generated test passed immediately, so this was deliberate conformance/property closure rather than a production runtime fix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_outer_caller_returned_inner_pointers_preserve_adjusted_parameter_identity_without_panics -- --nocapture
cargo test --test interpreter outer_caller_returned_derived_inner_pointers_match_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

The next distinct seam is a final receiving caller that re-forwards and wraps the already outer-returned pointer before final use.
