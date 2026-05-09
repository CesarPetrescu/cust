# Union-valued expression field access

2026-05-09 autonomous run.

## Scope

Added regression coverage for field access on union-valued expression results, mirroring the existing struct-valued expression support:

- Union compound literal scalar-field reads: `((union Number){4}).value`.
- Union assignment-expression results: `(left = right).value`.
- Union-valued conditional/comma expression results.
- Union-returning function calls followed by `.`.
- Union pointer-dereference assignment results followed by `.`.

## Implementation notes

The parser already routed aggregate-capable expression results through `Expr::AggregateFieldGet`, and `eval_struct_expr` already represents supported structs and unions through the shared `ReturnValue::Struct` aggregate path. The behavior gap was mostly undocumented coverage plus a diagnostic wording gap for naked union-returning calls used as scalar expressions.

`Interpreter::aggregate_kind_label()` now resolves a shared aggregate type name to `struct` or `union` for diagnostics. The scalar-evaluation call path uses it so `return make_number(1);` reports `union function 'make_number' used as scalar expression` instead of the older struct-specific wording.

## Verification

Focused commands:

```bash
cargo test --test interpreter supports_field_access_on_union_valued_expressions -- --nocapture
cargo test --test interpreter rejects_union_function_used_as_scalar_expression -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Caveat: `c_compat` has one test function that loops over all fixtures. Do not filter it by `union_expr_field_access`; that runs zero tests.
