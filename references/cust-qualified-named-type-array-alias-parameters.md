# Qualified named-type array alias parameters

The 2026-07-14 autonomous run generalized fixed-array typedef parameter-adjustment coverage from scalar and anonymous atomic aggregate elements to named structs, unions, and enums:

```c
typedef const struct Point ConstPoint;
typedef const union Number ConstNumber;
typedef const enum State ConstState;
typedef ConstPoint ConstPoints[2];
typedef ConstNumber ConstNumbers[2];
typedef ConstState ConstStates[2];
```

## Result

The prior `parse_params()` fix is type-family-independent. When a `DeclType::Array` alias appears as a parameter, alias-carried `const` becomes pointee qualification on the adjusted pointer, not constness of the pointer slot. Therefore:

- unnamed prototypes are compatible with named definitions;
- calls preserve named aggregate or enum pointee identity;
- parameter pointer slots can be legally reassigned inside the callee;
- writes through the adjusted pointer remain prohibited;
- mutable pointer declarations and function arguments cannot discard the element qualification; and
- lexical typedef shadowing still resolves the innermost alias.

No additional production-code change was needed. Focused interpreter coverage passed after correcting the enum array's expected existing write diagnostic from the aggregate-specific wording to `cannot modify read-only array 'states'`.

## Native oracle guidance

GCC and Clang both compile the shared fixture under `-std=c11 -Wall -Wextra -Werror` and return 127, matching Cust. Keep aggregate and enum layout checks ABI-independent:

```c
sizeof(ConstPoints) == 2 * sizeof(struct Point)
_Alignof(ConstStates) == _Alignof(enum State)
```

Use initialized static local const arrays in warning-free shared fixtures so compiler diagnostics about potentially uninitialized qualified objects do not obscure parameter-adjustment behavior.

## Focused verification

```bash
cargo test --test interpreter qualified_named_type_array_alias -- --nocapture
cargo test --test c_compat -- --nocapture
```
