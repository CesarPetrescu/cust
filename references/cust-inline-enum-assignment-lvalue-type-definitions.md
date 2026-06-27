# Inline enum definitions in assignment lvalue statements

Date: 2026-06-27

## Symptom

Inline enum definitions in `_Alignof(enum Tag { ... })` type-name expressions were already parsed, but lvalue-specific assignment statement parsers could return direct assignment statements without emitting pending inline `EnumDecl`s before evaluating the RHS.

A focused RED fixture failed with:

```text
undefined variable 'ARRAY_ALIGN'
```

for:

```c
values[0] = _Alignof(enum ArrayAlign { ARRAY_ALIGN = 3 }) + ARRAY_ALIGN;
```

## Root cause

`parse_assign_with_semi()` and `parse_deref_assign_with_semi()` had several early returns for lvalue statement forms:

- scalar array assignment / compound assignment
- direct struct field assignment / compound assignment
- direct struct array-field element assignment / compound assignment
- dereference assignment / compound assignment

Only plain scalar assignment and scalar compound assignment used `with_pending_inline_enum_decl(...)`, so pending enumerators could be delayed until a later wrapping statement, which was too late when the same RHS referenced the enumerator.

## Fix pattern

Wrap every statement returned after parsing an expression that can queue pending inline enum constants:

```rust
return Ok(self.with_pending_inline_enum_decl(Stmt::ArrayAssign { ... }));
```

Apply this at every early-return assignment-statement branch, not just `Stmt::Assign`.

## Verification

Focused interpreter fixture:

```bash
cargo test --test interpreter inline_enum_assignment_lvalue_type_definitions -- --nocapture
```

Compiler-oracle fixture:

```bash
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

The native fixture uses `_Alignof(enum Tag { ... }) == _Alignof(enum Tag)` style relationships and exit-code comparison rather than ABI-sensitive exact alignment values.
