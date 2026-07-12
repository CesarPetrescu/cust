# Qualified `_Atomic` typedef arguments

## C11 differential result

Local `cc -std=c11 -Wall -Wextra -Werror -fsyntax-only` rejects `_Atomic(Alias)` when `Alias` denotes a top-level-qualified type:

- `typedef const int Alias;`
- `typedef volatile int Alias;`
- `typedef int * const Alias;`
- `typedef int * volatile Alias;`

It accepts aliases for unqualified pointer slots whose pointees are qualified (`typedef const int *View;` and `typedef volatile int *View;`). It also accepts `_Atomic(ConstInt *)` when `ConstInt` is `typedef const int ConstInt`, because the added star forms an unqualified pointer to a const pointee.

## Implementation pattern

- Keep top-level-qualified typedef metadata separate from `const_type_alias_scopes`, which also drives Cust's runtime read-only behavior.
- Resolve qualifier metadata by zipping it with `type_alias_scopes` and stopping at the innermost scope containing the alias. Searching qualifier sets independently breaks legal inner unqualified aliases that shadow outer qualified aliases.
- In `_Atomic(type-name)` parsing, preserve the alias token before parsing. Reject a top-level-qualified alias at that token when no direct star follows.
- If a const scalar alias is followed by `*`, carry the alias's const metadata into `points_to_const`; do not reject it as a qualified pointer slot.
- For comma-separated typedef declarators, a qualifier before the base applies to no-star aliases and pointees of explicit-star aliases; qualifiers after a star apply to the pointer slot.

## Verification

Focused commands:

```bash
cargo test --test interpreter rejects_qualified_atomic_typedef_arguments_with_context -- --nocapture
cargo test --test interpreter supports_atomic_typedef_pointers_to_qualified_pointees -- --nocapture
cargo test --test interpreter atomic -- --nocapture
cargo test --test c_compat -- --nocapture
```

The valid compiler-oracle fixture should include lexical shadowing, pointer aliases to const/volatile pointees, and a const scalar alias followed by `*`.
