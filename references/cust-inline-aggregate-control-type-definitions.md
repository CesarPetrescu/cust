# Inline aggregate definitions in control expressions

2026-06-29 autonomous run.

## Scope

Coverage-only conformance closure for named `struct` definitions introduced inside control-expression type-name contexts:

- `if (sizeof(struct IfBox { ... }) == sizeof(struct IfBox)) { ... }`
- `while (... && sizeof(struct WhileCell { ... }) == sizeof(struct WhileCell)) { ... }`
- `switch (sizeof(struct SwitchBox { ... }) == sizeof(struct SwitchBox)) { ... }`

## Result

Focused interpreter coverage passed immediately, and the fixture also passed the native compiler-oracle path. No production parser/runtime change was needed: shared type-name parsing for `sizeof(...)` already installs inline aggregate tag definitions into the enclosing block scope before later statement parsing/execution.

## Fixture guidance

Keep native checks ABI-independent by comparing `sizeof(struct T { ... }) == sizeof(struct T)` rather than asserting exact native aggregate sizes. Use the later declared `struct T` object inside the same control-flow body to prove the tag remains visible after the controlling expression.
