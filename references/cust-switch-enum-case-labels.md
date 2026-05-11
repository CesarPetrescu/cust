# Cust switch enum case labels

Date: 2026-05-11

Feature: C-style enum constants in `switch case` labels and small additive integer constant expressions for enum initializers/case labels.

## Implementation notes

- Parser now maintains `enum_constant_scopes` alongside enum-tag/type-alias/aggregate scopes.
- `parse_enum_decl_body` records constants in a local map as they are parsed so later constants in the same enum may reference earlier constants (for example `DONE = BUSY + 3`).
- Completed enum declarations extend the current parser enum-constant scope so later switch labels in the same lexical scope can resolve identifiers.
- `parse_switch_case_value` reuses the shared integer-constant-primary helper and accepts identifiers plus simple `+`/`-` constant expressions.
- Runtime enum declarations still install enum constants for normal expression evaluation; parser scopes are metadata for parse-time case-label duplicate detection and value lowering.

## Coverage

- `tests/fixtures/valid/switch_enum_case_labels.c` covers global enum constants, enum initializer references, block-scoped enum constants used as case labels, and default fallback.
- `tests/fixtures/invalid/switch_duplicate_enum_case.c` proves identifier-valued case labels participate in duplicate-case diagnostics with the resolved value.
- `tests/fixtures/compat/valid/switch_enum_case_labels.c` is included in the C compiler-oracle suite.

## Pitfalls

- Do not let parser enum-constant scopes outlive block parsing; they must push/pop with `parse_block_after` or out-of-scope enum constants become accepted in later functions/blocks.
- Keep the supported integer constant-expression subset deliberately narrow unless new fixtures demand more C operators; broader expression parsing must remain non-evaluating and cannot depend on runtime variables.
