# Cust anonymous aggregate object declarations

Date: 2026-06-21

## Scope

This run added ordinary C anonymous aggregate object declarations for the existing supported struct/union object subset:

```c
struct { int x; int y; } point = {1, 2};
union { int value; char tag; } number = {5};
struct { int values[2]; } packet = {{6, 7}};
struct { int x; int y; } points[2] = {{8, 9}, {.x = 10, .y = 11}};
```

Coverage includes global and local object declarations, anonymous aggregate arrays, brace/designated initializers, scalar array fields inside anonymous aggregates, and field reads. The C compiler-oracle fixture avoids ABI layout assertions and only compares exit behavior.

## Implementation notes

- `parse_aggregate_var_decl` now detects `struct`/`union` followed by `{` before expecting a named aggregate type.
- It calls `parse_aggregate_definition_body(false, true)` so the parser allocates the same unique internal type identities used by anonymous aggregate typedef definitions.
- No source-level tag or typedef alias is installed; the anonymous type only survives as parsed program metadata for that declaration and its runtime storage.
- The ordinary aggregate declarator tail was split into `parse_aggregate_var_decl_after_type(kind, type_name, points_to_const)` so named and anonymous aggregate declarations share pointer/object/array initializer handling.

## Pitfalls

- Do not route `struct { ... } object;` through `is_aggregate_definition()`: a standalone named-definition-style parse would require a semicolon immediately after `}` and would lose the following declarator.
- Keep anonymous object declarations distinct from anonymous aggregate typedef definitions: typedef aliases install user-facing names, object declarations do not.
- The current completed slice covers object and array declarations. Future const/pointer/address-of forms should be added with focused RED/GREEN fixtures rather than assumed from named aggregate behavior.

## Verification

Focused RED/GREEN command:

```bash
cargo test --test interpreter supports_anonymous_aggregate_object_declarations -- --nocapture
```

Compiler-oracle coverage:

```bash
cargo test --test c_compat -- --nocapture
```
