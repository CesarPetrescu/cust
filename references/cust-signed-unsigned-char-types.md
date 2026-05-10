# Signed/unsigned char type spellings

2026-05-10 autonomous run.

## Implemented surface

Cust now accepts `signed char` and `unsigned char` as parser-level spellings for the existing deterministic `char` storage model. The spellings work wherever ordinary `char` already works through the shared declaration/type parser:

- globals, locals, static locals, and const locals
- function returns, prototypes, parameters, and pointer parameters
- one-level pointer declarations such as `unsigned char *p`
- typedef aliases such as `typedef unsigned char Byte;`
- scalar casts such as `(signed char)expr`
- `sizeof` scalar, pointer, and one-dimensional array type operands such as `sizeof(const signed char[3])`

## Semantics

This is syntax/conformance parity only. Cust continues to store scalar values in its existing deterministic integer representation and keeps `sizeof(char) == 1`; it does not add native `signed char`/`unsigned char` wraparound or implementation-defined range behavior.

## Tests

- `tests/fixtures/valid/signed_unsigned_char_types.c`
- `tests/fixtures/compat/valid/signed_unsigned_char_types.c`
- `tests/interpreter.rs::supports_signed_unsigned_char_type_spellings`

The compiler-oracle fixture keeps values in the portable non-overflowing char range so native signedness and warning behavior do not affect exit-code comparison.
