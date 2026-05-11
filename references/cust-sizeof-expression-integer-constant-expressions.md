# `sizeof` expression operands in integer constant expressions

Date: 2026-05-11

## Summary

Cust now accepts expression-form `sizeof(...)` operands in the parser-side integer constant-expression evaluator used for enum initializer values and `switch case` labels. This complements the prior type-name `sizeof(type-name)` support.

## Implementation notes

- `parse_integer_constant_sizeof` keeps the existing type-name path when the parenthesized operand starts with a supported type name.
- Otherwise it parses the parenthesized operand as an ordinary expression, then folds only its size metadata with `sizeof_integer_constant_expr`.
- The helper validates nested syntax/known enum constants but does **not** evaluate scalar operators, so expressions such as `sizeof(1 / 0)` are accepted without triggering division-by-zero evaluation.
- Cust's deterministic type model still applies: scalar expression sizes are Cust `int` size (`8`), string literals use their NUL-terminated byte length, scalar compound literals use their declared scalar size, and array/aggregate compound literals use their fixed or inferred deterministic object size.

## Coverage

- `tests/fixtures/valid/switch_enum_case_labels.c` adds `MODE_SIZEOF_EXPR = sizeof(1 / 0) == sizeof(int) ? 14 : 15` and switches on that enum value.
- `tests/fixtures/compat/valid/switch_enum_case_labels.c` mirrors the same expression with an ABI-independent equality check rather than exact native integer sizes.
- Focused verification:
  - `cargo test --test interpreter supports_enum_constants_as_switch_case_labels -- --nocapture`
  - `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`

## Pitfalls

- Do not evaluate the `sizeof` expression operand while folding enum/switch constants. Recursing through expressions should validate shape/metadata, not compute runtime scalar values.
- Keep compiler-oracle checks ABI-independent: `sizeof(1 / 0) == sizeof(int)` is stable across Cust/native even though Cust's exact `int` size differs from native compilers.
- The parser does not maintain a full variable type environment, so expression-form `sizeof` support should remain deliberately scoped unless a future run adds parser-side declaration metadata for variables/functions.
