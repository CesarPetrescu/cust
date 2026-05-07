# Cust aggregate pointer indexed values

Date: 2026-05-07

## Work package

Closed the aggregate/function gap where `p[i]` over a `struct`/`union` array pointer worked for field lvalues (`p[i].x`) but not as a by-value aggregate expression.

## RED

Added `tests/fixtures/valid/aggregate_pointer_indexed_values.c` plus a focused interpreter test expecting `79`. The fixture covers:

- `copy = p[1];` same-type struct copy assignment from an aggregate pointer index.
- `sum_point(p[0])` by-value struct argument from an aggregate pointer index, with callee mutation proving caller isolation.
- `picked = n[1];` and `sum_number(n[0])` for union array pointers.
- Native-compatible exit-code behavior in `tests/fixtures/compat/valid/aggregate_pointer_indexed_values.c`.

Initial focused run failed as expected:

```text
called `Result::unwrap()` on an `Err` value: CustError { message: "variable 'p' is not a struct array" }
```

## GREEN

Implementation in `src/lib.rs`:

- Added `indexed_struct_pointer_value()` to reuse existing safe pointer indexing/offsetting and `find_struct_pointer_fields()`.
- In `eval_struct_argument()`, `Expr::ArrayGet { name, index }` first attempts the pointer-indexed aggregate path; if the name is not a struct/union pointer, it falls back to the existing direct struct-array element behavior.
- In `eval_struct_expr()`, the same pointer-indexed aggregate path allows declaration initializers, copy assignment RHS expressions, return contexts, and aggregate-valued expression contexts that call `eval_struct_expr()`.
- The selected element is deep-cloned with `StructFieldValue::deep_clone_fields()` so aggregate value semantics remain isolated from caller storage.

## Pitfalls

- Do not route all pointer-indexed expressions through the aggregate path. Scalar `int *p; f(p[0])` in a struct-argument context should still be a non-struct argument, not a struct-pointer-index diagnostic. The helper checks `PointeeType::Struct(_)` before offsetting.
- Unions share the existing aggregate implementation (`ReturnValue::Struct` and `PointeeType::Struct` naming is historical); this feature works for named unions because union types are stored in the same aggregate type table with `AggregateKind::Union`.
- Native compiler-oracle fixtures should avoid ABI/layout assumptions and stick to by-value behavior/exit-code comparison.

## Verification

Focused:

```bash
cargo test --test interpreter pointer_indexed_aggregate_values -- --nocapture
cargo test --test c_compat -- --nocapture
```

Full gate after status updates:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```
