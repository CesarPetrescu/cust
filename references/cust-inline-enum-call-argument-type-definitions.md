# Inline enum definitions in function-call argument type expressions

2026-06-27 autonomous run.

## Scope

Coverage-only conformance fixture for inline enum definitions that appear in function-call argument subexpressions, including:

- `sizeof(enum ArgSize { ARG_SIZE = 4 })` inside a call argument.
- `_Alignof(enum ArgAlign { ARG_ALIGN = 6 })` inside a call argument.
- `(enum ArgCast { ARG_CAST = 8 })0 + ARG_CAST` inside a call argument.
- Later same-block use of `ARG_SIZE`, `ARG_ALIGN`, and `ARG_CAST` after the call.

## Result

The focused interpreter test passed immediately. The existing parser/runtime path is already correct because `parse_expr_stmt_with_semi`, declaration initializers, assignment statements, and return statements wrap pending inline enum declarations around the enclosing runtime statement before evaluating call arguments.

## Fixture guidance

Use warning-free native oracle checks:

- Compare `sizeof(enum Tag { ... }) == sizeof(enum Tag)` and `_Alignof(enum Tag { ... }) == _Alignof(enum Tag)` instead of exact enum size/alignment bytes.
- Keep enumerator arithmetic small enough to return directly as a process exit code.
- Initialize call arguments from scalar expressions; no native ABI assumptions are needed.

Focused command:

```bash
cargo test --test interpreter inline_enum_call_argument_type_definitions -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
