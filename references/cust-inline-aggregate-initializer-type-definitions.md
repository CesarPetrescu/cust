# Inline aggregate definitions in initializer expressions

2026-06-29 autonomous run: inline named aggregate definitions inside aggregate initializer expressions and designator indexes were covered as conformance-only work.

## Fixture pattern

Use warning-free, ABI-independent native-oracle checks:

- Positional aggregate initializers can contain `sizeof(struct InitBox { ... }) == sizeof(struct InitBox)` and inline aggregate compound literals such as `((struct LitBox { ... }){6}).value`.
- Designator indexes can contain inline aggregate type queries, for example `.values[sizeof(struct IndexBox { ... }) == sizeof(struct IndexBox)] = 9`.
- Field designators can contain matching inline `union` definitions, for example `.marker = sizeof(union MarkChoice { ... }) == sizeof(union MarkChoice)`.
- Later declarations in the same block should use the inline tags to prove the parser installed them in the enclosing block scope.

## Result

Focused interpreter and native compiler-oracle coverage passed immediately. Cust's shared aggregate initializer/designator expression parsing already routes through the same type-name and compound-literal machinery used by declaration, assignment, return, call-argument, and expression-statement contexts, so no production parser/runtime change was needed.
