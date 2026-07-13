# Inline enum definitions inside `_Atomic(type-name)`

The 2026-07-13 conformance run covered inline enum definitions nested directly inside C11 atomic type specifiers.

## C11 differential results

- GCC and Clang accept `_Atomic(enum Tag { VALUE = N })` for file-scope and block-scope objects, aggregate fields, `sizeof`, and `_Alignof`.
- Enumerators from file/block declarations and type queries follow their ordinary lexical scopes and are available to the declaration initializer or later statements in that scope.
- Inline enum definitions in atomic function parameters are accepted and their enumerators are visible in the function body, but both compilers emit a parameter-list tag-visibility warning. The repository's `-Wall -Wextra -Werror` oracle therefore omits that form while interpreter coverage retains it.
- Both compilers reject qualified and nested forms such as `_Atomic(const enum Tag { ... })` and `_Atomic(_Atomic(enum Tag { ... }))`.

## Cust coverage pattern

- The existing recursive `_Atomic(type-name)` parser delegates inline enum definitions to the shared enum type parser, which queues generated `EnumDecl` statements. Existing declaration, aggregate-field, function-parameter, and type-query wrappers already install those declarations in the correct lexical scope.
- Exact negative tests should retain the qualifier or nested `_Atomic` token location.
- Keep compiler-oracle type queries ABI-independent by comparing equivalent atomic enum spellings rather than atomic and non-atomic enum sizes or alignments.
- Immediate focused GREEN is valid for this deliberate conformance-coverage package; no production parser/runtime change is required when all routing and scope tests pass.

## Focused verification

```bash
cargo test --test interpreter atomic_inline_enum -- --nocapture
cargo test --test c_compat -- --nocapture
```
