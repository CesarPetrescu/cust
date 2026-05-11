# Cust static assertions

2026-05-11 autonomous run: Cust supports C11 `_Static_assert(condition, "message");` at top level and inside block/function scopes.

Implementation notes:
- Lexer recognizes `_Static_assert` as a keyword token.
- Parser lowers assertions to `Stmt::StaticAssert { condition, message }` and parses the condition with assignment-precedence parsing rather than full comma-expression parsing so the comma remains the assertion argument separator.
- Top-level assertions are stored in `Program::globals`, so they execute before `main()` along with other top-level declarations.
- Runtime uses existing Cust truthiness evaluation for the condition; false conditions report `static assertion failed: <message>`.

Fixture notes:
- Interpreter coverage: `tests/fixtures/valid/static_assertions.c` and `tests/fixtures/invalid/static_assertion_failure.c`.
- Compiler-oracle coverage: `tests/fixtures/compat/valid/static_assertions.c` uses only true assertions and ABI-stable size relationships.
