# Inline type definitions in array length contexts

Date: 2026-06-29

Autonomous coverage closure for inline enum and aggregate type definitions that appear inside parser-folded integer constant expressions used by array declarator lengths and array type-name lengths.

## What was verified

- `int values[sizeof(enum LengthTag { LENGTH_VALUE = 5 })];` emits the inline enum constants before later statements use `LENGTH_VALUE`.
- `typedef int CellSized[sizeof(struct Cell { char tag; })];` installs the inline aggregate tag so later `struct Cell` declarations work in the same block.
- `sizeof(int[sizeof(enum ArrayLengthTag { ARRAY_LENGTH_VALUE = 3 })])` works as an array type-name operand whose length expression defines an enum visible later in the block.
- `sizeof(struct Box { struct Cell inner; char tail; }[2]) == 2 * sizeof(struct Box)` locks in inline aggregate definitions inside aggregate array type-name operands while keeping native oracle checks ABI-independent.

## Implementation note

No production parser/runtime change was needed. Focused interpreter coverage passed immediately because the shared integer-constant-expression and type-name paths already preserve pending inline enum declarations and aggregate tag definitions for these contexts. Treat this as conformance coverage closure, not a runtime fix.

## Verification commands

```bash
cargo test --test interpreter supports_inline_type_definitions_in_array_lengths -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
