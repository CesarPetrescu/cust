# Function Pointer Declarator Diagnostics

2026-05-11 autonomous parser-diagnostics run.

## Scope

Cust still does not implement function pointer declarations or function pointer typedef aliases, but it now recognizes the common C declarator shape early enough to avoid misleading missing-name diagnostics:

- `int (*callback)(int);` reports `function pointer declarations are not supported` at the opening parenthesis.
- `typedef int (*Callback)(int);` reports `function pointer typedef aliases are not supported` at the opening parenthesis.

Function pointer parameters such as `int apply(int (*fn)(int), int x)` were already covered by the existing `parenthesized pointer parameters are not supported` diagnostic.

## Implementation notes

- `parse_function_declaration` checks for `(` followed by `*` immediately after parsing the return type and before expecting a function name.
- `parse_typedef_decl` checks for `(` followed by `*` after resolving the base alias type and before expecting the alias name.
- The diagnostics are parser-only unsupported-form handling; no runtime function-pointer representation was added.

## Coverage

- `tests/fixtures/invalid/function_pointer_declaration.c`
- `tests/fixtures/invalid/function_pointer_typedef_alias.c`
- Focused test filter: `cargo test --test interpreter rejects_function_pointer -- --nocapture`
