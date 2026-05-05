# Cust Missing Features

Prioritized backlog for autonomous implementation.

## P0 — correctness and developer trust

- [x] Parser errors with source spans: line, column, token context
- [x] Add token context snippets to lexer errors
- [x] Block scoping rules for variables inside `{ ... }`
- [ ] Better parser recovery/error messages (broad track largely expanded; missing-`=` diagnostics, missing-name/type diagnostics, context-aware unterminated-block diagnostics, array length edge diagnostics, pointer-parameter malformed-list coverage, explicit unsupported pointer-return/pointer-array/pointer-to-pointer diagnostics, and delimiter-aware parameter/call trailing-comma diagnostics are covered; remaining work should be driven by newly discovered malformed programs beyond current exact-error coverage)
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
- [x] pointer model design document before implementation
- [x] first scalar pointer milestone: pointer declarations, scalar address-of/dereference, dereference assignment, reassignment, null diagnostics, and out-of-scope scalar diagnostics
- [x] pointer parameters, array/string decay to pointer arguments, and pointer indexing for array-base pointers
- [x] `&array[index]` array-element pointers and dereference assignment/read support

## P3 — C-subset conformance tooling

- [x] Compare Cust output against `gcc`/`clang`/`cc` for supported programs as an external test oracle only
- [x] Add an initial corpus under `tests/fixtures/`
- [x] Add more conformance fixtures with explicit expected results and optional compiler-oracle comparisons where supported (pointer scalar/parameter/array-decay/array-element/truthiness/equality coverage plus invalid negative pointer-index and unsupported pointer/integer diagnostics have been added; continue expanding mixed supported-subset fixtures as new features land)
- [x] Add fuzz/property tests for lexer/parser safety

## P4 — product quality

- [x] CLI flag: `--tokens`
- [x] CLI flag: `--ast`
- [x] CLI flag: `--max-steps`
- [x] CLI flag: `--version`
- [x] Better README examples
- [x] Release notes and tags (`CHANGELOG.md` and annotated `v0.1` tag)
- [ ] License file
