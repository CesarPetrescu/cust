# Old-style function parameter diagnostics

2026-06-22 autonomous run.

Cust intentionally supports modern prototype-style function definitions and declarations, not K&R/old-style identifier-list function definitions such as:

```c
int add(x, y)
int x;
int y;
{
    return x + y;
}
```

Parser behavior:

- Detect an identifier-only parameter list entry only when the identifier is not a visible typedef alias, is followed by `,` or `)`, and the closing parameter list is followed by a declaration-looking token (`int`, `char`, `struct`, `const`, storage-class keywords, etc.).
- Report `old-style function parameter lists are not supported` at the first identifier-list parameter.
- Preserve the existing generic missing-type diagnostic for modern malformed prototypes/definitions such as `int identity(value) { ... }`, where `)` is followed by `{` rather than an old-style declaration list.

Focused verification:

```bash
cargo test --test interpreter rejects_old_style_function_parameter_lists_with_context -- --nocapture
cargo test --test interpreter reports_missing_parameter_types_before_parameter_names -- --nocapture
cargo test --test interpreter parameter -- --nocapture
```
