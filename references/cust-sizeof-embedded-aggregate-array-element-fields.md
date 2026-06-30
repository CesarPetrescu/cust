# sizeof embedded aggregate-array element fields

Date: 2026-07-01

## Problem

`sizeof(line.points[0].tag)` for an embedded aggregate-array element field was classified as Cust `int` size regardless of the selected field type. The parser lowers these expressions to `Expr::StructFieldArrayElementGet`, but `sizeof_expr()` returned `INT_SIZE` for all `StructFieldArrayElement*` get/set/compound-set variants.

This preserved non-evaluation but lost type metadata, so char fields selected from embedded aggregate-array elements failed `sizeof(expr) == sizeof(char)` relationships.

## Fix

Route `Expr::StructFieldArrayElementGet`, `StructFieldArrayElementSet`, and `StructFieldArrayElementCompoundSet` through a metadata helper that:

1. Resolves the containing struct variable and embedded aggregate-array field path.
2. Reads the embedded element aggregate type name from `StructFieldValue::StructArray` metadata.
3. Returns the selected field's deterministic size via `sizeof_aggregate_field_type(...)` without evaluating the element index or RHS.
4. Preserves targeted diagnostics for scalar arrays, scalar/pointer fields, and non-struct roots.

The same run also taught `sizeof_struct_element_array_indexed_value()` to return aggregate element sizes for `StructFieldValue::StructArray` instead of treating all non-scalar arrays as non-arrays, keeping struct-array element metadata parity.

## Verification

Focused RED before the fix:

```bash
cargo test --test interpreter supports_sizeof_embedded_aggregate_array_element_field_types_without_evaluating_operands -- --nocapture
# left: 2, right: 4
```

Focused GREEN after the fix:

```bash
cargo test --test interpreter supports_sizeof_embedded_aggregate_array_element_field_types_without_evaluating_operands -- --nocapture
```

Native compiler-oracle fixture:

```bash
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/sizeof_embedded_aggregate_array_element_fields.c -o /tmp/sizeof_embedded_aggregate_array_element_fields
/tmp/sizeof_embedded_aggregate_array_element_fields  # exit=4
```

Full `c_compat` includes `tests/fixtures/compat/valid/sizeof_embedded_aggregate_array_element_fields.c`.

## Pitfalls

- Do not evaluate the array element index or RHS under `sizeof`; type metadata is sufficient.
- Use ABI-independent native checks (`sizeof(expr) == sizeof(type)`) because Cust's `int` and aggregate layout are deterministic and not native-ABI based.
- Nested embedded-aggregate-array paths may route through different pointer/field AST shapes; add focused fixtures before broadening this helper further.
