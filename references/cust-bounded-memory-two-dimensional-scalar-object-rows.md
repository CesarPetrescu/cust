# Two-dimensional non-character row object bytes

## Scope

Exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` accept one selected row of an existing two-dimensional `int` or `_Bool` array. Supported routes include direct `matrix[i]` row decay, adjusted array parameters, explicit pointer-to-row indexing/dereference, and supported struct-field roots.

## Implementation

- Selected rows evaluate to the existing scalar `PointerValue::ArrayElement` representation with retained `ArrayValue::dimensions` metadata.
- `Interpreter::validate_memory_scalar_pointer_argument()` no longer rejects every non-character array solely because the backing `ArrayValue` is two-dimensional.
- `scalar_memory_available_bytes()` continues to call `character_sequence_array_end()`. For dimensioned storage, that helper derives the selected row end from the current flattened index and column count. Capacity therefore remains `remaining cells in this row * scalar width`; it never reaches the adjacent row.
- Existing `offset_array_pointer()` and byte-cell reconstruction preserve owner, lexical lifetime, recursive const, row bounds, deterministic eight-byte little-endian `int` bytes, canonical `_Bool` bytes, and aligned/interior pointer identity.
- Whole aggregate and union-backed storage remain unsupported. The change admits only scalar row elements; it does not define aggregate padding or union member byte aliasing.

## TDD and coverage

The initial direct-row regression failed with `function 'memcpy' currently supports only scalar object storage for argument 1`. Removing the obsolete two-dimensional admission guard made the shared row-local model GREEN.

Coverage includes:

- all five intrinsics over a direct selected `int` row;
- adjusted parameters, explicit pointer-to-row operations, struct-field `int` rows, and `_Bool` rows in a registered warning-free native fixture;
- row-local destination/source capacity, overlap, direct and nested const, huge row-index overflow, union, and expired-owner diagnostics;
- zero-count one-time evaluation and nested non-evaluating `sizeof(call)`;
- ABI-independent native counts expressed as `column_count * sizeof(element)` rather than relying on Cust's separate indexed-row `sizeof` surface.

## Pitfalls

- Do not use backing flattened array length as raw-memory capacity. Every byte range must stop at the selected row boundary.
- A destination at `row + 1` in a two-element row has only one scalar cell available. A two-cell range must fail rather than spilling into the next row.
- Adjacent one-cell source/destination ranges do not overlap. Overlap regression fixtures need at least three columns (for example source cells `[0, 2)` and destination cells `[1, 3)`).
- Native fixtures must avoid assumptions about host `int` width, endianness, or partial integer object representation. Full-element byte counts and copy/compare relationships are portable for the supported oracle use.
