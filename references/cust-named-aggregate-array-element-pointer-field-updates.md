# Named Aggregate-Array Element Pointer-Field Updates

## Scope

Cust supports stored pointer fields selected from ordinary named aggregate arrays:

```c
struct Node { int *cursor; };
struct Node nodes[2];

int *read = nodes[i].cursor;
int *assigned = (nodes[i].cursor = values + 1);
int *advanced = (nodes[i].cursor += 1);
int *old = nodes[i].cursor++;
int *current = ++nodes[i].cursor;
```

This path is distinct from pointer fields reached through embedded aggregate-array fields such as `box.nodes[i].cursor`.

## Parser and runtime routes

The parser already lowers these lvalues through:

- `Expr::StructElementGet`
- `Expr::StructElementSet`
- `Expr::StructElementCompoundSet`
- `Expr::Increment` targeting `StructElementGet`

The missing support was pointer-specific runtime dispatch. The implementation now:

1. Resolves the selected field's `StructFieldType` through `struct_element_field_metadata()`.
2. Treats `StructFieldType::Pointer` as the stored pointer value in `eval_pointer()` instead of entering the embedded-array decay fallback.
3. Includes get/set/compound-set routes in pointer shape, pointee type, and pointee const metadata classifiers.
4. Resolves mutation targets through `find_struct_element_assignment_pointer()`, which rejects mutation of const aggregate-array roots before writing a pointer slot.
5. Reuses the established pointer-field assignment and offset helpers so concrete pointee compatibility, const pointer slots, bounds, and prefix/postfix results remain consistent with direct and arrow field routes.

## Const model

Container constness and pointee constness are separate:

- Reading `nodes[i].cursor` through a const aggregate object is allowed.
- A mutable pointer stored in that field may still mutate its mutable pointee.
- Replacing or offsetting the field's pointer slot through a const aggregate object is rejected.
- `int * const cursor` rejects pointer-slot replacement/offset even in a mutable aggregate.
- `const int *cursor` remains pointer-to-const metadata, including assignment and increment expression results, so conversion to `int *` is rejected.

## Evaluation and `sizeof`

Side-effecting element indexes and assignment/offset RHS expressions execute once. Non-evaluating `sizeof` classifies get, assignment, compound-assignment, and increment results as pointer-sized without evaluating indexes, RHS expressions, or mutations.

## Coverage

Focused interpreter coverage includes:

- Pointer reads.
- Replacement assignment.
- Arithmetic compound assignment.
- Prefix/postfix increment results.
- One-time index and RHS evaluation.
- Const pointer slots, const aggregate roots, and pointee const-discard diagnostics.
- Concrete named aggregate pointer mismatch diagnostics.
- Mutable pointee access through const aggregate slots.
- Non-evaluating `sizeof`.

The valid and compiler-oracle fixtures are both named `named_aggregate_array_element_pointer_field_updates.c`; the warning-free native fixture returns 17 under `cc -std=c11 -Wall -Wextra -Werror`, matching Cust.

## Follow-up boundary

Pointer-field indexing is a separate AST seam. At this implementation point:

```c
nodes[0].cursor[1]
```

still reports `struct field 'cursor' is not an array`. The next work package should distinguish stored pointer fields from embedded array fields in direct/reverse subscript, write, compound/increment, address-of, and non-evaluating `sizeof` routes.
