# Cust long long type spellings

Date: 2026-05-10

## Summary

Cust accepts C `long long` integer type spellings as parser-level aliases for its existing deterministic `int` storage model. This mirrors the prior `long`/`short` syntax parity work without adding native-width or unsigned-wraparound semantics.

Supported spellings include:

- `long long`
- `long long int`
- `signed long long`
- `signed long long int`
- `unsigned long long`
- `unsigned long long int`

They work wherever scalar integer type spellings are parsed: globals, locals, static locals, `for` declarations, function returns, parameters/prototypes, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` scalar/pointer/one-dimensional array type operands.

## Implementation notes

- Lexer support was already present through `Token::Long`, `Token::Signed`, and `Token::Unsigned`.
- `Parser::parse_decl_type` now consumes an optional second `long` before an optional trailing `int` for both bare and signed/unsigned long-long spellings.
- `Parser::starts_function_definition` mirrors the same token skipping so `long long f(...)` and `unsigned long long f(...)` are recognized as function signatures rather than declarations.
- Runtime remains unchanged: all supported `long long` spellings lower to `CType::Int` with Cust's `i64` scalar storage and deterministic `sizeof(int) == sizeof(long long) == 8`.

## Test coverage

- `tests/fixtures/valid/long_long_type_spellings.c` covers interpreter behavior including globals, locals, static locals, functions/prototypes, pointers, typedefs, casts, `for` declarations, and `sizeof` type operands.
- `tests/fixtures/compat/valid/long_long_type_spellings.c` covers warning-free native C compiler-oracle parity while avoiding ABI-sensitive native `sizeof(long long)` assertions.
- `tests/interpreter.rs::supports_long_long_type_spellings`
- `tests/c_compat.rs::supported_programs_match_c_compiler_exit_codes`

## Pitfalls

- Do not assert native byte sizes for `long long` in compiler-oracle fixtures; Cust intentionally reports deterministic interpreter integer size.
- Remember to update both `parse_decl_type` and signature lookahead (`starts_function_definition`) when adding multi-token type spellings. Otherwise top-level functions are misclassified and fail with declaration-oriented diagnostics.
