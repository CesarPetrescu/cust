# Model-based field-backed mixed-qualification pointer parameters

Date: 2026-07-16

## Scope

`tests/fuzz_safety.rs` generates 96 fixed-seed `T *writer` / `const T *reader` pairs over scalar `int`/`char` array fields and embedded named `struct Point`/`union Number` array fields. Writer and reader expressions independently use direct field decay or `->` field decay.

Each pointee family balances four relationships:

- the same containing owner, field path, and element;
- the same owner/path at distinct elements;
- different fields in the same containing object;
- the same field path in different containing objects.

## Independent model

Field-backed pointer identity consists of the containing object, selected field path, and element index. Direct versus arrow syntax is only a route to that storage and must not create a different identity. Pointee qualification changes write permission but not storage identity.

Apply the writer replacement before reading the modeled reader cell. Compare:

- the reader observation;
- a position-weighted checksum over both fields in both containing objects;
- caller pointer differences from each original field root;
- callee-local writer/reader reassignment checks.

The weighted checksum prevents a write to the wrong field or containing object from passing by preserving a plain sum.

## Diagnostics and oracle boundary

Targeted generated programs retain exact diagnostics for:

- passing a field selected from a const containing object to a mutable writer parameter;
- pointer arithmetic beyond scalar or embedded aggregate field bounds;
- concrete `int`/`char`/named-struct/named-union parameter mismatches.

The warning-free C11 fixture covers same-element, distinct-element, different-field, and different-containing-object routes across all four pointee families. Cust, GCC, and Clang return 28.

## Result

Existing Cust behavior matched immediately, so this is deliberate property/conformance coverage with no production-code change. All eighteen fuzz-safety tests remain sub-second locally.

## Verification

```bash
cargo test --test fuzz_safety generated_field_backed_mixed_qualification_parameter_aliases_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety -- --nocapture
cargo test --test interpreter field_backed_mixed_qualification_parameter_alias_model_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
