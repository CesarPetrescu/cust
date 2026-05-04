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

## Supported language subset

- `int main() { ... }` plus additional `int name(...) { ... }` function definitions
- function calls as expressions with integer arguments, local parameter scopes, direct/mutual recursion support, arity diagnostics, undefined-function diagnostics, and a 256-call-depth safety limit with function-name context
- integer literals, character literals, and variables
- declarations: `int x = expr;` or `char x = expr;`
- `int` and `char` function parameters (stored as integer values in the current interpreter model)
- assignments: `x = expr;`
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

## Diagnostics

- Lexer errors include 1-based line and column plus a source-line/caret context snippet for unexpected characters and out-of-range integer literals.
- Parser errors include 1-based line and column plus token context for expected-token, identifier, expression, statement, and unterminated-block failures.
- Parser diagnostics now include targeted separator messages for malformed function parameter lists and function call argument lists, including missing commas and trailing commas.

## Verified commands

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after adding first-pass `char` support in the 2026-05-04 autonomous run. The suite now includes valid fixture coverage for `char` declarations, `char` function parameters, ordinary character literals, and escaped character literals (`\n`, `\'`, `\\`), plus invalid fixture coverage for malformed multi-character literals. Docker Compose emitted a non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

## Operating rule for autonomous agent

The autonomous agent must keep this `status/` directory current every run, even if no code changes are made.
