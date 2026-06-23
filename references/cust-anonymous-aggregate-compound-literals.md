# Anonymous aggregate compound literals

Run date: 2026-06-23

## Scope

Cust now accepts C99 compound literal type names that define an anonymous aggregate inline:

```c
((struct { int x; int y; }){.y = 5, .x = 2}).x
((union { int value; char tag; }){7}).value
((struct { int x; int y; }[]){{1, 2}, {.y = 9}})[1].y
```

The feature is expression-local: the parser creates a unique internal anonymous aggregate type identity and does **not** install a user-visible tag or typedef alias.

## Implementation notes

- Patch `parse_cast()`, not `parse_decl_type()`, for this expression path.
- Detect `struct`/`union` followed by `{` immediately after consuming leading qualifiers and before the ordinary named-tag `parse_decl_type("cast type")` path.
- Reuse `parse_aggregate_definition_body(false, true)` to parse fields and allocate the unique internal type.
- After the closing aggregate body:
  - `[]` or `[N]` before `)` routes to the existing `Expr::AggregateArrayLiteral` path with `parse_aggregate_array_compound_initializer(...)`.
  - Otherwise require `)` followed by `{...}` and route to existing `Expr::AggregateLiteral` / `parse_struct_initializer(...)`.
- Keep native compiler-oracle fixtures warning-free by using direct field/index behavior. Do not assign pointers between separately spelled anonymous aggregate type names; C treats each spelling as a distinct type.

## Verification

Focused RED/GREEN command:

```bash
cargo test --test interpreter supports_anonymous_aggregate_compound_literals -- --nocapture
```

Compiler-oracle command:

```bash
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
