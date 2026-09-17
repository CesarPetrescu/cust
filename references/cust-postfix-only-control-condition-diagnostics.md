# Postfix-Only Control-Condition Diagnostics

Use this note when auditing the first token of nonempty `if`, `while`, `do-while`, and `switch` conditions.

- The only Cust tokens that are valid only after a primary expression and can occur at the beginning of these condition forms are `.`, `->`, and `]`. Each must fail at its own token with `expected expression after <context>, found Dot/Arrow/RBracket`.
- Keep `Token::Dot`, `Token::Arrow`, and `Token::RBracket` in both `reject_missing_control_condition_expr()` and `reject_missing_switch_expr()` before `reject_invalid_control_condition_expr()`. This retains the established structural-diagnostic precedence.
- Do not reject `(`, `+`, `-`, `*`, `&`, `++`, or `--`: Cust accepts those as legal grouping or unary prefixes. Do not reject postfix forms after a valid primary.
- The exhaustive 12-cell exact-location regression is `rejects_postfix_only_control_condition_starts_with_context`; `supports_subscript_expressions_in_control_conditions` remains the positive preservation proof.
- This is a coverage-only closure: the post-fix audit and new exhaustive regression may be immediately GREEN because the prior parser guards already cover all three token kinds. Record that honestly rather than adding redundant production changes.
