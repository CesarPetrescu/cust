# Inline union definitions in control-flow type queries

Date: 2026-06-29

## Summary

Inline named `union` definitions inside `if`, `while`, and `switch` controlling expressions already work through Cust's shared `sizeof(type-name)` parser path. When the type query parses `sizeof(union Tag { ... })`, the parser installs the union tag in the enclosing block scope, so the corresponding statement body can declare `union Tag` objects.

## Coverage pattern

Use a warning-free native-oracle fixture that compares a type query against the same newly installed tag and then reads from an initialized first `int` field:

```c
if (sizeof(union IfChoice { int value; char tag; }) == sizeof(union IfChoice)) {
    union IfChoice choice = {3};
    total += choice.value;
}
```

Repeat the pattern in `while` and `switch` controlling expressions. Avoid ABI-sensitive assertions about native union byte sizes; compare `sizeof(union Tag { ... }) == sizeof(union Tag)` and use exit-code arithmetic below 256.

## Implementation note

No production-code change was needed for the 2026-06-29 coverage run. If a future adjacent control-flow/type-query case fails, first inspect pending tag/enum scope routing around the enclosing statement parser before changing runtime aggregate storage.
