# Model-based hidden scalar-array literal pointers

Date: 2026-07-15

## Scope

`tests/fuzz_safety.rs` uses a fixed-seed test-side model for interpreter-owned hidden scalar-array storage created by mutable `int`/`char` array compound literals, const array-typedef compound literals, and read-only string literals. Each literal is captured once in a pointer variable so generated arithmetic and comparisons can distinguish repeated access to one hidden root from access to a separately allocated literal root.

The model tracks:

- hidden storage identity (`left`/`right` per literal family);
- scalar pointee type (`int` or `char`);
- element index/value and four-element bounds, including string NUL elements;
- const-pointee metadata for const array-typedef literals;
- read-only storage metadata for string literals;
- exact bounds, cross-root subtraction/ordering, const-discard, retained-const write, string write, and pointee-type diagnostics.

Generated pointer expressions combine forward/reverse addition, subtraction, conditionals, comma expressions containing same-root pointer differences, dereference reads, pointer differences, equality, and ordering. `catch_unwind` protects every generated interpretation so the property test also enforces panic freedom.

## C oracle boundary

Only defined same-object pointer operations belong in the compiler-oracle fixture. Native C equality across distinct objects is defined, but relational comparisons and subtraction across distinct objects are not suitable oracle cases; Cust's deterministic safety diagnostics for those routes remain interpreter-only.

The warning-free fixture uses a side effect in the comma expression because GCC rejects a discarded side-effect-free pointer difference under `-Wall -Wextra -Werror`.

## Fixtures and verification

- `tests/fixtures/valid/hidden_scalar_array_literal_pointer_model_routes.c`
- `tests/fixtures/compat/valid/hidden_scalar_array_literal_pointer_model_routes.c`
- `tests/interpreter.rs::supports_hidden_scalar_array_literal_pointer_model_routes`
- `tests/fuzz_safety.rs::generated_hidden_scalar_array_literal_pointers_match_model_without_panics`

The model adds 240 generated expression cases plus 37 cross-root, const, read-only, and type checks. All ten fuzz-safety tests remain sub-second locally and in Docker. The warning-free C11 fixture returns 65 under Cust, GCC, and Clang.

Existing Cust behavior matched the independent model, so this package is deliberate property/conformance coverage rather than a production runtime change.
