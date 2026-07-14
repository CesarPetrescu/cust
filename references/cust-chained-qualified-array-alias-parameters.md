# Chained qualified array-alias parameters

The 2026-07-14 autonomous run extended fixed-array typedef parameter-adjustment coverage through an additional typedef layer and comma-separated declarator lists:

```c
typedef const int ConstInt;
typedef ConstInt ConstInts[3], ConstIntPair[2], *ConstIntView;
typedef ConstInts ChainedConstInts;
```

Matching named-struct and enum aliases are covered as well.

## Result

Cust's existing typedef metadata is transitive for this supported one-dimensional array shape. A chained array alias retains its element type and alias-carried `const`, so C parameter adjustment still produces an assignable pointer slot to a const pointee. Unnamed prototypes remain compatible with named definitions, calls preserve concrete pointee types, and lexical shadowing resolves the innermost alias.

Focused valid and exact-negative tests passed immediately as deliberate conformance coverage; no production-code change was needed. The regressions prove that direct writes remain rejected and mutable pointer declarations/arguments cannot discard qualification.

## Native oracle guidance

GCC and Clang compile the shared fixture under `-std=c11 -Wall -Wextra -Werror`; both binaries return 127, matching Cust. Keep named aggregate and enum checks ABI-independent:

```c
sizeof(ChainedConstPoints) == 2 * sizeof(struct Point)
_Alignof(ChainedConstStates) == _Alignof(enum State)
```

Initialized static local arrays avoid warning noise from qualified automatic objects.

## Focused verification

```bash
cargo test --test interpreter chained_qualified_array_alias -- --nocapture
cargo test --test c_compat -- --nocapture
```
