# Const pointer typedef aliases

2026-05-11 autonomous run.

Cust now preserves both C const dimensions for one-level pointer typedef aliases:

- `typedef const int *ConstIntView;` and `typedef const struct Point *ConstPointView;` create pointer aliases whose produced pointer values carry `points_to_const = true`, so alias-spelled declarations, parameters, and pointer-returning functions reject const-to-mutable conversions and writes through the pointee.
- `typedef int * const ConstIntSlot;` and `typedef struct Point * const PointSlot;` create aliases whose alias-spelled variables/parameters are const pointer slots (`is_const = true`) while their pointees remain mutable.

Implementation notes:

- `TypeAlias::Pointer` and `DeclType::Pointer` now carry `points_to_const` metadata separately from the existing `const_type_alias_scopes` top-level alias const set.
- `parse_typedef_decl` consumes qualifiers after `*` for typedef aliases. Leading const becomes pointee const for explicit pointer aliases; post-star const becomes top-level alias const.
- Alias-spelled pointer declarations/parameters/fields call `decl_type_points_to_const` so non-explicit-star aliases preserve pointee constness instead of treating all alias const metadata as pointer-slot constness.

TDD coverage:

- `tests/fixtures/valid/const_pointer_typedef_aliases.c`
- `tests/fixtures/invalid/const_pointer_typedef_alias_const_discard.c`
- `tests/fixtures/invalid/const_pointer_typedef_alias_slot_assignment.c`
- `tests/fixtures/compat/valid/const_pointer_typedef_aliases.c`

Pitfall: `typedef const int *Alias;` is not the same as `typedef int * const Alias;`; keep pointee const and pointer-slot const represented independently when adding future pointer typedef forms.
