# Star VLA array-length diagnostics

Date: 2026-06-28

Cust intentionally supports only fixed-size one-dimensional array declarators plus explicitly inferred `[]` declarations where the existing initializer paths can determine a deterministic length. C variable-length-array star declarators such as `int values[*]` stay outside the supported subset.

Implementation notes:

- Route `Token::Star` inside `Parser::expect_array_len()` to a targeted parser error before the generic integer-constant-expression parser runs.
- Diagnostic: `variable length array declarators are not supported` at the `*` token.
- This shared helper covers function prototype parameters, local/object array declarations, aggregate fields, and typedef array aliases.
- Preserve narrower existing diagnostics for empty `[]`, non-positive constant lengths, comma expressions in integer constant expressions, non-constant identifiers, flexible aggregate fields, and multidimensional array suffixes.

Verification pattern:

```bash
cargo test --test interpreter rejects_star_vla_array_lengths_with_context -- --nocapture
```

Native compiler-oracle fixtures are not appropriate for this unsupported-subset diagnostic. A local `cc -std=c11 -Wall -Wextra -Werror` smoke check confirmed that mixing `int values[*]` prototypes with ordinary array definitions can trigger `-Werror=vla-parameter`, so this remains interpreter-invalid coverage only.
