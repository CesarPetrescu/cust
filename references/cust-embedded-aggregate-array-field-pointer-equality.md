# Embedded aggregate-array field pointer equality

2026-05-10 autonomous run: closed the equality counterpart to embedded aggregate-array field pointer ordering.

## Feature

Pointers into the same embedded aggregate-array field now compare equal when their containing scope/name, optional containing struct-array element index, nested field path, and element index all match. This covers direct and nested forms such as:

```c
struct Point *start = line.points;
struct Point *first = &line.points[0];
start == first;
(line.points + 2) == &line.points[2];
box.line.points == &box.line.points[0];
```

Different element indices compare unequal with `!=` rather than falling through to scalar/array pointer identity.

## Implementation notes

- Patch point: `Interpreter::pointer_eq` in `src/lib.rs`.
- Use the same identity fields as `PointerValue::StructFieldElement` pointer ordering/subtraction: `scope_id`, `name`, `element_index`, `fields`, and `index`.
- Do not try to compare these via `Rc::ptr_eq`; embedded aggregate-array elements are not represented as scalar `ArrayValue` storage.

## Verification

- Focused RED test: `cargo test --test interpreter supports_pointer_equality_within_embedded_aggregate_array_fields -- --nocapture` initially returned `18` instead of `31`.
- GREEN: same focused test passed after adding the `StructFieldElement` arm in `pointer_eq`.
- Compiler oracle: `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture` includes `tests/fixtures/compat/valid/struct_field_pointer_equality.c`.

## Native fixture pitfall

Avoid direct self-comparisons like `p == p`; warning settings can turn tautological pointer comparisons into native compiler failures. Compare separately initialized pointer variables or expressions such as `line.points` and `&line.points[0]` instead.
