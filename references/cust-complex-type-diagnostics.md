# Unsupported complex type diagnostics

Date: 2026-07-01

Cust intentionally does not implement C complex or imaginary numeric runtime semantics. Lex `_Complex` and `_Imaginary` as dedicated tokens so parser routes can reject them before they fall through as identifiers.

Implementation notes:

- Add `Token::Complex` and `Token::Imaginary` in the lexer keyword table.
- Include both tokens in declaration/type-name/cast lookahead sets so top-level declarations, block locals, parameters, `sizeof`/`_Alignof` type names, casts, and aggregate fields all enter the shared type parser.
- In `parse_decl_type_with_embedded_qualifiers(...)`, reject both tokens immediately after `advance()` using `Parser::error_at(...)` so diagnostics point at the keyword.
- Keep the message stable: `complex types are not supported`.

Regression coverage:

```bash
cargo test --test interpreter rejects_complex_type_specifiers_with_context -- --nocapture
```

Acceptance examples:

- `_Complex global_value;`
- `_Imaginary local_value;`
- `int take(_Complex value)`
- `sizeof(_Complex)`
- `(_Imaginary)1`
- `struct Sample { _Complex value; };`
