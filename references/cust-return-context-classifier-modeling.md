# Cust return-context classifier modeling

Date: 2026-07-17

## Scope

Return-context coverage now exercises scalar, pointer, aggregate, and void functions across direct expressions, conditionals, commas, assignments, compound assignments, calls, casts, aggregate copies/literals/dereferences, pointer qualification/type boundaries, and empty/value return mismatches.

## Root cause and fix

`Interpreter::eval_return_value()` selected the evaluator from the declared function return type. That is correct for non-void returns because it preserves established expression-context diagnostics and pointer conversion checks, but it made value-bearing `return` statements in void functions go through scalar evaluation. Consequently, `return pointer_expr;`, `return aggregate_expr;`, and `return void_call();` leaked pointer/aggregate/void scalar-use errors instead of the existing function-level `void function '<name>' returned a value` diagnostic.

The focused RED case was a void function returning `(marker++, cursor)`: Cust reported `pointer 'cursor' used as scalar`. The fix is deliberately narrow: when the active return type is void, evaluate the expression exactly once with `eval_discard()` and pass a sentinel value to the existing `validate_return_value()` mismatch branch. Non-void return routing remains declared-type-driven so prior exact diagnostics do not change.

A broader metadata-first rewrite was rejected after the full fuzz-safety suite showed it changed the established scalar-function aggregate diagnostic from `struct variable 'point' used as scalar` to `struct value returned from scalar function 'main'`.

## Verification pattern

- `cargo test --test fuzz_safety return_context -- --nocapture`
- `cargo test --test fuzz_safety generated_discard_context_classification_matches_model_without_panics -- --nocapture`
- `cargo test --test interpreter supports_model_based_return_context_classification -- --nocapture`
- `cargo test --test c_compat -- --nocapture`
- Compile `tests/fixtures/compat/valid/return_context_classification_model_routes.c` with both GCC and Clang under `-std=c11 -Wall -Wextra -Werror`; Cust/GCC/Clang return 199.

Keep native fixture call results in temporaries before reading side-effect counters. Expressions such as `marker * 10 + produce().x` have native operand evaluation-order concerns and can also read Cust's marker before the call executes.
