# Complete tracked-output alias classifier/evaluator parity

## Scope

`tests/pointer_output_parity.rs` crosses all four supported tracked scalar output pointees (`char`, `int`, `_Bool`, and `double`) with four equivalent declaration spellings:

- direct `T **`;
- inner alias `typedef T *ValuePtr; ValuePtr *`;
- complete alias `typedef ValuePtr *CompleteOutput; CompleteOutput`;
- chained complete alias `typedef CompleteOutput ChainedOutput; ChainedOutput`.

The primary matrix is 4 pointee kinds × 4 spellings × 4 storage/forwarding routes × 6 consumers = 384 interpreted programs. Routes are local objects, file-global objects, block-static objects, and function parameters. Consumers are initialization, assignment, argument forwarding, equality, truthiness, and non-evaluating `sizeof`.

## Mechanical coverage

Use `#[repr(usize)]` enums and stable discriminants. Maintain identity-keyed counters for every kind, spelling, route, consumer, and Cartesian cell. Assert every cell exactly once; aggregate totals alone can hide duplicate and missing cells.

Supplement the primary matrix with:

- 16 qualification cells (all kinds × spellings);
- 32 lifetime programs (all kinds × spellings, split into non-observing `sizeof` success and evaluated dangling-use failure);
- 24 unsupported-shape cells (all spellings × non-scalar, deeper-pointer, array, aggregate-field, cast, and return boundaries).

These 72 supplemental programs preserve exact safety boundaries around the 384 valid programs. A non-scalar complete output alias is rejected while its prerequisite `typedef ItemPtr *ItemOutput` is parsed, so the chained non-scalar case includes the intended later chained typedef but expects the same prerequisite diagnostic; it cannot construct the chain after the unsupported complete alias fails.

## Interpretation rules

All spellings must lower to the same interpreter-owned tracked output identity. Tests should mutate through the selected output, compare independently declared output objects, validate null truthiness, and use same-type `sizeof` relationships without assuming host pointer representation.

Lifetime coverage must separate non-observation from evaluated use. A dangling pointee below `sizeof` followed by an unrelated call succeeds; a later `**output` read reports `pointer to out-of-scope variable 'local'`.

Qualification and unsupported-shape checks are deliberate diagnostic coverage. Immediate focused GREEN is valid because the package audits already implemented behavior; do not invent production changes solely to manufacture RED evidence.

## Verification

Run the real integration-test target, not a fixture-name filter:

```sh
cargo test --test pointer_output_parity -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```
