# Cust Struct Model

This document defines Cust's deliberately scoped, preprocessor-free `struct` roadmap.

## Supported milestones

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
- Same-type struct copy assignment:
  - `b = a;` copies field values from one same-type struct variable to another.
  - The copy is value semantics: later writes to `a.x` do not mutate `b.x`.
  - Mismatched struct types report `cannot assign struct '<Rhs>' to struct '<Lhs>'`.
- Field lvalue expressions:
  - `p.x = expr` is valid as an expression and returns the assigned scalar value.
  - `p.x += expr` and the other supported compound assignments read, update, store, and return the field value.
  - Prefix/postfix `++p.x`, `p.x++`, `--p.x`, and `p.x--` work with the same return-value semantics as scalar variables.

## Intentional limitations before later milestones

- No struct function parameters or return types yet.
- No pointers to structs and no address-of/member pointer access yet.
- No nested structs, arrays in structs, pointer fields, bit-fields, anonymous structs, unions, typedefs, or `const`.
- No native ABI layout or padding; Cust keeps interpreter-owned field maps and deterministic sizes.

## Implementation model

- Parser records top-level struct type definitions in `Program::struct_types`.
- Runtime struct variables are `Value::Struct { type_name, fields }`, where fields store scalar values plus declared `CType`.
- Struct fields are scoped as members of their owning value, not as independent variables.
- Member access is scalar expression syntax backed by field lvalue evaluation helpers for simple assignment, compound assignment, and increment/decrement expressions.

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
- Valid interpreter fixture: `tests/fixtures/valid/struct_lvalues_and_copy.c`
  - copies same-type structs by value;
  - covers field assignment expressions, compound assignments, and prefix/postfix increment.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/struct_lvalues_and_copy.c`
  - compares supported same-type copy and field lvalue behavior with native C.
- Invalid fixture: `tests/fixtures/invalid/struct_assignment_type_mismatch.c`
  - verifies mismatched struct copy assignment reports the targeted type diagnostic.

## Next struct work

1. Design and implement struct function parameters using by-value copies before considering pointers to structs.
2. Add struct return types only after by-value parameter semantics are verified.
3. Consider pointers to structs and `->` as a separate pointer-model extension.
