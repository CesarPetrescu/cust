# Final receiving-caller re-forwarding after outer pointer returns

Date: 2026-07-22

## Scope

After a derived inner `const int *` or `const struct Point *` crosses one/two adjusted-parameter callee returns and one/two outer caller returns, a final receiving caller may apply another one/two-hop const-preserving helper before a conditional/comma wrapper, after that wrapper, or after a nonzero offset. Final use must retain the direct hidden root or captured named/anonymous/union containing root/path, outer base, inner index, concrete pointee type, const qualification, and lexical lifetime provenance.

## Coverage pattern

- Generate 2,592 balanced routes over both pointee kinds, all four root families, prior promotion/re-forward/return dimensions, and balanced final placement/wrapper/offset/helper-depth dimensions.
- Derive base-3 final dimensions from already balanced loop indexes. Avoid a raw `(case_index / 243) % 3` selector: 2,592 is not divisible by 729 and produces imbalanced route counts.
- Use a five-element inner array. The callee stages produce indexes one and two, the outer caller produces index three, and the final receiving caller produces index four; check all intermediate values and identities.
- Keep final wrapper markers and helper call counters separate from callee and outer-caller markers. Reassign only copied pointer slots.
- Preserve eleven exact const-discard/write, bounds, cross-root, concrete-type, and out-of-scope lifetime diagnostics with the final stage inserted.
- Add a warning-free direct/captured fixture spanning scalar and aggregate pointees plus one/two-hop final helpers. Cust, GCC, and Clang return 30.

The generated matrix passed immediately, so this was deliberate conformance/property closure. The first static-fixture RED returned 28 instead of the expected 30 because the fixture expected two-hop helper counts while its `*_twice` receiving wrappers still delegated to one-hop helper bodies. Making those wrappers exercise the two-hop helpers and correcting final wrapper-call counts produced the intended warning-free matrix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_final_receiving_caller_reforwarded_inner_pointers_preserve_adjusted_parameter_identity_without_panics -- --nocapture
cargo test --test interpreter final_receiving_caller_reforwarded_derived_inner_pointers -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is parameter re-entry: pass the final received pointer and its pre-final base through copied `const T *` parameters and verify same-root equality/difference/ordering, local slot reassignment, and exact cross-root/type/lifetime boundaries.
