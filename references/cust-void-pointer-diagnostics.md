# Bounded first safe `void *` object/conversion slice

Date: 2026-08-05

Cust now supports a deliberately bounded one-level `void *` object model. The implementation stores only interpreter-owned `PointerValue` identities; it never exposes or relies on host addresses.

## Supported forms

- Local, file-global, block-static, `for`-initializer, and function-parameter `void *` / `const void *` objects.
- Null initialization/assignment, equality/inequality, truthiness, ordinary assignment, assignment results, conditional values, comma values, and forwarding through compatible functions.
- Implicit conversion from supported object pointers to `void *`, and from `void *` back to compatible object pointers when qualification is preserved.
- Explicit one-level `(void *)` casts, `sizeof(void *)`, `_Alignof(void *)`, and non-evaluating `sizeof` classification of valid assignment/conditional/comma forms.
- Existing owner, lexical lifetime, hidden-storage lifetime, and read-only metadata are retained across conversions.

## Intentional boundaries

- Dereference and indexing report `cannot dereference pointer to void` / `cannot index pointer to void`.
- Addition, subtraction, increment/decrement, compound arithmetic, unary arithmetic, and relational ordering report the corresponding `pointer to void ... is not supported` diagnostics, including beneath `sizeof`.
- Pointer difference, incompatible scalar/aggregate conversion, qualification discard, deeper pointers, pointer-to-array forms, and function-pointer forms remain unsupported with exact diagnostics.
- Equality and truthiness are supported; relational ordering is not.

## Implementation notes

- `PointeeType::Void` is a zero-sized type marker for compatibility classification only; `PointeeType::size()` rejects it. Pointer objects still have deterministic pointer size.
- `pointer_type_compatible()` models only the bounded object-pointer conversion rules and keeps qualification checking separate from pointee-type compatibility.
- `pointer_expr_pointee_type()` and the normal pointer evaluator preserve metadata without evaluating expressions in `sizeof` paths.
- `for` declaration lookahead must include `Token::Void`; otherwise a legal `for (void *p = ...; ...)` declaration falls into expression parsing.
- Native `cc` is used only as a warning-free compatibility oracle for defined valid behavior. Invalid dereference/arithmetic fixtures are interpreter-only because native extensions or undefined behavior are unsuitable oracles.

## TDD/review closure

Focused RED/GREEN regressions covered the missing `for`-initializer declaration route and invalid `void *` arithmetic/order/increment/compound/unary forms beneath non-evaluating `sizeof`. The complete slice is covered by focused interpreter tests, registered compiler-oracle coverage, invalid fixture tests, recursion-depth coverage, independent review, and the canonical local/Docker verification gate.
