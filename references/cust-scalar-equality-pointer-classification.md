# Scalar equality must classify pointer operands before evaluation

## Trigger

Nested scalar array-field element reads through a struct pointer can lower to
`Expr::StructPtrArrayGet`, for example:

```c
struct Outer *root = /* ... */;
return root->inner.values[1] == 23;
```

`eval_pointer()` supports this AST variant because the same syntax can index a
pointer-valued aggregate field. For an embedded scalar array field it returns
the selected element's address in pointer context. Speculatively calling
`eval_pointer()` on both sides of equality therefore misclassifies the scalar
field read as a pointer and reports:

```text
cannot compare pointer with nonzero integer
```

## Fix pattern

Use `expr_is_pointer_value()` before evaluating equality operands. When both
operands are classified as scalar, evaluate both with `eval()` and compare the
scalar results directly. Preserve the existing pointer equality and
pointer/nonzero-integer diagnostic paths whenever either operand is classified
as pointer-valued.

Do not remove the `eval_pointer()` support for `StructPtrArrayGet`; pointer-field
indexing depends on it. The bug is speculative evaluation in the classifier,
not the pointer evaluator's contextual route.

## Regression coverage

Use a fixture that compares both named-nested and anonymous-inner scalar array
field elements to nonzero integers through captured aggregate compound-literal
struct pointers. Pair it with pointer identity/null comparisons and retain the
existing exact `p == 1` diagnostic test.

Focused commands:

```bash
cargo test --test interpreter supports_nested_anonymous_aggregate_compound_literal_field_pointer_alias_mutation_model_routes -- --nocapture
cargo test --test interpreter rejects_pointer_comparison_with_nonzero_integer -- --nocapture
```

The 2026-07-17 model fixture also verifies that four captured compound-literal
roots are evaluated once and that nested field-path identity survives forwarding
and pointer wrappers.
