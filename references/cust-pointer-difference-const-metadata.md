# Pointer-difference const metadata propagation

Date: 2026-07-15

## Scope

Ensure pointer arithmetic derives pointee constness from the operand that actually supplies the pointer-valued result. A scalar pointer difference must not leak const metadata from either pointer consumed to produce that integer offset.

## Root cause

`Interpreter::pointer_expr_points_to_const()` handled both `Add` and `Sub` by recursively OR-ing both operands. That ignored expression result shape. In `values + (const_right - const_left)`, the right operand is scalar, but its source pointers made the whole addition appear const and caused `cannot discard const qualifier from pointer target`.

## Implementation pattern

- For addition, inspect the operand that is pointer-valued: prefer the left pointer base, otherwise use the right pointer base.
- For subtraction, propagate const only for pointer-minus-scalar, and only from the left pointer base.
- Use `expr_is_pointer_value()` for metadata-only classification; do not evaluate offsets or pointer expressions.
- Keep a genuinely const pointer base const, and retain runtime same-array/different-array checks.

This mirrors `pointer_expr_pointee_type()` and `expr_is_pointer_value()`, keeping pointer-valued classification, pointee type, and pointee qualification aligned.

## TDD and verification

The focused RED fixture failed with `cannot discard const qualifier from pointer target`. GREEN covers mutable-base addition and subtraction with differences formed from const pointers, plus a genuinely const base exact-negative regression. The warning-free GCC and Clang C11 fixture and Cust all return 9.

Run focused coverage with:

```bash
cargo test --test interpreter pointer_difference_const_metadata -- --nocapture
cargo test --test interpreter pointer_differences -- --nocapture
cargo test --test interpreter pointer_arithmetic -- --nocapture
cargo test --test c_compat -- --nocapture
```
