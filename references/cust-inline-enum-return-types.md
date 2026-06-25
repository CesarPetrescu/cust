# Cust inline enum return type definitions

Date: 2026-06-25

## Problem

Cust supported inline enum object declarations (`enum { A = 1 } value`) and direct named enum return types (`enum State f(void)` after a prior declaration), but top-level function return type declarations that define the enum in place were misrouted.

Example that failed before this fix:

```c
enum Status { STATUS_READY = 5, STATUS_BUSY = 7 } choose_status(int flag);

enum Status choose_status(int flag) {
    return flag ? STATUS_BUSY : STATUS_READY;
}
```

The parser's function-definition lookahead treated `enum` return types as only `enum Ident`, so `enum Ident { ... } function(...)` fell through to variable declaration parsing and reported `expected '=' after variable declaration, found LParen`.

## Fix pattern

- Teach `starts_function_definition()` to skip either `enum Tag` or `enum Tag { ... }` / `enum { ... }` as a scalar return type specifier before checking the function declarator.
- When `parse_function_return_type()` parses an inline enum definition, immediately take the pending `EnumDecl` and return it to `parse_program()` so file-scope enumerators are installed before `main()` executes.
- Clear `pending_inline_enum_constants` after parameter parsing so enum definitions encountered only in parameter lists do not leak as file-scope runtime constants by accident.

## Tests

- Add an interpreter fixture covering inline enum return-type prototype/definition, file-scope enumerator visibility after the prototype, and a parenthesized function declarator name for one inline enum return definition.
- Add a warning-free C compiler-oracle fixture. Keep the oracle fixture's inline enum definition in the prototype and use the named enum tag for the definition to avoid redefinition.

Focused RED/GREEN command:

```bash
cargo test --test interpreter supports_inline_enum_return_type_definitions -- --nocapture
```

Compiler oracle command:

```bash
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
