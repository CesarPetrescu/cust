# Cust `_Atomic` type qualifier/specifier syntax parity

Date: 2026-05-11

## Summary

Cust accepts C11 `_Atomic` syntax as parser-level metadata over the existing deterministic interpreter storage model.

Supported spellings:

- Bare qualifier form: `_Atomic int value;`, `_Atomic char marker;`, `_Atomic struct`/aggregate aliases through the existing qualified type paths.
- Type-specifier form: `_Atomic(int) value;`, `_Atomic(char) marker;`, and one-level pointer/array declarations over the resolved contained type.
- Typedefs: `typedef _Atomic int AtomicInt;`, `typedef _Atomic(char) AtomicChar;`.
- Parameters, locals, globals, and `for` initializer declarations.
- Function return type parsing is covered in interpreter-only fixtures; the compiler-oracle fixture intentionally avoids `_Atomic` function returns because native `cc -std=c11 -Wall -Wextra -Werror` reports `-Werror=ignored-qualifiers` for qualified return types.
- Type-query/cast contexts that already accept ordinary qualified types, e.g. `sizeof(_Atomic char)`.

Runtime semantics are intentionally no-op: `_Atomic` does not add native C atomic operations, memory ordering, lock-free guarantees, or storage layout changes. `const` remains the only qualifier with write-enforcement metadata.

## Implementation notes

- `Token::Atomic` is lexed from `_Atomic`.
- Bare `_Atomic` is consumed by the shared type-qualifier path only when it is not followed by `(`.
- `_Atomic(type-name)` is parsed in `parse_decl_type_with_embedded_qualifiers` by recursively parsing the contained type and returning the same `DeclType` metadata.
- Declaration-start and type-name lookahead sites include `Token::Atomic` so globals, locals, static/local storage-class wrappers, `for` declarations, casts, and `sizeof`/`_Alignof`-style type parsing stay consistent.

## Coverage

- Interpreter fixture: `tests/fixtures/valid/atomic_type_qualifiers.c`.
- C compiler-oracle fixture: `tests/fixtures/compat/valid/atomic_type_qualifiers.c`.
- Focused test: `cargo test --test interpreter supports_atomic_type_qualifiers -- --nocapture`.
- Full compiler-oracle test: `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`.

## Pitfalls

- Do not treat `_Atomic(type-name)` as a bare qualifier; consuming `_Atomic` before checking the following `(` makes declarations like `_Atomic(int) x;` fail at the parenthesis.
- Keep native compiler-oracle fixtures warning-free under `-std=c11 -Wall -Wextra -Werror`; compare exit behavior and stable relationships such as `sizeof(_Atomic char) == sizeof(char)`, not ABI-sensitive atomic integer sizes.
