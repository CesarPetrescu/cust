# Void cast expressions

Date: 2026-05-12

Cust supports C-style `(void)expr` casts as a no-value expression form for explicitly discarding a result while preserving operand side effects.

Implementation notes:

- Parser routing is in `parse_cast`: `starts_cast_type_after_lparen` treats `void` as a cast type start, and `(void)` lowers to `Expr::VoidCast` instead of going through `parse_decl_type`, because `void` is not an object declaration type in Cust.
- Runtime discard context is handled in `eval_discard` by evaluating the inner expression in discard context. This allows `(void)void_function_call()` and `(void)pointer_expr` without scalar conversion.
- Scalar/type-query contexts reject `Expr::VoidCast` with `void expression used as scalar`; `sizeof((void)expr)` therefore remains unsupported like other void-valued scalar uses.
- Fixture shape: keep native compiler-oracle code warning-free by using `(void)` casts on otherwise-unused scalar, pointer, and void-call expressions.

Coverage:

- `tests/fixtures/valid/void_cast_expressions.c`
- `tests/fixtures/compat/valid/void_cast_expressions.c`
- `tests/fixtures/invalid/void_cast_used_as_scalar.c`
