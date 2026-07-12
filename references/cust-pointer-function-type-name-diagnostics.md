# Pointer function type-name diagnostics

## Scope

Cust intentionally excludes abstract function types, including forms that spell a function returning a one-level pointer:

```c
(int *(void))0
sizeof(int *(void))
_Alignof(int *(void))
```

These are valid C type spellings but outside Cust's safe type-name subset.

## Parser rule

After parsing any supported one-level pointer suffix in `parse_cast()` or `parse_sizeof_like_type_name()`, call `reject_function_type_suffix(...)` before generic closing-parenthesis handling. Do this in every pointer route:

- anonymous aggregate pointer casts/type queries;
- direct scalar and named aggregate pointer casts/type queries; and
- pointer typedef aliases (`DeclType::Pointer`).

The helper deliberately preserves parenthesized-pointer handling by ignoring `(` followed by `*`; those forms retain their established dedicated diagnostics.

## Regression coverage

`rejects_function_type_cast_and_type_query_names_with_context` asserts exact source-located diagnostics for scalar, named aggregate, anonymous aggregate, and pointer-typedef spellings across casts, `sizeof`, and `_Alignof`.

## Verification

```bash
cargo test --test interpreter rejects_function_type_cast_and_type_query_names_with_context -- --nocapture
cargo test --test interpreter function_array -- --nocapture
cargo test --test interpreter parenthesized_pointer_cast -- --nocapture
```
