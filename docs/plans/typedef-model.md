# Cust Typedef Alias Model

Last updated: 2026-05-06

## Goal

Add a deliberately scoped, preprocessor-free `typedef` milestone that improves C-subset readability without changing runtime storage. Typedefs are compile-time parser aliases only; the interpreter continues to execute the same scalar, struct, array, pointer, and function machinery after parsing.

## Supported forms

- Top-level scalar aliases:
  - `typedef int Count;`
  - `typedef char Byte;`
- Top-level struct aliases after a prior struct declaration:
  - `struct Point { int x; char y; };`
  - `typedef struct Point Point;`
- Alias use in supported declaration sites:
  - globals and locals: `Count total = 1;`, `Byte c;`, `Point p;`
  - arrays: `Count values[3];`
  - one-level pointers by spelling `*` at the use site: `Count *slot = &values[0];`, `Point *p = &point;`
  - function definitions and prototypes: `Count add(Count x, Byte y);`, `Point make(Count x);`
  - function parameters and returns, including struct-by-value semantics already implemented for the underlying struct type
  - struct field declarations for scalar aliases such as `Count x;`
  - `sizeof(alias)` and `sizeof(alias *)`

## Runtime model

Typedef aliases are resolved during parsing into existing Cust type metadata:

- scalar aliases resolve to `CType::Int` or `CType::Char`;
- struct aliases resolve to the underlying struct type name;
- pointer declarations through aliases still use existing `PointeeType` values;
- no alias information is stored in runtime scopes or values.

This means aliases do not create distinct types: `Count` behaves exactly like `int`, and `Point` behaves exactly like `struct Point`.

## Unsupported forms for this milestone

- Pointer aliases such as `typedef int *IntPtr;` are intentionally rejected with `typedef pointer aliases are not supported`.
- Anonymous struct typedefs such as `typedef struct { int x; } Point;` are not supported.
- Block-scoped typedef lifetime/shadowing is not modeled yet; current alias support is intended for top-level declarations.
- Function pointer typedefs, array typedefs, enum typedefs, type qualifiers (`const`), and aggregate/nested struct fields remain future work.

## Acceptance coverage

- `tests/fixtures/valid/typedef_aliases.c` covers scalar and struct aliases in globals, locals, arrays, pointer declarations, prototypes, function parameters/returns, `sizeof(alias)`, and struct copy/field access.
- `tests/fixtures/invalid/typedef_missing_alias_name.c` covers the exact missing-alias diagnostic.
- `tests/fixtures/compat/valid/typedef_aliases.c` verifies a native C compiler oracle for alias use while avoiding Cust-vs-native ABI `sizeof(int)`/struct-padding differences.

## Follow-up candidates

1. Add pointer alias syntax (`typedef int *IntPtr;`) only after deciding how alias pointers should interact with existing pointer-array and pointer-to-pointer diagnostics.
2. Add block-scoped typedef shadowing if parser type scopes are generalized beyond the current top-level alias table.
3. Add enum typedefs after deciding whether Cust should support enum-typed variables or continue exposing only enum constants as integer values.
