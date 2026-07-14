# Typedef aliases of derived anonymous aggregate atomic declarators

The 2026-07-14 autonomous run closed conformance coverage for pointer and fixed-array typedef aliases derived from anonymous aggregate atomic value aliases, for example:

```c
typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef AtomicAnonValue *AtomicAnonPtr, AtomicAnonArray[2];
```

## Existing implementation path

- The original `AtomicAnonValue` aggregate alias owns one anonymous internal type identity. Deriving `TypeAlias::Pointer` and `TypeAlias::Array` entries from it retains that identity through global/local objects, aggregate fields, named/unnamed parameters, compatible prototypes/definitions, pointer arithmetic, array-parameter decay, `sizeof`, and `_Alignof`.
- Atomic qualification remains on the pointer pointee rather than the pointer slot. Consequently, `_Atomic(AtomicAnonPtr)` is a supported atomic pointer type; this differs from rewrapping an alias originally declared as `typedef _Atomic(T *) Alias`, whose pointer slot is already atomic and is rejected as nested qualification.
- A later `typedef AtomicAnonPtr const FixedAtomicAnonPtr` preserves const pointer-slot metadata and rejects reassignment with `cannot assign to const variable`.
- Existing safety boundaries remain exact through the derived aliases: another pointer star reports `pointer-to-pointer typedef aliases are not supported`, an array suffix on the pointer alias reports `pointer array typedef aliases are not supported`, and another array suffix on the fixed-array alias reports `multidimensional array typedef aliases are not supported`.
- The focused tests passed immediately, so this was deliberate conformance closure with no production-code change.

## Native-oracle findings

Local GCC and Clang accept the shared fixture with `-std=c11 -Wall -Wextra -Werror`; both binaries return 255, matching Cust. The fixture uses pointer identity and ABI-independent `sizeof`/`_Alignof` relationships rather than accessing fields of atomic aggregate objects.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_derived -- --nocapture
cargo test --test c_compat -- --nocapture
```
