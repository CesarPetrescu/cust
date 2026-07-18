# Embedded aggregate-array element field increment/decrement

## Scope

Cust represents `box.points[i].field` and scalar-field reverse aggregate subscripts such as `selector.value[points].field` as `Expr::StructFieldArrayElementGet`. Prefix/postfix `++` and `--` must accept that lvalue shape when the selected final field is scalar.

## TDD/root cause

- The parser's `increment_expr()` lvalue allowlist omitted `StructFieldArrayElementGet`, so every form stopped at `invalid increment/decrement target`.
- After parser acceptance, `eval_increment_expr()` also lacked this target.
- Reusing separate read and assignment helpers for direct embedded arrays evaluated a side-effecting index twice. Resolve the selected aggregate element to one interpreter-owned `PointerValue`, then read and write the final field through that captured pointer.
- Reverse scalar-field subscripts must first try `scalar_field_reverse_subscript_pointer(...)`; direct routes use `find_struct_array_field_pointer(...)`.
- Preserve direct root/field const checks before resolving the pointer. A directly named const aggregate array on the reverse route also needs `ensure_variable_mutable(pointer_name)` because `ensure_reverse_subscript_pointee_mutable()` intentionally delegates direct-array read-only checks to ordinary assignment paths.

## Coverage

Cover:

1. Direct and reverse aggregate subscript routes.
2. Prefix/postfix increment and decrement over `int` and `char` fields.
3. Side-effecting direct indexes and reverse pointer expressions evaluated exactly once.
4. Const containing fields, const selected scalar fields, const reverse roots, direct/reverse bounds, and non-aggregate pointer diagnostics.
5. `sizeof(++lvalue)` / `sizeof(lvalue--)` type inference with index/pointer markers and storage mutations suppressed.
6. Warning-free GCC/Clang compiler-oracle behavior for runtime-defined routes. Keep side-effecting `sizeof` probes interpreter-only because Clang promotes `-Wunevaluated-expression` under `-Werror`.

## Adjacent gap

`StructFieldArrayElementSet` and `StructFieldArrayElementCompoundSet` still need the same direct/reverse captured-target treatment. A direct probe of `selector.value[points].value += 2` currently reports `struct variable 'points' used as scalar`; direct compound assignment can also evaluate its index in separate read/write helpers. Address that as the next vertical slice.
