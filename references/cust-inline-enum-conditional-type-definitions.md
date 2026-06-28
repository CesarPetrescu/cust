# Inline enum definitions in conditional and short-circuit expressions

Date: 2026-06-28

## Summary

Coverage-only conformance closure for inline enum type definitions nested inside conditional (`?:`) operands and short-circuit logical operands. The existing pending inline enum wrapper already emits generated enum constants before runtime evaluation of the enclosing statement, so no production parser/runtime change was needed.

## Fixture shape

Use a warning-free native-compatible program that keeps its return value below 256:

```c
int main(void) {
    int flag = 0;
    int total = flag
        ? (sizeof(enum BranchA { A = 11 }) ? A : 0)
        : (sizeof(enum BranchB { B = 13 }) ? B : 0);
    total += A + B;
    total += (0 && ((enum ShortAnd { SA = 17 })0)) ? 1 : SA;
    total += (1 || (sizeof(enum ShortOr { SO = 19 }))) ? SO : 0;
    return total; /* 73 */
}
```

This proves enumerators introduced in both selected and unselected conditional branches, and in unevaluated short-circuit operands, are still available to later expressions in the same block.

## Verification

- `cargo test --test interpreter supports_inline_enum_conditional_type_definitions -- --nocapture`
- `cargo test --test c_compat -- --nocapture`

Immediate focused GREEN is valid here because the work package is explicit conformance coverage for an already-supported less-traveled surface.
