# Model-based mixed-qualification pointer-parameter aliasing

Date: 2026-07-16

## Scope

`tests/fuzz_safety.rs` generates fixed-seed `T *writer` / `const T *reader` pairs for `int`, named `struct Point`, and named `union Number` over mutable global and file-static arrays. Each pointee family receives ten same-element aliases, ten same-root distinct-index pairs, and ten cross-root pairs.

The callee writes a replacement through `writer`, observes the selected value through `reader`, then reassigns both local parameter slots. The caller's writer and reader slots must retain their original roots and indexes.

## Independent model

Qualification is expression metadata, not a second storage identity. Both pointers can refer to the same mutable interpreter-owned cell even though only the writer permits mutation. Apply the writer mutation before reading the model cell selected by the reader:

- same-element readers observe the replacement;
- same-root distinct-index readers observe the untouched value at their own index;
- cross-root readers observe the untouched value in the other root.

Compare a root- and position-weighted checksum for all mutable cells, the observed reader value, and separate caller pointer-difference terms. The callee returns additional checks after reassigning both local slots, proving parameter slots remain copies while pointee storage remains shared.

## Diagnostics and oracle boundary

The adjacent targeted mutation test keeps exact diagnostics for writes through const reader parameters, const backing storage passed to mutable writers, out-of-bounds writes, and concrete scalar/aggregate pointee mismatches. Every generated program runs through `assert_interpretation()` under `catch_unwind`.

The shared C11 fixture uses warning-free scalar, struct, and union routes. Cust, GCC, and Clang all return 63 with `-std=c11 -Wall -Wextra -Werror`.

## Result

Existing Cust behavior matched immediately. This is deliberate property/conformance coverage with no production-code change: 90 generated mixed-qualification alias cases, three additional exact const-storage writer diagnostics, one interpreter fixture regression, and one compiler-oracle fixture. All seventeen fuzz-safety tests remain sub-second locally.

The first fixture assertion expected 69, but direct Cust/GCC/Clang execution all returned 63; recomputing the fixture showed each pointee family contributes 21 checks, so the repository assertion was corrected before GREEN verification.

## Verification

```bash
cargo test --test fuzz_safety generated_mixed_qualification_pointer_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety pointer_parameter_mutation_diagnostics_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety -- --nocapture
cargo test --test interpreter pointer_parameter_mixed_qualification_alias_model_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
