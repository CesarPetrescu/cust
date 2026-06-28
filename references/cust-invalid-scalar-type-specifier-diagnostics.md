# Invalid scalar type specifier diagnostics

Date: 2026-06-28

Cust accepts many C scalar type spelling permutations by normalizing them to its deterministic `int`, `char`, or `_Bool` model. Invalid combinations such as `signed unsigned int`, `long long long`, `short long`, `char int`, and `_Bool unsigned` must remain unsupported.

Implementation note: keep `LocatedToken` metadata through `parse_scalar_decl_type_specifiers(...)` rather than passing only `Token` values. When the aggregate validity check fails, replay the consumed scalar specifier tokens with `invalid_scalar_type_specifier_token(...)` and report the token that first made the combination invalid via `Parser::error_at(...)`.

Regression fixture:

```c
int main(void) {
    signed unsigned int value = 1;
    return value;
}
```

Expected diagnostic:

```text
invalid scalar type specifier combination at line 2, column 12
```

No external documentation was needed; this is parser diagnostic polish for an already-unsupported malformed declaration shape.
