# Array return type diagnostics

Date: 2026-06-24

Cust intentionally excludes C function declarations that return arrays. Direct array-return declarator suffixes such as:

```c
int make(void)[2] { return 0; }
```

used to parse the `int make(void)` function header first and then fall through to the generic block parser, producing `expected '{' after function header, found LBracket`.

## Implementation note

After `parse_function_declaration()` consumes the closing `)` for the parameter list, check for `Token::LBracket` before accepting either `;` prototypes or `{` definitions, and report:

```text
array return types are not supported
```

at the `[` token. This covers ordinary direct array-return declarator suffixes while preserving typedef-backed array return diagnostics emitted by `parse_function_return_type()` when a return type alias resolves to `DeclType::Array`.

## Verification

Focused regression:

```bash
cargo test --test interpreter rejects_array_return_types_with_context -- --nocapture
```
