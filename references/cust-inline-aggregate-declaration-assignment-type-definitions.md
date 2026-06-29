# Inline aggregate definitions in declaration and assignment expressions

Date: 2026-06-29

Cust's shared type-name parsing already installs inline named `struct` / `union` tags in the enclosing block when those definitions appear inside declaration-list initializer expressions or assignment RHS expressions.

Coverage pattern:

- Declaration-list initializer: `int ok = sizeof(struct DeclBox { int value; }) == sizeof(struct DeclBox), value = 4;` followed by `struct DeclBox box = {value};`.
- Declaration-list initializer with union: `sizeof(union DeclChoice { ... })` followed by a same-block `union DeclChoice` declaration.
- Assignment RHS: `total = total + (sizeof(struct AssignBox { ... }) == sizeof(struct AssignBox));` followed by same-block tag use.
- Compound assignment RHS: `total += sizeof(union AssignChoice { ... }) == sizeof(union AssignChoice);` followed by same-block tag use.

Focused interpreter coverage passed immediately because no production-code change was needed. Treat this as conformance coverage closure rather than inventing a runtime fix. Native compiler-oracle fixtures can stay warning-free by using ABI-independent relationships such as `sizeof(Tag definition) == sizeof(Tag)` and by initializing all aggregate objects before reading fields.
