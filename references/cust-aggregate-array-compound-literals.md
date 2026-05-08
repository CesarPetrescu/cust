# Cust aggregate-array compound literals

Commit: pending in the 2026-05-08 autonomous aggregate-array compound literal run.

## Feature summary

Cust supports C99-style aggregate-array compound literals for supported struct/union element types:

- Unsized forms such as `(struct Point[]){{1, 2}, {.x = 3}}` and `(union Number[]){{7}, [2] = {.value = 9}}`.
- Fixed-size forms such as `(struct Point[3]){[2] = {.x = 5}, [0] = {1, 2}}`.
- The expression evaluates as a `struct T *` / `union U *` pointer to element zero.
- Positional entries, element designators, nested aggregate initializers, and omitted-element zero/default fill reuse the existing aggregate-array initializer semantics.
- Fixed-size excess positional initializers report `too many initializers for aggregate array compound literal`.

## Implementation notes

- Parser support lives in `parse_cast()` for `DeclType::Struct` followed by `[ ... ]` before `)`.
- `Expr::AggregateArrayLiteral` stores the aggregate type name, optional length, and `Vec<StructArrayInitializer>`.
- Evaluation creates a hidden current-scope `Value::StructArray` with a generated non-C identifier and returns `PointerValue::StructElement { index: 0 }` pointing at it.
- This intentionally reuses the existing aggregate-array pointer model for indexing, mutation, parameter binding, pointer type compatibility, and scope-lifetime diagnostics instead of adding host/native addresses.

## Pitfalls

- Do not convert aggregate-array compound literals to `Rc`-backed pointer targets unless the struct-pointer access helpers are refactored; existing `StructElement` pointer helpers expect scope/name/index lookup.
- The hidden current-scope storage means pointers follow Cust's existing block/function scope lifetime. If exact temporary lifetime diagnostics are added later, revisit compound literal escape behavior together with scalar-array compound literals.
- Keep native C oracle fixtures behavior-only; avoid native aggregate `sizeof` or ABI layout assertions.
