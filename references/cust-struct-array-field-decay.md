# Struct array-field decay and element address-of

Date: 2026-05-09 autonomous run

## Feature

Cust now treats scalar array fields embedded in supported structs as pointer-capable storage in pointer contexts:

- `packet.values` and `packet.tag` decay to array-base pointers for `int *` / `char *` slots and array-parameter spellings.
- `packets[i].values` decays the array field inside a struct-array element to the same pointer shape.
- `&packet.values[j]` and `&packets[i].values[j]` produce `PointerValue::ArrayElement` pointers into the embedded field storage.

The implementation reuses the existing `StructFieldValue::Array { value: Rc<RefCell<ArrayValue>>, .. }` storage, so pointer writes alias the struct field contents and avoid host addresses.

## Implementation notes

- Parser: address-of now accepts `Expr::StructArrayGet` and `Expr::StructElementArrayGet`, lowering them to dedicated `AddressOfStructArrayField` / `AddressOfStructElementArrayField` expressions.
- Pointer inference: `pointer_expr_pointee_type` already understands struct array fields; the new address-of expression variants route through that same array-field metadata.
- Pointer evaluation:
  - direct array fields use `find_struct_array_field_base_pointer` / `find_struct_array_field_pointer`.
  - struct-array element array fields use `find_struct_element_array_field_base_pointer` / `find_struct_element_array_field_pointer`.
- `eval_pointer(Expr::StructGet)` first tries array-field decay, then falls back to pointer-field reads so existing pointer-field behavior stays intact.
- `eval_pointer(Expr::StructElementGet)` supports array-field decay for struct-array elements; pointer fields inside struct-array elements remain a future feature unless separately covered.

## Tests

- Interpreter fixture: `tests/fixtures/valid/struct_array_field_decay.c`
- Native compiler-oracle fixture: `tests/fixtures/compat/valid/struct_array_field_decay.c`
- Rust tests: `supports_struct_array_field_decay_and_element_address_of` plus `tests/c_compat.rs` fixture registration.

## Pitfalls

- Keep native-oracle returns below 256 or account for process exit-code truncation; the fixture uses small numeric char escapes to avoid modulo mismatch.
- Do not convert embedded array-field storage into copied arrays when decaying; pointer mutation must alias the `Rc<RefCell<ArrayValue>>` stored in the field.
- Arrow (`->`) array-field indexing/decay is not covered by this run; it needs parser support for indexing `Expr::StructPtrGet` before implementation.
