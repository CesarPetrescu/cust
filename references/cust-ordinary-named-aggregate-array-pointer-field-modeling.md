# Ordinary Named Aggregate-Array Pointer-Field Modeling

Use this note when extending deterministic generated coverage for pointer fields selected through ordinary named aggregate-array elements such as `nodes[i].cursor`.

## Coverage shape

- Enumerate route dimensions structurally instead of taking consecutive low bits from an LCG. For this matrix, the low five case-index bits encode owner, outer array index, outer subscript direction, inner subscript direction, and pointee offset.
- Cross all four pointee kinds (`int`, `char`, named `struct`, named `union`) with nested named-holder, anonymous-holder, and union-containing-array paths.
- Assert exact coverage totals. Count the full 32-cell route tuple independently, and count the selected forwarding route crossed with conditional branch outcome; aggregate totals alone can hide correlated or dead routes.
- Model owner/path/index, pointee kind, and qualification before rendering source. Compute success values and exact bounds, const-discard, read-only, type-mismatch, and cross-root diagnostics from that model.
- Wrap each generated interpretation in `catch_unwind` so every generated source proves both semantic agreement and panic freedom.

## Operations

Cover direct and reverse outer/inner subscripts, element addresses, pointer-field replacement, compound assignment, prefix/postfix increment, conditional/comma forwarding, and non-evaluating `sizeof`. Use side-effect markers only in the interpreter property test where needed to prove non-evaluation.

## Compiler-oracle boundary

Keep the registered C fixture warning-free under both GCC and Clang with `-std=c11 -Wall -Wextra -Werror`. Clang diagnoses assignments and increments under `sizeof` as `-Wunevaluated-expression`, so the native fixture should use side-effect-free ABI-independent `sizeof` relationships; interpreter-only generated cases retain the mutation-result non-evaluation proofs.

## Review pitfall

A fixed seed alone does not guarantee independent route coverage. Consecutive low bits from a linear congruential generator can correlate owner/index/direction choices, while tying conditional selection to route parity leaves half of the route×branch matrix dead. Add explicit cell counters first; they should fail before correcting the generator.
