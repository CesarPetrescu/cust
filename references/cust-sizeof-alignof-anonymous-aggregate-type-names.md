# `sizeof` / `_Alignof` anonymous aggregate type names

2026-06-23 autonomous run.

## Scope

Cust now accepts anonymous `struct { ... }` and `union { ... }` specifiers in `sizeof(...)` and `_Alignof(...)` type-name operands.

Covered forms:

- `sizeof(struct { int x; char tag; })`
- `sizeof(union { int value; char tag; })`
- `sizeof(const struct { char tag; int values[2]; })`
- `sizeof(struct { int x; } *)`
- `sizeof(struct { char c; }[3])`
- `_Alignof(struct { char c; int x; })`
- `_Alignof(union { int x; char c; })`

## Implementation notes

`parse_sizeof_like_type_name()` now detects `struct`/`union` followed by `{` before calling `parse_decl_type(...)`, because the ordinary declaration-type parser expects a named aggregate tag and previously failed with `expected sizeof struct type name, found LBrace`.

It reuses `parse_aggregate_definition_body(false, true)` so field-list parsing, duplicate-field checks, nested named aggregate fields, array fields, pointer-field diagnostics, and deterministic size/alignment behavior remain shared with anonymous aggregate object declarations. The anonymous internal type is still registered in `struct_types` for nested metadata lookup, but no source-level tag or typedef alias is installed.

`SizeOfType::AnonymousAggregate(StructTypeDef)` stores a clone of the parsed type definition for direct anonymous type-name operands. Pointer and one-dimensional array suffixes continue through the existing `SizeOfType::Pointer` and `SizeOfType::Array(PointeeType::Struct(internal_type), len)` paths.

## Verification

Focused RED/GREEN:

```bash
cargo test --test interpreter supports_sizeof_and_alignof_anonymous_aggregate_type_names -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Native compiler-oracle fixture uses ABI-independent relationships (`sizeof(struct { int x; }) == sizeof(int)`, pointer size equality, `_Alignof(...) == alignof(...)`) rather than Cust-specific exact aggregate byte totals, because native C struct padding differs from Cust's no-padding deterministic model.
