# Inline named aggregate definitions inside `_Atomic(type-name)`

The 2026-07-13 conformance run covered inline named `struct` and `union` definitions nested directly inside C11 atomic type specifiers.

## C11 differential results

- GCC and Clang accept `_Atomic(struct Tag { ... })` and `_Atomic(union Tag { ... })` for file/block objects, aggregate fields, function parameters, `sizeof`, and `_Alignof`.
- Tags from file/block declarations, aggregate fields, and type queries follow ordinary lexical scope and remain available to later declarations in that scope.
- Parameter-list tags are visible in the function body but not after the function declaration. GCC and Clang emit parameter-list tag-visibility warnings rejected by the repository's `-Werror` flags, so the compiler-oracle fixture omits that route while interpreter coverage retains it.
- Both compilers reject qualified and nested forms such as `_Atomic(const struct Tag { ... })` and `_Atomic(_Atomic(union Tag { ... }))`.

## Cust coverage pattern

- The existing recursive `_Atomic(type-name)` parser delegates named aggregate definitions to `parse_aggregate_definition_body_after_keyword`, which installs the tag in the active aggregate-tag scope.
- Existing file, block, aggregate-field, function-parameter, and type-query scope routing already preserves visibility and non-leakage; focused tests therefore passed immediately and no production change was needed.
- Keep compiler-oracle assertions ABI-independent by comparing an atomic aggregate object/type query with the same named aggregate type, rather than asserting native byte sizes or alignments.
- Exact negative tests should retain the qualifier or nested `_Atomic` token location.

## Next boundary

Anonymous aggregate definitions inside atomic type specifiers remain unsupported by Cust: native GCC and Clang accept `_Atomic(struct { int value; })`, while Cust currently reports `expected _Atomic type name, found LBrace`. This is the next concrete P0 atomic conformance slice.

## Focused verification

```bash
cargo test --test interpreter atomic_inline_aggregate -- --nocapture
cargo test --test c_compat -- --nocapture
```
