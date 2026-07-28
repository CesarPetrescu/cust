# Changelog

All notable changes to Cust are documented here. Cust is still a small educational C-subset interpreter; release notes describe the supported subset, diagnostics, tooling, and verification status at each tag.

## Unreleased

### Language subset

- Added one-line object-like `#define` macros with deterministic nested expansion across declarations, enum/array integer constant expressions, and runtime expressions. Macro names inside comments and string/character literals remain untouched.
- Added bounded `#undef` directives: known definitions expire before subsequent tokens, unknown names are harmless, and removed names may be redefined with different replacement lists.
- Added bounded nested `#ifdef` / `#ifndef` conditional preprocessing with one `#else`, inactive-token/directive skipping, current macro-table definedness, and a 128-group nesting limit.
- Added bounded expression-form `#if` / `#elif` preprocessing with object-macro expansion, both `defined` spellings, ordered branch selection, C-style `intmax_t`/`uintmax_t` condition arithmetic, short-circuiting, and separate expression token/depth limits.
- Added bounded function-like macros with named parameters, zero/empty/nested arguments, balanced argument collection, argument prescan and substitution, replacement rescanning across object/function aliases and following calls, ordinary-code and `#if` expansion, and C-compatible temporary self-disable behavior.

### Diagnostics and verification

- Added exact source-context diagnostics for recursive expansion, conflicting object/function-like redefinitions, malformed and duplicate function parameters, invocation arity, unsupported variadic/stringification/token-pasting forms, unsupported `#include`, and bounded expansion depth/token/work exhaustion.
- Added exact source-context diagnostics for missing/invalid `#undef` names and trailing tokens after the identifier.
- Added exact source-context diagnostics for malformed, unmatched, duplicate, unterminated, and unsupported conditional directives, including multiline-comment and inactive-branch locations.
- Added exact condition diagnostics for malformed `defined`, invalid integer suffixes, division by zero, invalid shift counts, unexpected tokens, and excessive token/depth expansion; internal preprocessing integer metadata is normalized before public ordinary token output.
- Added focused interpreter tests and warning-free native compiler-oracle fixtures; the full local and Docker verification gates cover 1,078 tests.

## v0.3.0 — 2026-07-27

### Language subset

- Added fixed local, global, file-static, and block-static `int[R][C]` / `char[R][C]` objects and named/anonymous aggregate fields with nested initialization, zero filling, deep-copy isolation, double-index scalar lvalues, const protection, exact per-dimension bounds diagnostics, and deterministic type queries.
- Added two-dimensional scalar-array typedef aliases and comma-separated direct/typedef-backed object declarations across local, global, and static storage.
- Added C array-parameter adjustment for direct and typedef-backed two-dimensional scalar arrays while preserving caller-owned storage, element type, fixed column width, qualification, forwarding, and bounds metadata.
- Added interpreter-owned pointer-to-row values for direct arrays and supported two-dimensional aggregate fields, including explicit `T (*row)[C]` objects/parameters, pointer-to-row typedef aliases, `T (*function(params))[C]` returns, row-scaled arithmetic/difference/equality/ordering, and double indexing through call/conditional/comma expressions.

### Diagnostics and safety

- Added exact type, width, rank, row/column bounds, const-discard/write, and escaped-local diagnostics for two-dimensional arrays and row pointers without exposing host addresses.
- Preserved explicit boundaries for variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, scalar-pointer flattening, and unsupported pointer-to-row declarator shapes.

### CLI, packaging, and verification

- Versioned the Cargo package, exact CLI output, and Docker Compose runtime/test images as `0.3.0`.
- Added focused interpreter regressions, deterministic row-expression/field property coverage, exact invalid fixtures, and warning-free native compiler-oracle fixtures.
- Verified 1,039 tests at release time: 929 interpreter tests, 98 deterministic fuzz-safety tests, and 12 CLI, Docker, compiler-oracle, and repository tests.

### Known limitations

- Cust remains a deterministic educational C subset, not a full C implementation or native-ABI emulator.
- Unsupported areas include preprocessing/includes/macros, standard-library calls, floating-point and complex runtime values, multiple pointer levels and `void *`, function pointers and variadic calls, variable-length arrays, arrays with more than two dimensions, flexible array members and bit-fields, `goto`, and host ABI layout/promotion rules.

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
