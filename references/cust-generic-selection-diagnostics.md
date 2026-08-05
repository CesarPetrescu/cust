# Bounded C11 `_Generic` selections

Date: 2026-08-05

Cust supports a bounded first-pass C11 generic-selection slice over deterministic scalar, one-level pointer, and named aggregate association types. This replaces the earlier parser-only rejection documented here in 2026-05-12.

## Semantics

- Parse `_Generic(controlling-expression, association-list)` as `Expr::GenericSelection`.
- Classify the controlling expression from syntax and retained type metadata; never evaluate it.
- Select exactly one compatible typed association or the optional `default`.
- Evaluate only the selected expression, while structurally validating every association expression.
- Preserve the selected expression's scalar, pointer, aggregate, discard, `sizeof`, and supported integer-constant-expression behavior.
- Treat top-level qualifiers as non-distinguishing while retaining one-level pointer pointee qualification.
- String and supported array controls decay to one-level pointers for selection.

## Exact bounded diagnostics

- duplicate compatible association types
- more than one `default`
- no compatible association and no `default`
- unsupported `void`, array, function, anonymous-aggregate, pointer-to-pointer, and two-dimensional-array forms

Keep the `_Generic` keyword's `LocatedToken` so no-match errors retain the selection's source line and column. Unsupported association declarator suffixes must be rejected before generic delimiter handling.

## Runtime and parser fan-out

A generic-selection AST node must participate in:

- scalar `eval`, pointer `eval_pointer`, aggregate `eval_struct_expr`, discard evaluation, and truthiness
- pointer and aggregate metadata classifiers
- `sizeof_expr` and parser-folded integer constant expression classification/evaluation
- nested structural validation, including unselected expressions and short-circuited/conditional contexts
- postfix aggregate field access and other selected-result shape routing

Do not call any evaluator while selecting an association. Clone only the selected expression before mutable interpreter evaluation when borrowing requires it.

## Fixtures and focused verification

- `tests/fixtures/valid/generic_selections.c`
- `tests/fixtures/compat/valid/generic_selections.c`
- `tests/fixtures/invalid/generic_selection_*.c`
- `cargo test --test interpreter generic_selection -- --nocapture`
- `cargo test --test c_compat -- --nocapture`

The compiler-oracle fixture stays warning-free under `-Wall -Wextra -Werror` and avoids host ABI-sensitive size assertions. The 2026-08-05 inherited implementation had no recoverable original RED transcript, so that run did not re-claim one; independent review approved the completed diff and the focused plus canonical gates passed.
