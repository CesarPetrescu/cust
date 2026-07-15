# Model-based hidden aggregate-array literal pointers

Date: 2026-07-15

## Scope

`tests/fuzz_safety.rs` uses a fixed-seed test-side model for interpreter-owned hidden aggregate-array storage created by mutable `struct`/`union` array compound literals and const aggregate-array typedef compound literals. Each literal is captured once in a pointer variable so generated arithmetic and comparisons distinguish repeated access to one hidden root from separately allocated mutable, const, left, and right literal roots.

The model tracks:

- hidden storage identity by aggregate kind, storage class, and literal side;
- concrete named `struct Point` versus `union Number` pointee identity;
- element index/value and four-element bounds;
- const-pointee metadata for const aggregate-array typedef literals;
- exact bounds, cross-root subtraction/ordering, const-discard, retained-const write, mutable-write, and pointee-type diagnostics.

Generated pointer expressions combine forward/reverse addition, subtraction, conditionals, comma expressions containing same-root pointer differences, indexed field reads, pointer differences, equality, and ordering. `catch_unwind` protects every generated interpretation so the property test also enforces panic freedom.

## C oracle boundary

Only defined same-object pointer operations belong in the compiler-oracle fixture. Native C equality across distinct objects is defined, but relational comparisons and subtraction across distinct objects are not suitable oracle cases; Cust's deterministic safety diagnostics for those routes remain interpreter-only.

The warning-free fixture captures each aggregate-array compound literal once, uses a side effect in the comma expression, and avoids ABI-sensitive aggregate size assumptions.

## Fixtures and verification

- `tests/fixtures/valid/hidden_aggregate_array_literal_pointer_model_routes.c`
- `tests/fixtures/compat/valid/hidden_aggregate_array_literal_pointer_model_routes.c`
- `tests/interpreter.rs::supports_hidden_aggregate_array_literal_pointer_model_routes`
- `tests/fuzz_safety.rs::generated_hidden_aggregate_array_literal_pointers_match_model_without_panics`

The model adds 192 generated expression cases plus 56 cross-root, const, mutable-write, and concrete-type checks. All eleven fuzz-safety tests remain sub-second locally. The warning-free C11 fixture returns 164 under Cust, GCC, and Clang.

Existing Cust behavior matched the independent model, so this package is deliberate property/conformance coverage rather than a production runtime change.