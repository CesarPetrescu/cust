# Bounded aggregate-field `double` object bytes

## Scope

Exactly prototyped `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` accept interpreter-owned scalar and fixed one-dimensional `double` array fields in supported non-union structs. Direct, nested, struct-array-element, and `->` routes reuse Cust's deterministic eight-byte little-endian IEEE-754 binary64 representation.

This extension does not expose host addresses, call host libc, flatten adjacent fields, invent aggregate padding, or enable union-backed or two-dimensional double storage.

## Admission model

- Runtime admission starts from the evaluated interpreter pointer and accepts scalar/array field targets only when `memory_pointer_targets_union_field()` proves that no containing aggregate is a union.
- Array base/element pointers retain their existing `Rc<ArrayValue>` identity, dimensions, owner, path, and selected offset. Dimensioned rows remain rejected.
- Direct scalar field pointers (`StructField` and `StructFieldElementField`) are admitted only outside union ancestry.
- Whole aggregates containing `double` remain governed by the separate whole-object layout validator; this slice opens only selected field storage.

## Non-evaluating validation

`sizeof(memory_call)` must classify supported field roots without evaluating indexes, pointer expressions, or count side effects.

- Address-of scalar field expressions are classified from pointee metadata plus union ancestry.
- Direct one-dimensional array-field decay is classified from `StructFieldType::Array(CType::Double, _)`, not merely from a resulting `double *` type.
- Aggregate compound-literal scalar addresses (`AddressOfAggregateField`) and array decay (`AggregateFieldGet`) are classified directly from aggregate-literal field metadata without evaluating the hidden root.
- Pointer-valued aggregate fields continue to consult their current stored pointer. This distinction is required because a declared `double *` field may currently point at an unsupported row of `double[R][C]`.
- Direct/arrow union-backed one-dimensional array fields remain rejected even when their field type is otherwise supported.

## Safety invariants

- Byte capacity and overlap are local to the selected scalar cell or remaining elements of the selected field array.
- Recursive aggregate const, pointer-target qualification, lexical owner lifetime, and out-of-scope diagnostics remain unchanged.
- `PointerValue::ObjectByte` preserves aligned/interior `memchr` identity, byte arithmetic, and destination-aware typed coercion.
- Partial writes reconstruct exactly one affected binary64 cell from little-endian bytes.
- `memcpy` still rejects overlap; `memmove` snapshots before mutation.
- Union-backed roots, two-dimensional rows/objects, whole unsupported aggregate layouts, deeper pointers, and unprovable helper roots remain exact boundaries.

## TDD and review closure

The focused RED/GREEN sequence covered:

1. Direct scalar fields initially failed at the double-storage admission predicate.
2. Direct/nested one-dimensional array fields required aggregate-backed array pointer admission.
3. Non-evaluating direct and arrow field roots required metadata-only classification.
4. Independent review found that a broad `double *` shortcut laundered two-dimensional row provenance through pointer-valued struct fields; direct and arrow regressions failed with `Ok(8)` before field-kind-aware classification restored the diagnostic.
5. Follow-up review found non-evaluating union-array admission; a dedicated regression failed with `Ok(8)` before union ancestry was added to the direct-array-field classifier.
6. A stale scalar-field `memset` rejection test was converted to assert runtime zeroing and non-evaluating pointer-size success.
7. Pre-commit review found that scalar and array fields selected from aggregate compound literals worked at runtime but failed beneath `sizeof`; two focused regressions failed with the exact unsupported-storage diagnostic before explicit metadata-only admission restored parity.
8. The same review found that the native fixture read a `double` after all-zero `memset`, which C11 does not guarantee is a valid positive-zero representation. The fixture now checks the zeroed bytes only through `memchr`.
9. Follow-up review found that generic pointer-provenance analysis did not flag direct union compound literals. A scalar/array regression first returned `Ok(8)` before explicit aggregate type/path union ancestry restored the diagnostic.
10. Final review found `AddressOfAggregateField` returned pointer size without validating aggregate-literal initializers beneath `sizeof`. A focused invalid pointer-to-double initializer first returned `Ok(8)` before non-evaluating aggregate sizing restored the conversion diagnostic.

The registered compiler-oracle fixture uses only ABI-independent same-type copy/move/comparison, typed-value relationships before raw zero fill, and byte-level inspection afterward. Exact byte-order, all-zero typed floating interpretation, and partial-write assertions remain interpreter-only.

## Verification

Run:

```bash
cargo test --test interpreter aggregate_double_object_bytes_ -- --nocapture
cargo test --test interpreter direct_double_aggregate_field_memset_supports_runtime_and_sizeof_routes -- --nocapture
cargo test --test interpreter direct_double_pointer_review_uses_current_storage_for_raw_memory_validation -- --nocapture
cargo test --test c_compat -- --nocapture
```

Then run independent pre-commit review and the full canonical local and Docker gates.
