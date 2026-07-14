# Address-of non-variable aggregate pointer indexes

Date: 2026-07-15

## Scope

Complete `&expr[i]` for the safe one-level aggregate pointer model across pointer-returning calls, conditional/comma pointer expressions, direct pointer-valued aggregate fields, and `->` pointer fields.

## Root cause

The parser already lowers general `&expr[i]` to the indexed pointer expression by cancelling `&*`, so calls and conditional/comma expressions already reached `offset_array_pointer()`. Direct and arrow aggregate pointer fields instead lower through `AddressOfStructArrayField` and `AddressOfStructPtrArrayField`. Their field-index helpers always called scalar-only `checked_pointer_value_index()`, which rejects `PointerValue::StructElement` / `StructFieldElement` with `struct pointer is not indexable`.

## Implementation pattern

- Keep scalar pointer fields on `checked_pointer_value_index()` so their established null/scalar diagnostics do not change.
- When field metadata says `PointeeType::Struct`, route indexing through `offset_array_pointer()` and return the resulting interpreter-owned aggregate element pointer unchanged.
- Infer arrow-field metadata without evaluating the pointer expression. `struct_pointer_expr_field_metadata()` walks the declared aggregate type and nested field path.
- Distinguish pointer-slot `const` from pointee `const`: `T * const` and a `const` containing aggregate do not make `T` const, while `const T *` must reject mutable conversions. Embedded array fields still inherit containing-object/field constness.

## TDD and verification

Focused RED reproduced direct and arrow failures with `struct pointer is not indexable`. Separate RED tests showed `const T *` direct/arrow fields incorrectly allowed const discard. GREEN coverage includes named struct and union pointees, mutable access through const pointer slots and a const containing-aggregate view, pointer-returning call/conditional/comma expressions, mutation aliasing, pointer difference, exact const-discard diagnostics, and bounds diagnostics.

Run focused coverage with:

```bash
cargo test --test interpreter aggregate_pointer -- --nocapture
cargo test --test c_compat -- --nocapture
```

Native fixtures compile warning-free with GCC and Clang under `-std=c11 -Wall -Wextra -Werror`.

## Follow-up found

Do not place pointer-difference expressions directly inside a larger scalar additive chain until the classifier follow-up is fixed. `scalar + (left_ptr - right_ptr)` is currently misclassified as pointer-valued because `expr_is_pointer_value()` recursively sees pointer operands inside the scalar pointer-difference subexpression. Assign the difference to a scalar temporary in compatibility fixtures; the backlog records the production fix separately.
