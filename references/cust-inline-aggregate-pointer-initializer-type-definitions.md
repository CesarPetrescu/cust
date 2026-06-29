# Inline aggregate definitions in pointer initializer expressions

Date: 2026-06-30

## Summary

Inline named `struct` / `union` definitions inside pointer-valued initializer and assignment expressions are conformance-covered by `inline_aggregate_pointer_initializer_type_definitions.c`.

Representative supported forms:

```c
int *field = &((struct PointerBox { int value; }){5}).value;
struct PointerBox pointer_box = {3};

field = &((struct AssignPointerBox { int value; }){7}).value;
struct AssignPointerBox assign_pointer_box = {11};

int *union_field = &((union PointerChoice { int value; char tag; }){13}).value;
union PointerChoice pointer_choice = {17};
```

## Implementation decision

No production change was needed. Focused interpreter coverage passed immediately because Cust's existing aggregate compound-literal type-name parsing installs inline aggregate tags in the enclosing block scope before later declarations, and existing address-of aggregate compound-literal field support provides safe interpreter-owned scalar pointers.

## Verification notes

- Native oracle smoke check: `cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_aggregate_pointer_initializer_type_definitions.c ...` exits `56`.
- Interpreter focused test: `cargo test --test interpreter supports_inline_aggregate_pointer_initializer_type_definitions -- --nocapture`.
- Compiler oracle: add the compat fixture to `tests/c_compat.rs` and run `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`.

## Pitfalls

- Do not declare `struct PointerBox *p = ...` before the inline tag exists; use a scalar pointer to a field selected from the compound literal when the goal is tag-installation coverage in an initializer expression.
- Keep native return arithmetic in `0..255`; the compiler-oracle harness compares process exit codes.
