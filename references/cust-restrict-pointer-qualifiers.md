# Restrict pointer qualifiers

2026-05-11 autonomous run.

Cust now accepts C99 `restrict` as parser-level no-op qualifier syntax over the existing deterministic pointer model.

## Supported forms

- Post-star pointer declarations/parameters/fields: `int * restrict p`, `int * const restrict p`, `int * volatile restrict p`.
- The shared qualifier consumer also accepts `restrict` in the same parser qualifier loops as `const`/`volatile`, but only `const` changes runtime metadata; `volatile` and `restrict` are runtime no-ops.
- Function-signature lookahead skips `restrict` after `*` so pointer-returning/function declarations remain classified correctly.

## Tests

- Interpreter fixture: `tests/fixtures/valid/restrict_pointer_qualifiers.c`.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/restrict_pointer_qualifiers.c`.
- Focused commands:
  - `cargo test --test interpreter supports_restrict_pointer_qualifiers -- --nocapture`
  - `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`

## Pitfalls

- Native `restrict` is only valid on pointer-derived types, so compiler-oracle fixtures should avoid warning/error-prone forms like `restrict int value`.
- `restrict` must not set Cust's `is_const` or `points_to_const` metadata. It is syntax acceptance only, not alias analysis or optimization semantics.
