# Aggregate Field Parenthesized Pointer Declarator Diagnostics

Date: 2026-06-24

## Scope

Cust still does not support C function-pointer fields or parenthesized pointer declarators such as pointer-to-array fields inside `struct`/`union` definitions.

Covered unsupported forms:

```c
struct Hooks {
    int (*callback)(int);
};

struct Matrix {
    int (*row)[3];
};
```

## Root cause

`parse_aggregate_definition_body()` already handled ordinary `*field` pointer fields, pointer-to-pointer fields, pointer-array fields, bit-fields, and multidimensional field arrays. It did not mirror the generic declaration parser's early lookahead for `(` followed by `*` before expecting a field name, so `int (*callback)(int);` and `int (*row)[3];` fell through to:

```text
expected struct field name after type, found LParen
```

## Implementation pattern

Before calling `expect_ident_after("<kind> field name after type")` in the aggregate field declarator loop:

1. Check `!has_explicit_star && self.check(&Token::LParen) && matches!(self.peek_next(), Token::Star)`.
2. Use `parenthesized_pointer_declarator_is_function_at(self.pos)` to distinguish function-pointer fields.
3. Report:
   - `function pointer aggregate fields are not supported`
   - `parenthesized pointer aggregate fields are not supported`

This is parser-diagnostic polish only; do not add runtime support or field metadata for function pointers / pointer-to-array fields unless a future roadmap item explicitly designs those features.

## Verification

Focused RED/GREEN:

```bash
cargo test --test interpreter rejects_parenthesized_pointer_aggregate_fields_with_context -- --nocapture
```

The RED failure was the generic missing-field-name diagnostic. GREEN passed after adding the aggregate-field lookahead and invalid fixtures.
