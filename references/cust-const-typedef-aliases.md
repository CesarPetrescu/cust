# Const-qualified typedef aliases

Date: 2026-05-11

Cust now preserves leading `const` on non-pointer typedef aliases such as `typedef const int ConstInt;`, `typedef const char ConstChar;`, `typedef const struct Point ConstPoint;`, and `typedef const int Scores[3];`.

Implementation notes:

- `Parser` keeps a lexical `const_type_alias_scopes` stack parallel to `type_alias_scopes`.
- `parse_const_qualified_decl_type` merges explicit leading qualifiers with const metadata from the alias token before lowering to `DeclType`.
- Runtime enforcement reuses existing declaration/parameter const paths, so alias-spelled const scalars, aggregate values, and arrays are read-only after initialization.
- Pointer typedef aliases still use the existing pointer-alias metadata; this run intentionally covered non-pointer const aliases and did not add const-pointee pointer typedef semantics such as `typedef const int *ConstIntPtr`.

Verification coverage:

- `tests/fixtures/valid/const_typedef_aliases.c`
- `tests/fixtures/invalid/const_typedef_alias_assignment.c`
- `tests/fixtures/compat/valid/const_typedef_aliases.c`
- Focused RED/GREEN command: `cargo test --test interpreter const_typedef -- --nocapture`
- Compiler oracle: `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`
