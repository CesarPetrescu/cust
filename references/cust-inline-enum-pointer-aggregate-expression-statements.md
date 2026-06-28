# Inline enum definitions in pointer/aggregate expression statements

2026-06-28 autonomous run.

## Scope

Coverage-only conformance closure for inline enum definitions nested inside expression statements whose value is pointer-valued or aggregate-valued:

- pointer assignment expressions such as `(cursor = values + (sizeof(enum E { A = 2 }) ? A : 0));`
- aggregate assignment expressions such as `(point = (struct Pair){ .x = _Alignof(enum E { A = 4 }) ? A : 0, ... });`
- aggregate compound literals passed as call arguments from an expression statement

## Result

Focused interpreter coverage passed immediately. The existing `parse_expr_stmt_with_semi()` pending-inline-enum wrapper already emits generated `EnumDecl` statements before evaluating the enclosing expression statement, so no production parser/runtime change was needed.

## Fixture notes

- Keep native compiler-oracle arithmetic below 256 because the harness compares native process exit codes.
- Use enum size/alignment relationships (`sizeof(enum E { ... }) == sizeof(enum E)`, truthiness of `_Alignof(...)`) instead of exact ABI-dependent enum byte sizes.
- Avoid unused expression warnings under `-Wall -Wextra -Werror` by feeding expression results into totals or mutations.
