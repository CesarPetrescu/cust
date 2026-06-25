# Unparenthesized `sizeof` in integer constant expressions

Date: 2026-06-25

## Context

C permits `sizeof unary-expression` without parentheses, and `sizeof` results are integer constant expressions for non-VLA operands. Cust already supported unparenthesized runtime `sizeof` expressions, but the parser's enum/switch integer-constant-expression folder treated the unparenthesized form as a parsed integer unary operand and then always returned `INT_SIZE`.

## Root cause

`Parser::parse_integer_constant_sizeof()` had a parenthesized branch that used `sizeof_integer_constant_expr(...)`, but the unparenthesized branch called `parse_integer_constant_unary(...)` and returned `INT_SIZE`. This rejected string-literal operands such as `sizeof "abc"` and would also have lost the actual operand size for supported non-int operands.

## Implementation note

Parse the unparenthesized operand with `parse_unary()` and feed the resulting expression into `sizeof_integer_constant_expr(...)`. This keeps the integer-constant-expression parser non-evaluating while reusing the existing supported operand-size metadata for string literals, nested `sizeof`, scalar unary expressions, and existing literal forms.

## Fixtures

- `tests/fixtures/valid/sizeof_unparenthesized_integer_constant_expressions.c`
- `tests/fixtures/compat/valid/sizeof_unparenthesized_integer_constant_expressions.c`

The compat fixture keeps the result ABI-independent by checking native-compatible relationships and returning a small exit code.
