# Scalar-only shared union object bytes

Use this note when extending or reviewing bounded raw-memory operations over union members.

## Storage and identity model

- Every member of a union begins at byte offset zero. Canonical pointer equality, overlap, subtraction-compatible byte positions, and aligned `memchr` identity must resolve through the enclosing aggregate root rather than the selected field path.
- Admitted unions have a complete member list of non-`double` scalar fields and include at least one non-const `int` or `char` raw-byte carrier as wide as the largest member. Selected-member capacity remains that member's declared size; sharing an address does not widen a `char` or `_Bool` object to the maximum union size. A pointer to the whole admitted union uses maximum-layout capacity.
- Existing field maps can preserve one observable storage root by synchronizing through explicit deterministic bytes after each scalar member write. Seed from a widest `int`/`char` carrier, overwrite only the active member width, then decode every scalar view. Bounded intrinsic reads/writes must route through the carrier rather than a normalized `_Bool` slot so raw byte `2` remains visible while the language value is `1`.
- `int` uses Cust's eight-byte little-endian representation, `char` uses one byte, and `_Bool` decodes every nonzero byte as one. Do not use host addresses, Rust layout, or native ABI serialization.

## Required boundaries

- Keep all-`_Bool` layouts, layouts whose only full-width carrier is const, arrays, pointers, `double`, and nested aggregate members rejected until each has an explicit persistent shared representation.
- Preserve root and field const checks, lexical owner liveness, selected-member capacity, zero-count identity, and non-evaluating argument validation.
- `memcpy` must reject ranges selected through two different members of the same union when they overlap; `memmove` may use the usual snapshot semantics.
- Struct-contained scalar unions and scalar union elements reached through aggregate arrays reuse the same path-aware layout predicate and canonical root identity.

## TDD and oracle guidance

1. RED pointer equality between distinct members before the intrinsic runs.
2. RED the former blanket union-storage rejection after canonical identity is fixed.
3. GREEN a narrow byte write that changes only the low byte of a wider member.
4. Cover all five intrinsics, nested and aggregate-array-element routes, raw bytes beneath `_Bool` normalization, member-local capacity, const, lifetime, overlap, zero count, and `sizeof(call)` non-evaluation.
5. Cover whole admitted union pointers with maximum-layout capacity, nested struct and aggregate-array-element routes, and selected-member alias visibility. Retain exact negative cases for pointer, double, array, nested-aggregate, all-`_Bool`, and full-width-const-only layouts.
6. Register a warning-free native fixture using same-type integer members and ABI-independent relationships; keep mixed-width endian assertions interpreter-only.

After review-driven production or test edits, rerun the focused scalar-union filter and obtain fresh independent review before the canonical gate.

## Whole-object extension review closure

- Whole-union object-byte reads and writes must bypass sequential struct-field traversal and use the canonical full-width writable carrier. Synchronize all scalar views after each carrier-byte write.
- Nested range writeability must first honor the containing field's const qualifier, then treat an admitted union as one overlapping carrier-backed range. Iterating overlapping members makes acceptance depend on declaration order.
- Typed byte-result coercion must carry the expected scalar type into whole-object descent. At union offset zero, prefer a matching non-const member over a const sibling of the same type so declaration order cannot make a writable result spuriously read-only.
- Reconcile legacy whole-struct and character-view boundary tests when admission widens: admitted scalar-only unions become positive cases, while unsupported union layouts continue to assert the same targeted diagnostics.