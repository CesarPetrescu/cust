# Cust `_Bool` type spellings

2026-05-10 autonomous run.

## Implemented scope

- Lexer recognizes C99 `_Bool` as a distinct type token.
- Parser lowers `_Bool` declarations to `CType::Bool` in the same scalar family as Cust `int`/`char`.
- `_Bool` is accepted in globals, locals, static locals, `for` declarations, function returns, prototypes, parameters, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` scalar/pointer/one-dimensional array type operands.
- Cust defines `sizeof(_Bool) == 1`, matching its deterministic no-native-ABI scalar-size model.

## Intentional boundary

- This run is syntax/type-spelling parity over Cust scalar storage. It does not add broader C integer conversion/range semantics beyond the existing deterministic interpreter model.
- Compiler-oracle fixtures avoid depending on exact native `_Bool` byte size except through native self-relationships like `sizeof(_Bool[2]) == sizeof(_Bool) * 2`.

## Coverage

- Interpreter fixture: `tests/fixtures/valid/bool_type_spellings.c`.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/bool_type_spellings.c`.
- Focused tests:
  - `cargo test --test interpreter supports_bool_type_spellings -- --nocapture`
  - `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`

## Pitfalls

- Update both parser type starts and function-signature lookahead when adding scalar type tokens; otherwise `_Bool f(...)` can be misclassified as a top-level variable declaration.
- Keep native fixtures warning-free under `-Wall -Wextra -Werror`; use all parameters and avoid ABI-sensitive exact-size exits.
