# Enum typedef declaration-list conformance

Date: 2026-06-26

## Context

`typedef enum State { ... } State, *StatePtr, StateArray[N];` is an ordinary C declaration-list form adjacent to the already-covered named aggregate typedef declaration-list path. Cust lowers enum aliases onto its deterministic integer scalar model while preserving C syntax for pointer and array aliases.

## Implementation decision

The audited path already worked through the comma-separated typedef declarator machinery: after an inline enum typedef definition, each declarator can become a scalar alias, a one-level pointer alias, or a one-dimensional array alias. Add conformance fixtures rather than production code when focused interpreter coverage passes immediately.

## Fixture pattern

Use both interpreter and compiler-oracle fixtures covering:

- inline named enum typedef definition with comma-separated aliases;
- pointer alias (`StatePtr`) over an enum array;
- array alias (`StateArray`) decay to pointer parameters;
- enum compound literals and scalar casts;
- pointer arithmetic/indexing over the enum array;
- ABI-independent native checks such as `sizeof(StateArray) == N * sizeof(State)`, `_Alignof(StateArray) == _Alignof(State)`, and `sizeof(StatePtr) == sizeof(State *)`.

Avoid exact native enum byte-size or alignment assertions because Cust intentionally maps enum storage onto its deterministic integer model.
