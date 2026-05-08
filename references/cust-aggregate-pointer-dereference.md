# Cust aggregate pointer dereference values and copy assignment

Date: 2026-05-08

## Scope completed

Cust now treats `*p` over one-level supported `struct`/`union` pointers as an aggregate value in aggregate contexts:

- declaration initializers: `struct Point copy = *p;`
- by-value function arguments: `sum(*p)`
- aggregate returns: `return *p;`
- union equivalents such as `union Number picked = *n;`
- statement-level copy assignment through aggregate pointers: `*p = replacement;`

All copies deep-clone the supported aggregate field map, matching the existing struct/union by-value model rather than aliasing the source fields.

## RED/GREEN notes

1. Added `tests/fixtures/valid/aggregate_pointer_dereference.c`, `tests/fixtures/invalid/const_aggregate_pointer_deref_assignment.c`, and native C oracle fixture `tests/fixtures/compat/valid/aggregate_pointer_dereference.c`.
2. Wired focused tests in `tests/interpreter.rs` and `tests/c_compat.rs`.
3. Initial RED failure was `expected struct expression` for `*p`, showing `eval_struct_expr()` only accepted aggregate variables, fields, array elements, calls, conditionals, and comma expressions.
4. After adding aggregate `Expr::Deref`, the next failure was `struct variable 'src' used as scalar` for `*dst = src;`, showing statement-level `Stmt::DerefAssign` always routed through scalar dereference assignment.
5. GREEN added `assign_struct_pointer_copy()` and a struct-pointer branch in `Stmt::DerefAssign`, while preserving scalar pointer dereference assignment for `int *` / `char *`.

## Pitfalls

- `*dst = src;` parses as `Stmt::DerefAssign`, not `Expr::DerefSet`, in statement context. Update `exec_stmt()` for statement-level behavior.
- Preserve scalar dereference assignment by only taking the aggregate-copy path when the pointer variable is `Value::Pointer { ty: PointeeType::Struct(_), .. }`.
- For aggregate by-value parameters such as `sum(*p)` and `sum(make_point_ptr_value())`, update `eval_struct_argument()` to route `Deref`, aggregate-returning `Call`, `Conditional`, and `Comma` forms through `eval_struct_expr()`.
- Use `ensure_pointer_expr_pointee_mutable()` and `ensure_struct_pointer_target_mutable()` before mutating through aggregate pointers so `const struct T *` views and direct const targets keep reporting `cannot assign through pointer to const`.
- Whole-aggregate copy assignment should reject target aggregate types with const fields using the same `cannot assign to struct '<Type>' with const fields` boundary as direct struct copy assignment.

## Verification commands

```bash
cargo test --test interpreter aggregate_pointer_dereference -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```
