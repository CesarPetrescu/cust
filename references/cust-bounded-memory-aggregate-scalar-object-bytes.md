# Bounded memory operations on aggregate scalar object bytes

## Scope

Cust's exactly prototyped `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` accept deterministic object-byte views of `int` and `_Bool` scalar fields plus one-dimensional scalar-array fields in supported struct storage. Covered routes include direct and arrow selection, named/anonymous/nested fields, and scalar fields reached through embedded aggregate-array elements.

Whole aggregate objects, every union-backed member, and two-dimensional non-character rows remain unsupported. Cust does not synthesize aggregate padding, infer host layout, expose host addresses, or call host libc.

## Implementation decisions

- `PointerValue::StructField` and `PointerValue::StructFieldElementField` already preserve the selected scalar field path, lexical owner, recursive const metadata, and checked read/write routing. The completed standalone scalar byte reader/writer therefore works unchanged once the obsolete standalone-only admission guard is removed.
- One-dimensional scalar-array fields already decay to their owned `Rc<ArrayValue>`. Byte capacity is limited to the remaining selected field-array cells multiplied by Cust's deterministic scalar width. No operation can flow into an adjacent field.
- `memory_pointer_targets_union_field()` remains mandatory before admission. It traces direct field paths and live scalar-array containers so union members remain rejected even through nested or copied pointer routes.
- `PointerValue::ObjectByte` continues to retain interior byte offsets. Aligned results canonicalize only when an exact typed destination expects the underlying scalar pointer; character-pointer views retain byte-scaled identity, arithmetic, indexing, and writes.
- `_Bool` writes continue to normalize every nonzero byte to canonical stored value `1`.

## TDD and verification

1. A focused direct-field test first failed with `function 'memcpy' currently supports only standalone non-character scalar object storage for argument 1`.
2. Removing that explicit guard made all five direct-field operations GREEN.
3. Broader route coverage then proved direct/arrow, nested, anonymous, scalar-array, and embedded aggregate-array-element behavior through a registered compiler-oracle fixture.
4. Focused identity coverage checks aligned typed `memchr` results, interior byte differences/indexed writes, and embedded field identity.
5. Focused negative coverage checks field-local capacity, overlap, recursive const roots, expired owners, union rejection, and two-dimensional non-character rejection.
6. Non-evaluating `sizeof` coverage proves call operands and field writes do not execute.
7. Native fixture assertions use only full-object, ABI-independent relationships and compile under the existing `-Wall -Wextra -Werror` oracle harness.

Run focused coverage with:

```bash
cargo test --test interpreter aggregate_scalar_object_byte -- --nocapture
cargo test --test interpreter aggregate_non_character_scalar_fields -- --nocapture
cargo test --test c_compat -- --nocapture
```

Fixture-name filtering on `c_compat` runs zero tests; always run the actual harness function or the whole target.
