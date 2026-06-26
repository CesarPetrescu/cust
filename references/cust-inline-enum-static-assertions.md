# Inline enum definitions in `_Static_assert` conditions

Date: 2026-06-26

## Problem

`_Static_assert` parses its condition with assignment-precedence parsing so the separating comma before the message remains available. That condition can still contain C type specifiers with inline enum definitions, for example:

```c
_Static_assert(sizeof(enum TopLevelAssertEnum { TOP_ASSERT_VALUE = 13 }) == sizeof(int), "ok");
int main(void) { return TOP_ASSERT_VALUE; }
```

Before this fix, `parse_decl_type()` pushed the inline enum constants into `pending_inline_enum_constants`, but `parse_static_assert()` returned a bare `Stmt::StaticAssert`. Runtime evaluation of later statements therefore failed with `undefined variable 'TOP_ASSERT_VALUE'` because the generated enum declaration was never emitted.

## Fix pattern

After parsing the full `_Static_assert(condition, "message");`, wrap the resulting static-assert statement with `with_pending_inline_enum_decl(...)`, just like ordinary declarations/expression statements that may parse inline enum type specifiers:

```rust
Ok(self.with_pending_inline_enum_decl(Stmt::StaticAssert { condition, message }))
```

This prepends `Stmt::EnumDecl` before the `Stmt::StaticAssert`, so the assertion condition and later statements can resolve the enumerators at runtime.

## Coverage

- `tests/fixtures/valid/static_assertions.c` covers top-level and block-scope `_Static_assert(sizeof(enum Name { ... }) == sizeof(int), ...)` and later use of both generated constants.
- `tests/fixtures/compat/valid/static_assertions.c` mirrors the same warning-free C11 pattern for the native compiler oracle.

## Pitfalls

- Do not parse the `_Static_assert` condition with the full comma-expression parser; keep assignment-precedence parsing so the assertion-message comma is not consumed.
- The enum declaration must execute before the static assertion itself, not only before later ordinary statements, because the condition may combine the type definition and enumerator use in one expression.
