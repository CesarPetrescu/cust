# Cust discard-context classifier modeling

## 2026-07-17 autonomous run

Cust discard contexts must classify an expression's result shape before evaluation. The old `eval_discard()` tried scalar evaluation first and retried pointer evaluation only after the string error `pointer value used as scalar`; a pointer-valued comma expression therefore evaluated its left operand twice. Aggregate dereference assignment was similarly probed with `eval_pointer(pointer).is_ok()` before the real copy assignment, evaluating a side-effectful pointer operand twice. Aggregate variables were rejected as scalar values, and comma expressions ending in a void call leaked the void-call scalar-use diagnostic.

Implementation pattern:

1. Handle direct calls and explicit void casts once.
2. Classify void-valued conditional/comma expressions from function return metadata and recursively discard only the selected/ordered operands.
3. Use `expr_is_pointer_value()` before evaluation and call `eval_pointer()` exactly once.
4. Use `aggregate_expr_type_name()` as metadata-only aggregate classification, preserve mismatched-aggregate conditional diagnostics, and call `eval_struct_expr()` exactly once.
5. Evaluate only the remaining scalar routes with `eval()`; never use an evaluator failure string as runtime type metadata.

Regression coverage includes 81 fixed route/context combinations (27 routes across expression statements, `(void)` casts, and comma-left operands), exact scalar-use diagnostics, panic freedom, and focused single-evaluation regressions for pointer commas, aggregate dereference assignments, aggregate values, and void commas. The warning-free GCC/Clang fixture returns 240 like Cust.
