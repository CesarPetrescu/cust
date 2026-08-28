# Bounded `double` object bytes

## Scope

Exactly prototyped `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` accept interpreter-owned standalone scalar `double` objects and one-dimensional `double` arrays. The extension does not expose host addresses, call host libc, or depend on the host floating ABI.

## Deterministic representation

- Cust `double` has exactly eight bytes and uses IEEE-754 binary64 bits in fixed little-endian order.
- Reads use `f64::to_bits().to_le_bytes()`; partial or full writes update the selected bytes and reconstruct with `f64::from_bits(u64::from_le_bytes(...))`.
- This is Cust's deterministic object representation, not a claim about native C byte order. Exact endian and partial-write assertions therefore remain interpreter-only.
- The registered compiler-oracle fixture checks only ABI-independent relationships: same-type full-object copy/move, equal-object comparison, zero fill, first-byte identity, zero count, and preservation of typed `double` values.

## Runtime and safety model

- The existing `PointerValue::ObjectByte` wrapper carries a typed interpreter pointer plus a byte offset. Standalone `double` scalar and one-dimensional-array cells participate in the shared bounded byte reader, writer, capacity, overlap, and identity paths.
- Destination-aware coercion preserves a byte-view result when assigned to `char *`, while an aligned result converted back to the exact underlying `double *` canonicalizes to the typed scalar or array-element pointer. Interior `memchr` results retain byte-scaled identity and arithmetic.
- `memmove` snapshots source bytes before writes. `memcpy` compares interpreter-owned storage identity and byte ranges, so overlap remains rejected without host addresses.
- Const, lexical lifetime, selected-root capacity, checked byte offsets, zero-count validation, argument order, prototype validation, and user-definition precedence reuse the existing bounded-memory rules.

## Non-evaluating validation

- `sizeof(memory_call)` validates prototypes, arity, argument types, and structurally known unsupported storage without executing argument side effects, performing runtime range checks, or mutating object bytes.
- Standalone scalar literals, one-dimensional array storage, pointer casts, conditional branches, comma-right results, assignment values, selected `_Generic` associations, and bounded pointer addition can prove supported `double` storage structurally.
- Helper-call pointer provenance remains conservatively rejected in these non-evaluating paths when the exact storage root cannot be proven without executing the call.

## Exact boundaries

- Direct scalar/one-dimensional standalone `double` storage is supported.
- Scalar or array `double` fields in structs, every union-backed member, fixed two-dimensional `double` rows/objects, aggregate compound-literal fields, whole pointer objects, deeper pointers, and unprovable helper-returned targets retain exact unsupported-storage diagnostics.
- Byte writes still target one selected scalar cell or the remaining cells of one selected one-dimensional array. They never flatten adjacent roots or invent aggregate padding.

## TDD and review closure

- The inherited feature tests were already present when this recovery run started. Focused execution was initially RED in nested non-evaluating validation: conditional, comma, selected `_Generic`, and assignment wrappers were detected as containing double storage but were not recognized as provably supported roots.
- Each wrapper route was added incrementally and the focused regression was rerun after every change until GREEN, preserving the unsupported helper and aggregate boundaries.
- A separate interpreter-only coverage test proves exact binary64 little-endian bytes and partial reconstruction: `1.0` maps to `{0, 0, 0, 0, 0, 0, 240, 63}`, the two nonzero bytes are adjacent `memchr` results, and clearing only those bytes yields `0.0`.
- The compiler-oracle fixture is registered explicitly in `tests/c_compat.rs`; run the actual harness rather than filtering by fixture filename.

## Verification

Run all three focused interpreter tests with `cargo test --test interpreter double_object_bytes -- --nocapture`, then run the actual compiler oracle with `cargo test --test c_compat -- --nocapture`, followed by independent pre-commit review and the complete canonical local and Docker gates.
