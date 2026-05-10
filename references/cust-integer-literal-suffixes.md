# C integer literal suffixes

2026-05-10 autonomous run.

Cust now accepts the standard C integer suffix spellings for the existing deterministic integer literal model: `u`/`U`, `l`/`L`, `ll`/`LL`, and unsigned-long combinations such as `UL`, `lu`, `uL`, and `LLU`.

Implementation notes:

- The lexer parses the numeric value from the existing decimal/octal/hex digit span, then consumes a recognized suffix without changing the stored `i64` value.
- This keeps Cust's current storage semantics intact: suffixes are source-compatible syntax only, not a native-width/type-system change.
- The helper is intentionally lexer-local (`consume_integer_suffix`) so parser/runtime code continues to see only `Token::Number(value)`.

Coverage:

- `tests/fixtures/valid/integer_literal_suffixes.c`
- `tests/fixtures/compat/valid/integer_literal_suffixes.c`
- `tests/interpreter.rs::supports_c_integer_literal_suffixes`
- `tests/c_compat.rs::supported_programs_match_c_compiler_exit_codes`
