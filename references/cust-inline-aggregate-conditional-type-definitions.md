# Inline aggregate type definitions in conditional and short-circuit expressions

Date: 2026-06-29

## Summary

Conformance coverage locks in inline named `struct`/`union` definitions introduced from expression type-name operands inside conditional (`?:`) and short-circuit (`&&`/`||`) subexpressions.

The focused interpreter test passed immediately: Cust's shared type-name parsing already installs inline aggregate tags in the enclosing block scope while parsing all conditional/short-circuit operands, including unselected or unevaluated operands. No production parser/runtime change was needed.

## Fixture pattern

Use warning-free native-compatible expressions that compare relationships rather than native sizes:

```c
total += flag
    ? (sizeof(struct ThenBox { int value; }) == sizeof(struct ThenBox))
    : (sizeof(struct ElseBox { int value; }) == sizeof(struct ElseBox));
struct ThenBox then_box = {3};
struct ElseBox else_box = {5};

total += 0 && (sizeof(struct AndBox { int value; }) == sizeof(struct AndBox));
struct AndBox and_box = {7};

total += 1 || (sizeof(union OrChoice { int value; char tag; }) == sizeof(union OrChoice));
union OrChoice or_choice = {11};
```

## Verification notes

- Focused interpreter command: `cargo test --test interpreter supports_inline_aggregate_conditional_type_definitions -- --nocapture`.
- Compiler oracle command: `cargo test --test c_compat -- --nocapture` (the `c_compat` integration test has one test function; filtering by fixture name runs 0 tests).
- Keep native checks ABI-independent with `sizeof(Tag definition) == sizeof(Tag)` relationships and small return totals.
