# Direct `double *` aggregate-field pointers

Cust supports one-level `double *` values that address scalar `double` fields or decay one-dimensional `double` array fields in structs. Direct, aggregate-array element, reverse-subscript, arrow, nested embedded aggregate, compound-literal, and safely consumed aggregate-valued expression routes retain interpreter-owned root, element, field, const, and lexical-lifetime identity.

The existing pointer model supplies checked same-array indexing, arithmetic, difference, equality, pointer-field and function forwarding, and qualification-preserving `void *` conversion. `sizeof` and `_Generic` validate and classify these routes without evaluation; `_Alignof(double *)` remains the deterministic pointer-alignment query.

Still rejected with the established diagnostics: `double **`, multidimensional `double` fields and pointer-to-row forms, whole-array addresses, character/raw-memory access to double object bytes, and every union-backed double field address or decay. No host address is created.

Focused TDD began with `direct_double_aggregate_fields_support_scalar_addresses_and_array_decay` failing with `double pointers are not supported`, then passing after the shared unsupported-pointer guard was narrowed. The remaining focused valid/invalid tests and `tests/fixtures/compat/valid/direct_double_aggregate_field_pointers.c` cover the supported routes and retained boundaries.

## Recovery review closure (2026-08-24)

- Static-local provenance must survive multiple calls that are analyzed rather than evaluated. A helper that conditionally stores an aggregate pointer and later returns it beneath `sizeof` exercises synthetic static state rather than runtime `static_locals`.
- `UnionPointerProvenanceBinding::aggregate_target == None` is conservative mixed/unknown identity at a control-flow join. Do not replace it with a concrete target from the other path. `merge_union_pointer_provenance()` must clear target identity whenever branch targets differ, including `None` versus `Some(target)`.
- Retaining the concrete target lets later parameter writeback find the static binding by target identity and replace its union-backed `whole_object` provenance with a safe binding. The exact symptom is `sizeof(store_then_read(&choice.item)->values)` incorrectly returning eight.
- Debug by logging uniquely tagged static keys before/after each statement, at call-effect copyback, and at call-result analysis; remove all instrumentation before focused GREEN and review. Verify with the exact static regression, the shared `union_provenance` filter, the `direct_double_aggregate_field` filter, and the actual `c_compat` function.
