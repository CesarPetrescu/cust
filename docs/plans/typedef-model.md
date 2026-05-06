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
- Named enum aliases after a prior enum declaration:
  - `enum Status { READY = 1, BUSY };`
  - `typedef enum Status Status;`
  - enum aliases resolve to Cust's integer scalar type; Cust still exposes enum constants as scoped read-only integer identifiers and does not add distinct enum-typed runtime values.
  - enum tags follow Cust parser block scoping: aliases can reference tags declared in the same or an outer block, but not after the tag's block exits.
- Block-scoped aliases and shadowing inside function/control-flow blocks:
  - `{ typedef char Count; Count c = 'A'; }`
  - inner aliases may shadow outer aliases, and the outer alias is restored when the block ends.
- Alias use in supported declaration sites:
  - globals and locals: `Count total = 1;`, `Byte c;`, `Point p;`
  - arrays: `Count values[3];`
  - one-level pointers by spelling `*` at the use site: `Count *slot = &values[0];`, `Point *p = &point;`
  - one-level pointer aliases: `typedef int *IntPtr;`, `typedef char *CharPtr;`, and `typedef struct Point *PointPtr;`
  - pointer-alias globals/locals/parameters/prototypes: `IntPtr slot = &value;`, `void bump(IntPtr slot);`, `PointPtr point = &p;`
  - enum-alias globals/locals/arrays/parameters/prototypes/returns: `Status current = READY;`, `Status next(Status value);`
  - function definitions and prototypes: `Count add(Count x, Byte y);`, `Point make(Count x);`
  - function parameters and returns, including struct-by-value semantics already implemented for the underlying struct type
  - struct field declarations for scalar aliases such as `Count x;`
  - `sizeof(alias)`, `sizeof(const alias)`, `sizeof(alias *)`, and `sizeof(IntPtr)`

## Runtime model

Typedef aliases are resolved during parsing into existing Cust type metadata:

- scalar aliases resolve to `CType::Int` or `CType::Char`;
- enum aliases resolve to `CType::Int` and use existing enum constant runtime storage;
- struct aliases resolve to the underlying struct type name;
- pointer aliases and pointer declarations through aliases still use existing `PointeeType` values;
- no alias information is stored in runtime scopes or values.

This means aliases do not create distinct types: `Count` behaves exactly like `int`, and `Point` behaves exactly like `struct Point`.

## Unsupported forms for this milestone

- Pointer-to-pointer aliases such as `typedef int **IntPtrPtr;` and `typedef IntPtr *IntPtrPtr;` are intentionally rejected with `pointer-to-pointer typedef aliases are not supported`.
- Pointer-returning functions remain unsupported even when the return type is spelled through a pointer typedef alias.
- Anonymous struct typedefs such as `typedef struct { int x; } Point;` are not supported.
- Anonymous enum typedefs such as `typedef enum { READY } Status;` and enum-typed variables without a typedef alias are not supported.
- Function pointer typedefs, array typedefs, declaration-level type qualifiers (`const` outside the already supported `sizeof(const alias)` type context), and aggregate/nested struct fields remain future work.

## Acceptance coverage

- `tests/fixtures/valid/typedef_aliases.c` covers scalar and struct aliases in globals, locals, arrays, pointer declarations, prototypes, function parameters/returns, `sizeof(alias)`, and struct copy/field access.
- `tests/fixtures/valid/block_scoped_typedefs.c` covers nested block aliases, alias shadowing between `int`/`char`/`struct` aliases, scoped `sizeof(alias)`, and restoration of the outer alias after leaving a block.
- `tests/fixtures/invalid/typedef_missing_alias_name.c` covers the exact missing-alias diagnostic.
- `tests/fixtures/invalid/block_typedef_alias_out_of_scope.c` covers block-local alias expiry after scope exit.
- `tests/fixtures/valid/pointer_typedef_aliases.c` covers `int *`, `char *`, and `struct Point *` aliases in declarations, parameters, function calls, struct-pointer field access, and `sizeof(pointer_alias)`.
- `tests/fixtures/invalid/pointer_typedef_to_pointer.c` and `tests/fixtures/invalid/direct_pointer_to_pointer_typedef.c` cover pointer-to-pointer typedef alias rejection.
- `tests/fixtures/valid/enum_typedef_aliases.c` covers named enum typedef aliases in globals, locals, arrays, parameters, returns, block shadowing, and `sizeof(alias)` as Cust's integer size.
- `tests/fixtures/valid/sizeof_const_types.c` covers `sizeof(const alias)` for scalar, struct, and pointer typedef aliases plus const-qualified built-in scalar and pointer type spellings.
- `tests/fixtures/invalid/typedef_unknown_enum.c` covers the exact undefined enum-tag diagnostic for `typedef enum Missing Missing;`.
- `tests/fixtures/invalid/block_enum_tag_out_of_scope.c` covers enum tag scope expiry before typedef alias resolution.
- `tests/fixtures/compat/valid/typedef_aliases.c` verifies a native C compiler oracle for alias use while avoiding Cust-vs-native ABI `sizeof(int)`/struct-padding differences.
- `tests/fixtures/compat/valid/block_scoped_typedefs.c` verifies block-scoped alias shadowing against a native C compiler oracle without ABI-sensitive struct-size assertions.
- `tests/fixtures/compat/valid/pointer_typedef_aliases.c` verifies pointer alias declarations and calls against a native C compiler oracle while avoiding ABI-sensitive pointer-size assertions.
- `tests/fixtures/compat/valid/sizeof_const_types.c` verifies const-qualified `sizeof` type contexts against a native C compiler oracle using only same-ABI relative comparisons.
- `tests/fixtures/compat/valid/enum_typedef_aliases.c` verifies named enum typedef declarations and calls against a native C compiler oracle while avoiding ABI-sensitive enum-size assertions.

## Follow-up candidates

1. Add declaration-level const-qualified aliases/pointers only after a separate const-pointer design, because `sizeof(const alias)` is parser-only and does not create runtime qualifier metadata.
2. Add precise diagnostics for pointer-returning functions spelled with pointer typedef aliases if a user-facing fixture discovers an unclear location.
3. Consider anonymous enum typedef declarations only if Cust later needs enum tags/types beyond integer aliases.
