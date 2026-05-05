# Changelog

All notable changes to Cust are documented here. Cust is still a small educational C-subset interpreter; release notes describe the supported subset, diagnostics, tooling, and verification status at each tag.

## v0.1 — 2026-05-05

### Language subset

- Added function definitions and calls with local parameters, recursive and mutually recursive calls, arity diagnostics, undefined-function diagnostics, and a bounded call-depth guard.
- Added block-scoped variables, nested blocks, `if`/`else`, `while`, `for`, `break`, `continue`, empty statements, and expression statements.
- Added integer arithmetic and comparison coverage, unary plus/minus, logical `!`, `&&`, and `||` with C-style truth values and short-circuiting.
- Added `char` declarations/parameters/literals, one-dimensional `int`/`char` arrays, read-only NUL-terminated string literals, array parameters, indexed reads/writes, and deterministic negative/out-of-bounds diagnostics.
- Added the initial safe pointer model: scalar pointer declarations/reassignment, `&x`, `*p`, dereference assignment, null/out-of-scope diagnostics, pointer parameters, array/string decay to pointer arguments, `p[i]`, `&array[index]`, `&p[index]`, array-element pointers, pointer truthiness, and pointer equality/inequality against null and supported pointer targets.
- Added explicit diagnostics for unsupported pointer arithmetic, pointer ordering comparisons, and pointer-vs-nonzero-integer comparisons.

### Diagnostics and safety

- Parser diagnostics include source line/column metadata and context-specific messages for malformed function/parameter/call lists, missing semicolons, missing brackets/parens/braces, missing assignment operators, missing names/types, unmatched delimiters, unterminated blocks, malformed array lengths, and invalid `break`/`continue` placement.
- Lexer diagnostics include source-line/caret snippets for unexpected characters and out-of-range integer literals.
- Deterministic fuzz/property-style tests assert malformed generated programs and arbitrary bytes decoded through lossy UTF-8 do not panic the lexer/parser/interpreter path.
- Runtime diagnostics cover division by zero, loop execution limits, function call-depth limits, undefined variables/functions, array bounds, pointer null/out-of-scope/read-only/bounds failures, and unsupported pointer operations.

### CLI and tooling

- `cust <file.c>` interprets a source file and prints the integer returned by `main()`.
- `cust --version` prints the Cargo package version.
- `cust --tokens <file.c>` prints located lexer tokens without parsing or interpreting.
- `cust --ast <file.c>` prints a deterministic parser debug view without interpreting.
- `cust --max-steps N <file.c>` runs with an explicit total loop-iteration budget for bounded CLI execution.
- Docker Compose services use `pull_policy: build` so required cron verification rebuilds from the current checkout instead of silently reusing stale images.

### Test coverage and verification

- Added valid and invalid fixture corpora under `tests/fixtures/` for interpreter behavior, parser/lexer diagnostics, arrays/strings/pointers, and runtime errors.
- Added native C compiler compatibility tests for supported fixtures as an external oracle only; native compilers are never used as Cust's runtime path or implementation shortcut.
- Verified release gate on 2026-05-05:
  - `cargo fmt --check`
  - `cargo clippy -- -D warnings`
  - `cargo test`
  - `docker compose run --rm test`
  - `docker compose run --rm cust`

### Known limitations

- Cust is not a full C implementation.
- Unsupported areas include the preprocessor, `#include`, standard-library calls such as `printf`, floating-point values, structs/unions/enums, prototypes/declarations separate from definitions, `void` functions, multiple pointer levels, pointer arithmetic, and general assignment expressions.
- The repository does not yet include a confirmed `LICENSE` file; licensing remains blocked on project-owner choice.
