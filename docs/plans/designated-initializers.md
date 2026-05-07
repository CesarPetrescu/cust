# Cust Designated Initializer Model

Last updated: 2026-05-07

Cust supports a scoped, preprocessor-free subset of C designated initializers for already-supported one-dimensional scalar arrays and supported structs.

## Supported syntax

### Arrays

```c
int values[4] = {[2] = 5, [0] = 1};
int mixed[5] = {[3] = 10, [1] = 3, 2};
```

- Designator indices must be non-negative integer constants inside the declared length.
- Positional entries after a designator continue at the element after the most recent designator, matching C's useful subset for mixed initializer lists.
- Later entries for the same index overwrite earlier entries.
- Omitted elements are zero-filled.

### Structs

```c
struct Point { int x; int y; };
struct Point p = {.y = 2, .x = 1};
```

- Field designators use `.field = value` and may appear in any order.
- Positional entries after a field designator continue at the following declared field.
- Later entries for the same field overwrite earlier entries.
- Unknown field designators report `struct '<Type>' has no field '<field>'`.
- Omitted scalar, pointer, array, and nested struct fields keep the existing Cust zero/default initialization semantics.

### Nested aggregate fields

Nested brace lists may themselves use designators for supported nested structs and array fields:

```c
struct Inner { int x; int y; };
struct Packet { int values[3]; struct Inner inner; };
struct Packet packet = {
    .inner = {.y = 4, .x = 3},
    .values = {[1] = 6, [2] = 7},
};
```

## Deliberate exclusions

- Range designators such as `[0 ... 3]` are not supported.
- Path designators such as `.inner.x = 1` are not yet supported; write a nested brace initializer instead (`.inner = {.x = 1}`).
- Array element designators inside struct paths such as `.values[1] = 2` are not yet supported; write `.values = {[1] = 2}`.
- Native C ABI layout and padding are not used for interpretation; compiler-oracle fixtures compare behavior/exit codes only.

## Implementation notes

- Parser initializer AST keeps array entries as positional or indexed `ArrayInitializer` entries.
- Struct initializer parsing resolves both positional and `.field =` entries to field-designated `StructInitializer` entries, so runtime application is name-based and supports out-of-order fields.
- Runtime initialization starts from the existing zero/default value construction and applies initializer entries in source order, preserving overwrite behavior and const/read-only metadata after initialization.

## Acceptance coverage

- Valid interpreter and compiler-oracle fixture: `tests/fixtures/valid/designated_initializers.c` and `tests/fixtures/compat/valid/designated_initializers.c`.
- Invalid fixtures: `tests/fixtures/invalid/array_designator_out_of_bounds.c` and `tests/fixtures/invalid/struct_designator_unknown_field.c`.
