# Final-received derived inner pointers through copied const parameters

Date: 2026-07-22

## Scope

After a derived inner `const int *` or `const struct Point *` has crossed adjusted-parameter callee returns, outer caller returns, and a final receiving-caller re-forward stage, pass both the final pointer and its pre-final same-array base through copied `const T *` parameters. Parameter-slot reassignment must remain local while storage owner/path/index/type/qualification/lifetime metadata remains unchanged.

## Coverage pattern

- Generate 16 valid programs across both pointee kinds, direct/named/anonymous/union roots, and one/two-hop parameter helpers.
- In each helper, copy both parameters, reassign the original parameter slots in opposite directions, then verify final/base reads, equality, difference, ordering, and local-slot identity.
- After the call, verify caller pointer slots still retain the final and pre-final base values and identities.
- Keep exact const-write, cross-root subtraction, concrete aggregate type, and out-of-scope lifetime diagnostics under panic guards.
- Add a warning-free direct/captured fixture spanning scalar and aggregate pointees; Cust, GCC, and Clang return 33.

Existing runtime behavior passed immediately, so this was deliberate conformance/property closure rather than a production fix.

Focused commands:

```bash
cargo test --test fuzz_safety generated_final_received_inner_pointers_preserve_identity_through_const_parameter_reentry_without_panics -- --nocapture
cargo test --test interpreter final_received_const_parameter_reentry -- --nocapture
cargo test --test c_compat -- --nocapture
```

A distinct follow-up is parameter-return re-entry: have one/two helpers select and return either the final pointer or its pre-final base, then verify the next caller still sees the original same-array identity, const qualification, and lifetime boundaries.
