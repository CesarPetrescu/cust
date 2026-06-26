# Inline enum definitions in `sizeof` type-name expressions

Date: 2026-06-26

## Context

A conformance-closure run audited inline enum definitions in ordinary expression-form type queries such as:

```c
int first_size = sizeof(enum LocalSize { LOCAL_SIZE = 7 });
int same = first_size == sizeof(enum LocalSize);
int decl_size = sizeof(enum DeclSize { DECL_SIZE = 11 }), mirror = DECL_SIZE;
```

This is adjacent to prior inline enum work for cast type specifiers, control expressions, `_Static_assert`, and `switch case` labels.

## Finding

No production parser/runtime change was needed. The existing `sizeof` type-name parser path records the inline enum definition and the existing pending-inline-enum statement wrapper emits the generated `EnumDecl` before later runtime statements in expression, declaration-list initializer, and return contexts.

## Fixture pattern

- Interpreter fixture: `tests/fixtures/valid/inline_enum_sizeof_type_definitions.c`
- Native oracle fixture: `tests/fixtures/compat/valid/inline_enum_sizeof_type_definitions.c`
- Keep native assertions ABI-independent: compare `sizeof(enum Tag { ... }) == sizeof(enum Tag)` and use enumerator values for exit-code behavior rather than assuming a native enum byte size.

Focused coverage may pass immediately. Treat that as valid conformance closure, not a production-code fix.
