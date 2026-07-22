# Final returned/composed pointer reselection

Date: 2026-07-22

## Scope

Pass the final returned composed `const int *` or `const struct Point *` and the preserved pre-composition selected pointer through another copied-parameter selector. The selector may receive either argument order, choose either parameter, and use one or two helper calls; it must preserve exactly the chosen pointer's interpreter-owned provenance while leaving both caller slots unchanged.

## Coverage pattern

- Generate 64 valid programs across both pointee kinds, all four direct/named/anonymous/union roots, both final argument orders, both selection outcomes, and one/two-hop selector depth.
- Build each route on the prior nested selection, wrapper/offset composition, re-forwarding, and final return-boundary generator while varying those prerequisite routes by pointee/root.
- Verify the reselected value, equality, base-relative difference, ordering, difference from the preserved selected pointer, exact helper-call count, and unchanged returned/composed and selected/final-or-base slots.
- Retain nine panic-guarded diagnostics after final reselection: scalar/aggregate const writes, scalar/aggregate bounds, scalar/aggregate cross-root subtraction, scalar/aggregate out-of-scope lifetime use, and aggregate concrete-type mismatch.
- Register a warning-free fixture whose scalar route selects the preserved selected pointer through one helper and whose aggregate route selects the final returned pointer through two helpers. Cust, GCC, and Clang return 48.

The focused interpreter and generated-property tests passed immediately, so no production runtime change was needed. This is deliberate conformance/property closure.

Focused commands:

```bash
cargo test --test fuzz_safety generated_final_returned_composed_pointer_reselection -- --nocapture
cargo test --test interpreter final_returned_composed_pointer_reselection -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is selecting between the final returned pointer and a same-typed pointer from a different direct/captured storage root. That matrix should prove only the selected root contributes owner/path/base/index/lifetime/qualification metadata and keep cross-root subtraction/ordering diagnostics interpreter-only where native C behavior is undefined.
