# Aggregate expression classifier/evaluator parity

The deterministic regression `generated_aggregate_classifier_and_evaluator_routes_stay_in_parity` in `tests/fuzz_safety.rs` models every currently supported aggregate-valued AST route rather than a narrow source spelling.

## Matrix shape

- 22 `AggregateValueRoute` variants mirror the aggregate-producing variants accepted by `aggregate_expr_type_name()`.
- Four consumers exercise each route: declaration initializer, function argument, function return, and exact nominal type mismatch.
- The exact total is 88 generated programs (`22 × 4`), with counters keyed by stable route/context enum identity rather than iteration position so duplicate or omitted entries fail exact totals.
- Every generated program runs beneath `catch_unwind` so a Rust-host panic is a test failure.
- Success programs use nested aggregate fields, embedded aggregate-array elements, and marker functions to prove deep-copy isolation and one-time source evaluation.

## Defects exposed

1. `_Generic` selection could produce an aggregate value and `eval_struct_expr()` could evaluate it, but `eval_struct_argument()` rejected the wrapper with `expected struct expression`. Keep argument classification in parity with aggregate type inference and evaluation.
2. `aggregate_expr_type_name()` classified `Expr::StructPtrArrayGet` as an aggregate value, but `eval_struct_expr()` omitted it. Evaluate this route through the existing aggregate pointer, then deep-clone the resolved field map exactly like `Expr::StructElementArrayGet`.
3. Directly recursing into the selected `_Generic` association from `eval_struct_argument()` bypasses semantic validation of unselected associations. Route through `eval_selected_generic()` so aggregate arguments retain the same validation and nesting-depth behavior as scalar, pointer, discard, and aggregate-expression evaluators.

## Maintenance checklist

When adding an aggregate-valued `Expr` variant:

1. Add it to `aggregate_expr_type_name()`.
2. Audit `eval_struct_expr()`, `eval_struct_argument()`, aggregate return handling, assignment-result field access, and non-evaluating `sizeof` metadata.
3. Add a matching `AggregateValueRoute` generator case and increment the exact route count.
4. Preserve one-time evaluation by resolving pointer/index expressions once.
5. Preserve C by-value semantics with `StructFieldValue::deep_clone_fields()` rather than sharing field maps.
6. Retain nominal struct/union mismatch diagnostics; do not weaken type checks to make generated routes pass.
7. For `_Generic` wrappers, use `eval_selected_generic()` rather than selecting an association directly; add an unselected-invalid-association regression for each new evaluator consumer.

The focused command is:

```bash
cargo test --test fuzz_safety generated_aggregate_classifier_and_evaluator_routes_stay_in_parity -- --nocapture
```
