# Parenthesized function declarators

Date: 2026-06-24

## Scope

Cust accepts ordinary C parenthesized function declarator names for supported top-level definitions/prototypes and block-scope prototypes:

```c
int (add)(int left, int right);
static int (scale)(int value) { return value * 2; }
int call_local(void) { int (add)(int, int); return add(1, 2); }
int (main)(void) { return call_local(); }
```

This is syntax parity only; function pointer declarators such as `int (*callback)(int);` remain unsupported and keep the existing targeted diagnostic.

## Implementation notes

- Update `Parser::starts_function_definition()` after return-type/specifier scanning to accept both `Ident (` and `( Ident ) (` declarator shapes.
- Add a small `parse_function_declarator_name()` helper used by `parse_function_declaration()` so top-level functions and local prototypes share the same name parsing.
- Keep the existing `(` followed by `*` guard before parsing the function name; that path still reports `function pointer declarations are not supported`.
- Local function prototypes already reuse `parse_function_declaration()`, so the parser helper automatically covers block-scope prototypes.

## Tests

- Focused RED/GREEN: `cargo test --test interpreter parenthesized_function_declarators -- --nocapture`.
- Compiler oracle: add the warning-free fixture to `tests/c_compat.rs` and run `cargo test --test c_compat -- --nocapture`.
