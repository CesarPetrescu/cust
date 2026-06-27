# Unsupported flexible array aggregate field diagnostics

Date: 2026-06-27

Cust intentionally keeps C flexible array members outside its supported aggregate model because aggregate storage is deterministic and fixed-size. When parsing aggregate field declarator suffixes, detect `[]` immediately after a field name before the generic array-length helper runs.

Implementation note: `parse_aggregate_definition_body()` checks for `Token::RBracket` right after consuming a field's `[` suffix and reports `flexible array aggregate fields are not supported` at the `]` token. This preserves ordinary fixed-size field arrays, integer-constant-expression lengths, multidimensional-field diagnostics, and inferred-length object declarations elsewhere.

Focused verification:

```bash
cargo test --test interpreter flexible_array_aggregate_fields -- --nocapture
```
