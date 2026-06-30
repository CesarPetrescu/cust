# Inline aggregate type definitions in `sizeof(*pointer_expr)` contexts

Date: 2026-06-30

Selected work package: conformance coverage for inline named aggregate definitions appearing inside non-evaluating `sizeof` operands whose expression form dereferences pointer expressions.

## Fixture shape

Use warning-free, ABI-independent native checks such as:

```c
if (sizeof(*(values + (sizeof(struct SizePtrOffset { int value; }) == sizeof(struct SizePtrOffset)))) == sizeof(int)) {
    total += values[one];
}
if (sizeof(*(points + (sizeof(struct SizeAggPtr { char tag; }) == sizeof(struct SizeAggPtr)))) == sizeof(struct Point)) {
    total += points[1].x;
}
if (sizeof(*(&((struct SizeFieldPtr { int value; }){19}).value)) == sizeof(int)) {
    total += ((struct SizeReadBack { int value; }){23}).value;
}
```

Keep the assertions as relationships (`sizeof(*ptr_expr) == sizeof(element-type)`) rather than native byte counts, because Cust's deterministic `int`/aggregate sizes intentionally differ from native ABI layout.

## Native oracle pitfall

Local `cc -std=c11 -Wall -Wextra -Werror` accepts inline named aggregate definitions inside these `sizeof` pointer-expression operands, but tags introduced only inside those `sizeof` expression operands are not visible to later block declarations on this host. Do **not** write follow-up declarations such as `struct SizePtrOffset object = {...};` after the `sizeof` expression in the native compiler-oracle fixture. Cust currently matches that non-leaking behavior for this context.

## Implementation decision

No production parser/runtime change was needed. Cust's existing expression-form `sizeof` metadata path already infers scalar, aggregate-array, and address-of-field pointer pointee types without evaluating operands, and the inline aggregate definitions parse successfully inside the nested `sizeof(type-name)` / aggregate compound-literal type-name subexpressions. Immediate focused GREEN is valid conformance coverage for this run.
