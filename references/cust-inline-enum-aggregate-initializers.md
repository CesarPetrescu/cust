# Inline enum definitions in aggregate initializers

Date: 2026-06-28

Cust's existing declaration/initializer pending-inline-enum routing already covers inline enum type definitions nested inside aggregate initializer expressions and designator indexes. Coverage added in this run proves that generated enumerators are emitted before aggregate variable initialization and remain visible to later same-block expressions.

Fixture pattern:

```c
struct Box box = {
    .point = {
        .x = sizeof(enum InitX { INIT_X = 4 }) ? INIT_X : 0,
        .y = (enum InitY { INIT_Y = 7 })0 + INIT_Y,
    },
    .values = {
        [sizeof(enum SlotIndex { SLOT_INDEX = 1 }) ? SLOT_INDEX : 0] = SLOT_INDEX + 10,
        [2] = _Alignof(enum TailValue { TAIL_VALUE = 6 }) ? TAIL_VALUE : 0,
    },
};
```

Notes:

- This was coverage-only: focused interpreter and full compiler-oracle tests passed once the fixture arithmetic was corrected.
- Native compiler-oracle fixtures should use ABI-independent relationships such as `sizeof(enum E { A = 1 }) == sizeof(enum E)` and `_Alignof(enum E { A = 1 }) == _Alignof(enum E)` rather than exact enum sizes/alignments.
- Keep native fixture return arithmetic in `0..255`; this fixture returns `122`.
