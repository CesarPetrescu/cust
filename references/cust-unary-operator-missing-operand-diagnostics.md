# Unary operator missing-operand diagnostics

Date: 2026-07-02

Cust's prefix unary parser (`parse_unary`) consumes prefix operators and recursively parses the operand. Without a narrow pre-check, malformed forms such as `return !;`, `return ~);`, `return ++];`, `return *}`, and `return &;` fall through to generic primary-expression diagnostics (`expected expression, found ...`) that do not identify the operator that needs an operand.

Implementation pattern:

- After consuming a prefix unary operator (`++`, `--`, unary `+`, unary `-`, `~`, `!`, `*`, `&`), preserve the `LocatedToken` for the operator.
- Before recursing into `parse_unary()`, reject delimiter/terminator tokens (`Comma`, `Colon`, `RParen`, `RBracket`, `Semi`, `RBrace`, `Eof`) with `Parser::error_at(...)` on `peek_located()`.
- Diagnostic format: `expected expression after unary operator '<op>', found <Token>`.
- Keep `sizeof`/`_Alignof` on their route-specific parsers because they already have type/expression-specific diagnostics.

Focused verification:

```bash
cargo test --test interpreter rejects_missing_operands_after_unary_operators -- --nocapture
cargo test --test interpreter supports_logical_operators_short_circuiting_and_unary_plus -- --nocapture
cargo test --test interpreter supports_bitwise_and_shift_operators_with_c_precedence -- --nocapture
cargo test --test interpreter supports_increment_decrement_for_scalar_indexed_and_deref_lvalues -- --nocapture
```

Pitfall: Cargo test filters are substring-only; run the three positive regression tests as separate commands rather than passing multiple test names in one command.
