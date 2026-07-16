# Model-based pointer-parameter mutation

Date: 2026-07-16

## Scope

`tests/fuzz_safety.rs` uses a fixed-seed independent model for caller-visible mutations through scalar `int *`, named `struct Point *`, and named `union Number *` parameters. Backing roots are mutable global and file-static arrays, with generated indexes, replacement values, and one- or two-hop helper selection.

Each callee writes through its parameter, then reassigns its local pointer slot to a different array position and returns an identity check. The caller verifies the replacement through its original pointer and subtracts that pointer from its original root. The test-side expected value combines the replacement, retained index, and one/two successful callee-local slot checks, so a copied-pointee or caller-slot-reassignment bug changes the result.

## Diagnostic boundaries

Targeted cases cover all three pointee families and assert exact diagnostics for:

- writes through `const T *` parameters;
- indexed writes beyond the backing array after parameter binding;
- concrete scalar/struct/union pointee mismatches at the call boundary.

Every generated and targeted program runs through `catch_unwind` via `assert_interpretation()`.

## Fixed-seed pitfall

Do not choose routes only from a PRNG's low bit when each generated case consumes a fixed number of values. The initial focused run produced one-sided root/hop coverage because the low-bit sequence repeated at that stride. Use a higher bit for route selection and keep explicit minimum counters for both roots and both hop counts.

## Native-oracle boundary

The warning-free C11 fixture mutates defined global/file-static array elements, reassigns parameter slots locally, and returns 35 under Cust, GCC, and Clang with `-std=c11 -Wall -Wextra -Werror`. Callee parameter reassignments are used in returned identity checks to avoid native unused-but-set parameter warnings.

## Result

Existing Cust behavior matched after correcting the generator's route sampling. This run is deliberate property/conformance coverage with no production-code change: 120 generated mutation cases, nine targeted diagnostic cases, one interpreter fixture regression, and one shared compiler-oracle fixture. All fifteen fuzz-safety tests remain sub-second locally and in Docker.

## Verification

```bash
cargo test --test fuzz_safety generated_pointer_parameter_mutations_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety pointer_parameter_mutation_diagnostics_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety -- --nocapture
cargo test --test interpreter pointer_parameter_mutation_model_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
