# Cust C99 array parameter qualifiers

2026-05-12 autonomous run notes.

## Supported syntax

Cust accepts C99 array-parameter bracket qualifiers for the existing one-dimensional array-parameter decay model:

- `int values[static 3]`
- `int values[restrict 3]`
- `int values[const 3]`
- combined orderings used in fixtures, such as `const int values[static const 3]`
- aggregate array parameters such as `struct Point points[static 2]`

`static`, `restrict`, `volatile`, and `_Atomic` inside the brackets are parser-level metadata/no-ops for Cust's deterministic interpreter. `const` inside brackets is meaningful: it qualifies the decayed pointer parameter slot (C's `int a[const 3]` behaves like `int * const a`), so reassigning the parameter name reports `cannot assign to const variable '<name>'`. Leading `const` still qualifies the pointee (`const int a[static 3]`) and preserves existing const-discard/write diagnostics.

## Implementation notes

- `Parser::parse_array_parameter_length_and_qualifiers` consumes optional bracket qualifiers/static around the existing integer length parser and returns whether the pointer slot is const.
- The existing fixed/unsized array-parameter lowering to `ParamKind::Pointer` remains unchanged, so no runtime storage model changes were needed.
- Multidimensional-array checks still run immediately after the first bracket pair.

## Coverage

- `tests/fixtures/valid/array_parameter_qualifiers.c`
- `tests/fixtures/invalid/array_parameter_const_slot_reassignment.c`
- `tests/fixtures/compat/valid/array_parameter_qualifiers.c`
- Focused command: `cargo test --test interpreter array_parameter -- --nocapture`
- Compiler oracle: `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`
