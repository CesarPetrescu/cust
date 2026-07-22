# Caller-side const re-forwarding after derived inner-pointer returns

Date: 2026-07-22

## Scope

After an adjusted aggregate-parameter callee derives an inner `int *` or named-aggregate pointer, promotes it to `const T *`, optionally re-forwards it inside the callee, and crosses one/two return boundaries, the receiving caller may pass it through another one/two-hop const-preserving helper before a conditional/comma wrapper, after that wrapper, or after its nonzero same-array offset. The caller stage must retain the original direct compound-literal hidden root or captured named/anonymous/union containing root/path, nonzero outer base, inner index, concrete pointee type, const qualification, and lifetime provenance.

## Coverage pattern

- Generate 2,592 balanced routes across scalar/aggregate pointees, four direct/captured roots, three callee promotion placements, rotated and balanced callee wrapper/offset/re-forward dimensions, one/two callee return boundaries, three caller re-forward placements, one/two caller helper hops, and every caller wrapper/offset spelling.
- Initialize four inner elements. The callee promotion advances to element 1, callee re-forwarding advances to element 2, and caller re-forwarding advances to element 3. Assert the promoted/base/final values and relative identities so accidental rebasing cannot pass.
- Keep callee promotion, callee re-forward, caller wrapper, caller helper-call, and root markers separate. Reassign only a copied caller pointer slot and assert the original returned pointer remains stable.
- Re-run eleven exact const-discard/write, bounds, cross-root, concrete-type, and lifetime diagnostics with the caller helper inserted after the return boundary.
- Add a warning-free direct/captured scalar/aggregate fixture. It returns 28 under Cust, GCC, and Clang.

The first generated run was RED only because a base-3 rotating coverage counter used a period that did not divide the 2,592-case matrix (`[891, 891, 810]`). Derive that dimension from already balanced loop indexes instead of weakening the expected counts. After that generator correction, every interpreter route passed without production changes, so this is deliberate conformance/property closure.

Focused commands:

```bash
cargo test --test fuzz_safety generated_caller_reforwarded_inner_pointer_returns_preserve_adjusted_parameter_identity_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_caller_reforwarded_derived_inner_pointer_returns_match_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

The next distinct seam is an outer caller function that returns the already caller-re-forwarded pointer across another one/two return boundaries before final use.
