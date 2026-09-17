# Contextual Switch Leading-Comma Diagnostic

Use this note when maintaining `switch (...)` controlling-expression diagnostics.

- A comma cannot begin a Cust expression, so `switch (,)` must report `expected expression after switch, found Comma` at the comma token instead of the generic primary-expression error.
- Keep `Token::Comma` in `reject_missing_switch_expr()` rather than the shared operator guard: it preserves switch-specific structural diagnostic precedence and does not change the supported comma operator after a valid expression.
- Verify with `cargo test --test interpreter rejects_invalid_start_switch_expressions_with_context -- --nocapture`; retain `supports_unary_grouped_scalar_and_enum_switch_expressions` as positive coverage for unary, grouped, scalar, enum, and postfix-member selectors.
- The remaining leading operator tokens are either already rejected by `reject_invalid_control_condition_expr("switch")` or valid Cust unary prefixes that must continue into their own operand diagnostics.
