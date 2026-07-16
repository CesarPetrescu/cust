# Model-based field-backed pointer return and forwarding chains

Date: 2026-07-16

## Scope

`tests/fuzz_safety.rs` generates 128 fixed-seed pointer expressions over `int`/`char` scalar array fields and embedded named `struct Point`/`union Number` array fields. Each expression starts from direct field decay or `->` field decay, crosses a mutable or const one-hop/two-hop pointer-returning helper boundary, and composes two of arithmetic, conditional, comma, or indexed-address wrappers.

## Independent model

The test-side pointer model retains:

- the containing object (`left` or `right`);
- the selected field (`primary` or `secondary`);
- the element index;
- the concrete scalar/aggregate pointee family;
- the resulting pointee qualification.

Helper calls and direct-versus-arrow syntax must not replace this storage identity. Conditional expressions select one storage root while merging pointee qualification, arithmetic updates only the modeled index, comma expressions retain the right operand, and `&call(...)[0]` preserves the returned pointer.

Generated observations cover pointee reads, same-field differences, direct/arrow equality, and same-field ordering. Targeted programs retain exact cross-field subtraction, cross-owner ordering, bounds, const-discard, const-write, const-containing-object, and concrete pointee-type diagnostics.

## Oracle boundary

The warning-free C11 fixture exercises all four pointee families through direct and arrow field arguments, one/two-hop helpers, mutable-to-const promotion, arithmetic, conditional, comma, indexed address, mutation through mutable returned pointers, and same-field pointer differences. Cust, GCC, and Clang return 160.

Unrelated field/root pointer subtraction and ordering remain interpreter-only negative tests because native C does not define those operations.

## Result

Existing Cust behavior matched every generated semantic and diagnostic assertion. The first focused run failed only the test generator's overly strict mutable-result coverage threshold; qualification selection was then balanced deterministically, after which all 128 generated cases and targeted diagnostics passed. All nineteen fuzz-safety tests remain sub-second locally.

## Verification

```bash
cargo test --test fuzz_safety generated_field_backed_pointer_return_and_forwarding_results_match_model_without_panics -- --nocapture
cargo test --test fuzz_safety -- --nocapture
cargo test --test interpreter field_backed_pointer_return_forwarding_model_routes -- --nocapture
cargo test --test c_compat -- --nocapture
```

Follow with the canonical local and Docker gate.
