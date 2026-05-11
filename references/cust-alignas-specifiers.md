# C11 `_Alignas` declaration specifiers

Date: 2026-05-11

Cust accepts `_Alignas(...)` as parser-level declaration metadata for the supported subset. The alignment request does not affect runtime layout, `sizeof`, `_Alignof`, or pointer behavior because Cust uses deterministic interpreter sizes and no native ABI padding.

## Supported forms

- Top-level object declarations: `_Alignas(8) int global;`
- Block-scope locals: `_Alignas(8) int local = 1;`
- Static locals after the storage-class specifier: `static _Alignas(8) int saved = 1;`
- `for` initializer declarations: `for (_Alignas(8) int i = 0; ... )`
- Struct/union fields: `_Alignas(int) char tag;`
- Specifier argument syntax accepts either supported type names via the existing `sizeof`/`_Alignof` type-name parser or ordinary assignment-precedence expressions for integer constant-like forms.

## Implementation notes

- Lexer maps `_Alignas` to `Token::Alignas`.
- `consume_alignment_specifiers` parses and discards one or more alignment specifiers before declaration parsing.
- The parser routes `_Alignas` through top-level declarations, local declarations, static/auto/register declarations, `for` declaration initializers, and aggregate fields.
- Runtime intentionally has no additional storage metadata for requested alignment.

## Verification

- Interpreter fixture: `tests/fixtures/valid/alignas_specifiers.c` returns 16.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/alignas_specifiers.c` is warning-free under `cc -std=c11 -Wall -Wextra -Werror` and returns the same exit code.
