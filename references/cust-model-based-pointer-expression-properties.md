# Model-based pointer-expression properties

Date: 2026-07-15

## Scope

`tests/fuzz_safety.rs` uses fixed-seed, bounded AST generators plus an independent model for scalar-array pointer expressions. The generated surface includes mutable/const left and right array roots, pointer-plus/minus-scalar, reverse scalar-plus-pointer, pointer differences, conditional wrappers, and comma wrappers.

The model tracks:

- storage root and element index;
- pointee `const` qualification (including conditional branch qualification merging);
- scalar pointer-difference results;
- exact cross-array difference errors.

Every generated Cust program runs under `catch_unwind`. In-range model results are checked by writing a root-specific marker at the expected index and dereferencing the generated expression. Mutable-target conversions check exact `cannot discard const qualifier from pointer target` behavior. Cross-root differences check exact `cannot subtract pointers to different arrays` behavior.

## RED findings

The first generated run exposed two related failures:

1. A scalar wrapper ending in zero, such as `(0 ? 2 : (marker += 1, 0))`, was speculatively evaluated by `eval_pointer()` and interpreted as a null pointer. A legal `pointer + scalar_zero_wrapper` then failed with `cannot add two pointers`.
2. `eval_pointer_arithmetic()` discarded errors from speculative pointer probes. A cross-array difference in the discarded side of a pointer-valued comma operand was replaced by `expected pointer expression`.

A minimized exact regression is:

```c
const int *result = (((left + 1) - (right + 1)), left + 1)
    + ((left + 2) - (left + 1));
```

It must report `cannot subtract pointers to different arrays`.

## Implementation decision

`eval_pointer_arithmetic()` now asks `expr_is_pointer_value()` for both operand result shapes before evaluation. It evaluates exactly one pointer plus one scalar for supported arithmetic and no longer uses evaluator success/failure as type metadata.

Because this makes classifier completeness a correctness requirement, audit pointer-valued assignment-result variants accepted by `eval_pointer()`. This run added addressable scalar/aggregate literals and direct, arrow, and aggregate-compound-literal pointer-field assignment/compound-assignment results to `expr_is_pointer_value()`.

## Verification

- focused generated RED/GREEN tests: `cargo test --test fuzz_safety generated_pointer_expression -- --nocapture`;
- minimized pointer-difference and cross-array tests in `tests/interpreter.rs`;
- pointer-difference, pointer-arithmetic, pointer-field-lvalue, and recursion-depth regressions;
- warning-free GCC and Clang C11 execution of the expanded fixture at exit 50;
- full local and Docker gates.

## Follow-up

Extend the model to named struct/union array pointers and direct/arrow/aggregate-compound-literal pointer-valued fields. Keep generation bounded and fixed-seed so it remains suitable for every `cargo test` and Docker run.
