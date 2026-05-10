# Aggregate compound-literal pointer-field lvalues

Date: 2026-05-10

Cust now treats pointer-valued fields selected from aggregate compound literals as pointer lvalues for assignment, pointer compound assignment, and prefix/postfix increment/decrement expression contexts.

Covered forms:

- `(((struct Cursor){values}).p = values + 2)[-1]`
- `((struct Cursor){values + 1}).p += 2` flowing into a pointer parameter
- `--((struct Cursor){values + 3}).p` followed by dereference
- `((struct Cursor){values + 1}).p++` returning the previous pointer value
- `int * const` pointer-field assignment rejection on aggregate compound literals

Implementation notes:

- Parser already lowered `aggregate.field = ...` and `aggregate.field += ...` through `Expr::AggregateFieldSet` / `Expr::AggregateFieldCompoundSet`; the missing piece was pointer-context evaluation for those expression nodes.
- `eval_pointer` now handles aggregate compound-literal pointer-field assignment and pointer compound assignment by evaluating the temporary aggregate field metadata, checking const pointer slots and pointee type/const conversions, and returning the assigned/updated pointer value.
- Prefix/postfix pointer increments on `Expr::AggregateFieldGet` now return updated/previous pointer values in pointer contexts. Because the selected aggregate is a temporary, no persistent storage mutation is needed beyond the expression result.
- Scalar evaluation for these nodes remains unchanged for scalar fields, preserving prior scalar lvalue behavior and diagnostics.

Verification used:

```bash
cargo test --test interpreter supports_pointer_field_lvalues_on_aggregate_compound_literals -- --nocapture
cargo test --test interpreter aggregate_compound_literal -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
