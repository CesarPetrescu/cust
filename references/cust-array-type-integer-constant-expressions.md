# Array type-name integer constant expressions

Date: 2026-06-28

## Context

Cust already routed `sizeof`/`_Alignof` array type-name suffix lengths through the shared `expect_array_len()` helper, so array type operands could accept the same parser-folded integer constant expression subset as declarations.

## Coverage added

- `tests/fixtures/valid/array_type_integer_constant_expressions.c`
- `tests/fixtures/compat/valid/array_type_integer_constant_expressions.c`
- `supports_integer_constant_expressions_for_array_type_lengths`

The fixture covers enum constants, `sizeof` expression forms, conditional expressions, typedef element names, direct aggregate typedefs, and `_Alignof(T[N])` relationships.

## Notes

Focused interpreter and native compiler-oracle coverage passed immediately, so this was recorded as conformance coverage rather than a production-code fix. Keep native oracle assertions ABI-independent: compare relationships such as `sizeof(T[N]) == N * sizeof(T)` and `_Alignof(T[N]) == _Alignof(T)`, not Cust's deterministic byte sizes.
