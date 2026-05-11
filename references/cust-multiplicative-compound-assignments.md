# Multiplicative compound assignments (`*=`, `/=`, `%=`)

2026-05-11 autonomous run.

## Scope

Cust now supports C multiplicative compound assignment operators for existing scalar lvalue families:

- scalar variables: `x *= 2`, `x /= y`, `x %= 3`
- scalar array/pointer-index lvalues: `values[i] *= 2`, `values[i] /= 3`, `values[i] %= 5`
- dereferenced pointer lvalues: `*p %= 10`

The operators parse at the same assignment-precedence level as existing `+=`, `-=`, bitwise compound assignments, and ordinary `=`.

## Implementation notes

- Lexer tokens are separate `StarAssign`, `SlashAssign`, and `PercentAssign` variants so `*`, `/`, and `%` expression parsing remains unchanged.
- `CompoundOp` adds `Mul`, `Div`, and `Rem`; `apply_compound_op` reuses the same arithmetic semantics and `division by zero` diagnostic as binary `/` and `%`.
- Pointer-valued compound-assignment contexts must reject `*=`, `/=`, and `%=` through `pointer_compound_error`; all pointer-field/pointer-variable compound match arms need exhaustive coverage when adding new `CompoundOp` variants.

## Coverage

- Interpreter fixture: `tests/fixtures/valid/compound_assignments.c`
- Compiler-oracle fixture: `tests/fixtures/compat/valid/compound_assignments.c`
- Focused tests:
  - `cargo test --test interpreter supports_compound_assignment_expressions_for_scalar_array_and_deref_lvalues -- --nocapture`
  - `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`
