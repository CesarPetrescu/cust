# Cust postfix/interleaved const qualifiers

Date: 2026-05-11

## Scope

Cust accepts C declaration qualifier spellings where `const`/`volatile`/`restrict` appear after supported base type spellings, not only before them. The implemented const-preserving subset includes:

- `int const local`, `char const marker`, `_Bool const flag`
- `unsigned const int value`, `long const int value`, and related signed/unsigned/long/short spellings
- `int const *view` as a pointer to const int
- `struct Point const point` and const-qualified aggregate parameters
- `typedef int const ConstInt` and `typedef struct Point const ConstPoint`
- interleaved-qualified function definitions such as `unsigned const int helper(...)`

`volatile` and `restrict` remain parser-level no-op qualifiers except where existing `const` metadata is present.

## Implementation notes

- `parse_decl_type_with_embedded_qualifiers` returns both the parsed `DeclType` and whether a qualifier sequence around the base type contained `const`.
- `parse_const_qualified_decl_type` ORs together leading const, embedded/postfix const, and const typedef-alias metadata.
- `parse_aggregate_var_decl` consumes qualifiers after `struct/union Tag`; the resulting const bit feeds aggregate variable/array read-only metadata and pointer-to-const metadata for `struct T const *p`.
- `starts_function_definition` must skip type qualifiers between declaration-specifier pieces, not just before the first type token or after the whole type. Otherwise forms like `unsigned const int helper(...)` are misclassified as variable declarations and fail near `(`.

## Test coverage

- `tests/fixtures/valid/postfix_const_qualifiers.c`
- `tests/fixtures/compat/valid/postfix_const_qualifiers.c`
- `tests/interpreter.rs::supports_postfix_const_qualifiers`
- `tests/c_compat.rs::supported_programs_match_c_compiler_exit_codes`

## Pitfalls

- Native `cc -std=c11 -Wall -Wextra -Werror` rejects top-level const-qualified function return types as ignored qualifiers. Keep `int const f(...)` coverage interpreter-only, or use non-qualified return types in compiler-oracle fixtures.
- When adding more declaration specifier permutations, update both the parser (`parse_decl_type_with_embedded_qualifiers`) and function-definition lookahead (`starts_function_definition`) together.
