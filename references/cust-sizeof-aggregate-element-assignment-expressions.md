# `sizeof` aggregate element assignment expressions

Date: 2026-07-01

## Summary

Cust supports aggregate element copy assignment expressions such as `points[0] = replacement`, `*slot = replacement`, `line.points[1] = replacement`, and `line_ptr->points[0] = replacement`. In C, `sizeof(expr)` is non-evaluating, so `sizeof((points[0] = replacement))` must report the assigned aggregate element type size without mutating the element.

## Root cause

`sizeof_expr()` already handled many assignment-expression forms, but `Expr::ArraySet { name, .. }` used `sizeof_indexed_value(name)`. That helper treated `Value::StructArray` as an invalid scalar array shape and returned `struct variable '<name>' is not an array`, even though `sizeof(points[0])` and aggregate element assignment are supported elsewhere.

Direct embedded aggregate-array element assignment (`line.points[1] = replacement`) also flowed through `sizeof_struct_array_indexed_value()`, which previously reused the scalar array-field helper and rejected `StructFieldValue::StructArray` with `struct field 'points' is a struct array`.

## Implementation notes

- Teach `sizeof_indexed_value()` to return the deterministic element aggregate size for `Value::StructArray`.
- Teach `sizeof_struct_array_indexed_value()` to classify both scalar array fields and embedded aggregate-array fields, returning element size for the selected field kind.
- Keep non-array scalar/struct diagnostics targeted.
- `DerefSet` over aggregate pointers and `AddressOfStructPtrArrayField` already had enough pointee metadata once the direct array/field paths were fixed.

## Verification

Focused RED/GREEN:

```bash
cargo test --test interpreter supports_sizeof_aggregate_element_assignment_expressions_without_evaluating_operands -- --nocapture
```

Compiler oracle:

```bash
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/sizeof_aggregate_element_assignment_expressions.c -o /tmp/sizeof_aggregate_element_assignment_expressions
/tmp/sizeof_aggregate_element_assignment_expressions  # exit=12
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
