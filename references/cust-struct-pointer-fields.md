# Cust struct pointer fields — 2026-05-07

## Work package

Implemented one-level pointer fields inside supported struct definitions:

- scalar pointer fields such as `int *external;`
- struct pointer fields such as `struct Node *next;`, including self-referential links while parsing `struct Node`
- pointer-field initializers, reassignment, pointer truthiness/use in pointer contexts, chained `node.next->value`, and dereference such as `*node.external`
- const-pointee field metadata for `const int *field` / `const struct Point *field`, preserving const-discard diagnostics while allowing pointer-field reassignment unless the slot is `T * const field`
- targeted parser rejection of pointer-to-pointer and pointer-array struct fields

## TDD notes

The interrupted prior run had already added the primary RED fixtures and implementation. This recovery run added an extra RED regression for `const int *field` reassignability:

```bash
cargo test --test interpreter struct_pointer_field -- --nocapture
```

It failed as intended with:

```text
cannot assign to const struct field 'view'
```

Root cause: the first implementation stored the leading `const` on pointer fields as both field-slot const and pointee const. The fix mirrors normal pointer declarations/parameters:

- explicit `*`: `const T *field` => `points_to_const = true`, `is_const = false`
- explicit `* const`: `T * const field` => `is_const = true`, `points_to_const = false`
- pointer typedef field with leading `const`: keeps alias slot const behavior and does not infer pointee const

## Implementation details

- `StructFieldDef` carries `points_to_const` alongside field-level `is_const`.
- `StructFieldType::Pointer(PointeeType)` contributes deterministic `POINTER_SIZE` to Cust struct size.
- `StructFieldValue::Pointer` stores cloned `PointerValue`, `PointeeType`, field const, and pointee const metadata.
- Pointer fields copy pointer values by value when structs are copied, passed by value, or returned; pointees are not deep-cloned.
- Pointer-field initialization and reassignment call `ensure_pointer_conversion_preserves_const(...)` before `eval_pointer(...)`.
- Direct struct-field pointer helpers are named separately from existing pointer-target field helpers:
  - `read_direct_struct_pointer_field`
  - `assign_direct_struct_pointer_field`

## Pitfalls

- Do not conflate `const T *field` with `T * const field`; the former must be reassignable but cannot flow into mutable pointer targets.
- Avoid unsupported `&other.value` fixture syntax unless implementing address-of struct fields.
- Native C compiler-oracle fixtures should compare behavior/exit codes, not native struct layout or ABI padding.
- `cargo test` filters are substring-only; `cargo test --test interpreter struct_pointer_field -- --nocapture` covers the new focused tests.
