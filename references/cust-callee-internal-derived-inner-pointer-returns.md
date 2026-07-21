# Callee-internal derived inner-pointer promotion and returns

Date: 2026-07-21

## Scope

An adjusted `struct Item items[]` parameter may yield a scalar or named-aggregate inner pointer. Const promotion, conditional/comma selection, and a nonzero same-array offset may occur inside the returning callee rather than in the caller. The returned pointer must retain the caller-owned direct compound-literal hidden root or captured named/anonymous/union containing root, recursive field path, nonzero outer base, inner index, concrete pointee type, and const qualification through one/two return boundaries and further caller wrappers/offsets.

## Coverage pattern

- Cross two inner pointee kinds, four direct/captured root families, all three promotion placements, all three callee wrappers, all three callee offsets, one/two return hops, all three caller wrappers, and all three caller offsets.
- Use global callee wrapper markers because the pointer return value cannot carry a scalar score; keep caller and root markers separate.
- Initialize three inner elements in the returning callee, return element one, then advance once in the caller. Assert both returned values and `returned == base + 1` so an accidental rebase cannot pass.
- Retain the adjacent exact const-discard/write, bounds, cross-root, concrete-type, and out-of-scope lifetime regressions from the base callee-return matrix.
- Add a warning-free static fixture with direct and captured roots, scalar and aggregate inner pointees, all three promotion placements, one/two return hops, and caller conditional/comma/indexed-address composition.

The 3,888-case generated matrix and the static Cust/GCC/Clang fixture passed immediately, so this is deliberate conformance/property closure rather than a production-code fix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_callee_internal_promoted_inner_pointer_returns_preserve_adjusted_parameter_identity_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_callee_internal_derived_inner_pointer_returns_match_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

The next distinct seam is a second const-preserving re-forward wrapper/offset stage inside the returning callee before the pointer crosses into caller-side wrappers.
