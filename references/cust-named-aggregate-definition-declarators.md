# Named aggregate definition declarators

Date: 2026-06-25

## Scope

Cust now accepts C aggregate definitions that introduce a named tag and declare one or more objects in the same declaration, such as:

```c
struct Point { int x; int y; } point = {1, 2}, *slot = &point;
union Number { int value; char tag; } number = {7};
static struct Counter { int value; } counter = {4};
```

This is distinct from the already-supported standalone tag definition form:

```c
struct Point { int x; int y; };
```

## Root cause

`parse_stmt()` / top-level parsing treated every `struct Ident { ... }` or `union Ident { ... }` prefix as a standalone aggregate definition and immediately called `parse_aggregate_definition()`, which requires the closing brace to be followed by `;`. When a declarator followed the body, parsing failed with a generic message such as:

```text
expected ';' after struct declaration, found Ident("global_point")
```

## Implementation notes

- `is_aggregate_definition()` now scans the aggregate body and returns true only when the closing brace is followed by `;`, keeping standalone definitions on the existing path.
- `parse_aggregate_var_decl()` handles the named-definition-with-declarator shape (`struct/union Ident { ... } declarator...;`) by reusing `parse_aggregate_definition_body(false, false)` and then the existing `parse_aggregate_var_decl_after_type(...)` declarator-list machinery.
- This preserves comma-separated aggregate declarators, pointer declarators, const/read-only metadata, static-local wrapping, and pending inline enum constants from aggregate fields through the existing helpers.

## Verification

Focused TDD coverage:

```bash
cargo test --test interpreter named_aggregate_definition_declarators -- --nocapture
cargo test --test c_compat -- --nocapture
```

The native compiler-oracle fixture uses warning-free C11 with global, local, static-local, pointer, comma-separated declarator, union, and inline-enum field coverage.
