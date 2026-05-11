# Cust `_Alignof` type-name support

2026-05-11 autonomous run.

## Scope

Cust accepts C11 `_Alignof(type-name)` for the supported deterministic type-name subset:

- scalar spellings (`char`, `_Bool`, `int`, signed/unsigned/long/short spellings through existing lowering)
- leading/postfix/interleaved qualifiers through existing type parsing
- one-level pointer type names
- one-dimensional scalar and aggregate array type names
- direct `struct`/`union` type names and aggregate typedef aliases

`_Alignof(void)` reports `_Alignof(void) is not supported`, mirroring the existing `sizeof(void)` boundary. Expression operands are intentionally not supported; this matches C11 `_Alignof`, which takes a type name.

## Deterministic alignment model

Cust does not use native ABI padding. `_Alignof` is therefore defined by interpreter metadata:

- `char` and `_Bool`: `1`
- `int` and deterministic integer aliases: `8`
- pointers: `8`
- arrays: alignment of the element type
- structs/unions: maximum alignment of their fields; empty aggregates default to `1`

Native compiler-oracle coverage avoids Cust-specific exact integer/pointer/aggregate alignments and checks only portable relationships such as `_Alignof(char) == 1`, array alignment matching element alignment, and aggregate alignment being at least the alignment of each member type.

## Implementation notes

`src/lib.rs` reuses the existing `SizeOfType` metadata for type-name parsing and adds `alignment(...)` helpers on `CType`, `PointeeType`, `StructFieldType`, `StructTypeDef`, and `SizeOfType`. Parser support is deliberately routed through a shared `parse_sizeof_like_type_name(operator)` helper so future `sizeof`/`_Alignof` type-boundary diagnostics stay consistent.
