# Inline named aggregate definitions in expression type contexts

2026-06-26 autonomous run notes.

Cust's existing `parse_decl_type` path already handles named `struct`/`union` definitions in cast/type-name positions, so expression-local forms such as:

```c
((struct ExprPoint { int x; int y; }){3, 4}).x
struct ExprPoint point = {5, 6};
sizeof(struct SizeBox { char tag; int value; }) == sizeof(struct SizeBox)
```

work without production-code changes. Treat this as conformance coverage, not a runtime fix, when focused interpreter tests pass immediately.

Coverage guidance:

- Include both `struct` and `union` compound literals with later declarations using the newly defined tag in the same block.
- Include a type-query form (`sizeof(struct Tag { ... })`) and use only ABI-independent relationships in native compiler-oracle fixtures.
- Native `cc -std=c11 -Wall -Wextra -Werror` accepts these warning-free when every declared object is read and no native aggregate layout byte count is asserted.
