# Distinct-root selected-result composition and return

Date: 2026-07-23

## Scope

Carry a selected `const int *` or `const struct Point *` from one of two distinct direct/captured roots through a conditional/comma wrapper, a nonzero same-root offset, a one/two-hop const-preserving helper, and one/two return boundaries. The selected root alone must supply owner/path/base/index/type/qualification/lifetime metadata; both original root slots remain unchanged.

## Coverage pattern

- Generate 4,608 valid programs across both pointee kinds, all four primary direct/named/anonymous/union roots, both selected roots, all three wrapper forms, all three offset spellings, composition before/after the helper, one/two helper hops, and one/two return boundaries.
- Alternate `+1` and `-1` by primary-root family so selected-root lower and upper neighbors are balanced. Distinct value families (`5/7/9` and `15/17/19`) prove the carried result reads from the selected root.
- Preserve explicit copies of both original root slots and verify wrapper markers, helper/return call counts, selected-root pointer differences, values, and all Cartesian-product counters.
- Retain 17 exact checks after the carry boundary: bounds and const writes for both selected roots and pointee kinds; scalar/aggregate cross-root subtraction and ordering; live-versus-dangling selection; and aggregate concrete-type mismatch.
- Register a warning-free direct/captured fixture returning 31 under Cust, GCC, and Clang. Native coverage uses only same-root offsets/equality; cross-root arithmetic and dangling-pointer diagnostics remain interpreter-only.

## TDD result

The first focused run was RED because the generated program used `carry_selected`, `carry_unselected`, and `carry_comma` without declaring those marker variables. After fixing that test-generator defect, all 4,608 runtime routes and 17 exact diagnostics passed without production changes. This is deliberate conformance/property closure.

Focused commands:

```bash
cargo test --test fuzz_safety generated_distinct_root -- --nocapture
cargo test --test interpreter distinct_root_selected_result_carry -- --nocapture
cargo test --test c_compat -- --nocapture
```

The next seam is copied-parameter reselection between the post-composition returned pointer and the preserved unselected root, with swapped argument order and both outcomes proving that either the composed selected root or untouched alternate root survives intact.
