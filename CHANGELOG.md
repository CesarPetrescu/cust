# Changelog

All notable changes to Cust are documented here. Cust is still a small educational C-subset interpreter; release notes describe the supported subset, diagnostics, tooling, and verification status at each tag.

## Unreleased

No changes yet.

## v0.2.0 — 2026-07-25

### Language subset

- Added a deterministic aggregate model for named and anonymous structs/unions, enums, typedef-backed aggregate types, nested aggregate fields, scalar and aggregate arrays, pointer fields, aggregate parameters/returns, deep-copy assignment, designated initializers, and scalar/array/aggregate compound literals.
- Expanded declarations with comma-separated declarators and typedef aliases, inferred arrays, function prototypes, static/extern/thread-local storage-class syntax, `inline`/`_Noreturn`, `const`/`volatile`/`restrict`/`_Atomic`, `_Bool`, standard signed/unsigned/short/long scalar spellings, and C99 `__func__`.
- Expanded expressions with assignment and compound-assignment results, prefix/postfix increment and decrement, bitwise and shift operators, the conditional and comma operators, scalar/pointer/void casts, reverse subscripting, `sizeof`, `_Alignof`, and `_Static_assert`.
- Expanded the safe one-level typed pointer model with pointer-returning functions, scalar and aggregate array decay, aggregate/field/compound-literal storage roots, bounded arithmetic, same-array subtraction and ordering, address-of for supported lvalues, const-preserving conversions, and deterministic lifetime/type/bounds diagnostics.
- Added `do`/`while`, `switch`/`case`/`default` fallthrough, local function prototypes, block-scoped aggregate and enum definitions, C integer literal bases/suffixes, standard/numeric escapes, adjacent string-literal concatenation, and C line/block comments.

### Diagnostics and safety

- Added source-located diagnostics for unsupported preprocessor directives, floating/complex types, `void *`, function pointers/types, variadics, old-style parameters, `goto`/labels, `_Generic`, VLAs, multidimensional arrays, flexible array fields, bit-fields, forward declarations, and unsupported abstract declarator suffixes.
- Hardened parser boundaries with exact contextual diagnostics for malformed declarations, parameters, calls, control-flow headers, operators, type queries, array lengths/indexes/designators, initializers, casts, and unmatched delimiters.
- Added deterministic model-based coverage for pointer provenance/qualification/lifetime, aggregate and scalar expression classification, `_Bool` conversion boundaries, lexer/parser mutation matrices, comments/trivia, literals, adjacent strings, and first-error precedence.

### CLI, packaging, and verification

- Added `--tokens`, `--ast`, and `--max-steps`; retained `--version` with an exact `cust 0.2.0` release assertion.
- Added GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`) licensing so distributed and network-served modified versions remain open-source.
- Versioned the Cargo package and Docker Compose runtime/test images as `0.2.0`.
- Verified 984 tests at release time: 875 interpreter tests, 97 deterministic fuzz-safety tests, and 12 CLI, Docker, compiler-oracle, and repository tests.
- Continued using native C compilers only as external compatibility oracles for warning-free supported fixtures; Cust never delegates runtime execution to a native compiler.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Unsupported areas include preprocessing/includes/macros, standard-library calls, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic calls, VLAs and multidimensional arrays, flexible array members and bit-fields, `goto`, and host ABI layout/promotion rules.

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
- The repository is licensed under the GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`).
