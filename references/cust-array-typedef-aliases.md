# Cust array typedef aliases

2026-05-11 autonomous run.

## Implemented surface

Cust now supports one-dimensional array typedef aliases such as:

```c
typedef int Scores[3];
typedef char Word[4];
typedef struct Point Point;
typedef Point Points[2];
```

Supported uses:

- scalar and aggregate array variable declarations from aliases (`Scores s`, `Points p`)
- global, local, `const`, and initializer contexts using existing scalar/aggregate array initializer paths
- char-array string initializers through aliases (`Word w = "cat"`)
- function parameters where array aliases decay to existing one-level pointer parameters (`int f(Scores values)`)
- `sizeof(alias)` reports the full deterministic array-object size
- aliases of array aliases preserve the original array metadata

## Boundaries

- Pointer arrays remain unsupported and report `pointer array typedef aliases are not supported` for forms like `typedef IntPtr PtrArray[2];`.
- Multidimensional array typedefs remain unsupported and report `multidimensional array typedef aliases are not supported`.
- Pointer-to-array declarations/parameters/typedefs and array return types remain outside Cust's one-level pointer subset.

## Implementation notes

- `TypeAlias::Array(PointeeType, usize)` and `DeclType::Array(PointeeType, usize)` carry parser-only array alias metadata.
- Variable declarations route array aliases directly into existing `Stmt::ArrayDecl` / `Stmt::StructArrayDecl` runtime paths.
- Parameters classify array aliases as `ParamKind::Pointer`, reusing existing array-to-pointer decay and pointer binding semantics.
- `sizeof` lowers array aliases to `SizeOfType::Array` without runtime evaluation.

## Fixture notes

- `tests/fixtures/valid/array_typedef_aliases.c` covers scalar/char/aggregate array aliases, globals, locals, const arrays, parameter decay, string initializers, designated initializers, and `sizeof(alias)`.
- `tests/fixtures/compat/valid/array_typedef_aliases.c` mirrors the valid fixture for native exit-code comparison without ABI-sensitive exact struct-size assertions.
- `tests/fixtures/invalid/pointer_array_typedef_alias.c` locks in the unsupported pointer-array typedef diagnostic.
