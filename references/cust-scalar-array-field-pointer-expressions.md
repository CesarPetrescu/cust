# Scalar array-field pointer decay in pointer expressions

Date: 2026-06-25

Cust already allowed scalar array fields such as `packet.values`, `packets[i].values`, and `slot->values` to decay in declaration/parameter contexts, but pointer-expression classification did not recognize those same field selections for relational comparison, pointer subtraction/addition, or truthiness. The symptom was `struct field 'values' is an array` for expressions like `packet.values < &packet.values[3]`.

Implementation notes:

- Keep `struct_field_is_pointer(...)` narrow for actual pointer-field assignment/statement routing.
- Add a separate pointer-expression classifier that treats final `StructFieldValue::Pointer`, `StructFieldValue::Array`, and `StructFieldValue::StructArray` fields as pointer-valued only in expression contexts.
- Cover direct struct fields, struct-array element fields, and struct-pointer arrow fields separately; the arrow path should use metadata (`pointer_expr_pointee_type` plus `struct_types`) instead of evaluating the struct pointer while classifying the expression.
- Include address-of field-element expressions (`AddressOfStructArrayField`, `AddressOfStructElementArrayField`, `AddressOfStructPtrArrayField`, and aggregate-field addresses) in `expr_is_pointer_value`; otherwise valid pointer comparisons can fall through to scalar evaluation and report `pointer value used as scalar`.
- `eval_truthy` needs matching decay-aware cases for `StructGet`, `StructElementGet`, and `StructPtrGet`; otherwise array-field conditions still fall into scalar evaluation.
- Native compiler-oracle fixtures must avoid warning-prone tautological pointer self-comparisons and direct array address truthiness under `-Wall -Wextra -Werror`; compare distinct parameters/elements and use pointer variables for truthiness.

Verification from the implementation run:

```bash
cargo test --test interpreter scalar_array_field -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
