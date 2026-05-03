# Cust Missing Features

Prioritized backlog for autonomous implementation.

## P0 — correctness and developer trust

- [ ] Source spans in lexer/parser errors: line, column, token context
- [ ] Block scoping rules for variables inside `{ ... }`
- [ ] Better parser recovery/error messages
- [ ] Test fixtures for valid and invalid programs
- [ ] GitHub Actions CI using Docker and Cargo

## P1 — C subset expansion

- [ ] Function definitions and function calls
- [ ] Local function parameters
- [ ] Recursive calls with a bounded call-depth limit
- [ ] Boolean/logical operators: `&&`, `||`, `!`
- [ ] Unary plus
- [ ] `for` loops
- [ ] `break` and `continue`
- [ ] Empty statements and expression statements

## P2 — data types

- [ ] `char`
- [ ] arrays
- [ ] string literals as read-only byte arrays
- [ ] pointer model design document before implementation

## P3 — C compatibility tooling

- [ ] Compare Cust output against `gcc`/`clang` for supported programs in Docker
- [ ] Add a corpus under `tests/fixtures/`
- [ ] Add fuzz/property tests for lexer/parser safety

## P4 — product quality

- [ ] CLI flags: `--version`, `--ast`, `--tokens`, `--max-steps`
- [ ] Better README examples
- [ ] Release notes and tags
- [ ] License file
