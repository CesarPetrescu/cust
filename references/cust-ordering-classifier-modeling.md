# Relational ordering expression classification

Date: 2026-07-17

## Problem

Cust's relational branch checked whether either operand was pointer-valued, then speculatively called `eval_pointer()` for both operands. Some scalar AST variants are intentionally addressable in pointer contexts. In particular, a scalar nested array-field read such as `view->nested.values[1]` can yield an address from `eval_pointer()`, so `pointer < scalar_field_read` was misclassified as two pointers and reported `cannot compare pointers to different arrays` instead of the established mixed-ordering diagnostic.

## Fix pattern

Classify both operand result shapes with `expr_is_pointer_value()` before evaluation:

- scalar/scalar: evaluate each once with `eval()` and apply `<`, `<=`, `>`, or `>=`;
- pointer/pointer: evaluate each once with `eval_pointer()` and reuse `pointer_ordering()`;
- mixed pointer/scalar: report `pointer ordering comparisons are not supported` without speculative evaluation.

Keep same-root pointer ordering and exact cross-root/null diagnostics unchanged.

## TDD and verification

The fixed-seed model covers 96 cases: 32 scalar pairs, 48 pointer pairs, and 16 mixed pairs across all four relational operators. Routes include nested scalar field reads, literals/casts/unary and pointer-difference wrappers, array/indexed addresses, direct/arrow pointer fields, pointer-returning calls, conditionals, commas, pointer casts, same/cross-root pointers, null, side-effect counts, exact diagnostics, and panic freedom.

Focused commands:

```bash
cargo test --test fuzz_safety generated_scalar_and_pointer_ordering_classification_matches_model_without_panics -- --nocapture
cargo test --test interpreter ordering_classification -- --nocapture
cargo test --test interpreter nested_scalar_field_reads -- --nocapture
cargo test --test c_compat -- --nocapture
```

The warning-free C11 compiler-oracle fixture routes known same-array pointers through a helper to avoid tautological/address warnings; Cust, GCC, and Clang return 130.
