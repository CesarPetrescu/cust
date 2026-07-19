# Adjusted aggregate-array parameter pointer fields

Use this note when a specialized aggregate-element AST such as `StructElementGet` or `StructElementArrayGet` works for a local `struct`/`union` array but fails for the same expression inside a function with an array parameter.

## Regression shape

```c
struct Inner { int *values; };
union Choice { struct Inner nested; int marker; };

int update(union Choice choices[]) {
    choices[0].nested.values[0] = 7;
    return choices[0].nested.values[0];
}
```

C adjusts `union Choice choices[]` to a pointer parameter. Cust still parses the bare-name expression through its specialized `StructElement*` AST family, but the bound runtime value is `Value::Pointer { ty: PointeeType::Struct(..) }`, not `Value::StructArray`. Direct-only helpers therefore fail with `variable 'choices' is not a struct array`.

## Implementation pattern

1. Keep the specialized parser AST; syntax alone cannot distinguish local array storage from an adjusted pointer parameter.
2. Resolve the containing element with `indexed_struct_pointer(name, index)` first. If it returns `None`, fall back to `find_struct_element_pointer(name, index)` for stored aggregate arrays.
3. Reuse one shared resolver across pointer-field reads, subscripts, writes, compound writes, increment/decrement, and indexed address-of. This preserves one-time index evaluation and interpreter-owned target identity.
4. Make `struct_element_field_metadata()` accept both `Value::StructArray` and `Value::Pointer { ty: PointeeType::Struct(..) }`, then derive field type/const metadata from the declared aggregate type.
5. Keep `sizeof` metadata-only. For adjusted parameters, size the selected array/pointer element from `StructFieldType`; never evaluate the parameter or inspect a runtime first element.

## Verification

Use focused tests for:

- aggregate-valued reads/returns through `choices[i].nested.points[j]`;
- scalar replacement, compound assignment, and increment through `choices[i].nested.values[j]`;
- `&choices[outer++].nested.values[inner++]` one-time evaluation;
- non-evaluating `sizeof` over get/set/compound/increment results;
- exact const-pointee, inner-bounds, and concrete aggregate-pointer conversion diagnostics;
- a warning-free fixture covering named nested, anonymous-inner, and union-array roots.

The 2026-07-19 fixture `tests/fixtures/valid/nested_holder_array_pointer_field_subscripts.c` returns 32 under Cust, GCC, and Clang.
