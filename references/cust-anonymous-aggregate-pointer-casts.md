# Anonymous aggregate pointer casts

Date: 2026-06-23

Cust now supports pointer casts whose pointee type is an expression-local anonymous aggregate type name, for example:

```c
(struct { int x; } *)0
(const union { char tag; } *)0
sizeof(*(struct { char tag; } *)0)
```

Implementation notes:

- Patch `parse_cast()` before the anonymous aggregate array/compound-literal branches.
- After `parse_aggregate_definition_body(false, true)`, check for a following `*` and lower to `Expr::PointerCast { pointee: PointeeType::Struct(type_name), points_to_const: leading_const, ... }`.
- Consume post-star qualifiers as parser metadata/no-ops, matching named aggregate pointer casts.
- Do not install a source-level tag or typedef alias for the anonymous type; it remains a unique internal type identity.
- Fixtures should avoid assigning between separately spelled anonymous aggregate pointer types because C treats each spelling as a distinct anonymous type. Use null pointer comparisons and non-evaluating `sizeof(*(anonymous *)0)` metadata instead.

Verification used:

```bash
cargo test --test interpreter supports_anonymous_aggregate_pointer_casts -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
