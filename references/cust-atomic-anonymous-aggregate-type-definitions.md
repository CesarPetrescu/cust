# Anonymous aggregate definitions inside `_Atomic(type-name)`

The 2026-07-13 autonomous run added C11 anonymous `struct`/`union` definitions nested directly inside atomic type specifiers.

## Root cause and implementation

- `parse_decl_type_with_embedded_qualifiers()` already delegated named inline aggregate definitions to `parse_aggregate_definition_body_after_keyword(...)`, but its `struct`/`union` branch expected an identifier before `{`.
- Detecting a direct `{` after the aggregate keyword and enabling the existing `allow_anonymous` path creates the same unique internal aggregate identities used by other anonymous aggregate contexts.
- The outer `_Atomic(type-name)` parser remains unchanged, so supported objects, fields, parameters, `sizeof`, and `_Alignof` reuse deterministic no-op atomic behavior, while qualified/nested diagnostics retain their original source tokens.
- Separately spelled anonymous atomic aggregate types remain distinct; aggregate assignment between them must fail rather than becoming structurally compatible.

## Native-oracle caveat

GCC and Clang accept anonymous atomic aggregate objects, fields, locals, `sizeof`, and `_Alignof`. Anonymous aggregate definitions in parameter lists emit visibility warnings under `-Wall -Wextra -Werror`, so parameter coverage is interpreter-only. The warning-free compiler-oracle fixture uses ABI-independent size/alignment relationships and does not initialize or access atomic aggregate members.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate -- --nocapture
cargo test --test c_compat -- --nocapture
```
