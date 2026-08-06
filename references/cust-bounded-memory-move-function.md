# Bounded explicitly prototyped `memmove`

Date: 2026-08-06

Cust's second raw-memory intrinsic reuses the provenance-safe character-storage model and activates only for the normalized equivalent of:

```c
void *memmove(void *destination, const void *source, size_t count);
```

## Semantics and representation

- One interpreter-owned `char` scalar or one-dimensional `char` array element counts as one byte; integer and aggregate object representations remain unsupported.
- Destination, source, and count evaluate exactly once before intrinsic validation.
- The complete source range is capacity-checked and every exact deterministic character value is snapshotted before destination mutation. This implements C's as-if temporary-array rule for forward, backward, and self overlap.
- Snapshots retain `i64` character-cell values rather than narrowing through `u8`, preserving Cust's deterministic plain/signed/unsigned character model.
- Counts are bounded to 0..=4,096; destination/source const, lifetime, capacity, row, and character-storage checks reuse the reviewed `memcpy` machinery.
- The result retains the destination `PointerValue` identity with declared `void` pointee metadata. No host address or libc call enters execution.

## Dispatch and verification

Runtime and non-evaluating `sizeof` require the exact explicit declaration, validate arity plus pointer/count shapes, preserve user-defined function precedence, and report incompatible declarations in the intrinsic's own context. Thirty nested calls beneath `sizeof` validate linearly without evaluating markers.

The registered warning-free native fixture covers forward overlap, backward overlap, self overlap, returned destination identity, and negative `signed char` value preservation. Focused interpreter diagnostics cover destination/source capacity, destination const, expired storage, non-character storage, invalid count shapes/ranges, zero count, and exact runtime/`sizeof` prototype handling.

## TDD closure

The first focused fixture run failed with `undefined function 'memmove'`. Extending the shared memory-copy prototype/runtime/non-evaluating dispatch and skipping only `memcpy`'s overlap rejection made the fixture pass while retaining pre-write snapshot semantics. Existing `memcpy` overlap and safety regressions remain green. Independent read-only review returned `APPROVED` with no blocking or material findings.