# Cust Struct Model

This document defines Cust's deliberately scoped, preprocessor-free `struct` roadmap.

## Supported first milestone

- Top-level struct type declarations with scalar fields only:
  - `struct Point { int x; char y; };`
  - Fields may be `int` or `char`.
  - Duplicate field names are rejected during parsing.
  - Re-declaring a struct type name is rejected.
- Struct variables at top level and block scope:
  - `struct Point p;`
  - Fields are deterministic Cust values initialized to `0`.
  - Normal block/global scope rules apply; inner variables may shadow outer variables.
- Member access and member assignment:
  - `p.x` reads a scalar field.
  - `p.x = expr;` assigns a scalar field.
  - Unknown fields report `struct '<Type>' has no field '<field>'`.
  - `sizeof(p)` sums Cust field sizes (`int = 8`, `char = 1`) without native ABI padding.
  - `sizeof(p.x)` uses the declared field type size.

## Intentional limitations before later milestones

- No struct-valued assignment/copy yet (`a = b;` is unsupported for structs).
- No struct function parameters or return types yet.
- No pointers to structs and no address-of/member pointer access yet.
- No nested structs, arrays in structs, pointer fields, bit-fields, anonymous structs, unions, typedefs, or `const`.
- No native ABI layout or padding; Cust keeps interpreter-owned field maps and deterministic sizes.

## Implementation model

- Parser records top-level struct type definitions in `Program::struct_types`.
- Runtime struct variables are `Value::Struct { type_name, fields }`, where fields store scalar values plus declared `CType`.
- Struct fields are scoped as members of their owning value, not as independent variables.
- Member access is scalar expression syntax; full lvalue expression support for fields can be added later if compound assignments/increment on fields are selected.

## Acceptance fixtures

- Valid interpreter fixture: `tests/fixtures/valid/structs.c`
  - declares two struct types;
  - creates local struct variables;
  - assigns/reads `int` and `char` fields;
  - verifies block-scope shadowing.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/structs.c`
  - uses only C-compatible scalar fields and member reads/writes;
  - compares Cust exit code with native C.
- Invalid fixture: `tests/fixtures/invalid/unknown_struct_field.c`
  - reads/writes a missing field and expects the targeted field diagnostic.

## Next struct work

1. Add struct-valued copy assignment (`a = b;`) for same-type structs, with a type-mismatch diagnostic.
2. Add field lvalue expression support (`p.x += 1`, `++p.x`, `return p.x = 3;`).
3. Design and implement struct function parameters using by-value copies before considering pointers to structs.
