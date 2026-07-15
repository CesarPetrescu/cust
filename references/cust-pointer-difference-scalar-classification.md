# Pointer-difference scalar expression classification

Date: 2026-07-15

## Scope

Keep pointer-pointer subtraction scalar when its result appears inside larger additive, multiplicative, conditional, comma, or truthiness expressions, while preserving Cust's safe pointer arithmetic and cross-array diagnostics.

## Root cause

`Interpreter::expr_is_pointer_value()` used one recursive rule for both addition and subtraction: if either operand was pointer-valued, the whole expression was pointer-valued. That is correct for supported pointer addition and pointer-minus-scalar, but wrong for pointer-pointer subtraction, whose result is an integer difference. An enclosing expression such as `5 + (right - left)` was therefore routed to `eval_pointer()` and failed with `expected pointer expression` before scalar evaluation could call `pointer_difference()`.

## Implementation pattern

- Classify addition as pointer-valued when either operand is pointer-valued. This preserves pointer-plus-scalar, scalar-plus-pointer, and the existing `cannot add two pointers` diagnostic.
- Classify subtraction as pointer-valued only when the left operand is pointer-valued and the right operand is not. This preserves pointer-minus-scalar while making pointer-pointer subtraction scalar.
- Leave scalar-minus-pointer classified as scalar; the scalar evaluator still detects the pointer operand and reports the established invalid mixed-operand diagnostic.
- Do not evaluate expressions during classification. The existing metadata/runtime-variable classifier remains side-effect free.

## TDD and verification

Focused RED reproduced `expected pointer expression` for both a same-array nested difference and a cross-array nested difference. GREEN coverage includes direct addition, multiplication, conditional and comma results, truthiness, and exact cross-array diagnostics. The warning-free GCC and Clang C11 fixture returns 42, matching Cust.

Run focused coverage with:

```bash
cargo test --test interpreter pointer_differences -- --nocapture
cargo test --test interpreter pointer_arithmetic -- --nocapture
cargo test --test c_compat -- --nocapture
```

## Follow-up

`pointer_expr_points_to_const()` still recursively ORs const metadata across both operands of addition/subtraction. A mutable base plus a scalar difference formed from const pointers, such as `values + (const_right - const_left)`, therefore incorrectly appears to point to const. Cust reports `cannot discard const qualifier from pointer target`, while warning-free GCC and Clang return 9. The next P0 task should make const propagation follow the pointer-valued result base while retaining const on genuinely const pointer bases.
