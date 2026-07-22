# Final-received derived inner pointers through parameter-return selection

Date: 2026-07-22

## Scope

After final-received `const int *` and `const struct Point *` values re-enter copied parameters, have one/two-hop helpers conditionally return either the final pointer or its pre-final same-array base. The receiving caller must retain the selected pointer's storage owner/path/index, concrete pointee type, const qualification, and lexical lifetime provenance without changing either caller-side input slot.

## Coverage pattern

- Generate 32 valid programs across both pointee kinds, direct/named/anonymous/union roots, both selected parameters, and one/two-hop selector helpers.
- Verify selected reads, equality, differences from both inputs, ordering, original final/base reads and identity, root markers, and exact selector call counts.
- Keep exact returned-pointer const-write, cross-root subtraction, concrete aggregate type, and out-of-scope lifetime diagnostics under panic guards.
- Use a warning-free direct-scalar/captured-aggregate fixture that selects the final scalar pointer through two helpers and the base aggregate pointer through one helper; Cust, GCC, and Clang return 24.

Existing runtime behavior passed the first focused generated run, so this is deliberate conformance/property closure rather than a production fix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_const_parameter_return_selection_preserves_final_received_inner_pointer_identity_without_panics -- --nocapture
cargo test --test interpreter const_parameter_return_selection -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is nested parameter-return selection: feed the first selected result plus its alternate same-array input into a second one/two-hop selector in the receiving caller, swap argument order, and verify the ultimate result retains identity, qualification, and lifetime boundaries.
