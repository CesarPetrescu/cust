# Distinct-root final pointer reselection

Date: 2026-07-23

## Scope

Pair a final returned/composed `const int *` or `const struct Point *` with a same-typed pointer derived from a different direct or captured storage root, then pass both through a one/two-hop copied-parameter selector. The selected pointer must retain only its own owner/path/base/index/type/qualification/lifetime provenance; the unselected input must not contaminate it.

## Coverage pattern

- Generate 64 valid programs across both pointee kinds, all four primary direct/named/anonymous/union roots, both final argument orders, both selection outcomes, and one/two-hop selector depth.
- Pair each primary root with a distinct direct/captured alternate root and initialize primary/alternate pointees to separate value families (`5/7/9` versus `15/17/19`) so selected-root reads cannot pass through merged metadata.
- Verify selected identity/value, inequality with the unselected root, same-root differences, unchanged primary/alternate slots, exact helper-call counts, initializer markers, and balanced dimensions.
- Retain 17 exact diagnostic/lifetime checks: scalar/aggregate bounds and const writes for both selection outcomes, scalar/aggregate cross-root subtraction and ordering, scalar/aggregate live-versus-dangling lifetime selection, and aggregate concrete-type mismatch.
- Register a warning-free direct/captured fixture. Its scalar route selects the alternate captured root and its aggregate route selects the primary captured root; Cust, GCC, and Clang return 32. Cross-root subtraction/ordering and dangling-pointer use remain interpreter-only because native C does not define those operations usefully.

The focused property test passed immediately, so no production runtime change was needed. This is deliberate conformance/property closure.

Focused commands:

```bash
cargo test --test fuzz_safety generated_distinct_root_final_reselection -- --nocapture
cargo test --test interpreter distinct_root_final_reselection -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is to apply a conditional/comma wrapper and nonzero same-root offset after distinct-root selection, then cross another const-preserving helper/return boundary. That should prove all subsequent operations use only the selected root's provenance while both original root slots remain unchanged.
