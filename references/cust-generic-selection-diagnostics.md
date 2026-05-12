# Unsupported C11 `_Generic` selection diagnostics

Date: 2026-05-12

Cust remains a deliberately scoped interpreter and does not implement C11 generic selections. `_Generic(...)` previously lexed as an identifier, so expressions such as `return _Generic(1, int: 2, default: 3);` fell through into function-call parsing and reported a misleading downstream `expected expression, found Int` diagnostic at the first association type.

Implementation notes:

- Add a dedicated lexer keyword token for `_Generic` rather than treating it as an ordinary identifier.
- Reject it in `parse_primary` with `generic selections are not supported` at the `_Generic` keyword location.
- Do not parse the generic association list or add type-dispatch semantics.
- Focused regression: `cargo test --test interpreter rejects_generic_selections_with_context -- --nocapture`.

Fixture:

- `tests/fixtures/invalid/generic_selection.c`
