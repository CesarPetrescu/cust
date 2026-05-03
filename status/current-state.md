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

## Supported language subset

- `int main() { ... }`
- integer literals and variables
- declarations: `int x = expr;`
- assignments: `x = expr;`
- `return expr;`
- `if (...) { ... } else { ... }`
- `while (...) { ... }`
- `+ - * / %`
- `== != < <= > >=`
- `//` comments

## Diagnostics

- Lexer errors include 1-based line and column for unexpected characters and out-of-range integer literals.

## Verified commands

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after adding lexer error source locations in the 2026-05-04 autonomous run. Docker images were rebuilt before the final Docker verification so container tests used the updated source.

## Operating rule for autonomous agent

The autonomous agent must keep this `status/` directory current every run, even if no code changes are made.
