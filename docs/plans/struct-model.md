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
- Struct function parameters:
  - Function definitions and prototypes may name by-value struct parameters after a prior struct declaration, such as `int sum(struct Point p);`.
  - Calls copy same-type struct arguments into the callee parameter scope by value.
  - Writes to fields on the callee's copy do not mutate the caller's struct variable.
  - Mismatched struct arguments report `function '<name>' struct parameter '<param>' expected struct '<Expected>', got struct '<Actual>'`.
  - Non-struct arguments report `function '<name>' struct parameter '<param>' requires a struct argument`.
- Struct return types:
  - Function definitions and prototypes may return a prior struct type, such as `struct Point make_point(int x);`.
  - `return p;` clones the returned struct value so local return variables remain valid after the callee exits.
  - Struct-returning calls can be assigned to same-type struct variables, e.g. `p = make_point(1);`.
  - Mismatched struct return values report `struct function '<name>' expected return struct '<Expected>', got struct '<Actual>'`.
  - Empty returns from struct functions report `struct function '<name>' returned without a value`.
- Struct pointers:
  - Local/global declarations and function parameters may use one pointer level to a prior struct type, e.g. `struct Point *p = &point;`, `struct Point *p;`, `struct Point * const stable = &point;`, `const struct Point *view = &point;`, and `void set(struct Point *p);`.
  - `&point` produces an interpreter-owned pointer to the struct variable; null struct pointers use the existing `0` literal.
  - `p->x` and `(*p).x` read scalar fields through a struct pointer.
  - Struct pointer field lvalue expressions support simple assignment, compound assignment, and prefix/postfix increment/decrement for scalar fields when the pointer target is mutable.
  - `const struct Point *p` and direct pointers to `const struct Point` variables are read-only views: writes through `p->field` / `(*p).field`, field compound assignment, and field increment/decrement report `cannot assign through pointer to const`.
  - Null struct pointer field access reports `null pointer dereference`; pointers to ended block/function scopes report `pointer to out-of-scope variable '<name>'`.
- Const-qualified struct variables and by-value parameters:
  - `const struct Point p;` / `const Point p;` after a struct typedef create zero-initialized read-only struct variables.
  - `int f(const struct Point p)` copies the argument by value into a read-only parameter binding.
  - Direct struct copy assignment and field writes to const struct bindings report `cannot assign to const variable '<name>'`.

## Intentional limitations before later milestones

- No nested structs, arrays in structs, pointer fields, bit-fields, anonymous structs, unions, or const-qualified fields inside struct definitions.
- No native ABI layout or padding; Cust keeps interpreter-owned field maps and deterministic sizes.

## Implementation model

- Parser records top-level struct type definitions in `Program::struct_types`.
- Runtime struct variables are `Value::Struct { type_name, fields }`, where fields store scalar values plus declared `CType`.
- Struct fields are scoped as members of their owning value, not as independent variables.
- Member access is scalar expression syntax backed by field lvalue evaluation helpers for simple assignment, compound assignment, and increment/decrement expressions.
- Function signatures include struct parameter type names, so prototypes and later definitions must agree on the exact struct type.
- Struct parameter binding clones the struct value into the function parameter scope, preserving by-value behavior without host/native addresses.
- Function signatures also include struct return type names, so prototypes and definitions must agree on the exact return struct type.
- Return flow carries either scalar values or cloned struct field maps; callers receive by-value struct results without borrowing callee stack storage.
- Struct pointers extend the existing interpreter-owned pointer model with `PointerValue::Struct { scope_id, name }`, never host addresses.
- Struct pointer dereference checks live scope IDs before field access, preserving the same out-of-scope safety used by scalar pointers.
- Const struct variables/parameters reuse scope `const_variables` metadata; struct field writes and copy assignment check the owning struct binding before mutation.
- Const struct pointer declarations/parameters use the existing `points_to_const` pointer metadata, and direct pointer writes also check whether the referenced struct target binding is const.
- `->` parses as postfix pointer field access, and `(*p).x` is represented as field access through a dereferenced pointer expression.

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
- Valid interpreter fixture: `tests/fixtures/valid/struct_parameters.c`
  - covers struct parameters in definitions/prototypes, by-value copy semantics, and mixed scalar pointer output parameters.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/struct_parameters.c`
  - compares supported by-value struct parameter behavior with native C.
- Invalid fixtures: `tests/fixtures/invalid/struct_parameter_type_mismatch.c` and `tests/fixtures/invalid/struct_parameter_non_struct_argument.c`
  - verify targeted diagnostics for mismatched and non-struct arguments.
- Valid interpreter fixture: `tests/fixtures/valid/struct_return_functions.c`
  - covers struct-returning definitions/prototypes, returning local structs by value, assigning call results to struct variables, preserving by-value mutation isolation, and Cust deterministic `sizeof` for struct-returning calls.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/struct_return_functions.c`
  - compares supported struct return behavior with native C without relying on native ABI struct layout.
- Invalid fixtures: `tests/fixtures/invalid/struct_return_type_mismatch.c` and `tests/fixtures/invalid/struct_function_empty_return.c`
  - verify targeted diagnostics for mismatched and empty struct returns.
- Valid interpreter fixture: `tests/fixtures/valid/struct_pointers.c`
  - covers `struct Point *` declarations, function parameters/prototypes, `&point`, `p->x`, `(*p).x`, pointer equality with `&point`, field assignment, compound assignment, and increment through pointers.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/struct_pointers.c`
  - compares the supported struct pointer subset with native C without relying on native struct layout.
- Invalid fixtures: `tests/fixtures/invalid/struct_pointer_null_dereference.c` and `tests/fixtures/invalid/struct_pointer_out_of_scope.c`
  - verify null and ended-scope struct pointer diagnostics.
- Valid interpreter fixture: `tests/fixtures/valid/const_struct_qualifiers.c`
  - covers const struct variables, const by-value struct parameters, const struct pointer read-only views, and const struct pointer slots.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/const_struct_qualifiers.c`
  - compares supported const struct pointer reads and const-preserving parameter/declaration behavior with native C while avoiding ABI-sensitive layout checks.
- Invalid fixtures: `tests/fixtures/invalid/const_struct_field_assignment.c`, `tests/fixtures/invalid/const_struct_pointer_write.c`, and `tests/fixtures/invalid/const_struct_pointer_discard.c`
  - verify direct const struct field-write rejection, write-through-const-struct-pointer rejection, and const-discard struct pointer conversion diagnostics.

## Next struct work

1. Keep nested/aggregate fields and const-qualified struct fields as later separate milestones.
