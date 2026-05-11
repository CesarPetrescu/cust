# Casts in integer constant expressions

Date: 2026-05-11

Cust now folds scalar type-name casts inside the parser-side integer constant-expression evaluator used for enum initializer values and `switch case` labels.

## Implementation notes

- `parse_integer_constant_unary` recognizes parenthesized type names with the same conservative lookahead used by normal cast parsing.
- `parse_integer_constant_cast` accepts scalar `DeclType` forms, including scalar typedef aliases such as `typedef int Count;` and `typedef char Small;`, consumes the closing `)`, then folds the following unary integer constant expression.
- Casts are parser-only and keep Cust's existing deterministic scalar-cast behavior: they preserve the folded `i64` value instead of adding native-width truncation or signedness semantics.
- Pointer, pointer-alias, array-alias, and aggregate casts stay outside this supported constant-expression subset and report the existing targeted unsupported cast diagnostics rather than silently folding.

## Coverage

- `tests/fixtures/valid/switch_enum_case_labels.c` covers scalar typedef casts in both enum initializer values and `switch case` labels.
- `tests/fixtures/compat/valid/switch_enum_case_labels.c` mirrors those forms for native C compiler-oracle verification while keeping the total exit code below 256.

## Pitfalls

- Keep constant-expression parsing delimiter-aware. A cast expression should consume only a unary RHS; do not call the full comma-expression parser from enum initializer or case-label folding without a concrete delimiter-preservation design.
- Avoid native ABI-sensitive assertions. The current fixture casts small values through `int`/`char` typedefs without depending on native `char` truncation or signedness.
