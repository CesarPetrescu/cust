# Inline aggregate definitions in `_Static_assert` conditions

Date: 2026-06-29

## Context

Cust supports C11 `_Static_assert(condition, "message")` and inline named aggregate definitions in type-name contexts. A less-traveled conformance edge is defining an aggregate tag inside a static-assert condition, then using that tag later in the same block:

```c
_Static_assert(sizeof(struct AssertBox { int value; char tag; }) == sizeof(struct AssertBox), "ok");
struct AssertBox box = {7, 3};
```

The same pattern applies to `union` definitions.

## Result

Focused interpreter and native compiler-oracle coverage passed immediately. No production parser/runtime change was needed: shared `_Static_assert` condition parsing uses the existing type-name path, which installs inline aggregate tags in the enclosing block scope before later declarations are parsed/executed.

## Fixture guidance

- Keep native oracle checks ABI-independent by comparing `sizeof(struct Tag { ... }) == sizeof(struct Tag)` / `sizeof(union Tag { ... }) == sizeof(union Tag)` instead of exact byte sizes.
- Use warning-free post-assert declarations and first-field reads for unions under `cc -std=c11 -Wall -Wextra -Werror`.
- This is conformance coverage, not an implementation shortcut; native `cc` remains only an external oracle.

## Verification commands

```bash
cargo test --test interpreter supports_inline_aggregate_static_assert_type_definitions -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
