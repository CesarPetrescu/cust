# Cust Missing Features

Prioritized backlog for autonomous implementation.

## P0 — correctness and developer trust

- [x] Parser errors with source spans: line, column, token context
- [x] Add token context snippets to lexer errors
- [x] Block scoping rules for variables inside `{ ... }`
- [ ] Better parser recovery/error messages (broad track largely expanded; remaining work should be driven by newly discovered malformed programs beyond current exact-error coverage)
- [x] Initial test fixtures for valid and invalid programs
- [x] Improve local Docker test automation for repeatable cron runs

## P1 — C subset expansion

- [x] Function definitions and function calls
- [x] Local function parameters
- [x] Recursive calls with a bounded call-depth limit regression suite
- [x] Boolean/logical operators: `&&`, `||`, `!`
- [x] Unary plus
- [x] `for` loops
- [x] `break` and `continue`
- [x] Empty statements and expression statements

## P2 — data types

- [x] `char` literals, declarations, and function parameters
- [x] arrays
- [x] string literals as read-only byte arrays
- [ ] pointer model design document before implementation

## P3 — C compatibility tooling

- [x] Compare Cust output against `gcc`/`clang` for supported programs in Docker
- [x] Add an initial corpus under `tests/fixtures/`
- [ ] Add fuzz/property tests for lexer/parser safety

## P4 — product quality

- [ ] CLI flags: `--version`, `--ast`, `--tokens`, `--max-steps`
- [ ] Better README examples
- [ ] Release notes and tags
- [ ] License file
