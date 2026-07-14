# Const pointer views derived from anonymous aggregate atomic value aliases

The 2026-07-14 autonomous run closed conformance coverage for const-qualified pointer views derived from anonymous aggregate atomic value aliases:

```c
typedef _Atomic(struct { int value; }) AtomicAnonValue;
typedef const AtomicAnonValue *ConstAtomicAnonView;
```

## Existing implementation path

- Leading `const` on the aggregate value alias is carried into `points_to_const` when the following declarator is a pointer. The pointer retains the original anonymous aggregate identity and its atomic-qualified pointee metadata.
- Mutable pointers and array decay may flow into `ConstAtomicAnonView`; declarations, assignments, and arguments that convert the const view back to `AtomicAnonValue *` report `cannot discard const qualifier from pointer target`.
- Scalar aggregate-field writes through struct and union const views report `cannot assign through pointer to const`.
- A later `typedef ConstAtomicAnonView const FixedConstAtomicAnonView` makes only the pointer slot const and reports `cannot assign to const variable` on reassignment.
- Existing one-level boundaries remain exact: another `*` reports `pointer-to-pointer typedef aliases are not supported`, and an array suffix on the pointer alias reports `pointer array typedef aliases are not supported`.
- Focused tests passed immediately after correcting one expected source column, so this was deliberate conformance closure with no production-code change.

## Native-oracle findings

GCC and Clang accept the shared C11 fixture with `-Wall -Wextra -Werror`; both binaries return 255, matching Cust. GCC emits `-Wmaybe-uninitialized` when uninitialized local atomic aggregate arrays are passed through array parameters even if the callee only compares their decayed addresses. Use zero-initialized global arrays as backing storage for warning-free pointer identity/type-query fixtures while retaining local pointer-view declarations.

## Focused verification

```bash
cargo test --test interpreter atomic_anonymous_aggregate_const_pointer -- --nocapture
cargo test --test c_compat -- --nocapture
```
