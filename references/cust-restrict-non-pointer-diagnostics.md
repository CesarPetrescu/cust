# Restrict non-pointer diagnostics

Date: 2026-06-30

Cust supports C99 `restrict` as parser-level no-op metadata only on post-star pointer declarators, matching the existing supported fixture shape:

```c
int * restrict p;
int * const restrict slot;
struct Cursor { int * restrict p; };
int sum(int * restrict values, int length);
```

Native `cc -std=c11 -Wall -Wextra -Werror` rejects leading non-pointer forms such as `restrict int value;` with `invalid use of 'restrict'`. Cust previously accepted those because shared leading type-qualifier consumption treated `restrict` like `const`/`volatile` before base types.

Implementation notes:

- Add a parser helper that scans the leading qualifier sequence at the current token and reports the first `restrict` token before consuming it.
- Call it before shared declaration type parsing (`parse_const_qualified_decl_type` / `parse_decl_type_with_embedded_qualifiers`) and before aggregate field qualifier consumption.
- Keep `consume_type_qualifiers()` accepting `restrict` for post-star pointer declarators, so valid `int * restrict p` paths continue to parse unchanged.

Focused coverage:

```bash
cargo test --test interpreter rejects_restrict_on_non_pointer_declarations_with_context -- --nocapture
cargo test --test interpreter supports_restrict_pointer_qualifiers -- --nocapture
```

Exact diagnostic:

```text
restrict qualifiers are only supported on pointer declarators at line ..., column ...
```
