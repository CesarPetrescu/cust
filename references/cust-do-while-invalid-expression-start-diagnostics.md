# Do-While Invalid Condition-Start Diagnostics

## Scope

Cust accepts a `do` statement body, then requires `while (condition);`. The structural and keyword-start condition guards run before expression parsing. This note closes the remaining generic-primary-expression fallback for operator tokens that cannot begin a nonempty condition.

## Implementation

Use `Parser::reject_invalid_control_condition_expr(context)` after
`reject_missing_control_condition_expr(context)` and before `parse_expr()`.
The helper reports:

```text
expected expression after do-while, found <Token> at line <line>, column <column>
```

It is also used by `while`, retaining its existing diagnostic text. Keep the shared structural/keyword guard first so parentheses, delimiters, EOF, and invalid keywords preserve their established contextual diagnostics.

## Regression matrix

`rejects_invalid_nonempty_do_while_condition_starts_with_context` covers
binary (`/`, `&&`), assignment (`=`, `%=`), equality (`==`), and relational
(`<`) starts with exact locations. It also retains legal unary, grouped, and
scalar `do-while` conditions.

Run:

```bash
cargo test --test interpreter rejects_invalid_nonempty_do_while_condition_starts_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_nonempty_while_condition_starts_with_context -- --nocapture
```
