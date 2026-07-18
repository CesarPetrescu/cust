# Embedded aggregate-array element pointer-field updates

## Scope

Completed on 2026-07-18 for pointer-valued final fields selected through `StructFieldArrayElementGet`, `StructFieldArrayElementSet`, and `StructFieldArrayElementCompoundSet`, including direct, nested, and scalar-field reverse-subscript routes.

## Regression shape

```c
struct Node { int *cursor; };
struct Box { struct Node nodes[1]; };
int values[3] = {3, 5, 7};
struct Box box = {{{values}}};
int *result = (box.nodes[0].cursor = values + 1);
```

Before the fix, pointer declarations using the assignment result failed with `expected pointer expression`; reading the field back also failed because `StructFieldArrayElementGet` was scalar-only.

## Implementation checklist

1. Classify `Get`, `Set`, and `CompoundSet` consistently in all pointer metadata surfaces:
   - `pointer_expr_pointee_type()`
   - `pointer_expr_points_to_const()`
   - `expr_is_pointer_value()`
2. Resolve the selected aggregate element once, then delegate final pointer-field reads/writes to the established struct-pointer helpers. This preserves interpreter-owned identity, concrete pointee validation, pointer-slot const checks, and prefix/postfix result semantics.
3. Keep read and mutation resolution separate. A read through `const struct Box` is legal; using the assignment resolver for `Get` incorrectly reports `cannot assign to const variable 'box'`.
4. For replacement, preserve const before evaluating the RHS, evaluate the pointer RHS once, resolve the target once, then assign through `assign_struct_pointer_pointer_field()`.
5. For `+=`/`-=`, evaluate the scalar offset once, resolve the target once, then use `offset_struct_pointer_pointer_field()`. Keep multiplicative/bitwise/shift pointer compound operators rejected.
6. For prefix/postfix updates, resolve the target once and return the updated/current pointer according to prefix/postfix semantics.
7. `Expr::Increment` result constness must delegate to `pointer_expr_points_to_const(target)` rather than special-case only variables; otherwise a `const T *` field increment result can initialize `T *` and discard qualification.
8. `sizeof` already routes these AST variants through embedded-element field type metadata; prove index/RHS markers remain zero rather than adding runtime evaluation.

## Coverage and oracle boundaries

Cover direct, nested, and reverse routes; replacement, compound assignment, prefix/postfix updates; pointer-valued expression results; one-time target/RHS evaluation; reads through const containers; const pointer slots; const-discard (including increment results); concrete pointee mismatch; and non-evaluating `sizeof`.

The warning-free compiler-oracle fixture `embedded_aggregate_array_element_pointer_field_updates.c` returns 9 under Cust and native `cc`. Keep explicit side effects inside `sizeof` interpreter-only because Clang may promote `-Wunevaluated-expression` under the repository's `-Werror` policy.

## Adjacent boundary

Ordinary named aggregate-array element pointer fields such as `nodes[i].cursor` lower through `StructElementGet`, not this AST family. A minimized probe still reports `struct field 'cursor' is not a pointer`; treat that as a separate work package rather than broadening this fix implicitly.
