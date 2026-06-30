# `sizeof` aggregate conditional expression metadata

The 2026-06-30 autonomous run fixed a non-evaluating metadata gap for `sizeof(cond ? aggregate_a : aggregate_b)`.

## Root cause

`Interpreter::sizeof_expr` classified every `Expr::Conditional` as `INT_SIZE`, which was correct for ordinary scalar conditional operands but wrong when the conditional expression's selected type was an aggregate. Existing aggregate expression helpers already knew how to infer the common aggregate type for conditionals and report mismatches.

## Implementation pattern

- Keep `sizeof` non-evaluating: do not evaluate the condition, selected branch, unselected branch, assignment operands, or comma-left operands.
- Route `Expr::Conditional` through a helper that first asks `aggregate_expr_type_name(expr)` for an aggregate result type.
- If aggregate inference succeeds, return the deterministic Cust struct/union size from `struct_types`.
- If aggregate inference reports `conditional branches have mismatched aggregate types`, preserve that diagnostic because `sizeof` still requires a valid operand type.
- Otherwise fall back to pointer metadata (`pointer_expr_pointee_type(expr) -> POINTER_SIZE`) and then the scalar `INT_SIZE` default.

## Fixture guidance

Use ABI-independent native-oracle assertions:

```c
total += sizeof(flag ? left : right) == sizeof(struct Pair);
total += sizeof((marker = marker + 1, flag ? left : right)) == sizeof(struct Pair);
total += sizeof(1 ? (left = right) : right) == sizeof(struct Pair);
total += marker == 0;
total += left.field == original;
```

Avoid asserting exact native aggregate byte sizes because Cust intentionally uses deterministic no-padding sizes.
