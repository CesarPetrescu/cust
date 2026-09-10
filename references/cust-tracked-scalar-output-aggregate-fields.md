# Tracked scalar-output aggregate fields

This note records the completed implementation and acceptance model for unqualified `char **`, `int **`, `_Bool **`, and `double **` fields in supported structs and direct same-pointee unions. TODO 417 is complete: all 135 focused tests, generated output parity, registered compiler-oracle coverage, formatting, strict Clippy, the exact 2,449-test local/rebuilt-Docker gates, runtime output `10`, and diff hygiene pass. Bounded v0.59.0 release closure is the next task.

## Parser and representation

- Lower direct `T **`, inner-pointer aliases, and complete-output aliases to `StructFieldType::PointerOutput(CType)`.
- Reject qualified output fields, output-field arrays, deeper pointers, mixed-pointee direct unions, output/non-output union overlap, and unions that contain nested aggregate output storage.
- Keep nested aggregate graph inspection cycle-safe and bounded. Use visiting/completed sets and return recoverable diagnostics instead of recursively trusting source-defined type graphs.
- Represent values with `StructFieldValue::PointerOutput`; preserve tracked slot identity through initialization, copy, assignment, field selection, and function return.

## Runtime and non-evaluating parity

Audit every direct, indexed, nested, arrow, embedded-array, reverse-subscript, aggregate-expression, conditional, comma, `_Generic`, assignment-result, and function-call route in both runtime evaluation and metadata-only classification.

- Propagate containing-object owner/lifetime and recursive const ancestry before evaluating irrelevant indexes or RHS expressions.
- Validate output-field assignment type and slot eligibility before evaluation. This includes assignments used only as discarded callee statements beneath `sizeof(call)`.
- Validate unsupported increment/decrement and compound updates in callee analysis even when their result is discarded.
- Preserve lexical facts for local enum zero constants, arrays, row pointers, adjusted parameters, and typedef row aliases; do not fall back to absent runtime locals while analyzing an unevaluated callee.
- Classify the complete conditional/comma expression type before treating one output-valued branch as the whole expression. This is required for `void *` equality and qualification behavior.
- Apply static aggregate output-initializer rules during metadata-only callee analysis as well as runtime initialization. Admit only null constants or addresses of eligible mutable pointer slots with static storage; reject calls and field reads without evaluating them.
- Validate every `_Generic` association semantically, including output-slot qualification, but apply static storage-duration restrictions only to the selected association because controlling and unselected expressions are not evaluated.
- Apply the same all-association rule before parser-folding `_Generic` in enum values and array lengths, and include `_Static_assert` in both file-scope and unevaluated-callee statement walks.
- Keep evaluated global initializer calls on the ordinary runtime call-depth contract; do not impose the smaller metadata-analysis depth while checking static aggregate-output literals.
- Preserve complete aggregate-field array and dereferenced-row object types for lexical `sizeof`; perform ordinary array decay only in value contexts.
- Bound call, comma-expression, and aggregate metadata analysis separately so untrusted source produces deterministic Cust diagnostics rather than host stack overflow.

## Union and copy rules

Direct union output members are supported only when every output member has the same scalar pointee type. Synchronize the selected tracked identity across aliases. Reject mixed pointees and overlap with ordinary storage until a type-erased shared union identity exists. Aggregate copies clone the tracked output descriptor while preserving the referenced caller-owned slot identity.

## Verification pattern

1. Add focused interpreter REDs for each runtime and non-evaluating route.
2. Include exact negative diagnostics for qualification, static storage, const ancestry, mixed unions, invalid updates, and unsupported shapes.
3. Register one warning-free C11 fixture in `tests/c_compat.rs`; keep assertions ABI-independent.
4. Run the actual `c_compat` test function, not a fixture-name filter.
5. Run the complete local and Docker gates only after fresh independent review.

The package's focused filter is:

```bash
cargo test --test interpreter tracked_scalar_output_aggregate_fields -- --nocapture
```
