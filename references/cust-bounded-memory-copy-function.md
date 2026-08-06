# Bounded explicitly prototyped `memcpy`

Date: 2026-08-06

Cust's first raw-memory intrinsic is a provenance-safe character-storage slice. It is activated only by the normalized equivalent of:

```c
void *memcpy(void * restrict destination,
             const void * restrict source,
             size_t count);
```

## Cust-owned byte representation

- One interpreter-owned `char` scalar or one-dimensional `char` array element counts as one byte for copy ranges and capacity.
- Snapshots and writes preserve each exact deterministic stored character value. Cust lowers plain, signed, and unsigned character spellings to one scalar model, so reducing through `u8` would corrupt negative `signed char` values.
- `void *` erases the declared pointee type but retains the concrete interpreter `PointerValue`, owner, index/path, lifetime, and read-only identity.
- Integer and aggregate object representations are not serialized in this slice. Calls targeting them report that `memcpy` currently supports only character storage.
- Execution never converts a Cust pointer to a host address and never calls host libc.

## Runtime behavior and safety

- All three arguments are evaluated exactly once before intrinsic validation.
- Counts must be scalar, nonnegative, and at most 4,096 bytes.
- The complete source range is capacity-checked and its character-cell values are snapshotted before any destination mutation.
- The destination range is checked for mutability, lifetime, and capacity.
- Nonzero overlapping source/destination ranges are rejected exactly; zero-count identical ranges are allowed.
- The result retains the destination pointer identity with declared `void` pointee metadata.
- User-defined `memcpy` functions take precedence over the intrinsic.

Cust's general bounded pointer model rejects formation of one-past array pointers before library dispatch. This first slice does not weaken that safety boundary for zero-count calls.

## Non-evaluating and oracle coverage

`sizeof(memcpy(...))` reports pointer size while validating the exact explicit declaration, arity, destination/source pointer shapes, destination qualification, scalar count shape, and nested calls without running side effects. Incompatible declarations report the intrinsic's expected signature rather than falling through to `undefined function` or silently using their declared return type.

The registered native fixture limits comparison to defined, warning-free behavior: non-overlapping character arrays, offset pointers, embedded NUL bytes, exact byte copying, and returned destination identity. Invalid overlap, capacity, lifetime, and non-character-storage cases are interpreter-only because undefined native behavior or Cust's deliberate safe boundary is not a compiler oracle.

## TDD closure

Focused RED/GREEN covered missing intrinsic dispatch, overlap rejection, source-capacity detail, exact incompatible-prototype behavior beneath ordinary evaluation and `sizeof`, and contextual non-character rejection. Independent review then exposed `u8` snapshot corruption of negative `signed char` values; a focused compiler-oracle RED/GREEN now locks exact stored-value preservation. Additional focused coverage verifies destination const/capacity, invalid counts, expired storage, zero count, source immutability, user-definition precedence, and non-evaluation.