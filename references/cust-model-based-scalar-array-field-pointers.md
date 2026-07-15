# Model-based scalar array-field pointers

Date: 2026-07-15

## Scope

`tests/fuzz_safety.rs` uses a fixed-seed test-side model for scalar `int` and `char` array fields in named and anonymous `struct`/`union` containers. It generates direct decay, indexed address-of, arrow, nested-field, and aggregate-compound-literal routes, then wraps them in pointer arithmetic, pointer differences, conditionals, and comma expressions.

The model tracks:

- containing storage owner (`left`/`right`), nested field path, and hidden compound-literal identity;
- scalar pointee type (`int` or `char`);
- element index and four-element bounds;
- pointee `const` qualification;
- exact cross-owner, bounds, const-discard, read-only-write, and type-conversion diagnostics.

## Diagnostic distinctions

Scalar array fields decay to ordinary interpreter-owned scalar array pointers, so out-of-range offsets report `array pointer index <index> out of bounds for length 4`. This differs from embedded aggregate arrays, whose variable-backed field paths report `struct array field pointer index ...` and hidden literal storage reports `struct array pointer index ...`.

A `const T field[N]` route must reject mutable pointer conversion with `cannot discard const qualifier from pointer target`; retaining a `const T *` and writing through it must reject with `cannot assign through pointer to const`.

## Fixtures and verification

- `tests/fixtures/valid/scalar_array_field_pointer_model_routes.c`
- `tests/fixtures/compat/valid/scalar_array_field_pointer_model_routes.c`
- `tests/interpreter.rs::supports_scalar_array_field_pointer_model_routes`
- `tests/fuzz_safety.rs::generated_scalar_array_field_pointer_expressions_match_model_without_panics`

The warning-free C11 fixture returns 56 under Cust, GCC, and Clang. The generated property test adds 768 expression cases plus 128 route-specific const-write/type checks; all nine fuzz-safety tests remain sub-second locally.

Existing Cust behavior matched the independent model, so this package is deliberate property/conformance coverage rather than a production runtime change.
