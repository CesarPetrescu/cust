# Unsupported floating-point type diagnostics

Date: 2026-07-01

Cust intentionally keeps its deterministic scalar model to `int`, `char`, and `_Bool`; C floating-point types are outside the supported subset.

## Implementation notes

- Lex `float` and `double` as dedicated tokens instead of ordinary identifiers so parser routes can reject them intentionally.
- Route declaration/type contexts into `parse_decl_type_with_embedded_qualifiers(...)`, then reject `Token::Float` / `Token::Double` with `Parser::error_at(...)` so diagnostics preserve the keyword location.
- Include unsupported floating types in top-level/local declaration starts, cast lookahead, and `sizeof` / `_Alignof` type-name starts. Without those route updates, malformed programs fall through to generic `unexpected token`, grouped-expression, or unknown-variable diagnostics before `parse_decl_type(...)` can issue the targeted unsupported-feature error.

## Coverage shape

Focused invalid coverage should include at least:

- top-level `float global;`
- block-local `double local;`
- parameter `int f(float value)`
- `sizeof(float)`
- `(double)1`
- aggregate field `struct S { float value; };`

All should report `floating-point types are not supported` at the `float`/`double` keyword. No runtime/native-oracle fixture is appropriate until Cust deliberately designs floating-point value semantics.
