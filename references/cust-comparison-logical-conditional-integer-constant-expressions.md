# Comparison/logical/conditional integer constant expressions

Date: 2026-05-11

Cust now folds additional parser-side integer constant-expression operators for enum initializer values and `switch case` labels:

- Relational: `<`, `<=`, `>`, `>=`
- Equality: `==`, `!=`
- Logical: `&&`, `||`
- Conditional: `?:`

## Implementation notes

- The parser-only evaluator in `src/lib.rs` now starts at conditional-expression precedence and routes through logical-or, logical-and, bitwise-or/xor/and, equality, relational, shift, additive, multiplicative, unary, and primary helpers.
- Folded comparison/logical results use C-style integer truth values (`0` or `1`).
- Conditional expressions are right-associative; only the chosen folded value is returned, but both branches are parsed so syntax remains validated.
- This remains non-evaluating metadata: identifiers are limited to currently visible enum constants and local enum constants passed into the enum declaration parser. Runtime variables remain rejected in enum initializers/case labels.

## Coverage

- `tests/fixtures/valid/switch_enum_case_labels.c` covers enum initializer folding for comparison/equality/logical/conditional expressions and `switch case` labels using comparison/logical/conditional expressions.
- `tests/fixtures/compat/valid/switch_enum_case_labels.c` mirrors the supported subset for native C compiler-oracle verification. Keep the fixture's final return value below 256 because the compiler oracle compares process exit codes.

## Pitfalls

- Do not parse the first `_Static_assert` argument with this helper; static assertions already use assignment-precedence expression parsing so the argument-separating comma is preserved.
- C compiler-oracle fixtures compare exit codes, not full integer returns. If a fixture returns a value above 255, the native process truncates it modulo 256 and `c_compat` will fail even when Cust's interpreter value is mathematically correct.
