# Inline aggregate type definitions in function-call arguments

Date: 2026-06-29

## Summary

Cust already supports inline named `struct`/`union` definitions inside function-call argument subexpressions through the shared type-name and compound-literal parser paths. The 2026-06-29 autonomous run added conformance coverage for this less-traveled context rather than changing production code.

## Fixture pattern

Use a warning-free native-oracle fixture that:

- Defines a simple helper such as `int take(int left, int middle, int right)`.
- Calls it with arguments containing:
  - `sizeof(struct ArgBox { int value; }) == sizeof(struct ArgBox)`
  - `sizeof(union ArgChoice { int value; char tag; }) == sizeof(union ArgChoice)`
  - `((struct LitBox { int value; }){5}).value`
- Declares `struct ArgBox`, `union ArgChoice`, and `struct LitBox` later in the same block to prove tag visibility after argument parsing.
- Keeps return arithmetic below 256 for the native compiler-oracle exit-code comparison.

## Verification notes

Focused interpreter coverage passed immediately:

```bash
cargo test --test interpreter supports_inline_aggregate_call_argument_type_definitions -- --nocapture
```

Compiler-oracle coverage passed through the actual c_compat test function:

```bash
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```

Native checks should remain ABI-independent: compare `sizeof(Tag definition) == sizeof(Tag)` rather than exact host aggregate sizes.
