# Aggregate array element copy assignment

2026-06-21 autonomous run notes for direct aggregate-array element copy assignment.

## Scope

Cust now treats direct struct/union array element assignments such as `points[0] = other;` and indexed aggregate pointer assignments such as `cursor[0] = (struct Point){1, 2};` as aggregate copy assignments instead of scalar array writes.

## Implementation notes

- Parser shape did not need a new AST variant: `points[i] = rhs` already lowers to `Expr::ArraySet` in expression contexts and `Stmt::ArrayAssign` in statement contexts.
- Runtime dispatch is type-aware:
  - `eval_struct_expr` routes `Expr::ArraySet` to aggregate assignment when the LHS name is a `StructArray` or a pointer to a supported aggregate type.
  - statement/discard paths route `Stmt::ArrayAssign` and `Expr::ArraySet` through the same helper before scalar evaluation.
  - scalar `ArraySet` remains unchanged for ordinary scalar arrays and scalar pointers.
- The shared helper deep-clones RHS aggregate fields, verifies same aggregate type, preserves const/read-only struct array diagnostics, and reuses pointer-target mutability checks for `struct T *`/`union T *` indexed assignments.

## Fixture shape

Use both interpreter and compiler-oracle fixtures with warning-free C:

```c
struct Point points[2] = {{1, 2}, {3, 4}};
struct Point replacement = {5, 6};
struct Point returned = (points[0] = replacement);
replacement.x = 9;           /* proves copy isolation */
points[1] = (struct Point){7, 8};
struct Point *cursor = points;
cursor[0] = (struct Point){11, 12};
```

Avoid ABI-sensitive `sizeof(struct)` assertions in oracle fixtures; compare arithmetic over fields and exit codes.
