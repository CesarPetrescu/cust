# Bounded C-locale `strcoll` and `strxfrm`

## Scope

Cust implements explicitly prototyped C11 `strcoll` and `strxfrm` without calling host libc or reading host locale state:

- `strcoll` reuses bounded unsigned-byte `strcmp` ordering.
- `strxfrm` uses the identity transform required by Cust's deterministic C-locale model, returns the source length excluding NUL, writes at most `count` bytes, and writes NUL only when it fits.
- A null destination is accepted only when `count == 0`; the source must always be valid character storage.
- Both source length and count are bounded at `MAX_INTEGER_STRING_BYTES` (4096).

## Exact intrinsic activation

Use retained normalized signatures:

```c
int strcoll(const char *left, const char *right);
unsigned long int strxfrm(
    char *restrict destination,
    const char *restrict source,
    unsigned long int count
);
```

Cust normalizes supported integer spellings to `CType::Int`. A user definition always wins over intrinsic dispatch. Missing and incompatible declarations must remain distinct deterministic errors in runtime, nested validation, and `sizeof` dispatch.

## Runtime ordering and safety

For `strxfrm`, evaluate destination, source, and count exactly once in source order before validating their shapes. Then:

1. Validate nonnegative `count <= 4096`.
2. Validate and fully bound the source, even when `count == 0`.
3. If `count > 0`, validate mutable character destination storage, lexical ownership/lifetime, row shape, and complete requested capacity.
4. Reject overlapping source/destination ranges before mutation.
5. Copy `min(count, source_length + 1)` normalized bytes, including NUL only when it fits.
6. Return the full source length, not the number written.

Calling the existing dereference/assignment helpers before bulk writes preserves read-only and expired-owner diagnostics. Capacity and overlap checks must happen before the first write.

## Non-evaluating validation pitfall

`sizeof_expr()` begins by recursively calling `validate_nested_string_intrinsic_calls()`. If `sizeof_string_transform_call()` recursively validates the count and generic traversal then visits that count again, nested transforms obey an exponential recurrence. Final `sizeof` call dispatch can accidentally repeat the helper yet again.

The linear ownership split is:

- `sizeof_string_transform_call()` owns count-tree `sizeof_expr()` validation.
- The `strxfrm` branch in `validate_nested_string_intrinsic_calls()` validates destination/source subtrees once, then returns without generic argument traversal.
- Final `sizeof_expr()` call dispatch returns `INT_SIZE` after entry validation instead of invoking the helper again.

Protect this with `nested_string_collation_transform_validation_remains_linear`: depth 18 timed out above 300 seconds before the fix and completes in roughly 0.01 seconds afterward. Also keep explicit nested malformed-intrinsic tests in destination, source, and count expressions so the early return cannot create validation gaps.

## Verification

Focused commands:

```bash
cargo test --test interpreter string_collation -- --nocapture
cargo test --test interpreter nested_string_collation_transform_validation_remains_linear -- --exact --nocapture
cargo test --test c_compat -- --nocapture
```

The compiler-oracle fixture must set `LC_ALL` to `"C"`, avoid comparing implementation-specific transformed bytes outside that locale model, and use sign/length/collation relationships plus destination canaries.

Official references:

- WG14 N1570 §§7.24.4.3 and 7.24.4.5: <https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf>
- POSIX `strxfrm`: <https://pubs.opengroup.org/onlinepubs/9799919799/functions/strxfrm.html>
