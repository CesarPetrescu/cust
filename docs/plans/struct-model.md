# Cust Struct Model

This document defines Cust's deliberately scoped, preprocessor-free `struct` roadmap.

## Supported milestones

- Top-level struct type declarations with scalar, array, and nested struct fields:
  - `struct Point { int x; char y; };`
  - `struct Packet { int values[3]; char tag[2]; };`
  - `struct Rect { struct Point origin; int width; };` after `Point` is declared.
  - Fields may be `int`, `char`, one-dimensional `int`/`char` arrays, `const int`, `const char`, or a prior named struct type.
  - Duplicate field names are rejected during parsing.
  - Re-declaring a struct type name is rejected.
- Struct variables at top level and block scope:
  - `struct Point p;`
  - Fields are deterministic Cust values initialized to `0`.
  - Scalar brace initializers such as `struct Point p = {1, 2};` initialize fields in declaration order, evaluate initializer expressions left-to-right, zero-fill omitted trailing fields, and reject excess initializers.
  - Array field brace initializers such as `struct Packet p = {{1, 2}, {'A', 0}};` initialize elements left-to-right, zero-fill omitted trailing elements, and reject excess entries with the array field name.
  - Nested struct brace initializers such as `struct Rect r = {{1, 2}, 3};` recursively initialize prior named struct fields, including zero-filled omitted nested fields and excess-initializer diagnostics at the nested struct type.
  - Const fields can be initialized in a struct initializer but remain read-only after declaration.
  - Normal block/global scope rules apply; inner variables may shadow outer variables.
- Member access and member assignment:
  - `p.x` reads a scalar field.
  - `packet.values[0]` reads a scalar element of an array field.
  - `rect.origin.x` reads nested scalar fields through a struct-valued field path.
  - `p.x = expr;` and `rect.origin.x = expr;` assign scalar fields.
  - `packet.values[0] = expr;`, compound assignments, and prefix/postfix increment/decrement work for array-field elements.
  - Unknown fields report `struct '<Type>' has no field '<field>'` at the innermost type where lookup fails.
  - `sizeof(p)` sums Cust field sizes recursively (`int = 8`, `char = 1`) without native ABI padding.
  - `sizeof(p.x)` uses the declared field type size; `sizeof(packet.values)` uses array length times element size; `sizeof(packet.values[0])` uses the element size; `sizeof(rect.origin)` uses the deterministic Cust size of the nested struct field.
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
- Const-qualified scalar fields inside struct definitions:
  - `struct Config { const int magic; const char marker; int value; };` records field-level read-only metadata while preserving deterministic zero initialization and Cust field sizes.
  - Reads of const fields use ordinary `p.field` / `ptr->field` syntax.
  - Direct field writes, field assignment expressions, field compound assignments, and field increment/decrement against const fields report `cannot assign to const struct field '<field>'`.
  - Whole-struct copy assignment into a struct type containing const fields reports `cannot assign to struct '<Type>' with const fields`.

## Intentional limitations before later milestones

- No pointer fields, arrays of struct types, bit-fields, anonymous structs, unions, or const-qualified nested struct fields inside struct definitions.
- No native ABI layout or padding; Cust keeps interpreter-owned field maps and deterministic recursive sizes.

## Implementation model

- Parser records top-level struct type definitions in `Program::struct_types`.
- Runtime struct variables are `Value::Struct { type_name, fields }`, where fields store scalar values plus declared `CType`/field-level const metadata, one-dimensional array storage with declared element type/length, or nested struct field maps with the nested type name.
- Struct initializers are parsed as field-aware entries: scalar fields consume assignment-precedence expressions, array fields consume brace lists of assignment-precedence expressions, and nested struct fields may consume recursive brace initializer lists or same-type struct expressions. Entries are applied in declaration order before const field metadata prevents later writes; omitted array elements and nested fields are zero-initialized.
- Struct fields are scoped as members of their owning value, not as independent variables.
- Member access is scalar expression syntax backed by field-path lvalue evaluation helpers for simple assignment, compound assignment, and increment/decrement expressions; array fields are indexed before element lvalue evaluation.
- Function signatures include struct parameter type names, so prototypes and later definitions must agree on the exact struct type.
- Struct parameter binding deep-clones the struct value into the function parameter scope, preserving by-value behavior for scalar, array, and nested struct fields without host/native addresses.
- Function signatures also include struct return type names, so prototypes and definitions must agree on the exact return struct type.
- Return flow carries either scalar values or deep-cloned struct field maps; callers receive by-value struct results without borrowing callee stack storage.
- Struct pointers extend the existing interpreter-owned pointer model with `PointerValue::Struct { scope_id, name }`, never host addresses.
- Struct pointer dereference checks live scope IDs before field access, preserving the same out-of-scope safety used by scalar pointers.
- Const struct variables/parameters reuse scope `const_variables` metadata; struct field writes and copy assignment check the owning struct binding before mutation.
- Const struct pointer declarations/parameters use the existing `points_to_const` pointer metadata, and direct pointer writes also check whether the referenced struct target binding is const.
- Const struct fields are stored on each field value so cloned by-value parameters/returns preserve field-level read-only semantics.
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
- Valid interpreter fixture: `tests/fixtures/valid/const_struct_fields.c`
  - covers `const int`/`const char` fields, ordinary reads, writes to mutable sibling fields, by-value parameter clones, and pointer reads.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/const_struct_fields.c`
  - compares stable global zero-initialized const-field reads and mutable sibling writes against native C without relying on native struct layout.
- Invalid fixtures: `tests/fixtures/invalid/const_struct_member_assignment.c`, `tests/fixtures/invalid/const_struct_member_pointer_write.c`, and `tests/fixtures/invalid/const_struct_member_copy_assignment.c`
  - verify field-level const diagnostics for direct writes, pointer writes, and whole-struct copy assignment into a struct with const fields.

- Valid interpreter fixture: `tests/fixtures/valid/struct_initializers.c`
  - covers top-level/local/static-local/const struct brace initializers, expression entries, declaration-order field assignment, zero-filled omitted trailing fields, and initialized const fields.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/struct_initializers.c`
  - compares the supported scalar struct initializer subset with native C while avoiding ABI-sensitive layout checks and `-Wmissing-field-initializers` warnings.
- Invalid fixture: `tests/fixtures/invalid/struct_initializer_too_long.c`
  - verifies excess initializer entries report `too many initializers for struct '<Type>'`.

- Valid interpreter fixture: `tests/fixtures/valid/nested_struct_fields.c`
  - covers nested struct fields, `rect.origin.x` field-path reads/writes, passing `rect.origin` as a by-value struct argument, copying a nested struct field into a same-type variable, and deterministic Cust `sizeof(rect.origin)`.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/nested_struct_fields.c`
  - compares nested struct field-path reads/writes, by-value nested field arguments, and nested-field copy assignment with native C while avoiding native `sizeof(struct)` layout checks.
- Invalid fixture: `tests/fixtures/invalid/nested_struct_unknown_field.c`
  - verifies missing nested fields report the targeted innermost struct type diagnostic.
- Valid interpreter fixture: `tests/fixtures/valid/nested_struct_initializers.c`
  - covers recursive brace initialization of nested struct fields in global, local, static, and const declarations, including omitted nested-field zero-fill and deterministic `sizeof`.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/nested_struct_initializers.c`
  - compares fully spelled nested aggregate initializers with native C while avoiding native struct layout checks.
- Invalid fixture: `tests/fixtures/invalid/nested_struct_initializer_too_long.c`
  - verifies excess initializer entries inside a nested struct initializer report the nested struct type diagnostic.
- Valid interpreter fixture: `tests/fixtures/valid/struct_array_fields.c`
  - covers one-dimensional `int`/`char` array fields, nested array brace initializers, omitted-element zero-fill, element assignment/compound/increment lvalues, by-value struct parameter/copy isolation with array fields, and deterministic Cust `sizeof(array_field)`.
- Valid compiler-oracle fixture: `tests/fixtures/compat/valid/struct_array_fields.c`
  - compares supported struct array-field reads/writes, copy assignment, and by-value parameter behavior with native C while avoiding Cust-specific `sizeof(int)` assertions.
- Invalid fixture: `tests/fixtures/invalid/struct_array_initializer_too_long.c`
  - verifies excess entries inside an array field initializer report `too many initializers for array '<field>'`.

## Next struct work

1. Consider pointer fields as a separate milestone with explicit ownership/const/pointer-target design; do not rely on native ABI padding.
