# `sizeof` and `_Alignof` integer constant expressions

Date: 2026-05-11

Cust now folds `sizeof(type-name)` and `_Alignof(type-name)` inside the parser-side integer constant-expression evaluator used for enum initializer values and `switch case` labels.

## Implementation notes

- `parse_integer_constant_unary` routes `sizeof` and `_Alignof` before primary constants so these operators bind at unary precedence with the existing arithmetic/comparison/logical/conditional constant-expression helpers.
- The implementation reuses `parse_sizeof_like_type_name(...)`, then calls `SizeOfType::size(...)` or `SizeOfType::alignment(...)` against the parser's aggregate metadata. This keeps results aligned with Cust's deterministic type model rather than native ABI sizes.
- The supported constant-expression form is type-name based: `sizeof(char[5])`, `sizeof(struct Point)`, `_Alignof(char)`, `_Alignof(Point[2])`, etc. Runtime-expression `sizeof expr` remains outside this parser-only folding path until a concrete fixture demands it.
- The helper remains non-evaluating and parser-only; identifiers in enum/case constant expressions are still restricted to visible enum constants.

## Coverage

- `tests/fixtures/valid/switch_enum_case_labels.c` now covers enum initializers and switch case labels using `sizeof(char[N]) + _Alignof(char)`.
- `tests/fixtures/compat/valid/switch_enum_case_labels.c` mirrors the forms for native C compiler-oracle verification.

## Pitfalls

- Keep compiler-oracle exit totals below 256; native process exit codes truncate while Cust returns the full interpreted integer.
- Use ABI-independent native fixture forms for compiler-oracle coverage. `char` arrays and `_Alignof(char)` are stable; exact native `sizeof(int)`, aggregate padding, or pointer alignment should not be compared directly against Cust's deterministic model.
- If expression-form `sizeof` is added later, preserve non-evaluation semantics and avoid reusing the full runtime expression parser in contexts where delimiter commas or `case ...:` boundaries matter.
