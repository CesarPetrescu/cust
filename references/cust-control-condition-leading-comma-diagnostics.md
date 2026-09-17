# Contextual Leading-Comma Control-Condition Diagnostics

Use this note when maintaining `if`, `while`, and `do-while` condition diagnostics.

- A comma cannot begin a Cust expression. `if (,)`, `while (,)`, and `do { } while (,);` must report `expected expression after <context>, found Comma` at the comma token instead of falling through to generic primary-expression parsing.
- Keep `Token::Comma` in `reject_missing_control_condition_expr()` before `reject_invalid_control_condition_expr()`. This preserves the common structural/keyword/operator diagnostic precedence across all three controls.
- Do not reject commas after a valid primary: Cust supports the comma operator. Retain a positive program that evaluates `if (0, 1)`, `while (value, value < 2)`, and `do { ... } while (value, value < 4)`.
- Verify with `cargo test --test interpreter rejects_missing_control_flow_condition_expressions_with_context -- --nocapture` and `cargo test --test interpreter supports_comma_expressions_after_control_condition_primaries -- --nocapture`.
- The adjacent next audit is leading `]` in nonempty control conditions, which remains impossible but is not covered by the shared structural guard yet.
