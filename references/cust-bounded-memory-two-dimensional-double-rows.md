# Bounded-memory two-dimensional `double` row views

Completed on 2026-08-30.

## Scope

Cust admits one selected row of supported fixed two-dimensional `double[R][C]` storage to the exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` intrinsics. Supported roots include direct row decay, explicit row-pointer indexing/dereference, adjusted direct and typedef-backed parameters, and supported aggregate-field rows. Operations remain row-local and use Cust's deterministic eight-byte little-endian binary64 representation; adjacent rows are never flattened into the byte range.

## Implementation checklist

1. Treat `Array2D`/`Array2DPointer` double declarations as row-pointer types without classifying ordinary row decay or pointer arithmetic as the unsupported direct `double *` boundary.
2. Preserve dimensions, owner, lexical lifetime, and pointee qualification through `PointerValue::ArrayBase`/`ArrayElement`; selected row byte views use the row index and column count.
3. In bounded-memory target resolution, compute the selected row start with checked multiplication and preserve aggregate-field provenance before widening to byte storage.
4. Keep byte capacity at `columns * sizeof(double)`. Cross-row reads/writes must report the existing row-local capacity diagnostic even when contiguous backing storage exists.
5. Validate arithmetic twice: row selection against the outer row count and element/address selection against the selected row width. `checked_pointer_value_index()` must enforce the second check before bounded-memory conversion; otherwise `&row[columns]` can expose an adjacent row.
6. Separate array-parameter pointer-slot const from element const. Leading `const double rows[][C]` sets `points_to_const` but leaves the adjusted pointer slot assignable; `double rows[const R][C]` sets the adjusted pointer slot const. Runtime `const_params` must use `Param::is_const`, not `points_to_const`.
7. Keep `&row`, `&matrix[i]`, and equivalent aggregate-row addresses outside the one-level safe pointer subset. Aggregate compound-literal subscript address lowering normally erases `&` to pointer addition; preserve an address marker only when parser metadata proves that the selected literal field is two-dimensional, while retaining existing one-dimensional aggregate-literal array-field address behavior.
8. Non-evaluating `sizeof(call)` validation must classify the same supported roots and exact union/whole-aggregate/unprovable boundaries without executing indexes, initializers, or intrinsic calls.
9. Runtime alias scans for a later operand in a composed expression may encounter an already evaluated `PointerValue::Array2DRow`. Classify that row as double-backed storage by checking owner liveness and the backing element type directly; do not call the generic scalar-pointer type resolver, which intentionally rejects two-dimensional row decay.

## TDD and review regressions

- `&row[columns]` as a `memset` destination and `memcmp` input first returned success; both now report `two-dimensional array row pointer index ... out of bounds ...`.
- `const double rows[][C]` initially made the adjusted pointer slot const; a focused reassignment test failed with `cannot assign to const variable`. Runtime slot qualification was separated from pointee qualification.
- `double rows[const R][C]` initially allowed pointer-slot reassignment; direct parameter parsing now carries bracket `const` into `Param::is_const`.
- `&((struct T){...}).rows[0]` initially returned pointer size because generic address-of-subscript lowering erased row-address intent. Metadata-aware lowering now retains the unsupported `double row pointers are not supported` boundary without regressing `&((struct T){...}).one_dimensional[i]`.
- `(int *)rows` beneath `sizeof` initially inherited double-row metadata and returned pointer size. Row metadata now survives only a same-element scalar pointer cast; incompatible casts retain evaluated/non-evaluating boundary parity.
- Conditional branches with different row widths initially collapsed to scalar `double *` metadata and were accepted both when evaluated and beneath `sizeof`. Complete `Array2DPointer` type comparison now rejects the mismatch before selecting a branch.
- Limit complete conditional branch inference to conditionals with a detected row branch. Eager inference across every ternary regressed nested non-evaluating `strtol` end-pointer validation; the existing linear-depth regression is the adjacent guard.
- The first native fixture read a `double` after byte-zeroing it. The oracle now checks the resulting bytes with `memcmp` against unsigned-character storage and never assumes that all-zero bytes are a native floating zero.
- A row-returning `memset` comparison followed by row-byte `memcmp` in short-circuit `||` initially failed because runtime alias validation sent `PointerValue::Array2DRow` through unsupported scalar-pointer decay. Direct live-row storage classification preserves source order, one-time evaluation, adjacent-row isolation, and parity with equivalent sequential `if` statements.

## Verification guidance

Run:

```bash
cargo test --test interpreter two_dimensional_double -- --nocapture
cargo test --test interpreter supports_direct_indexing_and_address_of_array_fields_on_aggregate_compound_literals -- --nocapture
cargo test --test c_compat -- --nocapture
```

The registered native fixture must remain warning-free under GCC and Clang with `-std=c11 -Wall -Wextra -Werror`. Native checks may compare same-type copies, pointer identity/differences, and equality, but must not assert Cust's fixed endianness or object representation. A native compiler should reject reassignment of a bracket-qualified parameter such as `double rows[const R][C]`.
