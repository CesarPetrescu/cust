# Returned distinct-root selected-result reselection

Date: 2026-07-23

## Scope

Re-enter a post-composition returned `const int *` or `const struct Point *` and the preserved unselected distinct-root pointer through another one/two-hop copied-parameter selector. The selected final input alone must supply owner/path/base/index/type/qualification/lifetime metadata: choosing the returned pointer retains its nonzero same-root offset, while choosing the alternate restores that untouched root and index.

## Coverage pattern

- Generate 36,864 valid programs by crossing both pointee kinds, all four primary direct/named/anonymous/union roots, both initially carried roots, every earlier conditional/comma wrapper, offset spelling, helper placement/depth, return depth, final argument order, final selection outcome, and one/two-hop selector depth.
- Preserve explicit copies of the returned and unselected input slots. Check the final selected value, pointer identity, root-relative difference, selector call count, prior carry invariants, and balanced Cartesian-product counters.
- Retain 17 exact checks after the final selector: bounds and const writes for both possible final selections and both pointee kinds; scalar/aggregate cross-root subtraction and ordering; live-versus-dangling selection; and aggregate concrete-type mismatch.
- Register a warning-free direct/captured fixture returning 39 under Cust, GCC, and Clang. Native coverage uses pointer equality, local dereference, and same-root relationships only; unrelated-root arithmetic and dangling-pointer use remain interpreter-only.

## TDD result

The complete focused matrix passed immediately. This was an explicitly selected conformance/property closure package, so no production change was invented. Existing copied-parameter and return-value pointer handling already copied one complete interpreter pointer value without merging provenance from the unselected input.

Focused commands:

```bash
cargo test --test fuzz_safety generated_returned_distinct_root_results_survive_final_parameter_reselection_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_returned_distinct_root_reselection_matches_fixture -- --nocapture
cargo test --test c_compat -- --nocapture
```

The next seam is a fresh wrapper/nonzero-offset/helper/return composition after final reselection, balancing whether that selector chose the already composed returned pointer or the untouched alternate root.
