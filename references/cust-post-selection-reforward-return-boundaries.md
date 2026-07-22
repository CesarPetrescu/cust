# Post-selection re-forwarding and final return boundaries

Date: 2026-07-22

## Scope

Carry the `const int *` or `const struct Point *` produced by nested copied-parameter selection plus a conditional/comma wrapper and nonzero same-array offset through an additional one/two-hop const-preserving helper and one/two final return boundaries. The pre-composition selected slot must remain unchanged while the returned pointer retains the composed storage identity.

## Coverage pattern

- Generate 18,432 valid programs across both pointee kinds, direct/named/anonymous/union roots, both selector stages and helper depths, both second-stage argument orders/selections, all three wrappers, all three offset spellings, both wrapper placements, both re-forward helper depths, and both final return counts.
- Verify selected/composed/re-forwarded/returned values, equality and base-relative difference; original final/base/first-stage/alternate slots; root and wrapper markers; exact selector/re-forward/return call counts; and balanced route counters.
- Keep nine panic-guarded exact diagnostics after the added boundaries: scalar/aggregate const writes, scalar/aggregate bounds, scalar/aggregate cross-root subtraction, scalar/aggregate out-of-scope lifetime use, and aggregate concrete-type mismatch.
- Register a warning-free direct-scalar/captured-aggregate fixture. Its scalar route uses a one-hop re-forward plus two return boundaries; its aggregate route uses a two-hop re-forward plus one return boundary. Cust, GCC, and Clang return 38.

The first focused run was RED because the test generator inserted returned-pointer assertions without inserting the corresponding declarations; changing the transformation to locate the final `return (` fixed the harness. A second diagnostic RED expected index 3 while the generated probe reached index 4; using the intended `+ 2` boundary produced the established exact index-3 diagnostic. Cust runtime behavior required no production change, so this is deliberate conformance/property closure.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_selection_reforward_and_return_boundaries -- --nocapture
cargo test --test interpreter post_selection_reforward_return_boundaries -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is re-entering the final returned pointer and the preserved pre-composition selected pointer into another copied-parameter selector, balancing argument order, selection outcome, and helper depth while retaining exact provenance diagnostics.
