# Inline Enum Declaration Contexts

Date: 2026-06-22

## Scope

This run extended the prior inline enum object declaration work from ordinary local/global declarations into declaration contexts that have their own parser entry points:

- `for (enum { START = 2 } i = START; ... )`
- block-scope `static enum { SAVED = 4 } saved = SAVED;`
- block-scope `auto enum { A = 1 } value = A;`
- block-scope `register enum { R = 1 } value = R;`

The same parser routing was also opened for `_Thread_local` and `_Alignas` declaration paths so they do not regress on the same `enum` token boundary, though the focused fixture exercises the warning-free native `for`/`static`/`auto`/`register` subset.

## Root cause

The prior inline enum object support was implemented in `parse_var_decl()` by allowing `parse_decl_type()` to parse `enum { ... }` as an integer-like scalar type and stash the parsed constants in `pending_inline_enum_constants`. `parse_var_decl()` then wraps the resulting declaration as:

```rust
Stmt::Many(vec![Stmt::EnumDecl { constants }, variable_decl])
```

Special declaration entry points such as `parse_for()` initializers and storage-class handlers did not all route `Token::Enum` into `parse_var_decl()`. In particular, `static enum { ... } saved = SAVED;` failed before implementation with:

```text
expected declaration after static, found Enum
```

## Implementation notes

- Add `Token::Enum` to declaration-context routing for `for` initializers and local storage/alignment handlers (`static`, `_Thread_local`, `auto`, `register`, `_Alignas`).
- Do **not** wrap generated `Stmt::EnumDecl` nodes in `Stmt::StaticLocal`; enum constants are runtime declarations, not static variables.
- Static local wrapping must recurse through `Stmt::Many`, preserve `Stmt::EnumDecl` as-is, and assign static-local IDs only to actual variable declarations (`VarDecl`, `PointerDecl`, `ArrayDecl`, `StructVarDecl`, `StructArrayDecl`).
- This matters for same-statement static initializers: the enum constants must be inserted before the first-time static variable initialization expression is evaluated.

## Verification pattern

Focused RED/GREEN command:

```bash
cargo test --test interpreter inline_enum_object_declarations_in_storage -- --nocapture
```

Native oracle smoke check for expected exit arithmetic:

```bash
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_enum_declaration_contexts.c -o /tmp/cust-inline-enum-contexts && /tmp/cust-inline-enum-contexts; printf 'exit=%s\n' "$?"
```

The native oracle returned `41`; if the new interpreter fixture returns another value, recompute fixture arithmetic before assuming a runtime regression.
