# Array-length comma-expression diagnostics

Date: 2026-06-27

Cust supports parser-folded integer constant expressions in fixed-size one-dimensional array declarator lengths, but it intentionally rejects comma expressions in integer constant expression contexts. After array-length parsing was broadened from numeric tokens to `parse_integer_constant_expr(...)`, source such as:

```c
int values[1, 2];
```

parsed the leading `1` and then fell through to the generic bracket helper, producing `expected ']' after array length, found Comma`.

Implementation note: keep `parse_integer_constant_primary(...)`'s existing parenthesized-comma guard for forms like `[(1, 2)]`, and also check for `Token::Comma` immediately after `expect_array_len()` calls `parse_integer_constant_expr(...)`. Report:

```text
comma operator is not allowed in integer constant expression
```

at the comma token before positive-length and closing-bracket checks proceed.

Coverage:

```bash
cargo test --test interpreter rejects_comma_operator_in_array_length_integer_constant_expressions -- --nocapture
cargo test --test interpreter rejects_non_constant_array_lengths_with_context -- --nocapture
```
