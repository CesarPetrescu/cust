# Pointer Return Function Model

Last updated: 2026-05-07

Cust supports a deliberately safe subset of C pointer-returning functions without exposing host addresses.

## Supported syntax

- Scalar pointer returns: `int *choose(int *a, int *b) { return a; }` and `char *tail(char *text) { return text + 1; }`.
- Const-pointee returns: `const int *view(const int *value) { return value; }` preserve read-only pointer metadata.
- Aggregate pointer returns after a prior declaration: `struct Point *pick(struct Point *points, int i)` and `union Number *pick(union Number *values, int i)`.
- Pointer typedef aliases may be used as return types, for example `typedef int *IntPtr; IntPtr id(IntPtr value) { return value; }`.
- Prototypes use the same return-type signature checks as other function declarations.

## Runtime model

Pointer return values are `PointerValue` metadata plus declared `PointeeType` and `points_to_const` metadata. Returning a pointer copies this metadata by value; it never materializes a host memory address. Existing live-scope checks still apply when callers dereference a returned pointer, so returning a pointer to an expired local continues to fail through the existing out-of-scope diagnostics.

Return validation enforces the same conversion boundaries as pointer declarations, assignments, pointer fields, and pointer parameters:

- null returns are compatible with every pointer return type;
- concrete pointee type mismatches report targeted diagnostics such as `cannot convert pointer to char to pointer to int`;
- returning a const-pointee expression from a mutable pointer-returning function reports `cannot discard const qualifier from pointer target`;
- pointer-to-pointer return types remain unsupported.

## Acceptance coverage

- `tests/fixtures/valid/pointer_return_functions.c` covers scalar, char, const, struct, union, and typedef-spelled pointer returns.
- `tests/fixtures/compat/valid/pointer_return_functions.c` compares the supported subset against a native C compiler as an external oracle.
- `tests/fixtures/invalid/pointer_return_type_mismatch.c` and `tests/fixtures/invalid/pointer_return_const_discard.c` cover conversion diagnostics.
- `tests/interpreter.rs` keeps pointer-to-pointer return parsing rejected with an exact source-location diagnostic.
