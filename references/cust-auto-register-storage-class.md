# Cust auto/register local storage-class specifiers

2026-05-11 autonomous run.

## Scope implemented

- Lexer recognizes C storage-class keywords `auto` and `register`.
- Parser accepts them as block-scope/local declaration prefixes only.
- They are lowered to the existing ordinary local declaration paths:
  - scalar declarations (`auto int`, `register _Bool`, signed/long/short spellings through existing type parsing)
  - pointer declarations (`register int *cursor`)
  - aggregate declarations (`auto struct Point point` / union aliases through existing aggregate variable parsing)
  - `for` initializer declarations (`for (auto int i = ...; ...)`, `for (register int j = ...; ...)`)
- Runtime behavior is deliberately unchanged from ordinary local automatic storage. Cust does not model native register allocation or address-taking restrictions.

## Coverage

- `tests/fixtures/valid/auto_register_storage_class.c`
- `tests/fixtures/compat/valid/auto_register_storage_class.c`
- `tests/interpreter.rs::supports_auto_and_register_local_storage_class_specifiers`
- Added to `tests/c_compat.rs` compiler-oracle fixture list.

## Pitfalls

- Keep these specifiers local/block-scope only unless a later roadmap item deliberately designs top-level `extern`/storage-class behavior.
- Native `register` variables cannot safely appear in fixtures where their address is taken; keep compiler-oracle fixtures warning-free under `-std=c11 -Wall -Wextra -Werror`.
- `register`/`auto` in function parameter declarations is not implemented in this run; add a separate TDD fixture before expanding parameter grammar.
