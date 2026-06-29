# Inline aggregate definitions inside return expressions

Date: 2026-06-29

Work package: conformance coverage for inline named `struct` and `union` definitions introduced inside `return` expression type-name contexts.

## Finding

Cust's shared type-name and compound-literal parsing already installs inline aggregate tags in the enclosing function block while parsing return expressions. No production change was needed for forms like:

```c
return (sizeof(struct ReturnBox { int x; int y; }) == sizeof(struct ReturnBox))
    + ((struct ReturnBox){5, 6}).x;
```

and matching `union` type-query/compound-literal return expressions.

## Verification pattern

- Add an interpreter fixture under `tests/fixtures/valid/` with a focused test in `tests/interpreter.rs`.
- Add a warning-free compiler-oracle fixture under `tests/fixtures/compat/valid/` and register it in `tests/c_compat.rs`.
- Keep native checks ABI-independent by comparing relationships such as `sizeof(Tag definition) == sizeof(Tag)` and return totals below 256.

Focused tests may pass immediately; treat that as conformance coverage closure rather than inventing production changes.
