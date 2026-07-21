# Callee-internal const re-forwarding before derived inner-pointer returns

Date: 2026-07-22

## Scope

After an adjusted `struct Item items[]` callee derives and promotes an inner `int *` or named-aggregate pointer to `const T *`, a second one/two-hop `const T *` helper may run before a second conditional/comma wrapper, after that wrapper, or after a second nonzero same-array offset. The final pointer may then cross one/two return boundaries and caller-side wrappers/offsets. Every copy and transformation must retain the caller-owned direct compound-literal hidden root or captured containing root/path, nonzero outer base, inner index, concrete pointee type, and const qualification.

## Coverage pattern

- Generate 2,592 balanced routes across scalar/aggregate pointees, direct/named/anonymous/union roots, all three initial promotion placements, both helper depths, all three second re-forward placements, first/second wrapper and offset spellings, one/two return boundaries, and all caller wrapper/offset combinations.
- Use four inner elements because the first callee stage, second callee stage, and caller stage each advance by one. Assert values at the promoted, returned-base, and caller-returned positions plus relative identities so an accidental rebase cannot pass.
- Keep first-stage, second-stage, caller, and root markers separate. This proves selected/unselected/comma evaluation without conflating storage identity with score bookkeeping.
- Retain the adjacent eleven exact const-discard/write, bounds, cross-root, concrete-type, and lifetime regressions from the base callee-return test.
- Add a warning-free static direct/captured scalar/aggregate fixture. It returns 24 under Cust, GCC, and Clang.

The generated matrix and static fixture passed immediately, so this is deliberate conformance/property closure rather than a production-code change.

Focused commands:

```bash
cargo test --test fuzz_safety generated_callee_internal_reforwarded_inner_pointer_returns_preserve_adjusted_parameter_identity_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_callee_internal_reforwarded_derived_inner_pointer_returns_match_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

The next distinct seam is caller-side const-preserving re-forwarding after the pointer has crossed the callee return boundary and before/after caller wrappers and offsets.
