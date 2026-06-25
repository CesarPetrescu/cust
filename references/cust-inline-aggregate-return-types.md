# Inline aggregate definitions in function return type specifiers

Date: 2026-06-25

Cust now supports named `struct`/`union` definitions in top-level function return type specifiers, for example:

```c
struct Pair { int x; int y; } make_pair(int base);
union Number { int value; char tag; } pick_number(int flag) { ... }
```

Implementation notes:

- Function lookahead must skip a full `struct Tag { ... }` / `union Tag { ... }` definition body before checking for the function declarator name.
- `parse_decl_type_with_embedded_qualifiers` handles named aggregate definitions by routing through `parse_aggregate_definition_body_after_keyword(...)` after the aggregate keyword has already been consumed.
- Return-type inline aggregate definitions register the aggregate tag before parsing the function declarator, so the function body and later declarations can use `struct Tag` / `union Tag` normally.
- Inline enum constants inside aggregate fields still flow through the existing pending-inline-enum mechanism; `parse_function_declaration()` emits that pending `EnumDecl` before function registration.
- Native compiler-oracle fixtures should avoid top-level `const` qualifiers on aggregate return types because `cc -std=c11 -Wall -Wextra -Werror` treats them as `-Werror=ignored-qualifiers`; keep `const` return coverage interpreter-only.

Verification used:

```bash
cargo test --test interpreter inline_aggregate_return_type_definitions -- --nocapture
cargo test --test c_compat -- --nocapture
```
