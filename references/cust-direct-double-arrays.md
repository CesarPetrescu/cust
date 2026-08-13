# Direct one-dimensional bounded `double` arrays

Date: 2026-08-13

## Completed slice

Direct local, file-global, and block-static `double` arrays support fixed and initializer-inferred lengths, positional/designated initialization, zero fill, direct and reverse indexed reads, replacement/compound/prefix/postfix updates, scalar conversion, `_Generic`, and deterministic object/element/type-name `sizeof` plus `_Alignof` relationships.

## Runtime representation

Array elements reuse `ArrayValue` scalar cells and store exact `f64::to_bits()` payloads. Every read or update result must be classified as `CType::Double` before decoding the cell; integer-style incrementing the raw bits is incorrect.

## Reverse-subscript pitfall

For `index[values]`, parser lowering produces `Expr::ArrayGet { name: "index", index: Var("values") }`. Consequently:

- `expr_is_double_value()` must combine the ordinary named-array element type with `scalar_variable_reverse_subscript_pointee_type(name, index)`.
- `_Generic(index[values], double: ..., default: ...)` depends on that classification before generic scalar fallback examines the offset variable.
- Reverse prefix/postfix increment must derive the pointee type from the temporary pointer and apply f64 arithmetic to stored bits.
- `sizeof(index[values])` already routes through reverse-subscript metadata and must remain non-evaluating.

## Pointer and intrinsic boundaries

No ordinary `double *` is enabled. Reject decay, dereference, address-taking, pointer declarations, pointer casts, pointer arithmetic, and multidimensional double arrays. Internal element pointers are still needed for reverse subscript and existing raw-memory validation. Do not reject every internal pointer in `eval_pointer()`: exactly prototyped `mem*` calls must materialize the pointer and report their established `does not yet support double object storage` diagnostic.

## Verification

Run:

```bash
cargo test --test interpreter double -- --nocapture
cargo test --test c_compat -- --nocapture
gcc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/direct_double_arrays.c -o /tmp/direct-double-gcc
clang -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/direct_double_arrays.c -o /tmp/direct-double-clang
```

The compiler oracle uses only warning-free, ABI-independent size/alignment relationships.
