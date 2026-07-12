# Nested and qualified `_Atomic(type-name)` diagnostics

The 2026-07-12 autonomous run closed accepted-invalid nested and directly qualified C11 atomic type arguments.

## Differential boundary

Local `cc -std=c11 -Wall -Wextra -Werror -fsyntax-only` rejects nested atomic specifiers, qualified scalar arguments, and qualified pointer arguments. It accepts `_Atomic(const int *)` and `_Atomic(volatile int *)`: those are unqualified pointer types whose pointees are qualified.

## Parser implementation

- Reject an inner `Token::Atomic` before recursively parsing the `_Atomic(...)` argument.
- Preserve leading and postfix qualifier tokens consumed around the nested base type.
- If no direct pointer star follows, reject a qualified scalar argument at its qualifier token.
- If a direct pointer star follows, treat base qualifiers as pointee qualifiers and preserve them.
- Reject qualifiers consumed after that star because they qualify the pointer argument itself.
- Preserve supported bare `_Atomic` qualifier syntax and existing atomic pointer/array/function suffix diagnostics.

## TDD and regressions

Use exact-output interpreter tests for declarations, `sizeof`, `_Alignof`, and compound literals. Run the focused nested/qualified tests, then the shared `atomic` filter so `_Atomic(int *)`, `_Atomic(const int *)`, direct atomic-array diagnostics, and unsupported pointer suffixes remain green.