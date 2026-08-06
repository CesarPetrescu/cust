# Bounded character-storage `memchr`

## Scope

Cust recognizes the exact normalized declaration:

```c
void *memchr(const void *source, int value, unsigned long int count);
```

The intrinsic searches at most 4,096 cells of interpreter-owned standalone `char` scalar or one-dimensional `char` array storage. Integer/aggregate object representations, aggregate-backed character fields, multidimensional arrays, host addresses, and host-libc execution remain outside the bounded model.

## Runtime behavior

1. Evaluate source, search value, and count exactly once in source order.
2. Normalize the search scalar and each stored character cell modulo 256 for `unsigned char` comparison, while leaving stored values unchanged.
3. Validate source owner/lifetime/type and the requested capacity under the shared bounded-memory count limit. A zero count reads no cells.
4. Return the original source target for an offset-zero match, an offset pointer into the same storage for an interior match, or null when no byte matches.
5. Preserve the selected source pointer's owner/lifetime identity and derive returned read-only qualification from the source expression.

## Dispatch and type analysis

- A matching interpreted function body always takes precedence over intrinsic dispatch, including when an exact prototype appears before the definition.
- An incompatible declaration receives the dedicated unsupported-`memchr` declaration diagnostic; an absent declaration remains `undefined function 'memchr'`.
- Intrinsic pointer-result const classification requires exactly three arguments and no user-defined body. This prevents malformed calls from indexing an absent argument and prevents user functions that return unrelated mutable storage from inheriting source constness.
- Direct and nested `sizeof(memchr(...))` validate declaration, arity, and argument shapes recursively without evaluating side effects, then return Cust's pointer size.

## TDD and review closure

The first focused regression failed before intrinsic dispatch existed. Independent review later found two concrete metadata-path defects: malformed zero-argument pointer initializers could panic by indexing `args[0]`, and a user-defined `memchr` following an exact prototype inherited intrinsic source constness. Focused RED tests reproduced both defects; the final guards require exact arity and absence of a function body. Additional regressions cover source-order one-time evaluation and escaped interior-pointer lifetime.

The warning-free compiler-oracle fixture checks unsigned-byte first-match behavior, embedded NULs, offsets, null results, zero count, and one-time argument evaluation. Native C does not define function-argument evaluation order, so source-order arithmetic is asserted only in the interpreter test; the native fixture uses independent counters.

## Verification

Run:

```bash
cargo test --test interpreter memchr -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```
