# Cust Function Specifiers (`inline`, `_Noreturn`)

2026-05-11 autonomous run.

## Scope

Cust supports C function specifier syntax for top-level function declarations/prototypes:

- `inline int f(...) { ... }`
- `static inline int f(...);`
- `static inline int f(...) { ... }`
- `inline static int f(...) { ... }`
- `_Noreturn void f(void);`

Both `inline` and `_Noreturn` are parser-level metadata only. They do not alter Cust's single-file function table, call dispatch, recursion limits, return-shape checks, or runtime control flow.

## Implementation notes

- Lexer keywords map to `Token::Inline` and `Token::Noreturn`.
- `Parser::consume_function_specifiers()` consumes one or more top-level function specifiers before an optional top-level storage-class token (`static`/`extern`) and again after it, so common orders like `static inline int f` and `inline static int f` both parse.
- If a consumed function specifier is not followed by a function declaration/prototype, parsing reports `function specifiers are only supported on function declarations` instead of treating the specifier as an identifier.
- No runtime representation was added because existing `FunctionSignature`/`Function` behavior is unchanged.

## Fixtures

- `tests/fixtures/valid/function_specifiers.c`
- `tests/fixtures/compat/valid/function_specifiers.c`

The compat fixture uses a warning-free `static inline` helper definition and an unused `_Noreturn` prototype. Avoid defining a `_Noreturn` function that returns normally: native `cc -std=c11 -Wall -Wextra -Werror` rejects that with noreturn-related warnings/errors.
