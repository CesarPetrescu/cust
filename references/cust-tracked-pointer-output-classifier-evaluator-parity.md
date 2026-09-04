# Deterministic tracked pointer-output classifier/evaluator parity

## Scope

The generated integration suite in `tests/pointer_output_parity.rs` keeps the four supported tracked scalar pointer-output families in parity:

- `char **`
- `int **`
- `_Bool **`
- `double **`

The primary matrix crosses four pointee kinds, nine stable output-value routes, and six consumers for 216 exact interpreted programs. Routes include direct `&slot`, local/file-global/block-static objects, conditional/comma/`_Generic` wrappers, and an assignment-result expression. Consumers cover initialization, assignment, argument binding, equality, truthiness, and non-evaluating `sizeof`.

## Mechanical coverage rules

Keep enum discriminants stable and index the coverage arrays directly. Assert every Cartesian cell exactly once in addition to aggregate totals:

- 54 cases per pointee kind
- 24 cases per route
- 36 cases per consumer
- 216 cells total

Count-only assertions are insufficient because generator bugs can duplicate one cell while omitting another.

## Evaluation-safety pitfalls

- A route with an assignment side effect must assert the assigned scratch slot, not only the selected output slot.
- Equality needs both matching and distinct output objects. If an expression is evaluated twice by two separate comparisons, its marker expectation must prove one evaluation per comparison.
- Truthiness needs both non-null and null output values.
- `_Generic` controlling expressions are non-evaluating. Use a marker-producing function call as the control and require the marker to remain unchanged.
- `sizeof` must preserve route/type classification without running conditional, comma, generic-control, or assignment side effects.

## Diagnostic prevalidation

The type-diagnostic matrix crosses all four pointee kinds with initializer, assignment, argument, equality, and conditional consumers. Trap choice matters: division-by-zero can be translated by pointer-output initializer handling into the same diagnostic the test expects, creating a false positive. Use a dangling-pointer dereference whose `pointer to out-of-scope variable` diagnostic survives that translation, and independently execute the trap once as a witness that it is live.

## Lifetime and const rules

Keep lifetime behavior split into two programs:

1. A dangling output value used only below `sizeof` plus an unrelated valid call must succeed, proving non-observation.
2. A later evaluated dereference must report the exact out-of-scope diagnostic.

Const conversion failures remain non-evaluating and must reject before assignment side effects. These tests verify existing interpreter behavior; immediate focused GREEN is valid for deliberate parity/coverage closure and must not be turned into an unnecessary runtime change.

## Verification

Run:

```sh
cargo test --test pointer_output_parity -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```
