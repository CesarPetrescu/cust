# Anonymous aggregate fields

Date: 2026-06-23

Cust now supports anonymous `struct { ... }` / `union { ... }` definitions as fields inside supported aggregate definitions, for example:

```c
struct Box {
    struct { int x; int y; } point;
    union { int value; char tag; } number;
    struct { int value; } items[2];
};
```

Implementation notes:

- Patch `parse_aggregate_definition_body()` in the aggregate-field type branch.
- When a field declaration starts with `struct`/`union` and `peek_next()` is `{`, call `parse_aggregate_definition_body(false, true)` before the existing named-tag field path.
- Lower the returned unique internal anonymous type name to `DeclType::Struct(...)`; existing field declarator-list logic then handles direct fields, arrays, pointer fields, const metadata, initializers, deep-copy semantics, `sizeof`, and field access.
- Do not install source-level tags or typedef aliases for these field-local anonymous types.
- Keep distinct anonymous type identities distinct: separately spelled anonymous aggregate declarations are not assignment-compatible even when their field layouts match. The regression `anonymous_aggregate_distinct_type_pointer_assignment.c` locks this in.

Verification used:

```bash
cargo test --test interpreter supports_anonymous_aggregate_fields -- --nocapture
cargo test --test interpreter rejects_distinct_anonymous_aggregate_pointer_assignments -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
