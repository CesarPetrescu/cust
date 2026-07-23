# Freshly composed distinct-root result reselection

Date: 2026-07-23

## Scope

Re-enter a freshly wrapped, offset, const-forwarded, and returned `const int *` or `const struct Point *` together with the rejected pointer preserved by the preceding distinct-root selection. A one/two-hop copied-parameter selector must return one complete pointer value without mixing owner, recursive path, base/index, concrete type, qualification, or lexical lifetime provenance.

## Coverage pattern

- Generate 9,216 valid programs by extending the preceding 1,152-route composition matrix with both final argument orders, both selection outcomes, and one/two-hop selector depths.
- Balance both pointee kinds, all four primary direct/named/anonymous/union roots, both fresh source roots, all wrappers/offsets/helper placements/helper depths/return depths, and both selected outcomes.
- Preserve the fresh result slot, rejected input slot, final pre-composition slot, and all earlier inherited slots. Check selected identity/value, selected-root-relative difference, and selector call count.
- Retain 17 exact bounds, const-write, cross-root subtraction/ordering, live/dangling lifetime, and aggregate concrete-type checks after the final selector.
- Extend the warning-free direct/captured fixture to return 63 under Cust, GCC, and Clang. Native checks use equality, dereference, and same-root arithmetic only; unrelated-root arithmetic and dangling reads remain interpreter-only.

## TDD result

The new generated test and exact diagnostic layer passed immediately. This was deliberate property/conformance closure: existing copied-parameter binding and return semantics already preserve one complete interpreter-owned pointer value through the additional selector, so no production runtime change was necessary.

Focused commands:

```bash
cargo test --test fuzz_safety generated_freshly_composed_distinct_root_results_survive_parameter_reselection_without_panics -- --nocapture
cargo test --test interpreter fresh_distinct_root_reselection -- --nocapture
cargo test --test c_compat -- --nocapture
```

The explicit pointer-provenance queue is now complete. The next language slice should move to a new bounded C feature rather than adding an unbounded number of equivalent selector/wrapper repetitions.
