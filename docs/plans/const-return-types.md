# Const-Qualified Return Types

Cust's first-pass const return-type support is intentionally parser-level and value-oriented.

## Supported syntax

- Scalar returns: `const int f(void)` and `const char f(void)`.
- Scalar typedef returns: `typedef int Count; const Count f(void)`.
- Aggregate returns: `const struct Point make(void)`, `const union Number make(void)`, and typedef-spelled forms such as `const Point make(void)`.
- Existing pointer return syntax keeps its existing meaning: `const int *view(void)` returns a pointer-to-const view, while a post-star `const` on a function return is accepted as syntax but does not create a mutable storage slot because function return values are not assignable variables in Cust.

## Runtime model

Top-level `const` on scalar and aggregate return values does not introduce new runtime storage. Returned scalar values remain Cust `i64` values, and returned structs/unions remain by-value deep clones. This matches Cust's existing rvalue-oriented return flow and avoids introducing fake mutable return objects.

## Compatibility boundary

Native C compilers accept these declarations but GCC/cc with the repository's `-Wall -Wextra -Werror` oracle flags reports `-Werror=ignored-qualifiers` for top-level const-qualified function return types. Therefore this feature is covered by interpreter fixtures rather than by the native compiler-oracle list.

## Acceptance coverage

- `tests/fixtures/valid/const_return_types.c` covers const scalar, scalar typedef, direct aggregate, aggregate typedef, and union return spellings.
- `tests/interpreter.rs::supports_const_qualified_return_types` verifies the fixture.
