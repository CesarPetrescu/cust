# Contextual Switch Selector-Token Diagnostics

Use this note when maintaining the controlling expression in `switch (...)`.

- `switch (.)` and `switch (->field)` cannot begin a Cust expression. Run `cargo test --test interpreter rejects_invalid_start_switch_expressions_with_context -- --nocapture`; both must report `expected expression after switch, found Dot/Arrow` at the selector token.
- Keep `Token::Dot` and `Token::Arrow` in `reject_missing_switch_expr()` with the existing delimiter-only starts. This preserves switch-specific diagnostic precedence without changing generic postfix parsing.
- Retain a positive member-access selector, such as `switch (choice.value)`, because dots and arrows remain valid after a primary expression.
