# Cust nested union fields

2026-05-07 RED/GREEN notes for expanding scalar union support beyond root variables.

## Selected scope

- Nested union fields inside supported structs: `struct Holder { union Number number; };`
- One-dimensional arrays of scalar-field unions: `union Number values[2] = {{1}, {2}};`
- Existing by-value aggregate flows for union copies/parameters remain isolated.
- Deterministic Cust `sizeof` continues to use max union field size, not native ABI layout.

## RED tests

- Added `tests/fixtures/valid/nested_union_fields.c` and `supports_nested_union_fields_initializers_copy_and_parameters`.
- Initial failure showed nested union initializer/assignment did not synchronize sibling scalar views consistently.
- Adding union array element coverage exposed `numbers[1].value = 9; numbers[1].tag` returning the stale initializer value (`65` instead of expected `68`).

## Implementation notes

- Root union assignment already synchronized via `sync_union_scalar_fields_from_active`.
- Recursive aggregate initialization needed to call the same sync helper when the nested aggregate type is `AggregateKind::Union`.
- Scalar field assignment needed a parent-map-aware helper (`assign_scalar_field_in_map`) rather than returning only the final field value: the sync must run on the union-typed field map that owns the assigned scalar, whether it is a root variable, a nested field, or a struct/union-array element.
- Keep compiler-oracle fixtures limited to small scalar writes and exit-code comparison; Cust still does not model native union byte representations or ABI padding.

## Verification

Focused:

```bash
cargo test --test interpreter nested_union -- --nocapture
cargo test --test c_compat -- --nocapture
```

Full gate used by the autonomous maintainer remains:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```
