# Aggregate field assignment result field access

Date: 2026-07-01

Cust supports aggregate-valued assignment expressions for root aggregate variables, aggregate array elements, and aggregate pointer dereferences. A nearby C parity gap was aggregate-valued struct fields: `(line.start = replacement).x` and `(slot->end = other).tag` should mutate the selected aggregate field, return the assigned aggregate value, and allow postfix `.` selection on that by-value result.

Implementation notes:

- `parse_postfix_suffix()` must classify `Expr::StructSet` and `Expr::StructPtrSet` as aggregate-valued `.` targets, just like `Assign`, `ArraySet`, `StructArraySet`, and `DerefSet`.
- `aggregate_expr_type_name()` must recover the assigned aggregate field type from direct struct metadata or from the struct-pointer pointee type plus field path so `sizeof((line.end = other).tag)` stays non-evaluating and reports the selected scalar field size.
- Runtime aggregate evaluation needs struct-field assignment helpers that deep-clone same-type RHS aggregate fields into `StructFieldValue::Struct` targets and return the by-value assigned aggregate copy. Preserve const-field diagnostics and existing mismatch messages.
- Warning-free native oracle fixtures can use `(line.start = replacement).x`, `(slot->end = other).tag`, and `sizeof((marker = marker + 1, (line.end = other).tag)) == sizeof(char)`; the marker proves `sizeof` does not evaluate the assignment expression.
