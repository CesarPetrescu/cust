# Cust aggregate compound-literal scalar-field lvalues

2026-05-09 autonomous run.

## Feature completed

Cust now treats scalar fields selected from aggregate compound literals as modifiable lvalue expression targets for the supported scalar-field subset:

- direct assignment: `((struct Point){1, 2}).x = 7`
- compound assignment: `((struct Point){3, 4}).y += 5`
- prefix increment/decrement: `++((struct Point){5, 6}).x`
- postfix increment/decrement: `((struct Point){7, 8}).y++`
- nested scalar fields: `((struct Box){{1, 2}, 3}).point.x = 4`

Initializer expressions run before the lvalue operation, so side effects in the compound-literal initializer are preserved. The temporary aggregate remains local to expression evaluation; the operation result follows C scalar lvalue operator result rules.

## Implementation notes

- Parser lowering reuses dotted `Expr::AggregateFieldGet` for aggregate compound literal fields.
- Assignment and compound assignment lower to `Expr::AggregateFieldSet` and `Expr::AggregateFieldCompoundSet`.
- Prefix/postfix increment reuse `Expr::Increment` with an `AggregateFieldGet` target.
- Runtime evaluation centralizes scalar field lookup in `eval_aggregate_literal_field_scalar`, which evaluates the aggregate expression once, checks the selected nested field, returns its scalar value, and reports non-scalar field use consistently.
- Const scalar fields are rejected with the existing `cannot assign to const struct field '<field>'` diagnostic.

## Tests

- `tests/fixtures/valid/aggregate_compound_literal_field_lvalues.c`
- `tests/fixtures/invalid/aggregate_compound_literal_const_field_assignment.c`
- `tests/fixtures/compat/valid/aggregate_compound_literal_field_lvalues.c`
- `tests/interpreter.rs` focused valid/invalid regressions
- `tests/c_compat.rs` native compiler-oracle fixture registration

Focused commands:

```bash
cargo test --test interpreter aggregate_compound_literal -- --nocapture
cargo test --test c_compat -- --nocapture
```
