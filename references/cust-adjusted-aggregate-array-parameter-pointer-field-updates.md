# Pointer-field slot updates through adjusted aggregate-array parameters

Use this note when `items[i].nested.pointer_field = rhs`, pointer compound assignment, or pointer increment works for stored aggregate arrays but fails when `items` is an array parameter.

## Regression shape

```c
struct Inner { int *cursor; };
union Choice { struct Inner nested; int marker; };

int replace(union Choice choices[], int *replacement) {
    choices[0].nested.cursor = replacement;
    return choices[0].nested.cursor[0];
}
```

C adjusts `choices[]` to an aggregate pointer parameter. Cust still emits `StructElementSet` / `StructElementCompoundSet` / pointer increment over `StructElementGet`, while the parameter is bound as `Value::Pointer`, not `Value::StructArray`.

## Implementation pattern

1. Keep one mutating containing-element resolver for all pointer-slot operations.
2. For `Value::Pointer`, reject a const aggregate pointee with `pointer_variable_points_to_const(name)`, then resolve the indexed target through `indexed_struct_pointer(name, index)`.
3. Do not call `ensure_variable_mutable(name)` for an adjusted parameter before resolving its pointee. A const parameter pointer slot (for example `T items[const N]`) forbids rebinding the parameter itself but must not forbid mutation through it when the aggregate pointee is mutable.
4. For stored `Value::StructArray`, retain `ensure_variable_mutable(name)` and `find_struct_element_pointer(name, index)` so direct const-array diagnostics stay unchanged.
5. Delegate replacement and arithmetic updates to `assign_struct_pointer_pointer_field()` / `offset_struct_pointer_pointer_field()`. Those helpers retain pointer-field slot constness, concrete pointee type checks, and pointer-valued expression results.
6. Keep `sizeof` classification metadata-only. Assignment, compound-assignment, and increment operands must not evaluate the containing index or RHS.

## Verification boundaries

Cover replacement, `+=`/`-=`, prefix/postfix `++`/`--`, one-time index/RHS evaluation, const aggregate views, const parameter pointer slots, const pointer fields, pointer-to-const fields, const-discard diagnostics, and non-evaluating `sizeof`.

Clang with `-Wall -Wextra -Werror` rejects side-effecting expressions inside `sizeof` as `-Wunevaluated-expression`. Keep those marker assertions interpreter-only; use side-effect-free pointer-size relationships in the compiler-oracle fixture. The 2026-07-19 warning-free fixture returns 63 under Cust, GCC, and Clang.
