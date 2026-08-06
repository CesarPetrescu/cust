# Bounded character-storage `memcmp`

## Scope

Cust recognizes only the normalized explicit declaration:

```c
int memcmp(const void *left, const void *right, unsigned long int count);
```

A user-defined `memcmp` body takes precedence. Missing declarations remain undefined functions, and incompatible declarations receive the dedicated standard-library declaration diagnostic at runtime and beneath `sizeof`.

## Runtime model

1. Evaluate `left`, `right`, and `count` exactly once in source order.
2. Require a scalar nonnegative count no greater than 4,096.
3. Validate both pointers as live interpreter-owned character storage and validate each remaining capacity independently.
4. Read exact Cust character-cell values without mutation. Overlap is permitted because comparison has no write range.
5. Normalize each value with Euclidean modulo 256 for C `unsigned char` ordering. Do not narrow the shared storage snapshot used by `memcpy`/`memmove`; negative Cust `signed char` values must remain intact outside this comparison.
6. Return `-1`, `0`, or `1` according to the first differing normalized byte. C requires the sign, not a particular magnitude. Zero count returns zero.

Integer and aggregate object representations remain deliberately unsupported. Native compilers are test oracles only; Cust never calls host `memcmp`.

## Non-evaluating and dispatch audit

Add the intrinsic consistently to:

- exact prototype recognition;
- runtime dispatch before generic undefined-function handling;
- incompatible-prototype diagnostics;
- structural nested-call validation beneath non-evaluating contexts;
- top-level `sizeof(call)` return-size classification;
- user-definition precedence.

`sizeof(memcmp(...))` returns Cust's deterministic `int` size without evaluating arguments, but still validates arity and pointer/count shapes.

## TDD and verification

Focused regressions should cover:

- equality, first-difference ordering, embedded NULs, offset pointers, and unsigned high-byte ordering;
- zero count and one-time source-order evaluation;
- independent left/right capacity diagnostics, expired owners, non-character storage, negative/oversized counts, and count-shape errors;
- exact declaration handling at runtime and beneath `sizeof`;
- nested non-evaluation and user-definition precedence;
- a warning-free native compiler-oracle fixture registered in `tests/c_compat.rs`.

Run `cargo test --test interpreter memcmp -- --nocapture` and the actual `cargo test --test c_compat -- --nocapture` harness, then the canonical local and Docker gates.

## Verification pitfall discovered

The pre-existing `character_pointer_object_conditional_sizeof_validation_remains_linear` test compares depth 8 with depth 40. A 4x timing ceiling is below the expected 5x work increase for genuinely linear traversal and produced a Docker-only false positive. An 8x ceiling tolerates linear scaling while still rejecting quadratic 25x growth; verify threshold changes with repeated focused runs and an independent review before rerunning the canonical gate.
