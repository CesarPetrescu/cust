# Cust Union Model

Cust's first-pass `union` support is deliberately scoped to safe, interpreter-owned storage instead of native host addresses or ABI layout.

## Supported syntax

- Named top-level union type declarations: `union Number { int value; char tag; };`
- Top-level, local, static-local, by-value parameter, return-value, and one-dimensional array variables: `union Number n;`, `union Number n = {5};`, `union Number values[2] = {{1}, {2}};`
- Scalar member reads/writes using the existing aggregate postfix syntax: `n.value`, `n.tag = 2`, `n.value += 1`, `++n.tag`, and `values[i].value = 3`
- Nested union fields inside supported structs: `struct Holder { union Number number; };`, with recursive initializers such as `struct Holder h = {{3}};` and field paths such as `h.number.tag`
- `sizeof(union_variable)`, `sizeof(union_array[i])`, and `sizeof(union_variable.field)` with Cust's deterministic type sizes (`int=8`, `char=1`)

## Storage semantics

- Union type definitions reuse Cust's aggregate field metadata, but `sizeof(union Name)` is the maximum field size rather than the sum of fields.
- Scalar union fields share one logical storage slot in the interpreter. Initializing or assigning one scalar field updates all scalar views for root union variables, union array elements, and nested union fields inside structs, which preserves the C-compatible behavior needed for supported integer/character scalar unions without exposing native representation bytes.
- Uninitialized union variables default all supported fields to zero.
- Brace initialization accepts at most one positional initializer, matching C's first-field default. Excess initializers report `too many initializers for union '<Name>'`.

## Intentional exclusions for this milestone

- Native ABI byte layout and padding are not modeled.
- Union pointers, pointer fields inside unions, and pointers to unions are deferred until explicitly designed.
- Type-punning through object representation is intentionally not exposed; scalar views share the same Cust integer value rather than host bytes.

## Acceptance fixtures

- `tests/fixtures/valid/unions.c` verifies scalar field initialization, shared scalar field writes, and deterministic `sizeof`.
- `tests/fixtures/valid/nested_union_fields.c` verifies nested union fields inside structs, union arrays, by-value union copies/parameters, recursive initializer synchronization, and deterministic nested `sizeof`.
- `tests/fixtures/invalid/union_initializer_too_long.c` verifies targeted excess-initializer diagnostics.
- `tests/fixtures/compat/valid/unions.c` and `tests/fixtures/compat/valid/nested_union_fields.c` compare supported scalar union behavior against a native C compiler as an external oracle.
