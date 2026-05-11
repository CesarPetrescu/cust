# Function Pointer Declarator Diagnostics

2026-05-11 autonomous parser-diagnostics run.

## Scope

Cust still does not implement function pointer declarations, function pointer parameters, or function pointer typedef aliases, but it now recognizes the common C declarator shape early enough to avoid misleading missing-name or overly broad parenthesized-pointer diagnostics:

- `int (*callback)(int);` reports `function pointer declarations are not supported` at the opening parenthesis.
- `int main(void) { int (*callback)(int); }` reports `function pointer declarations are not supported` at the opening parenthesis.
- `int apply(int (*callback)(int), int value)` reports `function pointer parameters are not supported` at the opening parenthesis.
- `typedef int (*Callback)(int);` reports `function pointer typedef aliases are not supported` at the opening parenthesis.

## Implementation notes

- `parse_function_declaration` checks for `(` followed by `*` immediately after parsing the return type and before expecting a function name.
- `parse_typedef_decl` checks for `(` followed by `*` after resolving the base alias type and before expecting the alias name.
- `parenthesized_pointer_declarator_is_function_at` distinguishes unsupported function-pointer declarator syntax from pointer-to-array parenthesized pointer declarators by looking for `(*name)(...)` before the existing declaration/parameter diagnostics run.
- The diagnostics are parser-only unsupported-form handling; no runtime function-pointer representation was added.

## Coverage

- `tests/fixtures/invalid/function_pointer_declaration.c`
- `tests/fixtures/invalid/function_pointer_local_declaration.c`
- `tests/fixtures/invalid/function_pointer_parameter.c`
- `tests/fixtures/invalid/function_pointer_typedef_alias.c`
- Focused test filter: `cargo test --test interpreter function_pointer -- --nocapture`
