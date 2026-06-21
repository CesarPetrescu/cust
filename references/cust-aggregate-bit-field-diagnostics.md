# Unsupported aggregate bit-field diagnostics

Date: 2026-06-21

Cust intentionally does not implement C bit-field storage/layout semantics. When a `:` follows a parsed `struct` or `union` field declarator name, report a targeted parser error at the colon instead of falling through to the generic field-declaration semicolon helper.

Covered forms:

```c
struct Flags { unsigned ready : 1; };
union Bits { int value : 3; };
```

Implementation note: `parse_aggregate_definition_body` already parses shared aggregate field specifiers and per-declarator names for comma-separated field lists. The bit-field check belongs immediately after `expect_ident_after("<kind> field name after type")` and before duplicate-field insertion / array suffix parsing, so unsupported `name : width` is rejected before `expect_semicolon_after("struct field declaration")` produces a misleading message.

Focused verification:

```bash
cargo test --test interpreter rejects_aggregate_bit_fields_with_context -- --nocapture
```
