# Anonymous aggregate return type diagnostics

Date: 2026-06-22

Cust supports anonymous aggregate object declarations and typedef-backed anonymous aggregate types, but it does not support function return types spelled directly as anonymous aggregates such as:

```c
struct { int x; } make(void) { ... }
union { int value; } pick(void);
```

These return types create unnameable aggregate identities at the function boundary and are outside the current named/typedef-backed aggregate function model.

Implementation notes:

- Detect the unsupported form before top-level or block-scope function declaration routing falls through to aggregate variable declaration parsing.
- Use a conservative lookahead from `struct`/`union` followed by `{ ... }`, optional qualifiers/stars, then `identifier(`.
- Report `anonymous aggregate return types are not supported` at the aggregate keyword.
- Keep anonymous aggregate object declarations such as `struct { int x; } point = {1};` routed through `parse_aggregate_var_decl`.

Focused verification:

```bash
cargo test --test interpreter rejects_anonymous_aggregate_return_types_with_context -- --nocapture
cargo test --test interpreter anonymous_aggregate -- --nocapture
```
