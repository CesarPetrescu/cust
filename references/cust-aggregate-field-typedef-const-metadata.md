# Aggregate field typedef const metadata

Date: 2026-06-21

Cust's aggregate field parser consumes a shared field declaration specifier once, then parses comma-separated per-field declarators. When the shared specifier is a typedef alias, field const metadata must include both explicit qualifiers and alias-carried const metadata.

Concrete pitfall fixed in this run:

```c
typedef int * const ConstIntSlot;
struct Cursor { ConstIntSlot fixed, backup; };
```

`ConstIntSlot` is a const pointer slot alias. Before the fix, `parse_aggregate_definition_body` called `parse_decl_type(...)` after consuming explicit qualifiers but did not consult `type_alias_is_const(...)`, so aggregate fields spelled with const-qualified typedef aliases lost their field-level `is_const` metadata. That allowed `cursor.fixed = other;` even though native C rejects assignment to the const-qualified field.

Implementation pattern:

1. After consuming explicit aggregate-field qualifiers, check whether the next token is an alias identifier and call `type_alias_is_const(name)` before `parse_decl_type(...)` consumes it.
2. OR the alias const flag into the field's effective leading const metadata.
3. Let the existing pointer-field logic derive:
   - pointer-slot const from `is_const` for pointer aliases;
   - pointee const from `DeclType::Pointer { points_to_const, .. }`;
   - scalar/aggregate/array const from the same effective leading const flag.

Regression coverage:

- `tests/fixtures/valid/aggregate_field_typedef_declaration_lists.c`
- `tests/fixtures/invalid/aggregate_field_typedef_const_slot_assignment.c`
- `tests/fixtures/compat/valid/aggregate_field_typedef_declaration_lists.c`
- Focused command: `cargo test --test interpreter aggregate_field -- --nocapture`
- Compiler oracle: `cargo test --test c_compat -- --nocapture`
