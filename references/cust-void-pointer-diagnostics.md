# Unsupported `void *` diagnostics

Date: 2026-07-01

Cust's pointer model explicitly excludes `void *` from the supported safe one-level pointer subset. Keep the rejection narrow and source-located at the `*` token so users can distinguish an unsupported void pointer from a malformed ordinary `void` function or `(void)expr` cast.

Covered forms:

- Return types: `void *make(void)` reports `void pointers are not supported` at `*` before generic function-name parsing.
- Parameters: `int f(void *slot)` reports the same diagnostic at `*` before the special empty-`void` parameter-list rule.
- Block declarations: `void *slot;` inside a block reports the same diagnostic before local function-prototype routing consumes `void`.
- Casts: `(void *)0` reports the same diagnostic before ordinary `(void)expr` cast parsing expects `)`.
- Type queries: `sizeof(void *)` and `_Alignof(void *)` report the same diagnostic at `*` before the older `sizeof(void)` / `_Alignof(void)` diagnostic at `void`.

Implementation notes:

- Use a token helper that recognizes `Token::Void` followed by `Token::Star` at the current parser position and returns the star token for location-preserving errors.
- Check it at top-level/function-return routing, local-statement routing, parameter-list `void` handling, cast parsing, and shared `sizeof`/`_Alignof` type-name parsing.
- Preserve valid `void` surfaces: `void f(void)`, empty `void` parameter lists, and scalar `(void)expr` casts.
