# Post-selection wrappers and offsets after nested const-parameter return selection

Date: 2026-07-22

## Scope

Compose the ultimate `const int *` or `const struct Point *` returned from two copied-parameter selector stages with a conditional/comma wrapper placed before or after a nonzero same-array offset. Preserve the selected pointer and all original final/base/first-stage slots while the composed pointer moves to the adjacent element.

## Coverage pattern

- Generate 4,608 valid programs across both pointee kinds, direct/named/anonymous/union roots, both first selections and helper depths, both second-call argument orders/selections/helper depths, all three conditional/comma wrappers, three offset spellings, and both wrapper placements.
- Choose the offset direction from the selected identity: final pointers move by `-1` to base, while base pointers move by `+1` to final. This keeps every valid route in bounds and gives an independent expected composed identity.
- Verify selected and composed values/identity/difference/ordering, original final/base/first-stage/alternate slots, root markers, exact selector counts, one-time wrapper markers, balanced dimensions, and qualification/type/lifetime metadata.
- Keep nine panic-guarded exact diagnostics: scalar/aggregate const writes, upper bounds, cross-root subtraction, out-of-scope lifetime use, and one aggregate concrete-type mismatch.
- Use a warning-free direct-scalar/captured-aggregate compiler-oracle fixture. The scalar route wraps before `-1`; the aggregate route wraps after `+1`; Cust, GCC, and Clang return 40.

The generated matrix passed immediately, so this is deliberate conformance/property closure rather than a production fix. The fixture's initial expected score of 41 was incorrect; direct Cust/GCC/Clang execution all returned 40, so the independent test oracle was corrected before GREEN.

Focused commands:

```bash
cargo test --test fuzz_safety generated_post_selection_wrappers_and_offsets_preserve_nested_const_parameter_return_identity_without_panics -- --nocapture
cargo test --test interpreter post_selection_wrappers_and_offsets -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is re-forwarding the composed result through another one/two-hop const-preserving helper and return boundary while retaining the pre-composition selected slot and exact diagnostics.
