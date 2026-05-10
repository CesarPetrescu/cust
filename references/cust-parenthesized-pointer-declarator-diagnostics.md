# Parenthesized pointer declarator diagnostics

Date: 2026-05-10

Cust deliberately supports the one-level non-parenthesized pointer subset (`int *p`, `struct T *p`, pointer typedef aliases, and array-parameter decay spellings) but not C's parenthesized pointer declarators.

This run added exact parser diagnostics for `(` followed by `*` immediately after a supported base type in two contexts:

- Parameters: `int sum(int (*row)[3])` now reports `parenthesized pointer parameters are not supported` at the opening parenthesis.
- Local/global declarations: `int (*row)[3];` now reports `parenthesized pointer declarations are not supported` at the opening parenthesis.

Implementation notes:

- The check belongs after `parse_const_qualified_decl_type(...)` and any explicit direct `*` handling, before calling the contextual missing-name helpers.
- Use `self.check(&Token::LParen) && matches!(self.peek_next(), Token::Star)` so ordinary malformed declarations keep the existing missing-name diagnostics.
- Focused RED test: `cargo test --test interpreter parenthesized_pointer -- --nocapture`.
- Fixtures: `tests/fixtures/invalid/parenthesized_pointer_parameter.c` and `tests/fixtures/invalid/parenthesized_pointer_declaration.c`.
