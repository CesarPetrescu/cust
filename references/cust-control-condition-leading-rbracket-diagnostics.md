# Contextual Leading-`]` Control-Condition Diagnostics

Use this note when maintaining `if`, `while`, `do-while`, and `switch` condition diagnostics.

- A closing bracket cannot begin a Cust expression. `if (])`, `while (])`, `do { } while (]);`, and `switch (])` must report `expected expression after <context>, found RBracket` at the bracket token instead of falling through to generic primary-expression parsing.
- Keep `Token::RBracket` in both `reject_missing_control_condition_expr()` and `reject_missing_switch_expr()` before the invalid-operator guards. This preserves structural diagnostic precedence for all four control forms.
- Do not reject brackets after a valid primary: Cust supports subscripts. Retain execution coverage for `values[index]` in each control condition.
- Verify with `cargo test --test interpreter rejects_missing_control_flow_condition_expressions_with_context -- --nocapture`, `cargo test --test interpreter rejects_invalid_start_switch_expressions_with_context -- --nocapture`, and `cargo test --test interpreter supports_subscript_expressions_in_control_conditions -- --nocapture`.
- Follow up only with demonstrated generic fallbacks for other postfix-only starts; do not broaden the pre-expression guards in ways that reject legal postfix expressions.
