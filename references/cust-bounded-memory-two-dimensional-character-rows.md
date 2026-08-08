# Bounded raw-memory intrinsics over two-dimensional character rows

Date: 2026-08-08

## Scope

Cust's exactly prototyped `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` accept scalar `char *` views produced by selecting one row of an existing two-dimensional `char` array. Supported roots include direct arrays, adjusted 2D parameters, explicit pointer-to-row expressions, and existing aggregate-field row views. This does not flatten multidimensional storage or define host-layout bytes.

## Runtime model

- `PointerValue::Array2DRow` remains the pointer-to-row representation.
- Dereferencing or indexing a row pointer yields `PointerValue::ArrayElement` with retained `(rows, columns)` metadata and a flattened backing index.
- Raw-memory validation accepts that scalar row-element representation, but still rejects `ArrayBase`/`Array2DRow` because those designate arrays of rows rather than character cells.
- Reads and destination preflight use `character_sequence_array_end()`, so `(flat_index % columns)` determines the selected row's remaining capacity. A count may not cross into the next row even though storage is backed by one flattened `ArrayValue`.
- Existing pointer identity and overlap comparisons remain valid after row-local capacity preflight because independently selected rows have disjoint flattened ranges.

## Direct row-expression routing

Single-index expressions such as `matrix[i]`, adjusted `matrix[i]`, `holder.rows[i]`, and `holder_ptr->rows[i]` are scalar `char *` values in C. Route these through metadata-first 2D row classification, evaluate the row index once, offset the row pointer with checked arithmetic, then lower the selected row start to `ArrayElement`. Keep pointer-expression, pointee-type, const, and runtime evaluators in parity.

Use checked row-offset arithmetic. A very large index must report `two-dimensional array row pointer index overflow during pointer arithmetic`, never panic in host Rust.

## Const propagation pitfall

Nested aggregate-array element paths need every const ancestor, not only the root object or final field. For `outer.leaves[0].nested.rows[0]`, where `nested` is a const aggregate field, raw-memory destinations must reject qualification discard. Walk the aggregate type path to detect any const ancestor after the embedded aggregate-array element. This also applies to other pointer conversions that use `pointer_expr_points_to_const()`.

## Verification shape

- Focused interpreter success: all five intrinsics, direct/adjusted/pointer-to-row/aggregate roots, offsets, overlap, identity, zero count, and non-evaluating `sizeof`.
- Focused diagnostics: source/destination row capacity, const root, nested const ancestor, escaped local-row lifetime, and huge row-index overflow.
- Registered compiler oracle: warning-free C11 fixture with ABI-independent byte/value and pointer-identity checks.
- Run the actual `c_compat` harness; fixture-name filtering executes zero tests and is invalid evidence.
