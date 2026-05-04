# Cust Current State

Last updated: 2026-05-04

## Repository

- Path: `/root/hermes-workspace/cust`
- Remote: `git@github.com-cust:CesarPetrescu/cust.git`
- Default branch: `main`
- Current version: `v0.1`

## Implemented

- Rust binary + library crate named `cust`
- Tiny C-subset interpreter with pipeline:
  - lexer
  - parser
  - AST
  - interpreter
  - CLI wrapper
- CLI command shape: `cust <file.c>`
- Example: `examples/sum.c`
- Docs:
  - `README.md`
  - `docs/v0.1.md`
- Docker:
  - `Dockerfile`
  - `docker-compose.yml`
  - safe runtime service with no network, non-root user, read-only FS, dropped capabilities
  - Compose services force source rebuilds with `pull_policy: build` to avoid stale-image test/runtime runs
  - build/test image installs `gcc`/`libc6-dev` so Rust integration tests can compile supported C compatibility fixtures inside Docker

## Supported language subset

- `int main() { ... }` plus additional `int name(...) { ... }` function definitions
- function calls as expressions with integer arguments, local parameter scopes, direct/mutual recursion support, arity diagnostics, undefined-function diagnostics, and a 256-call-depth safety limit with function-name context
- integer literals, character literals, string literals, variables, and one-dimensional `int`/`char` arrays
- declarations: `int x = expr;`, `char x = expr;`, `int xs[N];`, or `char cs[N];`
- `int` and `char` function parameters (stored as integer values in the current interpreter model)
- one-dimensional array parameters such as `int values[3]`, passed by reference to the same array storage; string literals can be passed to `char` array parameters as read-only NUL-terminated byte arrays
- assignments: `x = expr;` and `xs[index] = expr;`
- array indexing expressions `xs[index]` and string literal indexing expressions `"text"[index]` with runtime negative/out-of-bounds diagnostics
- `return expr;`
- nested block statements `{ ... }` with per-block variable scopes, inner shadowing, and outer-scope assignment lookup
- `if (...) { ... } else { ... }`
- `while (...) { ... }`
- `for (init; condition; increment) { ... }` with optional clauses, declaration/assignment initializers, assignment increments, loop-local initializer scope, and the shared 1,000,000-iteration safety limit
- `break;` and `continue;` in `while` and `for` loop bodies, including propagation through nested blocks/conditionals and diagnostics when used outside loops
- empty statements (`;`) and expression statements (`expr;`) in block bodies and C-style `for` initializer/increment clauses
- `+ - * / %`, unary `+`, unary `-`
- `== != < <= > >=`
- logical operators `&&`, `||`, and `!` with C-style integer truth values and short-circuit evaluation for `&&`/`||`
- `//` comments

## Test/tooling coverage

- `tests/c_compat.rs` compiles supported compatibility fixtures with `$CC`, `gcc`, `clang`, or `cc`, runs the resulting native executables, and asserts their process exit codes match `cust::interpret` results. The Docker build/test image installs `gcc` and `libc6-dev` so this coverage runs under `docker compose run --rm test`.
- Compatibility corpus currently covers arithmetic/control flow/logical operators plus functions, recursion, arrays, and string-literal/char-array interactions under `tests/fixtures/compat/valid/`.

## Diagnostics

- Lexer errors include 1-based line and column plus a source-line/caret context snippet for unexpected characters and out-of-range integer literals.
- Parser errors include 1-based line and column plus token context for expected-token, identifier, expression, statement, and unterminated-block failures.
- Parser diagnostics now include targeted separator messages for malformed function parameter lists and function call argument lists, including missing commas and trailing commas.
- Parser diagnostics now include targeted missing-semicolon messages after variable declarations, array declarations, scalar/indexed assignments, expression statements, and return statements.
- Parser diagnostics now include targeted missing-`]` messages for array declaration lengths, array parameter lengths, indexed assignments, indexed array expressions, and string-literal indexing expressions.
- Parser diagnostics now include targeted missing-`)` messages for grouped expressions, function call arguments, function definition parameters, and `if`/`while`/`for` headers.
- Parser diagnostics now include targeted missing-`{` messages after function headers and `if`/`else`/`while`/`for` control-flow headers.
- Parser diagnostics now include targeted missing-`(` messages after function names and `if`/`while`/`for` keywords, targeted missing-semicolon messages after `break`, `continue`, and `for` conditions, unmatched closing delimiter messages for stray `)`/`]` in statements and extra `}` at top level, and explicit rejection of `break`/`continue` in non-body `for` clauses.

## Verified commands

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after adding C compiler compatibility tests in the 2026-05-04 autonomous run. The suite now includes `tests/c_compat.rs`, which compiles supported fixture programs with a native C compiler and compares the compiled C exit code to Cust's interpreted result. Docker Compose emitted a non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

## Operating rule for autonomous agent

The autonomous agent must keep this `status/` directory current every run, even if no code changes are made.
