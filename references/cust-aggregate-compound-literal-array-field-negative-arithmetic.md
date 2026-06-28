# Aggregate compound-literal array-field negative pointer arithmetic

Date: 2026-06-28

## Context

Cust already had negative pointer arithmetic coverage for separate string literals, separate scalar/aggregate array compound literals, and distinct field-backed array storage roots. A less-traveled combined path remained: array fields selected from separately evaluated aggregate compound literals, for example:

```c
((struct Packet){{1, 2}}).values - ((struct Packet){{3, 4}}).values;
((struct Line){{{1, 2}, {3, 4}}}).points < ((struct Line){{{5, 6}, {7, 8}}}).points;
```

## Finding

Focused interpreter coverage passed immediately. No production code change was needed: aggregate compound-literal field decay already allocates separate hidden storage roots per compound literal, and existing pointer owner/path identity checks report:

- `cannot subtract pointers to different arrays` for pointer difference.
- `cannot compare pointers to different arrays` for relational ordering.

## Fixture pattern

Use invalid interpreter fixtures only. Native C can compile some pointer comparisons, but comparing unrelated objects is undefined behavior, so a compiler-oracle fixture is not appropriate.

Coverage added:

- Scalar array fields selected from separate aggregate compound literals.
- Embedded aggregate-array fields selected from separate aggregate compound literals.
- Both subtraction and relational ordering diagnostics.

## Pitfall

Treat immediate focused GREEN as valid conformance/diagnostic coverage for this work package. Do not invent production changes when existing hidden compound-literal storage metadata already preserves distinct array identities.
