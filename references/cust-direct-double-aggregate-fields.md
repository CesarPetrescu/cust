# Direct scalar `double` aggregate fields

## Scope

Cust supports direct scalar `double` fields in supported structs and scalar-only unions. The slice includes positional/designated initialization, direct/indexed/arrow/nested scalar lvalues, aggregate copy, aggregate parameters/returns, deterministic field type queries, recursive const protection, and scalar-union shared storage.

The following remain intentionally unsupported:

- `double *` fields and addresses of `double` fields;
- one- or multidimensional `double` aggregate-array fields;
- character/raw-memory views of aggregates containing double storage;
- unions that combine double storage with non-scalar field layouts.

## Aggregate expression fan-out

Parser lowering distinguishes nested aggregate field expressions by route:

- `object.nested` -> `Expr::StructGet`;
- `objects[i].nested` -> `Expr::StructElementGet`;
- `pointer->nested` -> `Expr::StructPtrGet`;
- aggregate-valued rvalue fields -> `Expr::AggregateFieldGet`.

Supporting scalar `double` leaves is not sufficient by itself. When a containing aggregate crosses a function/copy boundary, every aggregate-valued route must stay in parity across:

1. `aggregate_expr_type_name()` for metadata and non-evaluating classification;
2. `eval_struct_argument()` for by-value parameters;
3. `eval_struct_expr()` for aggregate returns, assignments, and nested consumers.

A focused regression should combine indexed and arrow routes:

```c
struct Sample { double reading; };
struct Box { struct Sample sample; };
double read(struct Sample value) { return value.reading; }
struct Sample select(struct Box *box) { return box->sample; }
```

Before the parity fix, `read(boxes[0].sample)` failed with a struct-argument diagnostic and returning `box->sample` failed with `expected struct expression`.

## Numeric and safety metadata

- Field reads and assignment/update results must be classified as `CType::Double`; otherwise integer bit values leak into arithmetic, comparison, `_Generic`, or increment paths.
- `sizeof` validation must preserve double type without evaluation, while still validating invalid operators, assignments, initializers, const ancestry, and unsupported addresses.
- Aggregate copy rejection must recurse through nested const fields.
- Raw-memory intrinsic validation must recursively detect double storage and preserve contextual diagnostics such as `function 'memcpy' does not yet support double object storage for argument 1`.
- Scalar unions may share deterministic eight-byte bits across `double`, `int`, and `char`; do not infer host ABI layout or addresses.

## Verification

Run:

```bash
cargo test --test interpreter direct_double_ -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
git diff --check
```

The `c_compat` target contains one harness test, so run the actual test rather than filtering by fixture filename.
