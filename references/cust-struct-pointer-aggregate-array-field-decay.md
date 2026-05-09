# Struct-pointer aggregate-array field decay

Run: 2026-05-09 autonomous maintenance

## Feature

Cust now supports aggregate-array fields reached through a struct pointer:

```c
struct Point { int x; int y; };
struct Line { struct Point points[3]; };

int mutate(struct Point points[]) { points[1].x += 5; return points[1].x; }

int f(struct Line *line) {
    struct Point *p = line->points;
    p = line->points + 1;
    p->y = 7;
    return mutate(line->points) + (&line->points[2])->x;
}
```

Supported forms include:

- `slot->points` decay to `struct Point *` / `union T *` pointer contexts.
- `&slot->points[i]` element address-of.
- `slot->points + n` aggregate-array field pointer arithmetic through existing `PointerValue::StructFieldElement` routing.
- `slot->points[i].field` parser lowering via `StructPtrArrayGet` as the base pointer for ordinary `StructPtrGet` field access.
- Const propagation from `const struct Line *slot` so mutable aggregate pointer flows reject with `cannot discard const qualifier from pointer target`.

## Implementation notes

- Parser-only change: allow `.` after `Expr::StructPtrArrayGet` by wrapping it as `Expr::StructPtrGet { pointer: StructPtrArrayGet(...), fields: ... }`.
- Runtime change: `find_struct_pointer_array_field_base_pointer` and `find_struct_pointer_array_field_pointer` now first detect `StructFieldValue::StructArray` and return `PointerValue::StructFieldElement` instead of falling through to scalar-array storage.
- Pointer type inference needed `StructFieldType::StructArray` in `pointer_expr_pointee_type` for conversions into `struct T *` parameters/variables.
- The initial RED failure was a parser error on `line->points[0].y` (`invalid struct field access target`), which confirmed the syntax gap before runtime work.

## Tests

Focused commands:

```bash
cargo test --test interpreter struct_pointer_aggregate_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
```

Fixtures:

- `tests/fixtures/valid/struct_pointer_aggregate_array_field_decay.c`
- `tests/fixtures/invalid/struct_pointer_aggregate_array_field_const_discard.c`
- `tests/fixtures/compat/valid/struct_pointer_aggregate_array_field_decay.c`
