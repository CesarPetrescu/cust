# Model-based two-pointer parameter aliasing

Date: 2026-07-16

## Scope

`tests/fuzz_safety.rs` generates fixed-seed pairs of mutable `int *`, named `struct Point *`, and named `union Number *` parameters over mutable global and file-static array roots. Each type receives ten cases for each required relationship: same-element aliases, same-root distinct indexes, and cross-root pairs.

The callee writes a replacement through the first parameter, then applies a compound delta through the second parameter. It subsequently reassigns both local pointer slots to known storage positions and returns identity checks. The caller's two slots must retain their original roots and indexes.

## Independent model

Model the two writes in source order. Same-element aliases therefore finish at `replacement + delta`; same-root distinct and cross-root pairs mutate separate modeled cells.

Compare a root- and position-weighted checksum of all modeled cells, not a plain sum. A plain sum can stay unchanged if Cust writes the correct value to the wrong element. Add separate caller pointer-difference terms so local parameter-slot reassignment cannot masquerade as correct pointee mutation.

The generator uses an explicit `case_index % 3` pattern schedule and asserts exactly 30 cases for every relationship across the three pointee families. Root choice and indexes remain fixed-seed generated, with minimum coverage assertions for both first-parameter roots.

## Diagnostic and oracle boundaries

The adjacent targeted mutation test retains exact const-pointee write, out-of-bounds, and concrete pointee-type diagnostics for all three families. Every generated program runs under `catch_unwind` through `assert_interpretation()`.

The warning-free C11 fixture covers all three alias relationships and pointee families, ordered writes, both local slot reassignments, caller identity, and final storage checks. Cust, GCC, and Clang return 54 with `-std=c11 -Wall -Wextra -Werror`.

## Result

Existing Cust behavior matched immediately. This is deliberate property/conformance coverage with no production-code change: 90 generated alias cases, one interpreter fixture regression, and one shared compiler-oracle fixture. All sixteen fuzz-safety tests remain sub-second locally.

## Verification

```bash
cargo test --test fuzz_safety generated_two_pointer_parameter_alias_mutations_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety pointer_parameter_mutation_diagnostics_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety -- --nocapture
cargo test --test interpreter pointer_parameter_alias_mutation_model_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
