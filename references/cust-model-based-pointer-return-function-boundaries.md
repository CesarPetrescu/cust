# Model-based pointer-return function boundaries

Date: 2026-07-16

## Scope

`tests/fuzz_safety.rs` now uses a fixed-seed independent model for pointers returned from Cust functions over scalar `int` arrays and named `struct`/`union` arrays. Functions select either a global or file-static root and return mutable or const-qualified pointers at generated indexes.

Generated expressions cover direct calls, pointer arithmetic, reverse addition, subtraction, conditional and comma wrappers, and `&call(...)[0]` indexed-address forms. Result operations cover reads, same-root pointer difference, equality, and ordering.

## Model and diagnostics

The test-side model tracks:

- scalar versus concrete named aggregate pointee type;
- global/static left-versus-right storage identity;
- element index and expected value;
- mutable versus const pointee metadata;
- the first out-of-bounds index.

Targeted cases assert exact cross-root subtraction/ordering diagnostics, unequal cross-root identity, const-discard and const-write diagnostics, pointee type mismatch diagnostics, and out-of-scope scalar/struct/union local-object diagnostics after a pointer-returning callee exits. Every generated program runs through `catch_unwind`.

## Native-oracle boundary

The shared C11 fixture uses only defined operations on global and file-static arrays. GCC and Clang both accept it under `-std=c11 -Wall -Wextra -Werror` and return 114, matching Cust. Returning pointers to automatic locals is intentionally interpreter-only because dereferencing those pointers is undefined in native C.

## Result

The focused model test passed immediately, so this was deliberate conformance/property coverage rather than a production-code change. All twelve fuzz-safety tests remain sub-second locally.

## Verification

```bash
cargo test --test fuzz_safety generated_pointer_return_function_results_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety -- --nocapture
cargo test --test interpreter pointer_return -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
