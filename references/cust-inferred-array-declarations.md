# Inferred scalar array declarations

2026-05-12 autonomous run: Cust now supports C-style inferred-length scalar array declarations when an initializer is present.

## Supported forms

- `int values[] = {1, 2, [4] = 5, 6};`
- `char word[] = "cat";`
- `const int table[] = {[1] = 3, [3] = 4};`

The parser consumes the empty bracket pair, requires `=`, parses either a direct char string-literal initializer or an unbounded scalar array brace initializer, then reuses `infer_array_initializer_len` to turn the declaration into the existing fixed-length `Stmt::ArrayDecl` runtime path. Designated initializers set the next positional index just like existing fixed arrays and array compound literals.

## Boundaries and diagnostics

- `int values[];` remains unsupported because C requires an initializer to complete the object type in this context; Cust reports `expected '=' after inferred array declaration`.
- Direct string literal initializers are still char-array-only and reuse the existing `string literal initializer requires char array '<name>'` diagnostic.
- Multidimensional arrays remain unsupported; fixed-size `int values[2][3]` still uses the existing targeted diagnostic path.
- This change is parser-local for scalar arrays. Aggregate array inferred declarations (`struct Point points[] = ...`) are a possible follow-up if needed.

## Coverage

- Interpreter fixture: `tests/fixtures/valid/inferred_array_declarations.c` covers inferred lengths from positional entries, designators, direct string literals, const arrays, `sizeof`, zero-fill, and array-parameter decay.
- Compiler-oracle fixture: `tests/fixtures/compat/valid/inferred_array_declarations.c` checks native-compatible runtime behavior without relying on Cust's deterministic `sizeof(int)`.
