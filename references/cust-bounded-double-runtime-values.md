# Bounded `double` runtime values

Date: 2026-08-11

## Completed slice

Cust supports decimal `double` literals and direct local, file-global, and block-static scalar storage. The bounded slice covers initialization, replacement and compound assignment, prefix/postfix update, mixed arithmetic, comparisons, conditional/comma forwarding, truthiness, scalar function-call arguments converted to supported parameter types, scalar function returns converted to supported return types, scalar compound literals, `_Generic`, integer-constant casts, and deterministic `sizeof(double) == 8` / `_Alignof(double) == 8`. Direct double parameter and return declarations remain outside this first slice.

`double` values are stored as exact `f64::to_bits()` payloads in the existing scalar `i64` slot. Every numeric read must therefore use the expression's scalar-type metadata before interpreting the bits. Integer destinations use Rust's defined saturating float-to-integer conversion, while `_Bool` conversion is based on `value != 0.0`, not an intermediate integer cast.

## Exact boundaries

The first slice rejects `float`, `long double`, hexadecimal floating literals, floating suffixes, non-finite literals, double arrays, aggregate fields, pointers, pointer arithmetic/ordering/equality involving double values, and direct double-returning or double-parameter function declarations outside the supported scalar-call boundary. Keep adjacent unsupported syntax source-located.

## Non-evaluating and generic validation

- `sizeof` and `_Generic` must classify double-valued expressions without evaluating side effects, but still validate operand constraints.
- Do not let assignment result size alone validate `sizeof(pointer = double_value)`; pointer assignment compatibility remains required.
- `generic_selection_type()` must validate unary/binary operator constraints in every association, including unselected associations. A broad “binary expressions are int” classification can hide invalid double remainder/bitwise/shift operations.
- Integer-constant generic controlling expressions must inspect through casts; `(int)(1.0 % 2.0)` is still invalid because the wrapped remainder constraint is checked before the result type is classified.

## Pointer boundary

`Expr::AddressOf` is syntactically pointer-valued before runtime type metadata is available. Any scalar consumer, including truthiness, must consult `expr_is_unsupported_double_pointer()` before generic pointer handling, or `&double_object` can bypass the bounded no-double-pointer rule.

Discarded and ordering expressions are scalar-consumer boundaries too: `double value; &value;` and `&left < &right` must reject the unsupported pointer before `eval_pointer()` can materialize it.

## Generic-selection linearity

Validate every controlling expression and selected/unselected association exactly once before evaluation. Returning only the selected AST and then revalidating it makes a chain of selected `_Generic` associations exponential. Retain the selected expression's already-computed `DeclType`, and during recursive runtime evaluation select nested associations from that validated metadata without repeating whole-subtree constraint walks. Exercise both nested controlling expressions and nested selected associations in timing regressions.

AST-address scalar-type caches must be isolated for cloned function bodies without destroying the caller's cache. Swap in a fresh cache for callee execution and restore the caller cache afterward; clearing globally at every call makes right-associated arithmetic with nested calls quadratic.

Whole-suite review matters beyond focused double tests: generic type validation must classify explicit one-level pointer casts from 2D row expressions by the cast destination before rejecting raw 2D controls, and `sizeof` dereference must preserve exact pointer-to-void subscript diagnostics before recursively validating the parser's pointer-addition lowering.

## TDD and review regressions

The final review-driven regressions cover:

1. `_Bool` conversion of positive and negative fractional double constants through direct and selected-generic integer-constant paths.
2. Truthiness of `&double_object`.
3. Double-to-pointer assignment hidden beneath `sizeof` and `_Generic`.
4. Invalid double operators wrapped by integer casts in generic controlling expressions.
5. Invalid double operators in unselected generic associations.
6. Discarded double-address expressions.
7. Linear validation/evaluation for deeply nested selected generic associations.
8. Exact double-address ordering rejection and linear right-associated arithmetic across nested calls.

Run `cargo test --test interpreter double -- --nocapture` and the actual `cargo test --test c_compat -- --nocapture`; fixture-name filters run zero compiler-oracle tests and are not valid evidence. The registered native fixture uses warning-free, ABI-independent relationships and is also checked directly with GCC and Clang under `-std=c11 -Wall -Wextra -Werror`.
