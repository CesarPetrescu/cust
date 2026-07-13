# Anonymous aggregate pointer arguments inside `_Atomic(type-name)`

The 2026-07-13 autonomous run closed conformance coverage for direct pointers to anonymous `struct`/`union` definitions nested inside atomic type specifiers.

## Existing implementation path

- The recursive `_Atomic(type-name)` parser already lowers an anonymous aggregate definition to a unique `DeclType::Struct` identity, then consumes one supported pointer star and maps it to `DeclType::Pointer`.
- Leading `const` on the anonymous aggregate becomes pointer `points_to_const` metadata. A write such as `cursor->value = 1` through `_Atomic(const union { ... } *) cursor` therefore reports `cannot assign through pointer to const` before dereference.
- Repeating the same anonymous spelling creates distinct pointee identities. Two function prototypes using separately spelled `_Atomic(struct { ... } *)` parameters conflict, proving that the pointer layer does not erase anonymous aggregate identity.
- Existing post-star guards already report exact source-located diagnostics for second-star, pointer-array, and function suffixes. No production-code change was needed; this was deliberate conformance coverage.

## Native-oracle findings

Local GCC and Clang with `-std=c11 -Wall -Wextra -Werror` both accept warning-free global/local/aggregate-field and `sizeof`/`_Alignof` uses of `_Atomic(struct { ... } *)` and `_Atomic(const union { ... } *)`. Anonymous aggregate parameter spellings emit parameter-list visibility warnings under `-Werror`, so parameter coverage remains interpreter-only.

Both compilers accept `_Atomic(struct { ... } **)` as C syntax, while Cust deliberately rejects it at the second star because the interpreter's safe pointer subset is one level deep. Both compilers reject atomic-qualified pointer arrays and function types. The shared oracle fixture therefore covers only supported one-level pointer forms with ABI-independent pointer size/alignment relationships.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_pointer -- --nocapture
cargo test --test c_compat -- --nocapture
```
