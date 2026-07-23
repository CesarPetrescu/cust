# Final-reselected distinct-root result carry

Date: 2026-07-23

## Scope

Compose a final-reselected `const int *` or `const struct Point *` through a fresh conditional/comma wrapper and nonzero same-root offset, place a one/two-hop const-preserving helper before or after that composition, and cross one/two return boundaries. Only the root selected by the preceding distinct-root selector may supply owner/path/base/index/type/qualification/lifetime metadata.

## Coverage pattern

- Generate 1,152 valid programs by crossing both pointee kinds, all four primary direct/named/anonymous/union roots, both final selection outcomes, all three fresh wrappers and offset spellings, both helper placements, and one/two-hop helper and return depths.
- Preserve the final selected slot, prior returned slot, and rejected alternate slot. Check the fresh result's value, pointer identity, selected-root-relative difference, wrapper markers, helper/return call counts, and balanced Cartesian-product counters.
- Retain 17 exact checks after the fresh stage: bounds and const writes for both final selections and both pointee kinds; scalar/aggregate cross-root subtraction and ordering; live-versus-dangling selection; and aggregate concrete-type mismatch.
- Expand the warning-free direct/captured fixture to return 51 under Cust, GCC, and Clang. Keep unrelated-root arithmetic and dangling-pointer use interpreter-only.

## TDD result

The first focused RED returned 45 while the test expected 44 because the extension added eleven independent checks, not ten. The second RED affected alternate-root routes because the oracle compared a freshly offset alternate pointer to its root at delta zero. Correcting the model to use the fresh selected-root-relative delta made all 1,152 routes and 17 diagnostics green without production changes.

Focused commands:

```bash
cargo test --test fuzz_safety generated_final_reselected_distinct_root_results_survive_fresh_composition_without_panics -- --nocapture
cargo test --test interpreter final_reselected_distinct_root_carry -- --nocapture
cargo test --test c_compat -- --nocapture
```

The next seam is another copied-parameter selector over the freshly composed/returned pointer and its preserved rejected input, balancing argument order, selection outcome, and selector depth while retaining all earlier slots.
