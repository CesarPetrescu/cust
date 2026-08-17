# Direct-double aggregate pointer-field review closure

## Trigger

Use this note when finishing direct one-dimensional `double` array fields reached through aggregate pointer fields, especially `holder.items[0].values[i]`, `holders[j].items[0].values[i]`, wrapped conditional/comma/`_Generic` forms, assignment-result field access, and `sizeof` validation.

## Required semantic split

Track these independently:

1. **Containing owner/field-slot constness** — protects replacement of a const aggregate object or const pointer slot.
2. **Pointer-field `points_to_const`** — protects the aggregate reached through `const struct Item *`.
3. **Selected aggregate-element type** — `Pointer(Struct)` indexing must produce the pointee aggregate type, not fall through to embedded-`StructArray`-only metadata.
4. **Storage identity** — zero-offset subscripting of a pointer to a standalone aggregate object must preserve the original aggregate pointer target.

A `const struct Holder` containing `struct Item *items` does **not** make the pointee const. Conversely, a mutable `struct Holder` containing `const struct Item *items` must reject writes through `items[0]`.

## Focused RED matrix

Cover direct and struct-array owners:

- `holder.items[0].values[0]`
- `holders[index].items[0].values[0]`
- conditional, comma, and `_Generic` wrappers around the aggregate element
- read, assignment, compound assignment, increment, `_Generic`, and `sizeof(write)`
- mutable pointee: succeeds and mutates original storage
- pointer-to-const: exact const-pointee diagnostic before side effects
- const owner + mutable pointee: pointee write succeeds; pointer-slot replacement remains rejected
- index marker increments exactly once in evaluated routes and zero times in `sizeof`

Also retain reverse aggregate assignment-result coverage:

- `(index[items] = replacement).values[0]`
- `(index[boxes].items[0] = replacement).values[0]`

## Known review failures from 2026-08-16

- Direct `holder.items[0].values[0]` failed with `expected pointer expression`; `sizeof` and `_Generic` reported `struct field 'items' is not an array` because metadata accepted only embedded `StructArray`, not `Pointer(Struct)`.
- Wrapped `const struct Item *items` writes were allowed, including under `_Generic`, and `sizeof(write)` returned 8 instead of rejecting.
- A const owner with a mutable pointer field was over-rejected as `cannot assign through pointer to const`.
- A later fresh review found the same qualification channel missing specifically from reverse aggregate subscripts: `i[holders].items[0].values[0]` used scalar `i` as the non-contextual metadata root. The focused RED/GREEN fix routes `Expr::StructElementArrayGet` const classification through `struct_element_expr_field_metadata(name, index, fields)` so the aggregate-valued index supplies the selected pointer-field metadata.
- Final re-review found `validate_non_evaluating_aggregate_double_array_base_mutable()` returned immediately after proving a `Pointer(Struct)` field had a mutable pointee. That skipped const fields inside the pointee, so `sizeof(holder.items[0].values[0] = 2.0)` returned 8 for `struct Item { const double values[1]; }`. Resolve the pointer field's `PointeeType::Struct` and run the remaining field path through aggregate const validation before returning. The regression must cover direct const-array fields and nested const aggregate ancestors across assignment, compound assignment, and increment in evaluated and `sizeof` contexts; retain the separate const-owner/mutable-pointee success case.

## Closure order

1. Preserve the inherited working tree; do not restart the large direct-double package.
2. Add one focused failing regression per semantic channel above and confirm exact RED.
3. Fix shared type/qualification metadata before evaluator-specific symptoms.
4. Run focused GREEN, `cargo test --test interpreter direct_double_ -- --nocapture`, and full interpreter tests.
5. Obtain fresh independent review. Any code/test edit invalidates approval.
6. Only after approval run the canonical local and Docker gate, update all status files, commit, and push.
