# Cust union pointers and pointer fields (2026-05-07)

## RED

Added `tests/fixtures/valid/union_pointers.c`, `tests/fixtures/invalid/union_pointer_to_pointer_field.c`, and a C compiler-oracle fixture. The focused valid test failed first with:

```text
undefined union type 'Node'
```

Root cause: aggregate field parsing only special-cased self-referential `struct Name *field` syntax. `union Node *next;` inside `union Node { ... }` was routed through generic `parse_decl_type`, which requires the aggregate type to already be declared.

## GREEN

- Generalized aggregate field parsing for both `struct` and `union` direct spellings, allowing self-referential one-level pointer fields while keeping pointer-to-pointer fields rejected.
- Made pointer-to-pointer and pointer-array field diagnostics use the current aggregate keyword (`struct` vs `union`).
- Fixed scalar writes through aggregate pointers to reuse `assign_scalar_field_in_map`, so writes through `union Number *p` synchronize sibling scalar views just like direct `n.value = ...` and nested union-field assignments.

## Pitfalls

- Union pointer field fixtures should avoid treating multiple pointer fields in the same union as independently live in native C; writing one pointer field overwrites the active union storage for another pointer field. Use separate union types or only read the active pointer field.
- Native oracle fixtures should continue to compare small exit-code behavior only. Do not assert native `sizeof(union)` or byte-reinterpretation details.
- If a scalar field write happens through `p->field`, call the same union synchronization helper used by direct field assignment; otherwise `p->tag = 6` updates only that field and diverges from Cust's logical scalar-sharing union model.

## Verification notes

Focused commands used during development:

```bash
cargo test --test interpreter union_pointer -- --nocapture
cargo test --test interpreter pointer_to_pointer_union -- --nocapture
cargo test --test c_compat -- --nocapture
```
