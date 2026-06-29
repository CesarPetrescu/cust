# Inline aggregate definitions in for clauses

2026-06-29 autonomous run.

## Scope

Coverage-only conformance closure for named aggregate definitions introduced inside `for` statement clauses:

- `for (int i = sizeof(struct ForInitBox { ... }) == sizeof(struct ForInitBox); ...; ...) { ... }`
- `for (; cond && sizeof(struct ForCondBox { ... }) == sizeof(struct ForCondBox); ...) { ... }`
- `for (; cond; i += sizeof(union ForIncChoice { ... }) == sizeof(union ForIncChoice)) { ... }`

## Result

Focused interpreter coverage and the native compiler-oracle fixture passed immediately. No production parser/runtime change was needed: shared type-name parsing for `sizeof(...)` already installs inline aggregate tag definitions early enough for same-loop-body declarations that use those tags.

## Fixture guidance

Keep native checks ABI-independent by comparing `sizeof(struct/union T { ... }) == sizeof(struct/union T)` rather than exact byte sizes. Use simple loop bounds and read an initialized first `int` field in the loop body to avoid `-Wall -Wextra -Werror` traps while proving same-body tag visibility.