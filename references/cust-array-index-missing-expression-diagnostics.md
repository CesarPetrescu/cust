# Array index missing-expression diagnostics

Date: 2026-07-02

Cust's array/string/postfix index parser routes bracket contents through `parse_index_expr()`, which intentionally parses only up to logical-or precedence so comma separators and closing brackets remain visible to the surrounding grammar.

Malformed empty or delimiter-only indexes such as `values[]`, `values[;`, `"hi"[]`, and `values[}` used to fall through to the generic primary-expression diagnostic (`expected expression, found ...`). The narrow fix is to reject delimiter/terminator tokens at the start of `parse_index_expr()` and report `expected array index expression, found ...` at the offending token before delegating to expression parsing.

Verification pattern:

```bash
cargo test --test interpreter rejects_missing_array_index_expressions_with_context -- --nocapture
cargo test --test interpreter array -- --nocapture
```

This preserves existing valid array, pointer, string, and aggregate-array indexing behavior while improving parser-trust diagnostics for malformed indexes.
