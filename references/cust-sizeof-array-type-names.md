# Cust sizeof array type names

Date: 2026-05-10

## Summary

Cust now supports one-dimensional array type operands in `sizeof(...)` for scalar and supported aggregate element types:

- `sizeof(int[3])`
- `sizeof(char[4])`
- `sizeof(const int[2])`
- `sizeof(struct Pair[2])`
- `sizeof(TypedefAggregate[3])`

The result uses Cust's deterministic interpreter size model (`int = 8`, `char = 1`, aggregate sizes from the no-padding struct/union model) multiplied by the parsed array length. The operand is a type query, so no runtime storage is created or evaluated.

## Implementation notes

- `SizeOfType::Array(PointeeType, usize)` stores the element type and length.
- `Parser::parse_sizeof_array_type_len` consumes the bracketed positive length after a non-pointer scalar or aggregate type name.
- Pointer array type operands remain outside the supported subset and report `pointer array sizeof types are not supported`.
- Multidimensional array type operands remain outside the supported subset and report `multidimensional sizeof array types are not supported` after the first parsed dimension.

## Fixtures

- Interpreter fixture: `tests/fixtures/valid/sizeof_array_types.c`
- Native compiler-oracle fixture: `tests/fixtures/compat/valid/sizeof_array_types.c`
- Regression test: `supports_sizeof_array_type_names`

## Native oracle caveat

The C compiler-oracle fixture checks size relationships such as `sizeof(int[3]) == sizeof(int) * 3` and `sizeof(struct Pair[2]) == sizeof(struct Pair) * 2` rather than Cust-specific byte counts, avoiding host ABI assumptions.
