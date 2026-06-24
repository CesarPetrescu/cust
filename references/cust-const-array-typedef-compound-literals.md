# Const Array Typedef Compound Literals

Date: 2026-06-24

## Context

Cust already lowered one-dimensional array typedef compound literals through `Expr::ArrayLiteral` / `Expr::AggregateArrayLiteral`, but the literal expression did not remember whether the type name was top-level `const` via either an explicit qualifier (`(const int[2]){...}`) or a const-qualified array typedef (`typedef const int Scores[2]; (Scores){...}`).

## Implementation pattern

- Add read-only/const metadata to array compound-literal expression nodes, not just declaration nodes.
- In `parse_cast()`, compute `compound_literal_read_only` from the leading `const` consumed before the type name plus `type_alias_is_const(name)` when the cast type begins with a typedef identifier.
- Pass the flag through scalar-array and aggregate-array compound literal construction:
  - `Expr::ArrayLiteral { read_only, ... }` -> `make_array_compound_literal(..., read_only)` -> `ArrayValue::read_only`.
  - `Expr::AggregateArrayLiteral { read_only, ... }` -> `make_aggregate_array_compound_literal(..., read_only)` -> `Value::StructArray { read_only }`.
- Include these expression nodes in metadata-only `pointer_expr_points_to_const()` so mutable pointer declarations/assignments/arguments reject const array compound literals before evaluation.
- Keep `sizeof` paths metadata-only and ignore the read-only flag there.

## Tests

Use both acceptance and negative coverage:

```bash
cargo test --test interpreter const_array_typedef_compound_literal -- --nocapture
cargo test --test c_compat -- --nocapture
```

Fixtures:

- `tests/fixtures/valid/const_array_typedef_compound_literals.c`
- `tests/fixtures/compat/valid/const_array_typedef_compound_literals.c`
- `tests/fixtures/invalid/const_array_typedef_compound_literal_discard.c`
- `tests/fixtures/invalid/const_array_typedef_compound_literal_write.c`

## Pitfall

A write through `const int *p = (ConstArray){...}; p[0] = 9;` may report Cust's existing pointer-const diagnostic (`cannot assign through pointer to const`) before runtime read-only array storage is consulted. That is acceptable and matches the established const-pointer safety gate.
