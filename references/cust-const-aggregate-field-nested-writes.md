# Const aggregate field nested writes

Date: 2026-06-23

## Scope

Cust supports const-qualified aggregate fields, including named and anonymous nested aggregate fields such as:

```c
struct Point { int x; int y; };
struct Box { const struct Point point; };
struct AnonBox { const struct { int x; int y; } point; };
```

A const aggregate field is read-only as a whole and through its nested scalar field paths. Initializers may still populate the field, but later writes like `box.point.x = 3;` or `anon.point.y = 4;` must report `cannot assign to const struct field 'point'`.

## Implementation note

The root gap was `assign_scalar_field_in_map()`: it checked `is_const()` only when the final path component was assigned. For a path with remaining nested components, it recursed into `StructFieldValue::Struct` without checking whether the parent aggregate field was const. The fix checks `field_value.is_const()` before recursing, so const parent aggregate fields protect all nested scalar members while existing const scalar-field diagnostics for final fields remain unchanged.

## Verification

Focused RED/GREEN tests:

```bash
cargo test --test interpreter rejects_assignment_to_nested_fields_of_const -- --nocapture
cargo test --test interpreter const_aggregate -- --nocapture
cargo test --test interpreter const_struct -- --nocapture
```

The focused RED failed with `unwrap_err()` on `Ok(3)` / `Ok(4)`, proving Cust previously allowed the invalid writes. GREEN passes after the recursive const guard.
