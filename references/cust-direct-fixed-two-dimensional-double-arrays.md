# Direct fixed two-dimensional `double` arrays

## Scope

Cust supports direct fixed `double[R][C]` objects at local, file-global, and block-static scope. They reuse `Value::Array` with `dimensions: Some((rows, columns))`; no new host-backed storage or native compiler runtime path is involved.

Supported behavior includes:

- nested positional initialization with zero fill;
- double-index reads, assignment expressions/statements, compound assignment, and prefix/postfix increment/decrement;
- one-time row and column index evaluation;
- source-order declaration lists;
- deterministic full-object, row, element, `sizeof(type-name)`, and `_Alignof` relationships;
- scalar `double` expression metadata in evaluated and non-evaluating contexts.

## Exact retained boundaries

Do not generalize this slice into arbitrary pointer-to-row support. Direct double pointer-to-row declarations/parameters/returns, row-pointer arithmetic/indexing, row addresses such as `&values[0]`, aggregate fields, array compound literals, three-dimensional arrays, and raw-memory access to double object storage remain unsupported.

`expr_is_unsupported_double_pointer()` must distinguish:

- the whole 2D array expression, which decays to an unsupported pointer to a row;
- a row address, represented as `Expr::AddressOfArray` over a dimensioned double array;
- a fully indexed `Expr::Array2DGet`, which is a valid scalar double value.

Do not classify every binary expression containing `Array2DGet` as a pointer. A valid expression such as `sizeof(values[0][0] + 1.0)` must remain scalar.

## Parser and runtime fan-out

The direct slice requires parity across:

- `Stmt::Array2DDecl` parsing, including comma-separated declarator tails;
- `Value::Array` element type and dimensions;
- `ArrayValue::mutable_zeroed_2d()` allocation;
- `Expr::Array2DGet` scalar evaluation and `expr_is_double_value()`;
- 2D assignment, compound-assignment, and increment/decrement routes;
- `sizeof_variable()`, `sizeof_expr()`, and `SizeOfType::Array2D`;
- `_Generic` scalar classification;
- unsupported double row-pointer classification.

Const checks must also run beneath non-evaluating `sizeof`. In the `Expr::Increment` size path, call `ensure_two_dimensional_array_mutable()` for `Array2DGet` targets before returning the element size.

## Resource and panic safety

Both dimensions are source-controlled integer constant expressions.

- Fold additive, multiplicative, unary negation, and shift-count operations with checked arithmetic and source-located overflow diagnostics.
- Check `rows * columns` before allocation.
- Use fallible `Vec::try_reserve_exact()` before resizing 2D storage so capacity overflow returns `two-dimensional array storage is too large` rather than panicking.
- Keep `SizeOfType::Array2D` multiplication checked and report `array size overflow`.

A zero first dimension is not a valid escape hatch for reaching huge row-size arithmetic: Cust rejects non-positive array lengths during parsing.

## Tests and oracle guidance

Focused interpreter coverage should include:

- local/global/static initialization and zero fill;
- scalar reads/updates and one-time indexes;
- declaration lists;
- non-evaluating scalar expressions;
- const, row/column bounds, incomplete rank, pointer-to-row, parameter, field, compound-literal, raw-memory, and resource-overflow boundaries;
- `catch_unwind` around huge dimensions to prove diagnostics replace host panics.

Register a warning-free fixture in `tests/c_compat.rs`. Keep oracle assertions ABI-independent: compare `sizeof(array)` with element counts times `sizeof(double)`, row size with columns times `sizeof(double)`, and `_Alignof(double[R][C])` with `_Alignof(double)`. GCC and Clang are test oracles only.

## Verification

Run the exact focused functions or the shared `direct_fixed_two_dimensional_double` substring, then the actual `c_compat` test function. Independent review should specifically probe row-pointer arithmetic, `sizeof(&values[0])`, const increments beneath `sizeof`, checked constant folding, and fallible large-dimension allocation before the canonical gate.
