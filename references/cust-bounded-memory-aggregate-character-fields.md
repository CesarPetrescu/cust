# Bounded memory operations on struct-backed character fields

## Scope

Cust's exactly prototyped `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` accept interpreter-owned `char` scalars and one-dimensional `char` arrays selected from named, anonymous, or nested struct fields. Direct (`object.field`) and arrow (`pointer->field`) array decay, interior offsets, and scalar-field addresses reuse existing `PointerValue` identity rather than host addresses or a synthesized aggregate byte layout.

## Implementation decisions

- Aggregate character-array fields already decay to their owned `ArrayValue`; bounded reads/writes therefore reuse the existing array capacity, overlap, const, and lifetime machinery.
- Scalar struct fields use `PointerValue::StructField`; scalar fields reached through embedded aggregate-array elements use `PointerValue::StructFieldElementField`. Both have capacity one and read/write through the ordinary checked dereference paths.
- `struct_field_with_container_type_by_scope()` preserves the containing aggregate type while resolving a field. Raw-memory validation uses it to distinguish struct-backed fields from union-backed fields.
- Array pointers do not directly retain the containing aggregate type. `array_pointer_targets_union_field()` therefore traces the exact `Rc<ArrayValue>` through live/static aggregate storage and recursively carries ancestor aggregate kinds before raw-memory dispatch. This is metadata-only, does not evaluate an expression, and keeps provenance attached to the current container rather than the clonable array value. A struct copied from a union member consequently becomes independent struct-backed storage instead of retaining stale union provenance.
- Union-backed character storage remains unsupported. Cust stores union member values separately and does not yet model raw-byte aliasing among members, so accepting member pointers would make writes and `memcpy` overlap checks disagree with C. Runtime validation reports `function '<name>' does not yet support union-backed scalar object storage for argument <n>` for direct, embedded-array-element, and array-field routes.

## TDD and review closure

1. Add the aggregate-field compiler-oracle fixture and focused direct/arrow/nested/anonymous array tests.
2. Add scalar struct-field memory-operation tests; they must first fail with the old aggregate-backed rejection.
3. Add embedded aggregate-array element scalar-field tests; they must first fail with a zero-capacity diagnostic.
4. Extend bounded reads/capacity to both scalar field pointer variants.
5. Add direct, embedded-element, and array-field union regressions before introducing aggregate-kind validation.
6. Preserve the shared string-operation interpretation of dereferenced two-dimensional rows; reject those rows only in the bounded raw-memory validator.
7. Add a union-member-to-struct copy regression before moving union provenance from `ArrayValue` flags to live container lookup.
8. Verify field-local capacity, overlap, const, escaped lifetime, zero-count, non-character rejection, and non-evaluating `sizeof` behavior.
9. Run the actual `supported_programs_match_c_compiler_exit_codes` test; fixture-name filtering runs zero tests.

## Native-oracle boundary

The registered C fixture uses only defined, warning-free struct-backed operations. Do not use native executions for invalid union-overlap or unrelated-pointer programs. GCC and Clang are test oracles only; Cust never dispatches these calls to host libc.
