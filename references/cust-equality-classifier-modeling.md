# Cust Equality Classifier Modeling

The 2026-07-17 equality-model run added fixed-seed scalar, pointer, and mixed pointer/integer comparisons to `tests/fuzz_safety.rs`.

## Model shape

- Scalar routes cover literals, unary-plus and scalar-cast wrappers, conditionals, comma expressions, pointer differences, and nested scalar array-field element reads through a struct pointer.
- Pointer routes cover array decay/arithmetic, indexed address-of, pointer-returning calls, direct and arrow pointer fields, conditionals, comma expressions, and pointer casts.
- Pointer identity is modeled as storage root plus element index, with null as a distinct root.
- Every generated program returns the comparison result together with a side-effect marker, proving operands are evaluated once.
- Mixed pointer/integer routes cover wrapped zero and nonzero values in both operand orders, plus aggregate/scalar type diagnostics.

## Root cause and fix

`eval_equality()` previously classified scalar/scalar pairs first but then handled every mixed pair by speculatively calling `eval_pointer()` on both operands. Its fallback recognized only raw `Expr::Number(0)` / nonzero literals. Valid null-pointer constants such as `+0` and `(int)0`, and wrapped nonzero integers, therefore leaked `expected pointer expression` instead of comparing with null or reporting `cannot compare pointer with nonzero integer`. Aggregate operands similarly leaked pointer-context diagnostics.

Classify both operands exactly once with `expr_is_pointer_value()`, then evaluate according to the four scalar/pointer shape pairs. For a mixed pair, evaluate the scalar route once: zero compares against `PointerValue::Null`; nonzero reports the established diagnostic. This also preserves scalar-context errors for aggregate operands.

## Oracle pitfall

GCC `-Werror=address` rejects a direct comparison between a known local array and null as tautologically false. Route null checks through a pointer-returning helper in compiler-oracle fixtures. The warning-free fixture returns 129 under Cust, GCC, and Clang.
