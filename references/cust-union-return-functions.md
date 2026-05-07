# Cust union return functions

Date: 2026-05-07

## Scope

Completed the direct `union Name f(...)` function/prototype spelling for Cust's existing safe aggregate-by-value runtime. This closes a gap where union values already shared the struct-backed `ReturnType::Struct`/`Value::Struct` machinery, but top-level dispatch only recognized `struct Name f(...)` as aggregate-returning function syntax.

## RED

Added `tests/fixtures/valid/union_return_functions.c`, wired it through `tests/interpreter.rs`, and added native compiler-oracle coverage in `tests/fixtures/compat/valid/union_return_functions.c` / `tests/c_compat.rs`.

The focused RED command failed as expected:

```bash
cargo test --test interpreter supports_union_return_functions_and_prototypes -- --nocapture
```

Initial failure:

```text
expected ';' after struct variable declaration, found LParen at line 6, column 25
```

This showed the parser treated `union Number make_number(...)` as a union variable declaration instead of a function prototype/definition.

## GREEN

Implementation changed `starts_struct_function_declaration()` to recognize `Token::Struct | Token::Union` before `Ident Ident (`. The existing return-type parser, signature compatibility checks, return validation, aggregate copy semantics, and `sizeof` support already operate over aggregate type names.

The fixture also exposed that assignment-expression statements such as `n = make_number(5);` route through `eval_discard()`, not `Stmt::Assign`, because assignment is now a general expression. `eval_discard()` now recognizes aggregate variable assignment expressions and delegates to `assign_struct_copy()`, so side-effect-only struct/union copy assignment from aggregate-returning calls works without trying to evaluate the returned aggregate as a scalar.

## Verification

Focused checks used during the TDD loop:

```bash
cargo test --test interpreter supports_union_return_functions_and_prototypes -- --nocapture
cargo test --test interpreter union -- --nocapture
cargo test --test c_compat -- --nocapture
```

Full verification was run before commit by the autonomous maintenance gate.

## Pitfalls

- Direct aggregate initialization from a function call (`union Number n = make_number(5);`) is not supported yet; use a declaration plus copy assignment for now.
- Native C compiler-oracle fixtures should continue to avoid ABI-size/padding assertions for unions.
- The existing diagnostic strings still say `struct` for the shared aggregate value path in some cases; avoid broad diagnostic churn unless driven by exact parser/interpreter tests.
