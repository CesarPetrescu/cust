# Anonymous aggregate parameter diagnostics

Date: 2026-06-22

Cust supports anonymous aggregate object declarations and typedef-backed anonymous aggregate types, but it intentionally does not support anonymous `struct { ... }` / `union { ... }` types directly in function parameter lists. Those forms create unnameable parameter types that are awkward to use consistently with Cust's current parser-only aggregate identity model.

## Diagnostic pattern

- Fixture: `tests/fixtures/invalid/anonymous_aggregate_parameter.c`
- Focused test: `cargo test --test interpreter rejects_anonymous_aggregate_parameters_with_context -- --nocapture`
- Expected error: `anonymous aggregate parameters are not supported` at the `struct` / `union` keyword, before `parse_decl_type` consumes the keyword and falls through to a generic `expected parameter type, found LBrace` message.

## Implementation note

`Parser::parse_params` checks `anonymous_aggregate_parameter_token()` before parsing the declaration type. The helper skips leading type qualifiers so qualified forms keep the same targeted boundary, then detects `struct`/`union` followed immediately by `{`.
