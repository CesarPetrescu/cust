# Const aggregate-array field writes on aggregate compound literals

Date: 2026-06-23

## Problem

The supported aggregate compound-literal path already preserved const safety for aggregate-array fields, but scalar writes through an indexed const aggregate-array field selected from a compound literal produced the generic pointer diagnostic:

```c
struct Point { int x; int y; };
struct Box { const struct Point points[2]; };
((struct Box){{{1, 2}, {3, 4}}}).points[1].x = 9;
```

Before the fix this failed with `cannot assign through pointer to const`. That was safe but less precise than direct aggregate-array field writes, which identify the const parent field.

## Implementation note

`((struct Box){...}).points[1].x = 9` parses as a `StructPtrSet` whose pointer expression is pointer arithmetic over `AggregateFieldGet { fields: ["points"] }`. `eval_aggregate_literal_field_pointer()` materializes the selected aggregate-array field as a hidden read-only struct array, so the existing write guard reported the generic pointer-to-const diagnostic.

Patch `ensure_pointer_expr_pointee_mutable()` to keep the generic safety check but, when the pointer expression syntax originated from an aggregate literal's const field path, report:

```text
cannot assign to const struct field '<field>'
```

The helper is metadata-only: it inspects `Expr::AggregateFieldGet` / pointer `+` or `-` expressions and `StructTypeDef` field metadata without evaluating compound-literal initializers.

## Tests

Focused RED/GREEN:

```bash
cargo test --test interpreter rejects_assignment_to_nested_fields_of_const_aggregate_array_fields_on_compound_literals -- --nocapture
cargo test --test interpreter const_aggregate -- --nocapture
cargo test --test interpreter aggregate_compound_literal -- --nocapture
```

Also keep direct const aggregate-array field coverage (`box.points[i].x = ...`) to prove non-compound-literal paths still diagnose the parent field.
