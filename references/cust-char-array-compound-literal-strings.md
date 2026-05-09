# Char-array compound literal string initializers

2026-05-09 autonomous run.

## Feature

Cust now accepts string literals as scalar-array compound literal entries for `char` arrays:

```c
char *word = (char[]){"cat"};
char *fixed = (char[4]){"dog"};
```

The literal is routed into the existing `ArrayInitializer::StringLiteral` path, so unsized literals infer length including the NUL terminator and fixed-size literals use the same copy/truncation rules as declaration-level `char[N] = "..."` initialization.

## Implementation notes

- `parse_array_compound_initializer` now receives the compound literal element `CType`.
- `Token::StringLiteral` is consumed as `ArrayInitializer::StringLiteral` only when the element type is `CType::Char`.
- Fixed-size overlong strings are rejected before runtime storage creation with `initializer string for char array compound literal is too long`.
- Runtime storage needed no new machinery because `make_array_value` and unsized length inference already handled `ArrayInitializer::StringLiteral`.

## Tests

- `tests/fixtures/valid/array_compound_literals.c`
- `tests/fixtures/compat/valid/array_compound_literals.c`
- `tests/fixtures/invalid/array_compound_literal_string_too_long.c`
- Focused: `cargo test --test interpreter array_compound_literal -- --nocapture`
- Oracle: `cargo test --test c_compat -- --nocapture`
