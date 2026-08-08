# Bounded scalar object bytes

## Scope

Exactly prototyped `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` accept interpreter-owned standalone scalar objects and one-dimensional scalar arrays in addition to the established character-storage routes. Cust does not expose host addresses, call host libc, infer host integer widths, or serialize aggregate padding.

## Deterministic representation

- Cust `int` has exactly eight bytes and uses a fixed little-endian two's-complement representation. Reads use `i64::to_le_bytes`; partial writes update one selected byte and reconstruct with `i64::from_le_bytes`.
- Cust `char` keeps the established one-cell behavior, including exact stored `i64` values on copy/move; byte comparison/search and fill continue to normalize modulo 256 where C requires unsigned-character semantics.
- Cust `_Bool` has one object byte. Any nonzero byte write normalizes stored `_Bool` to `1`, so a later object-byte read returns canonical `1`; zero writes return `0`.
- Whole structs/unions, aggregate arrays, non-character scalar/array fields, two-dimensional non-character scalar arrays, and every union-backed member remain outside this standalone-object slice.

## Runtime model

- `PointerValue::ObjectByte` carries an interpreter pointer plus a byte offset. Non-character `memchr` results retain this byte view even when aligned; exact underlying typed destinations unwrap an aligned view to the ordinary scalar/array-element pointer.
- Range checks derive capacity from the selected scalar cell or the remaining one-dimensional array cells, then subtract any interior byte offset. Reads and writes resolve `(cell index, byte within cell)` through checked interpreter pointers, retaining owner/lifetime/const diagnostics.
- `memmove` snapshots all source bytes before writes. `memcpy` compares storage identity plus byte-scaled starts/ends, so overlap across integer array elements is rejected exactly.
- `memchr` preserves a byte-view wrapper for non-character results, including aligned matches, so conversion to `char *` retains byte-scaled arithmetic. Conversion of an aligned result back to its exact underlying `int *`/`_Bool *` type canonicalizes to the ordinary scalar/array-element pointer. Interior pointers support equality, subsequent bounded memory calls, indexed byte lvalues, checked byte arithmetic, and pointer difference against the same scalar object.

## TDD and review closure

- Initial focused RED: the compatibility fixture failed with `function 'memcpy' currently supports only character storage for argument 1`.
- GREEN generalized the shared byte reader/writer/range/overlap paths while preserving character behavior.
- A focused interior-result test then went RED because `memset(memchr(...))` returned a pointer that compared unequal; `pointer_eq` gained byte-offset identity.
- A pointer-difference regression went RED with `scalar pointer arithmetic is not supported`; byte-position difference support made it GREEN.
- Initial independent review found noncanonical aligned `memchr` results and stale legacy rejection tests. A new aligned-array regression returned `10` before canonicalization and `0` afterward; legacy tests now reject whole aggregates rather than newly supported integers.
- Final independent review then found that fully discarding the aligned byte-view wrapper broke valid `char *` conversion and byte arithmetic, and that generalized scalar validation admitted non-character aggregate fields outside the documented standalone scope. Focused RED/GREEN now preserves aligned byte views until destination-type coercion, covers declaration, statement/expression assignment, parameter, return, and aggregate pointer-field destinations, and rejects direct plus array-backed non-character aggregate fields while retaining the existing character-field slice.
- The registered native fixture uses only ABI-independent relationships: full-object copy/move equality, zero fills, equal-object comparison, zero-byte search, and `_Bool` clearing. Deterministic endian/partial-write assertions remain interpreter-only.

## Verification

Run all seven focused interpreter tests with `cargo test --test interpreter scalar_object -- --nocapture`, then the actual compiler oracle with `cargo test --test c_compat -- --nocapture`, followed by the complete canonical local and Docker gates.
