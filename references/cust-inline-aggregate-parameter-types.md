# Inline aggregate definitions in function parameter type specifiers

Date: 2026-06-26

## Symptom

`struct Pair { int x; int y; }` / `union Number { ... }` definitions in function parameter type specifiers parsed through the existing declaration-type path, but their tags were inserted into the surrounding file-scope aggregate tag map. That made function bodies parse, but incorrectly leaked parameter-list tags to later file-scope code.

## Decision

Treat function parameter-list aggregate definitions as a function-parameter tag scope:

1. In `parse_function_declaration()`, push a temporary aggregate type scope before `parse_params()`.
2. Keep that scope active while parsing the function body so declarations such as `struct Pair copy = point;` inside the body can resolve the parameter-list tag.
3. Pop the temporary scope after parsing the prototype/body so `struct Pair` from `int f(struct Pair { ... } p)` is not visible to later file-scope declarations.
4. Keep inline enum parameter constants on the existing separate pending-`EnumDecl` path; only aggregate tags use the temporary aggregate scope.

## Coverage

- Valid interpreter fixture: `tests/fixtures/valid/inline_aggregate_parameter_type_definitions.c` covers struct and union parameter definitions plus function-body declarations using those tags.
- Invalid interpreter fixture: `tests/fixtures/invalid/inline_aggregate_parameter_tag_leaks.c` verifies the parameter-list tag expires after the function definition.
- Native compiler-oracle coverage is intentionally skipped: this host's `cc -std=c11 -Wall -Wextra -Werror` warns that tags declared inside parameter lists are not visible outside the declaration, and warning-free calls are not practical for this syntax.
