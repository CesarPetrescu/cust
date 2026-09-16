---
title: Contextual invalid expression starts after C return
---

# Contextual invalid nonempty `return` expression starts

## Scope

A bare `return;` is valid only in a `void` function, but a nonempty `return`
statement must start with a Cust expression. Impossible binary, assignment, and
comparison starts should name the `return` context rather than falling through
to the generic primary-expression diagnostic.

## Implementation

Keep the decision in `Parser::reject_missing_return_expr()`, immediately after
`parse_return()` accepts that the next token is not the valid bare-return
semicolon. Reject binary-only operators, all assignment operators, comparison
operators, closing/structural punctuation, and EOF with the existing
source-located form:

```text
expected expression after return, found <Token>
```

Do not reject valid unary starts (`+`, `-`, `++`, `--`, `!`, `~`, `*`, `&`),
parenthesized expressions, literals, identifiers, or `_Generic`. Preserve
`reject_keyword_start_expression("return")` after the token guard so its
more specific keyword/type-specifier diagnostics retain precedence.

## Regression shape

Use one exact matrix covering representative binary (`/`, `&&`), assignment
(`=`, `%=`), equality (`==`), and relational (`<`) starts with line/column
assertions. In the same focused test, preserve a valid void bare return plus
scalar, pointer, and aggregate-valued return expressions.

Run:

```bash
cargo test --test interpreter rejects_invalid_nonempty_return_expression_starts_with_context -- --nocapture
cargo test --test interpreter rejects_missing_return_expressions_with_context -- --nocapture
cargo test --test interpreter rejects_keyword_start_return_and_control_expressions_with_context -- --nocapture
```

## Boundary

This is diagnostic closure only: it does not broaden the expression grammar or
change function return-type validation.
