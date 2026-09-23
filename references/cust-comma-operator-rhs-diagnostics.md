# Comma-operator RHS diagnostics

## Scope

Cust accepts the C comma operator only after a valid left expression. Invalid token starts after the comma must retain the context `expected expression after comma operator` rather than falling through to generic primary-expression errors.

## Implementation

- Keep `parse_comma_expr()` and the comma loop in `parse_index_expr()` on one `reject_invalid_comma_operator_rhs()` helper.
- The helper preserves precedence: structural/postfix-only starts (`[`, `{`, `?`, `,`, `:`, `.`, `->`, closing delimiters, `;`, EOF), then binary/assignment-only starts, then keyword starts.
- Do not reject valid unary starts (`+`, `-`, `*`, `&`, `!`, `~`, `++`, `--`) or a parenthesized expression.
- Keep the complete diagnostic construction outside recursive expression parsing frames. Adding the matcher directly to `parse_comma_expr()` made the nested `sizeof(strtol(...))` linearity regression overflow the normal test-thread stack; the helper restores the bounded stack behavior.

## Regression coverage

- Test ordinary parenthesized comma expressions over binary, assignment, comparison, repeated comma, and colon token starts with exact locations.
- Test the independent array-index comma loop (`values[0, /]`) so it cannot bypass the shared context.
- Retain `sizeof_base_integer_string_conversion_endptr_validation_remains_linear` and the bounded deeply nested conversion regression as stack-depth preservation checks.

## Verification

Invalid-program diagnostic wording is Cust-local, so native compiler-oracle fixtures are not appropriate. Run the focused interpreter tests, then the full local and Docker gates.
