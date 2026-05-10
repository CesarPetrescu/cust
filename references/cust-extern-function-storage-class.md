# Top-level extern function storage-class syntax

2026-05-11 autonomous run.

Cust treats top-level `extern` on function prototypes and function definitions as single-file linkage metadata, matching the existing top-level `static` parser treatment. There is no runtime linkage model; the parser consumes the storage-class token before normal function/prototype classification and then reuses existing signature/prototype/definition compatibility checks.

Coverage:

- `tests/fixtures/valid/extern_function_storage_class.c`
- `tests/fixtures/compat/valid/extern_function_storage_class.c`
- `tests/interpreter.rs::supports_extern_function_storage_class_specifiers`
- `tests/c_compat.rs` compiler-oracle list

Implementation notes:

- Add `Token::Extern` and lex the `extern` keyword.
- In `Parser::parse_program`, consume one optional top-level `static` or `extern` storage-class specifier before the existing function/global/type-dispatch logic.
- Keep local `extern` declarations outside the supported subset for now; this feature is scoped to top-level function declarations/definitions.
- Avoid `extern int global = ...` in C compiler-oracle fixtures because native compilers may warn about initialized `extern` declarations under `-Wall -Wextra -Werror`.
