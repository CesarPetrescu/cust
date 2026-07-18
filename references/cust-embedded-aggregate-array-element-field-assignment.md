# Embedded aggregate-array element field assignment

## Scope

`Expr::StructFieldArrayElementSet` and `Expr::StructFieldArrayElementCompoundSet` represent scalar-field replacement and compound assignment for direct embedded aggregate-array routes such as `box.points[i].value` and scalar-field reverse aggregate subscripts such as `selector.value[points].value`.

## TDD and root cause

- Reverse replacement assignment delegated to `assign_struct_field_array_element_field()`, which assumed the syntactic left name owned an embedded aggregate array. For `selector.value[points].value = 9`, it therefore scalar-evaluated `points` and reported `struct variable 'points' used as scalar`.
- Direct compound assignment independently called read and write helpers. Each helper evaluated the element index, so `box.points[index++].value += rhs` read one element and could write another.
- Reverse compound assignment had the same unsupported route as replacement assignment.
- Resolve the selected aggregate element once to an interpreter-owned `PointerValue`, then read/write the selected scalar field through `read_struct_pointer_field()` and `assign_struct_pointer_field()`. The shared `struct_field_array_element_assignment_pointer()` helper centralizes direct/reverse target resolution and const checks and is reused by increment/decrement.
- The same AST can represent scalar-field mutation after indexing an aggregate pointer field. Handle pointer fields before root/field const checks: `const T *field` rejects pointee writes, while `T * const field` and pointer slots inside const containing aggregates still allow indexed pointee mutation.
- Replacement assignment preserves the established RHS-before-target evaluation used by this AST path. Compound assignment captures and reads the target, evaluates the RHS once, applies the operator, and writes through the captured target.

## Coverage

Cover:

1. Direct and scalar-field reverse aggregate-subscript replacement assignment over `int` and `char` fields.
2. Direct and reverse arithmetic compound assignments with expression result checks.
3. Side-effecting replacement/compound direct indexes, reverse pointer expressions, and RHS expressions executing exactly once.
4. Const embedded aggregate fields, const selected fields, const reverse roots, direct/reverse bounds, and non-aggregate pointer diagnostics.
5. `sizeof(assignment)` and `sizeof(compound-assignment)` retaining `int`/`char` metadata without evaluating index/pointer/RHS markers or mutating storage.
6. A warning-free compiler-oracle fixture returning 16 under Cust, GCC, and Clang.
7. Aggregate pointer-field indexing preserves pointee constness separately from pointer-slot and containing-aggregate constness.

## Follow-up

Pointer-valued fields reached through embedded aggregate-array elements remain a separate result-shape gap. A probe of `box.nodes[0].cursor = values + 1` currently reports `pointer value used as scalar`; add pointer replacement/compound-assignment support without weakening concrete pointee/const/pointer-slot checks.
