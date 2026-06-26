# Inline enum definitions inside `_Alignof` type names and assignment statements

Date: 2026-06-27

Work package: inline enum definitions in ordinary `_Alignof(enum Tag { ... })` type-name expression contexts, plus the runtime statement wrapping gap exposed by assignment statements.

Root cause found by RED fixture:

- `parse_sizeof_like_type_name("_Alignof")` already parsed inline enum type definitions and queued their constants via `push_pending_inline_enum_constants(...)`.
- Expression statements and returns already called `with_pending_inline_enum_decl(...)`, so prior `sizeof` coverage passed in declaration initializers and returns.
- Plain and compound assignment statements (`x = _Alignof(enum E { A = 1 });`, `x += ...`) returned `Stmt::Assign` / `Stmt::Expr(CompoundAssign)` directly from `parse_assignment(...)`, leaving queued inline enum constants un-emitted. The following statement then failed at runtime with `undefined variable '<ENUMERATOR>'`.

Fix pattern:

- Wrap final plain assignment statements and scalar compound-assignment statements with `with_pending_inline_enum_decl(...)` after parsing the RHS and semicolon.
- This emits generated enum constants before evaluating the assignment statement, matching expression/return/static-assert/control/declaration-list handling.

Coverage notes:

- Interpreter fixture: `tests/fixtures/valid/inline_enum_alignof_type_definitions.c` covers `_Alignof(enum ...)` in a plain assignment RHS, declaration-list initializer, and return expression.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/inline_enum_alignof_type_definitions.c` is warning-free under the repository `cc -std=c11 -Wall -Wextra -Werror` path and checks `_Alignof(enum Tag { ... }) == _Alignof(enum Tag)` rather than exact native/Cust enum alignment bytes.
- Focused RED: `cargo test --test interpreter inline_enum_alignof_type_definitions -- --nocapture` initially failed with `undefined variable 'LOCAL_ALIGN'`.
