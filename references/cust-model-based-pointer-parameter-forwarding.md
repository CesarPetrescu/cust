# Model-based pointer-parameter forwarding

Date: 2026-07-16

## Scope

`tests/fuzz_safety.rs` now uses a fixed-seed independent model for scalar `int` pointers and named `struct`/`union` pointers forwarded through one or more pointer parameters and pointer-returning helper calls. Backing roots include global and file-static mutable/const arrays.

Generated expressions cover one- and two-hop forwarding, nested forwarding calls, pointer arithmetic, reverse addition, subtraction, conditional and comma wrappers, and indexed-address forms. Result operations cover reads, same-root difference, equality, and ordering.

## Model and diagnostics

The test-side model tracks backing-storage constness separately from the forwarded expression's pointee qualification. This distinction covers mutable roots promoted through `const T *` parameters without changing storage identity. It also tracks:

- scalar versus concrete named aggregate pointee type;
- global/static left-versus-right storage identity;
- element index and expected value;
- the first out-of-bounds index.

Targeted cases assert exact cross-root subtraction/ordering diagnostics, unequal cross-root identity, parameter-boundary const discard and pointee-type diagnostics, return-boundary const/type diagnostics, const writes, and scalar/struct/union automatic-local lifetime failures after forwarding through two helpers. Every generated program runs through `catch_unwind`.

## Native-oracle boundary

The shared C11 fixture uses only defined operations on global and file-static arrays. GCC and Clang both accept it under `-std=c11 -Wall -Wextra -Werror` and return 121, matching Cust. Ensure every file-static root is exercised because native `-Werror` rejects unused static const arrays. Forwarded pointers to automatic locals remain interpreter-only because native dereference would be undefined.

## Result

The focused model test passed immediately, so this was deliberate conformance/property coverage rather than a production-code change. It adds 144 generated cases plus 39 targeted boundary cases. All thirteen fuzz-safety tests remain sub-second locally.

## Verification

```bash
cargo test --test fuzz_safety generated_pointer_parameter_forwarding_results_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety -- --nocapture
cargo test --test interpreter pointer_parameter_forwarding -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
