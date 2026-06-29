# Inline aggregate definitions in expression statements

Date: 2026-06-29

Cust's shared type-name and aggregate compound-literal parsing already installs inline named `struct` / `union` tags in the enclosing block when those definitions appear inside expression statements whose value is discarded with `(void)`.

Coverage pattern:

- Void-cast expression statement with a type query: `(void)(sizeof(struct VoidBox { int value; }) == sizeof(struct VoidBox));` followed by `struct VoidBox box = {...};` in the same block.
- Void-cast expression statement over an aggregate compound literal field: `(void)((struct LiteralBox { int value; }){7}).value;` followed by `struct LiteralBox` declarations in the same block.
- Matching union type-query coverage: `(void)(sizeof(union VoidChoice { int value; char tag; }) == sizeof(union VoidChoice));` followed by a same-block union declaration.

Focused interpreter coverage passed immediately because no production-code change was needed. Treat this as conformance coverage closure rather than inventing a runtime fix. Native compiler-oracle fixtures can remain warning-free under `-Wall -Wextra -Werror` by casting the expression statement to `void` and using ABI-independent `sizeof(Tag definition) == sizeof(Tag)` relationships instead of exact aggregate sizes.
