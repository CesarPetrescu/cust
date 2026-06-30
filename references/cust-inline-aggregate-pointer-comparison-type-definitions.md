# Inline aggregate definitions in pointer comparison expressions

Date: 2026-06-30

## Scope

Conformance coverage for inline named aggregate definitions that appear inside supported pointer comparison and pointer-difference expressions:

- pointer equality operands using `sizeof(struct PtrEq { ... }) == sizeof(struct PtrEq)` as an offset
- pointer relational operands using `sizeof(union PtrRel { ... }) == sizeof(union PtrRel)` as an offset
- pointer difference operands using `sizeof(struct PtrDiff { ... }) == sizeof(struct PtrDiff)` as part of the offset expression
- aggregate compound-literal field-address initializers adjacent to pointer comparison coverage

## Outcome

No production parser/runtime change was required. Focused interpreter coverage passed immediately, and a warning-free native `cc -std=c11 -Wall -Wextra -Werror` smoke test exited with the same value as Cust (`65`). The existing expression/type-name parsing paths already install inline aggregate tags where Cust supports later use, and pointer comparison/difference evaluation reuses established interpreter-owned pointer metadata.

## Pitfalls

- Native compilers may warn on tautological pointer self-comparisons such as `(values + 1) == values + 1` under `-Werror=tautological-compare`; use a local variable offset to keep oracle fixtures warning-free.
- Tags introduced solely in an `if` condition expression are not necessarily visible to later declarations in the function body under the native compiler's scope rules. Keep same-block later-declaration checks only for contexts already proven warning-free, such as declaration initializer compound-literal type names.
- Keep oracle return values in the `0..255` range because the `c_compat` harness compares process exit codes.

## Verification

```bash
cargo test --test interpreter supports_inline_aggregate_pointer_comparison_type_definitions -- --nocapture
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_aggregate_pointer_comparison_type_definitions.c -o /tmp/inline_aggregate_pointer_comparison_type_definitions && /tmp/inline_aggregate_pointer_comparison_type_definitions
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
