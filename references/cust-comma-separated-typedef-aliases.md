# Comma-separated typedef aliases

Date: 2026-06-22

## Feature shape

Cust now parses C typedef declarator lists such as:

```c
typedef int Count, *CountPtr, Counts[3];
typedef const int ConstCount, *ConstCountView, ConstCounts[2];
typedef struct Point Point, *PointPtr, Points[2];
typedef struct { int value; int extra; } Anon, *AnonPtr, Anons[2];
```

Each declarator derives from the shared declaration specifiers but has its own pointer star, post-star qualifiers, and array suffix.

## Implementation notes

- `parse_typedef_decl` now parses a base `DeclType` once, then loops over typedef declarators until the terminating semicolon.
- `parse_typedef_declarator` mirrors ordinary declaration-list handling for per-declarator `*`, `const` after `*`, and `[N]` suffixes.
- `register_typedef_alias` centralizes alias insertion, duplicate detection, const-alias metadata registration, and anonymous aggregate display-name assignment.
- For `typedef const int *View, Table[2];`, `View` is a pointer to const int, while `Table` is a const array alias because the shared leading const applies to non-pointer aliases.
- For `typedef int *Ptr, * const Slot;`, only `Slot` is a const pointer-slot alias because the post-star const is per declarator.

## Verification pattern

Focused tests:

```bash
cargo test --test interpreter comma_separated_typedef -- --nocapture
cargo test --test c_compat -- --nocapture
```

Full autonomous gate still requires `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `docker compose run --rm test`, and `docker compose run --rm cust`.

## Pitfalls

- Do not parse the first typedef alias with the old single-alias path and then special-case commas; C typedefs use declarator-list syntax, so pointer stars and array suffixes are per declarator.
- Keep pointer-to-pointer, pointer-array, function-pointer, function-typedef, and multidimensional-array typedef diagnostics in the declarator helper so each list entry is checked at its own source location.
- Anonymous aggregate display names should be assigned for the struct/union alias declarator (`typedef struct { ... } Alias, *AliasPtr;`). Avoid overwriting it for pointer/array aliases in the same list.
