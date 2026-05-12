# Permuted scalar type specifiers

2026-05-12 autonomous run.

Cust now accepts C-compatible permutations of supported scalar type specifiers where the base spelling appears before `signed`/`unsigned` or where `int` appears before `long`/`short`, for example:

- `int unsigned`
- `char signed` / `char unsigned`
- `int const unsigned`
- `int long signed` / `int long unsigned`
- `int short unsigned`
- `int long long unsigned`

Implementation notes:

- `parse_scalar_decl_type_specifiers` consumes a run of scalar type specifier tokens interleaved with supported qualifiers, then lowers them to Cust's deterministic `int`, `char`, or `_Bool` model.
- `starts_function_definition` now uses the same scalar-specifier skipping helper so permuted return types/prototypes such as `char signed f(char unsigned value);` are routed to function parsing instead of global variable parsing.
- The parser keeps simple combination validation so invalid mixtures like `int char` or `short long` do not become silently accepted.

Coverage:

- `tests/fixtures/valid/permuted_scalar_type_specifiers.c`
- `tests/fixtures/compat/valid/permuted_scalar_type_specifiers.c`
- `cargo test --test interpreter supports_permuted_scalar_type_specifiers -- --nocapture`
- `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`
