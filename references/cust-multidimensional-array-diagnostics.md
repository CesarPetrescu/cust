# Multidimensional array diagnostics

2026-05-10 autonomous run: Cust deliberately keeps the current array subset one-dimensional. When adding exact diagnostics for unsupported multidimensional arrays, fail immediately after the first parsed `[...]` if another `[` follows, before generic semicolon/list-boundary handling can run.

Covered parser sites:

- Local/global scalar or aggregate array declarations: `int matrix[2][3];`, `struct Point points[2][3];` → `multidimensional array declarations are not supported` at the second `[`.
- Function parameter spellings after scalar or aggregate array decay syntax: `int matrix[2][3]`, `struct Point points[][2]` → `multidimensional array parameters are not supported` at the second `[`.
- Aggregate field declarations: `int matrix[2][3];`, `struct Point points[2][3];` inside `struct`/`union` definitions → `multidimensional array <struct|union> fields are not supported` at the second `[`.

Pitfall: the second bracket used to fall through to `expected ';' after array declaration`, `expected ',' or ')' after function parameter`, or `expected ';' after struct field declaration`. Keep new checks immediately after `expect_closing_bracket_after(...)` for the first dimension so the diagnostic points at the unsupported second dimension.
