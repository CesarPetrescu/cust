# Derived inner const pointers across adjusted-parameter callee returns

Date: 2026-07-21

## Scope

A `struct Item items[]` parameter is adjusted to a copied aggregate pointer slot. A callee may derive an `int *` or named-aggregate pointer from an embedded array, promote it to `const T *` through its return type, and return it to the caller. One/two-hop return helpers and caller-side conditional/comma wrappers or same-array offsets must preserve the original direct compound-literal hidden root or captured containing root/path, the nonzero outer base, the inner index, the concrete pointee type, and const qualification.

## TDD result

The generated matrix covers both scalar and named-aggregate inner pointees; direct aggregate-array compound literals plus named, anonymous, and union-containing captured field roots; all caller conditional-true, conditional-false, and comma wrappers; all caller `pointer + 1`, `1 + pointer`, and `&pointer[1]` offsets; and one/two-hop returning callees. This produces 144 valid identity/read programs. Eleven panic-guarded invalid programs retain exact const-discard/write, inner-bounds, cross-root subtraction, concrete aggregate type, and out-of-scope lifetime diagnostics.

RED exposed a real scalar embedded-array lifetime bug. `PointerValue::ArrayBase` and `PointerValue::ArrayElement` retained their `Rc<ArrayValue>` after a local containing aggregate left scope, but unlike aggregate pointer targets they carried no owner scope or root name. A pointer returned through an adjusted parameter could therefore read the expired local storage.

## Implementation pattern

- Add optional `ArrayPointerOwner { scope_id, name }` metadata to scalar array pointer values.
- At a pointer return boundary, recursively locate the returned `Rc<ArrayValue>` in owned `Value::Array`, `Value::Struct`, or `Value::StructArray` storage and attach the lexical owner.
- Do not classify `Value::Pointer` parameter slots as owners: they borrow the caller's storage.
- Propagate owner metadata through array arithmetic and indexed-address operations.
- Check owner liveness before pointer type inference, arithmetic/indexing, assignment indexing, and dereference.
- Keep strings and otherwise unowned hidden array storage as `owner: None`; this change is scoped to storage whose lexical root can be identified.

Focused command:

```bash
cargo test --test fuzz_safety generated_derived_inner_const_pointer_callee_returns_preserve_adjusted_parameter_identity_without_panics -- --nocapture
```

Follow with the canonical local/Docker gate. The next distinct seam is placing const promotion plus conditional/comma wrappers and nonzero inner offsets inside the returning callee before the pointer crosses the return boundary.
