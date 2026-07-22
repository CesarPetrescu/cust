# Nested const-parameter return selection after final receipt

Date: 2026-07-22

## Scope

Take the first helper-selected `const int *` or `const struct Point *`, pair it with the alternate same-array final/base pointer, and select again through a one/two-hop copied-parameter helper. Exercise both second-call argument orders and both selections while preserving the ultimate pointer's storage owner/path/index, concrete pointee type, qualification, and lexical lifetime provenance.

## Coverage pattern

- Generate 256 valid programs across both pointee kinds, direct/named/anonymous/union roots, both first selections, one/two-hop first helpers, both second-call argument orders, both second selections, and one/two-hop second helpers.
- Verify the ultimate and intermediate selected values, equality/difference/ordering against original final/base pointers, unchanged caller slots, root markers, balanced ultimate identities, and exact combined helper call counts.
- Keep seven exact nested-stage const-write, cross-root subtraction, concrete aggregate type, and out-of-scope lifetime checks under panic guards.
- Use a warning-free direct-scalar/captured-aggregate fixture: the scalar route swaps the second call's arguments and selects its second parameter, while the aggregate route keeps argument order and selects its first parameter. Cust, GCC, and Clang return 30.

The first generated run reached every interpreter assertion and failed only because the test-side expected case count was miscomputed as 512 instead of 256. After correcting the five binary dimensions, existing runtime behavior passed; this is deliberate conformance/property closure rather than a production fix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_nested_const_parameter_return_selection_preserves_final_received_inner_pointer_identity_without_panics -- --nocapture
cargo test --test interpreter nested_const_parameter_return_selection -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is post-selection composition: wrap the ultimate nested result in conditional/comma expressions before or after a nonzero same-array offset, then prove identity, qualification, one-time selection counts, and exact bounds/cross-root/lifetime diagnostics remain stable.
