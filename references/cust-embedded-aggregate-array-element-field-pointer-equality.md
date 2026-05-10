# Embedded aggregate-array element field pointer equality

2026-05-10 autonomous run.

## Feature

Cust now treats scalar field pointers reached through pointers into embedded aggregate-array fields as comparable by identity. Examples:

```c
struct Point *mid = line.points + 1;
struct Point *same_mid = line.points + 1;
int *mid_x = &mid->x;
int *same_mid_x = &same_mid->x;
return mid_x == same_mid_x;
```

The same owner metadata also works through nested aggregate-array fields such as `box.line.points + 2`.

## Implementation note

`PointerValue::StructFieldElementField` stores the containing scope/name, optional root array element, embedded aggregate-array field path, embedded array index, and selected scalar/aggregate field path. Equality must compare every one of those identity components in `Interpreter::pointer_eq`; otherwise two pointers to the same embedded element field fall through to generic inequality even though dereference/assignment already alias correctly.

## Fixtures

- `tests/fixtures/valid/struct_field_element_field_pointer_equality.c`
- `tests/fixtures/compat/valid/struct_field_element_field_pointer_equality.c`
- focused test: `supports_pointer_equality_for_fields_reached_through_embedded_aggregate_array_pointers`

Compiler-oracle fixture avoids tautological self-comparisons by creating independently computed pointer variables before comparing them under `-Wall -Wextra -Werror`.
