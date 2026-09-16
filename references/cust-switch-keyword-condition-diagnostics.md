# Contextual Switch Keyword Condition Diagnostics

Use this note when maintaining diagnostics for the controlling expression in `switch (...)`.

- Run `cargo test --test interpreter rejects_keyword_start_switch_expressions_with_context -- --nocapture` before changing production code. Representative declaration and control keywords (`int`, `struct`, `return`, `if`) must fail with `expected expression after switch before '<keyword>'` at the keyword location.
- Preserve the delimiter diagnostic precedence in `reject_missing_switch_expr()` before invoking `reject_keyword_start_expression("switch")`; this keeps empty and punctuation-only selector errors contextual.
- Do not reject `Token::Generic`: `reject_keyword_start_expression()` deliberately permits valid C11 `_Generic` expressions. Verify `switch (_Generic(...))` still evaluates the selected association while leaving the controlling expression unevaluated.
- The switch route uses its own structural guard because its opening-brace diagnostics differ from statement-bodied controls; reuse the shared keyword helper rather than routing it wholesale through the `if`/`while` condition helper.
