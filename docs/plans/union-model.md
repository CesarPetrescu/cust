# Cust Union Model

Cust's first-pass `union` support is deliberately scoped to safe, interpreter-owned storage instead of native host addresses or ABI layout.

## Supported syntax

- Named top-level union type declarations: `union Number { int value; char tag; };`
- Top-level, local, static-local, by-value parameter, return-value, and one-dimensional array variables: `union Number n;`, `union Number n = {5};`, `union Number n = make_number(5);`, `union Number make_number(int value);`, `union Number values[2] = {{1}, {2}};`
- One-level union pointers and pointer fields inside unions: `union Number *p = &n;`, `union Number *p = &values[1];`, `p->value`, `(*p).tag`, `p += 1` for union-array element pointers, `union Node { int value; union Node *next; };`, and scalar pointer fields such as `union External { int *external; };`
- Scalar member reads/writes using the existing aggregate postfix syntax: `n.value`, `n.tag = 2`, `n.value += 1`, `++n.tag`, `values[i].value = 3`, and the same scalar field lvalues through union pointers (`p->value += 1`)
- Nested union fields inside supported structs: `struct Holder { union Number number; };`, with recursive initializers such as `struct Holder h = {{3}};` and field paths such as `h.number.tag`
- `sizeof(union_variable)`, `sizeof(union Name)`, `sizeof(union_array[i])`, and `sizeof(union_variable.field)` with Cust's deterministic type sizes (`int=8`, `char=1`)

## Storage semantics

- Union type definitions reuse Cust's aggregate field metadata, but `sizeof(union Name)` is the maximum field size rather than the sum of fields.
- Scalar union fields share one logical storage slot in the interpreter. Initializing or assigning one scalar field updates all scalar views for root union variables, union array elements, union pointer targets, and nested union fields inside structs, which preserves the C-compatible behavior needed for supported integer/character scalar unions without exposing native representation bytes.
- Union pointer values reuse Cust's interpreter-owned struct/aggregate pointer targets: root variables, union-array elements, and self-referential one-level union pointer fields can be addressed without host addresses. Pointer fields inside unions store ordinary Cust pointer metadata, pointer arithmetic over union-array element pointers reuses the same bounds-checked struct-array element pointer target model, and pointer-to-pointer union fields remain rejected.
- Uninitialized union variables default all supported fields to zero.
- Brace initialization accepts at most one positional initializer, matching C's first-field default. Excess initializers report `too many initializers for union '<Name>'`.

## Intentional exclusions for this milestone

- Native ABI byte layout and padding are not modeled.
- Type-punning through object representation is intentionally not exposed; scalar views share the same Cust integer value rather than host bytes.
- Pointer arithmetic on standalone union variables, arrays of pointer fields, and pointer-to-pointer union fields remain unsupported unless a later milestone extends the safe pointer model deliberately.

## Acceptance fixtures

- `tests/fixtures/valid/unions.c` verifies scalar field initialization, shared scalar field writes, and deterministic `sizeof`; `tests/fixtures/valid/sizeof_aggregate_types.c` verifies direct `sizeof(struct Name)` / `sizeof(union Name)` type spellings, const aggregate type spelling, aggregate pointer type spelling, and union typedef spelling.
- `tests/fixtures/valid/nested_union_fields.c` verifies nested union fields inside structs, union arrays, by-value union copies/parameters, recursive initializer synchronization, and deterministic nested `sizeof`.
- `tests/fixtures/valid/union_pointers.c` verifies union pointers to root variables and union-array elements, scalar writes through `->`/`(*p).field`, function pointer parameters, self-referential union pointer fields, scalar pointer fields inside unions, and scalar synchronization through pointer-targeted writes.
- `tests/fixtures/valid/aggregate_pointer_arithmetic.c` verifies bounds-checked pointer arithmetic and pointer difference over struct-array and union-array element pointers.
- `tests/fixtures/valid/union_return_functions.c` verifies union-returning prototypes/definitions, by-value union returns from local variables, union-copy assignment from return calls, by-value union parameter isolation, and supported scalar field reads after returned copies.
- `tests/fixtures/valid/aggregate_initializer_expressions.c` verifies same-type struct and union declarations initialized directly from aggregate-returning call expressions, including `const struct` initialization and type-mismatch diagnostics.
- `tests/fixtures/invalid/union_initializer_too_long.c` verifies targeted excess-initializer diagnostics.
- `tests/fixtures/invalid/union_pointer_to_pointer_field.c` verifies targeted rejection of pointer-to-pointer union fields.
- `tests/fixtures/compat/valid/unions.c`, `tests/fixtures/compat/valid/sizeof_aggregate_types.c`, `tests/fixtures/compat/valid/nested_union_fields.c`, `tests/fixtures/compat/valid/union_pointers.c`, `tests/fixtures/compat/valid/aggregate_pointer_arithmetic.c`, `tests/fixtures/compat/valid/union_return_functions.c`, and `tests/fixtures/compat/valid/aggregate_initializer_expressions.c` compare supported scalar union behavior against a native C compiler as an external oracle.
