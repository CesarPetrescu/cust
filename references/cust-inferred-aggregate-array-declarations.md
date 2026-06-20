# Cust inferred aggregate array declarations

Date: 2026-06-20

## Scope

Cust accepts one-dimensional inferred-length aggregate array object declarations when a brace initializer is present:

```c
struct Point points[] = {{1, 2}, {.y = 4}, [3] = {5, 6}};
const struct Point fixed[] = {{7, 8}, {.x = 9}};
union Number numbers[] = {{3}, [2] = {.value = 5}};
```

The parser lowers these to the existing fixed-length `Stmt::StructArrayDecl` path after inferring the length from positional entries and designator indexes with `infer_struct_array_initializer_len`.

## Implementation notes

- Direct `struct`/`union` declarations are parsed in `parse_aggregate_var_decl`; typedef-spelled aggregate declarations flow through `parse_decl_stmt` with `DeclType::Struct`.
- Both paths check `[]` before trying `expect_array_len()`, require an immediate `=`, parse the initializer with the existing unbounded aggregate-array initializer helper, infer the length, then reuse fixed aggregate-array runtime initialization.
- Initializer-less incomplete aggregate arrays are intentionally rejected with `expected '=' after inferred aggregate array declaration`.
- Runtime behavior is inherited from fixed aggregate arrays: omitted elements zero-fill, `sizeof(array) / sizeof(array[0])` sees the inferred length, array-to-pointer decay and pointer arithmetic work, const aggregate arrays keep const-discard enforcement, and pointer mutations alias the original storage.

## Tests

- Interpreter fixture: `tests/fixtures/valid/inferred_aggregate_array_declarations.c` covers struct and union arrays, designators, omitted-element zero-fill, const arrays, `sizeof` length queries, pointer decay/arithmetic, and mutation aliasing.
- Invalid exact-output test: `reports_inferred_aggregate_array_declarations_without_initializers` covers `struct Point points[];`.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/inferred_aggregate_array_declarations.c` verifies native-compatible exit behavior while avoiding native ABI-sensitive exact aggregate sizes.

## Pitfalls

- Do not rely on native `sizeof(struct)` byte counts in expected values. The compat fixture uses `sizeof(array) / sizeof(array[0])`, which is stable across Cust's deterministic layout and native C layout.
- The existing `parse_struct_array_initializer(name, type_name, len)` is bounded and rejects entries beyond a known length; inferred declarations need the unbounded aggregate-array initializer helper before length inference.
