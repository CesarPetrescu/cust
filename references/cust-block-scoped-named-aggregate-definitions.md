# Block-scoped named aggregate definitions

Date: 2026-06-25

Cust already supported top-level `struct Name { ... };` / `union Name { ... };` definitions and block-scoped aggregate typedef definitions, but plain block-scope named aggregate tag definitions were routed through `parse_aggregate_var_decl()` and failed with `undefined struct type '<Name>'` before the tag body was parsed.

Implementation notes:
- In `parse_stmt()`, check `is_aggregate_definition()` before the generic `Token::Struct | Token::Union` aggregate-variable path.
- Reuse `parse_aggregate_definition()` for block-scope standalone definitions so tag metadata is installed in the current aggregate type scope and naturally expires when `parse_block_after()` pops the scope.
- Return any pending inline-enum `EnumDecl` from `parse_aggregate_definition()` as the local statement; otherwise lower the standalone definition to `Stmt::Empty`.
- Mirror top-level forward-declaration diagnostics in block scope so `struct Hidden;` / `union Hidden;` do not fall into misleading variable-declaration errors.

Acceptance coverage:
- `tests/fixtures/valid/block_scoped_named_aggregate_definitions.c` covers local `struct Pair { ... };`, nested-block `union Number { ... };`, and inline enum constants in a local `struct Flags { enum State { ... } state; };` definition.
- `tests/fixtures/invalid/block_scoped_named_aggregate_tag_out_of_scope.c` proves local named aggregate tags expire at block end.
- `tests/fixtures/compat/valid/block_scoped_named_aggregate_definitions.c` locks warning-free native compiler-oracle parity for the supported valid subset.
