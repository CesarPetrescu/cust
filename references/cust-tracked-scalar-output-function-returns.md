# Tracked scalar-output function returns

## Supported slice

Cust accepts safe function returns of tracked scalar output pointers for `char **`, `int **`, `_Bool **`, and `double **`. Direct spellings and pointer-typedef, complete-output-typedef, and chained aliases lower to `ReturnType::PointerOutput` while preserving the scalar pointee.

Returned values retain the caller-owned pointer-output identity. They can be forwarded through compatible calls, assigned to compatible output objects, dereferenced for typed pointer/scalar updates, and classified by non-evaluating `sizeof(call)` without executing the callee.

## Safety boundaries

- Reject callee-local output objects and output objects whose tracked pointee escapes its owner.
- Preserve const-slot and const-pointee conversion checks at return and assignment boundaries.
- Keep unsupported non-scalar, deeper-pointer, pointer-array, aggregate-field-array, cast, arithmetic, relational-ordering, and compound-update forms targeted. A later slice added restricted unqualified scalar-output aggregate fields themselves; it did not enable arrays of those fields.
- Do not manufacture host addresses; returned values reuse interpreter-owned tracked identities.

## Validation-depth pitfall

Function-return validation reaches the existing recursive non-evaluating intrinsic-call validator. Do not lower the global `MAX_NON_EVALUATING_CALLEE_EXPRESSION_DEPTH` to protect this heavier route: ordinary `sizeof` intrinsic analysis has established depth-30 coverage and a limit of 128.

Instead, temporarily cap only tracked scalar-output return-call validation with `MAX_POINTER_OUTPUT_RETURN_EXPRESSION_DEPTH`, restore the prior active limit after validation, and retain the shared 128-level contract everywhere else. Regress both sides: a deeply nested ordinary `sizeof(memcmp(...))` expression must remain valid, while a tracked-output return call beyond the dedicated bound must fail deterministically rather than overflowing the host stack.

## Verification anchors

- Interpreter filter: `cargo test --test interpreter tracked_scalar_output_function_return -- --nocapture`
- Parity suite: `cargo test --test pointer_output_parity -- --nocapture`
- Compiler oracle: `cargo test --test c_compat -- --nocapture`
- Registered fixture: `tests/fixtures/compat/valid/tracked_scalar_output_function_returns.c`
