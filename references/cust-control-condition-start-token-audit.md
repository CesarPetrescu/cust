# Control-condition start-token audit

## Scope

Use this note when extending or auditing diagnostics for the first token of a nonempty `if`, `while`, `do-while`, or `switch` controlling expression.

## Completed audit

The test `audits_rejected_control_condition_start_tokens_with_context` exhaustively covers every lexer token that can reach the parser as an invalid first token in these four contexts:

- 11 structural/postfix-only tokens: `)`, `;`, `,`, `:`, `]`, `}`, `[`, `{`, `?`, `.`, and `->`.
- 25 binary/assignment-only operator tokens.
- 41 declaration/control keywords rejected by `integer_constant_invalid_start_label()`.
- EOF immediately after the opening parenthesis.

Every case asserts the exact contextual message and token source column. The only lexer values deliberately outside this matrix are valid expression starts (identifiers, numeric/string literals, grouping, unary prefixes, `sizeof`, `_Alignof`, and `_Generic`) and preprocessing-only/internal token forms that cannot reach ordinary expression parsing. `supports_grouped_and_unary_control_condition_starts` preserves grouped plus unary `+`, `-`, `--`, `!`, `~`, `*`, and `&` starts.

## Decision

This audit is coverage closure, not a parser change: all tested invalid starts already reach the shared contextual guards. Do not pre-reject legal unary or grouped forms merely to expand the structural lists. If a new lexer token is introduced, classify whether it can reach `parse_expr()` in a control condition; add an exact regression only if it is invalid there or needs a new contextual diagnostic.
