# Truthiness classifier modeling

## Scope

The 2026-07-17 autonomous run added deterministic scalar/pointer truthiness coverage across direct conditions, unary `!`, `&&`, `||`, conditional conditions, and loop conditions.

## Root cause and fix

`Interpreter::eval_truthy()` previously maintained a partial list of pointer-valued AST variants. Pointer-valued direct and arrow aggregate-field assignment results were routed to scalar `eval()`, so forms such as `while (score == 0 && (view->primary = (int *)0))` failed with `pointer value used as scalar`.

Classify the complete expression with `expr_is_pointer_value()` before runtime evaluation. Pointer-valued results must be evaluated once with `eval_pointer()` and tested with `pointer_truthy()`; scalar results continue through the existing route-specific scalar evaluation. This keeps truthiness in parity with pointer arithmetic, equality, and ordering classification and prevents hand-maintained AST lists from drifting.

Balancing true/false values within every pointer route exposed an adjacent pointer-cast seam: `eval_pointer()` delegated `(int *)(int)0` to pointer evaluation of the inner scalar cast and returned `expected pointer expression`. Pointer casts should classify their operand, preserve pointer-valued routes, and evaluate scalar routes once; zero becomes `PointerValue::Null`, while nonzero retains the existing diagnostic.

## Regression shape

- Generate balanced scalar and pointer routes over literals, casts, nested scalar array-field reads, pointer differences, direct/arrow pointer fields, pointer-returning calls, conditionals, commas, and pointer casts.
- Exercise true and false/null values in six contexts: direct `if`, `!`, `&&`, `||`, `?:` condition, and `while` condition.
- Count side effects to prove operands and short-circuit branches execute exactly once when selected.
- Keep aggregate and void exact diagnostics in the property test.
- Register one warning-free native fixture in `c_compat` for defined routes.

## Native-oracle pitfall

GCC with `-Wall -Wextra -Werror` diagnoses truthiness of statically known array-derived addresses with `-Waddress`. Route known non-null pointers through a non-inline helper selected by runtime data in compiler-oracle fixtures. GCC and Clang then compile the fixture warning-free; the fixture added by this run returns 227 under both compilers and Cust.
