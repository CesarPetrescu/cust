# Cust object-like macro undefinition

Use this note when extending or reviewing Cust's bounded one-line `#undef` implementation.

## Supported semantics

- `#undef NAME` removes `NAME` from the translation-unit object-macro table before subsequent source tokens are lexed.
- Undefining an unknown identifier is harmless.
- A later `#define NAME ...` begins a fresh definition lifetime and may use a replacement list that differed from the removed definition.
- Existing replacement lists retain preprocessing-token identifiers, so aliases rescan against the current macro table: an alias used while its nested name is undefined emits that identifier normally, while a later use after redefinition expands through the new definition.

These decisions follow ISO C11 draft N1570 §6.10.3.5.

## Parser/lexer boundaries

1. Reuse `skip_preprocessor_whitespace()` before and after the identifier so spaces, tabs, vertical tab, form feed, carriage return, line comments, and bounded same-line block comments have the same treatment as `#define` directive whitespace.
2. Parse the macro name as a preprocessing identifier without macro-expanding it.
3. Require a separator after `undef`, then exactly one identifier. Missing and non-identifier names report `expected macro name after '#undef'` at the current source column.
4. After the identifier, consume only preprocessing whitespace/comments. Report `unexpected tokens after '#undef' macro name` at the first remaining token.
5. Validate the complete directive before removing the table entry. A malformed directive must not partially mutate macro state even though Cust stops at the first lexer error.
6. Keep one-line continuation rejection in the shared directive preflight so `#undef NAME \\` retains the existing exact unsupported-continuation diagnostic.

## TDD and verification

Use vertical RED/GREEN tracers:

1. Valid definition lifetime: known removal, ordinary identifier use while undefined, unknown removal, later different redefinition, and alias rescanning.
2. Missing/non-identifier names with exact source context.
3. Trailing tokens with an exact caret at the first offending token.

Register one warning-free C11 fixture in `tests/c_compat.rs`, then run the actual `c_compat` test function rather than filtering by fixture name. Obtain independent review before the canonical local/Docker gate, update README/changelog and every status file, and only then commit/push.
