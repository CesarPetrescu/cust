# Struct-pointer array-field decay and element address-of

Commit: 2026-05-09 autonomous run (see git history for final hash).

## What changed

Cust now treats scalar array fields reached through a struct pointer (`slot->values`) like array fields reached through direct structs:

- `slot->values` decays to an interpreter-owned array-base pointer in pointer contexts, so it can initialize `int *` / `char *` variables or bind to pointer/array parameters.
- `slot->values[index]` reads scalar elements through the embedded array storage.
- `&slot->values[index]` creates an interpreter-owned array-element pointer that aliases the original embedded field storage.
- Assignment and compound assignment to `slot->values[index]` are lowered through the existing dereference assignment path by taking the element address internally.
- Const-pointee metadata is preserved for `const struct T *slot`: decaying `slot->values` into a mutable pointer target reports `cannot discard const qualifier from pointer target`.

## Implementation notes

- Parser lowering adds `Expr::StructPtrArrayGet` for postfix `[...]` on `Expr::StructPtrGet`, plus `Expr::AddressOfStructPtrArrayField` for `&slot->values[i]`.
- Runtime helpers mirror the prior direct-struct field helpers but begin from `find_struct_pointer_fields(pointer)` so struct variables and struct-array element pointers both work.
- `eval_pointer(StructPtrGet)` now tries array-field base-pointer decay before falling back to pointer-field reads, preserving `slot->p` pointer-field behavior.
- `pointer_expr_pointee_type` and `pointer_expr_points_to_const` need explicit struct-pointer array-field cases; otherwise function argument conversion can miss scalar element type or const-discard diagnostics.

## Fixture caveats

- Native C process exit status is modulo 256, so compiler-oracle fixtures should keep the final return value within `0..=255` even if intermediate sums are larger.
