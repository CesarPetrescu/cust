# Aggregate element assignment result field access

Date: 2026-07-01

Cust supports aggregate assignment expressions returning by-value aggregate copies. A less-traveled C expression shape is selecting a field from an aggregate-array element assignment result, for example:

```c
(points[0] = replacement).x
(line.points[1] = replacement).tag
(line_ptr->points[0] = replacement).x
```

Parser pitfall: `parse_postfix_suffix()` must classify assignment-result AST variants that can produce aggregate values as valid `.` targets. Root aggregate-array element assignment is `Expr::ArraySet` when the named object is a `Value::StructArray`; embedded aggregate-array element assignment is `Expr::StructArraySet`; struct-pointer embedded aggregate-array assignment is lowered through `Expr::DerefSet` over `AddressOfStructPtrArrayField` and was already allowed.

Metadata pitfall: `aggregate_expr_type_name()` must infer the result aggregate type for these assignment-result variants without evaluating side effects. For `Expr::StructArraySet`, use `direct_struct_aggregate_array_field_type(...)` first so embedded aggregate-array element assignments resolve to the element type instead of trying to treat the array field itself as a nested aggregate field.

Verification pattern:

```bash
cargo test --test interpreter supports_field_access_on_aggregate_element_assignment_results -- --nocapture
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/aggregate_element_assignment_field_access.c -o /tmp/aggregate_element_assignment_field_access && /tmp/aggregate_element_assignment_field_access
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
