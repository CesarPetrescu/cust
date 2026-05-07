# Cust Designated Initializer Model

Last updated: 2026-05-07

Cust supports a scoped, preprocessor-free subset of C designated initializers for already-supported one-dimensional scalar arrays, supported structs, and one-dimensional arrays of supported structs/unions.

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

Nested brace lists may themselves use designators for supported nested structs and array fields. Cust also supports C path designators for nested struct fields and one-dimensional scalar array fields inside structs:

```c
struct Inner { int x; int y; };
struct Packet { int values[3]; struct Inner inner; };
struct Packet packet = {
    .inner = {.y = 4, .x = 3},
    .values = {[1] = 6, [2] = 7},
};
struct Packet packet2 = {
    .inner.x = 3,
    .inner.y = 4,
    .values[1] = 6,
};
```

Path designators apply in source order over zero/default-initialized aggregate storage. Multiple path entries for the same nested aggregate merge into the existing nested value instead of replacing sibling fields, so `.inner.x = 1, .inner.y = 2` leaves both fields initialized.

### Aggregate arrays

One-dimensional arrays of supported structs and unions accept C-style array designators at the aggregate-element level:

```c
struct Point { int x; int y; };
union Number { int value; char tag; };

struct Point points[3] = {[2] = {.y = 6, .x = 5}, [0] = {1, 2}};
union Number numbers[3] = {[1] = {.tag = 7}, [2] = {4}};
```

- Designated aggregate-array indices use the same non-negative in-bounds integer-constant rule as scalar array designators.
- Positional aggregate elements after a designator continue at the element after the most recent designator.
- Omitted aggregate elements are zero/default-initialized through the existing struct/union element construction.
- Later entries for the same aggregate-array index overwrite the earlier element value.

## Deliberate exclusions

- Range designators such as `[0 ... 3]` are not supported.
- Native C ABI layout and padding are not used for interpretation; compiler-oracle fixtures compare behavior/exit codes only.

## Implementation notes

- Parser initializer AST keeps array entries as positional or indexed `ArrayInitializer` entries.
- Aggregate-array initializer AST keeps element entries as positional or indexed `StructArrayInitializer` entries and applies them over prebuilt zero/default aggregate elements.
- Struct initializer parsing resolves both positional and `.field =` entries to field-designated `StructInitializer` entries, so runtime application is name-based and supports out-of-order fields.
- Runtime initialization starts from the existing zero/default value construction and applies initializer entries in source order, preserving overwrite behavior and const/read-only metadata after initialization.

## Acceptance coverage

- Valid interpreter and compiler-oracle fixtures: `tests/fixtures/valid/designated_initializers.c` / `tests/fixtures/compat/valid/designated_initializers.c`, `tests/fixtures/valid/path_designated_initializers.c` / `tests/fixtures/compat/valid/path_designated_initializers.c`, and `tests/fixtures/valid/aggregate_array_designated_initializers.c` / `tests/fixtures/compat/valid/aggregate_array_designated_initializers.c`.
- Invalid fixtures: `tests/fixtures/invalid/array_designator_out_of_bounds.c`, `tests/fixtures/invalid/struct_designator_unknown_field.c`, `tests/fixtures/invalid/struct_path_designator_unknown_field.c`, `tests/fixtures/invalid/struct_array_path_designator_out_of_bounds.c`, and `tests/fixtures/invalid/struct_array_designator_out_of_bounds.c`.
