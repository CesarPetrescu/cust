# Adjusted aggregate-parameter alias mutation modeling

Date: 2026-07-19

## Scope

`tests/fuzz_safety.rs` generates 48 fixed adjusted-aggregate-parameter cases over scalar `int *` and named `struct Point *` pointers selected from embedded arrays. Root aggregate arrays and arrays decayed from aggregate fields cover six relationships: same element, same embedded array at different indexes, different outer elements, different field paths, different owners, and root-versus-field storage.

Each case forwards two mutable writers and one `const` reader through balanced one- and two-hop pointer helpers. The callee performs ordered replacement/compound writes, records reader observations between and after writes, then reassigns all copied parameter slots. The independent model checks weighted observations, final selected cells, retained caller pointer identity, qualification, and panic freedom.

## Runtime pitfall and fix

An outer aggregate-array field element can itself select a nested scalar or aggregate array, for example `wrapper.items[0].nested.values` or `.points`. The parser lowers the outer selection to `Expr::StructFieldArrayElementGet`; this expression is pointer-valued not only when the selected field is `StructFieldType::Pointer`, but also when it is `Array` or `StructArray` and therefore decays.

Keep two paths in parity:

1. `expr_is_pointer_value()` must classify `StructFieldArrayElementGet` as pointer-valued for pointer, scalar-array, and aggregate-array field metadata. Assignment-result variants remain pointer-valued only for actual pointer fields.
2. `eval_pointer()` must first try `find_struct_pointer_array_field_base_pointer()` on the selected outer element and fall back to `read_struct_pointer_pointer_field()` for ordinary pointer fields.

Without both changes, direct nested access such as `wrapper.items[0].nested.values[1] = 7` reaches pointer arithmetic with both operands misclassified as scalar and reports `expected pointer expression`.

## Oracle and verification

The warning-free C11 fixture `adjusted_aggregate_parameter_alias_mutation_model_routes.c` covers root and field-decayed outer arrays, scalar and named-aggregate embedded pointers, one/two-hop forwarding, ordered writer/const-reader observations, copied-slot reassignment, caller identity, and the nested-array decay regression. Cust, GCC, and Clang return 40 under `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test interpreter aggregate_array_field_elements_decay_embedded_arrays_for_direct_access -- --nocapture
cargo test --test fuzz_safety generated_adjusted_parameter_alias_mutations_match_model_without_panics -- --nocapture
cargo test --test interpreter adjusted_aggregate_parameter_alias_mutation_model_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
