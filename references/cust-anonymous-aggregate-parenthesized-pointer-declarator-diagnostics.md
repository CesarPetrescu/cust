# Anonymous aggregate parenthesized pointer declarator diagnostics

Date: 2026-06-23

Cust does not support parenthesized pointer declarators such as `int (*row)[3]` or function-pointer declarators. Anonymous aggregate object declaration parsing has its own `parse_aggregate_var_decl_after_type` path, so it must mirror the generic declaration parser's early lookahead before expecting an aggregate variable name.

Relevant unsupported form:

```c
int main(void) { struct { int x; } (*slot); return 0; }
```

Before the fix, this fell through to `expected struct variable name, found LParen`. The parser now checks `(` followed by `*` immediately after a named or anonymous aggregate type and reports `parenthesized pointer declarations are not supported` at the `(`. If the parenthesized declarator is function-shaped, reuse the existing `parenthesized_pointer_declarator_is_function_at(self.pos)` check and report `function pointer declarations are not supported`.

Focused verification:

```bash
cargo test --test interpreter anonymous_aggregate_parenthesized_pointer -- --nocapture
```
