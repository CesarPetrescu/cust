# Parenthesized ordinary declarator names

Date: 2026-06-24

Cust now accepts ordinary C parenthesized declarator names in supported declaration positions, e.g.:

- `int (value)` parameters and locals/globals
- `int *(cursor)` pointer declarators after an explicit `*`
- `int (items)[3]` scalar arrays
- `struct Point (point)` aggregate objects
- aggregate pointer declarations and comma-separated declaration-list tails
- aggregate field names such as `struct Box { int (value); };`

Implementation notes:

- Keep the existing early `(` followed by `*` checks before calling the declarator-name helper. Those checks preserve targeted unsupported diagnostics for function pointers and pointer-to-array style declarators such as `int (*callback)(int)` and `int (*row)[3]`.
- Use the shared parser helper (`parse_declarator_name`) wherever a supported declarator name may appear after the base type or after an explicit `*`.
- The helper is intentionally narrow: it accepts only `(` `Ident` `)` as a name wrapper and otherwise falls back to the existing contextual `expect_ident_after(...)` diagnostics.

Verification from the implementation run:

```bash
cargo test --test interpreter supports_parenthesized_variable_declarators -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Coverage lives in `tests/fixtures/compat/valid/parenthesized_variable_declarators.c` and is registered in both `tests/interpreter.rs` and `tests/c_compat.rs`.
