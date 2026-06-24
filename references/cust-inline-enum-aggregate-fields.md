# Inline enum aggregate fields

Date: 2026-06-24

Cust supports C enum specifiers used as scalar fields inside supported `struct`/`union` definitions, including nested named/anonymous aggregate definitions and anonymous aggregate typedef definitions.

## Implementation notes

- `parse_decl_type_with_embedded_qualifiers()` lowers inline enum definitions in field type positions to `DeclType::Scalar(CType::Int)` and records the parsed `EnumConstant`s through `push_pending_inline_enum_constants()`.
- `parse_aggregate_definition()` now returns an optional `Stmt::EnumDecl` so standalone aggregate definitions such as `struct Flags { enum State { READY = 1 } state; };` install the enumerators before later globals/functions use them.
- `parse_aggregate_var_decl()` wraps anonymous aggregate variable declarations in `Stmt::Many(EnumDecl, decl)` when enum fields appear in the anonymous type, preserving initializer visibility for forms like `struct { enum { A = 1 } e; } value = {A};`.
- `parse_typedef_decl()` combines ordinary enum-typedef constants and aggregate-field inline enum constants, so `typedef struct { enum { A = 1 } e; } T; T t = {A};` works without leaking stale pending constants into the next declaration.
- Multiple inline enum field definitions in one aggregate must append pending constants rather than overwrite them.

## Verification

Focused RED/GREEN:

```bash
cargo test --test interpreter inline_enum_aggregate_fields -- --nocapture
cargo test --test c_compat -- --nocapture
```

Full gate for the run also covered `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and Docker Compose verification.
