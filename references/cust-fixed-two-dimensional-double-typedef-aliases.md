# Fixed two-dimensional `double` typedef aliases

## Supported surface

Cust accepts fixed two-dimensional aliases such as `typedef double Matrix[R][C]` and alias chains over them. Direct local, file-global, and block-static objects reuse `Stmt::Array2DDecl` and typed `Value::Array` storage, including:

- nested positional initialization and zero fill;
- double-index reads, assignment, compound assignment, and prefix/postfix updates;
- one-time row and column indexes;
- comma-separated object declaration lists;
- alias-carried and chained const qualification;
- scalar `_Generic` classification;
- deterministic whole-object, row, element, alias `sizeof`, and alias `_Alignof` relationships.

The parser change is deliberately narrow: `parse_typedef_declarator()` now lowers scalar `double` followed by two fixed brackets to the existing `TypeAlias::Array2D(CType::Double, rows, columns)` representation.

## Exact retained boundaries

Opening the typedef constructor must not silently enable adjacent pointer or aggregate surfaces:

- direct and alias-spelled two-dimensional double array parameters remain unsupported;
- array returns, pointers to the aliased array, and pointer-to-row derivatives remain unsupported;
- aggregate fields of the alias remain unsupported;
- alias-spelled multidimensional compound literals/casts remain unsupported;
- arrays with more than two dimensions remain unsupported;
- bounded raw-memory intrinsics continue to reject double object storage.

Once `TypeAlias::Array2D(Double, ...)` became constructible, `parse_function_return_type()` also needed to reject `DeclType::Array2D` before `decl_type_to_return_type()`; otherwise an alias-spelled array return reached an internal `unreachable!` and panicked the Rust host. Keep the `catch_unwind` regression when extending array aliases.

Likewise, `parse_params()` must reject `DeclType::Array2D(CType::Double, ...)` at the alias token before generic two-dimensional parameter adjustment. Direct `double values[R][C]` already had a separate bracket-syntax guard, but it does not cover alias-spelled parameters.

## Tests and oracle guidance

The registered fixture `tests/fixtures/compat/valid/fixed_two_dimensional_double_typedef_aliases.c` covers direct and chained aliases, chained const, local/global/static storage, zero fill, scalar updates, one-time indexes, declaration lists, and ABI-independent type relationships. It must compile and exit zero under both GCC and Clang with `-std=c11 -Wall -Wextra -Werror`.

Focused commands:

```bash
cargo test --test interpreter fixed_two_dimensional_double_typedef_alias -- --nocapture
cargo test --test c_compat -- --nocapture
gcc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/fixed_two_dimensional_double_typedef_aliases.c -o /tmp/cust-2d-double-typedef-gcc
clang -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/fixed_two_dimensional_double_typedef_aliases.c -o /tmp/cust-2d-double-typedef-clang
```

Independent review should audit every consumer newly reachable from `DeclType::Array2D(CType::Double, ...)`, especially function returns and parameters, before the canonical local and Docker gate.
