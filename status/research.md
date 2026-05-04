# Cust Research Log

Research notes for the autonomous agent. Add links, summaries, and decisions here.

## Useful documentation targets

- Rust Book: https://doc.rust-lang.org/book/
- Cargo Book: https://doc.rust-lang.org/cargo/
- Rust API docs: https://doc.rust-lang.org/std/
- Docker Compose docs: https://docs.docker.com/compose/
- C language reference: https://en.cppreference.com/w/c/language

## Research rules

- Prefer official documentation.
- Use local `man` pages when command syntax, libc/C/POSIX behavior, or unavailable external docs make them useful.
- Add the date, URL or `man <page>`, and concise finding.
- If a researched detail affects implementation, mention the file/function changed.
- Keep notes short; link out instead of copying large docs.

## Findings

- 2026-05-04: Autonomous research may use local manual pages (`man`) when needed for command syntax, C/POSIX/libc behavior, or when external docs are unavailable. Record concise `man <page>` findings here just like URL-based findings.

- 2026-05-04: No external documentation was needed for lexer line/column tracking; implementation used simple 1-based position accounting over Rust `char`s in `src/lib.rs`.
- 2026-05-04: `docker compose run --rm test` initially reused the existing `cust-test:v0.1` image and showed only the previous 4 integration tests. Running `docker compose build test cust` first rebuilt images, after which `docker compose run --rm test` showed all 6 tests. Next Docker ergonomics work should prevent stale-image verification.
- 2026-05-04: Docker Compose service reference documents `pull_policy: build` as forcing Compose to build the image and rebuild it if already present: https://docs.docker.com/reference/compose-file/services/#pull_policy. Applied to both `cust` and `test` services in `docker-compose.yml` so required cron verification commands rebuild from the current checkout.
- 2026-05-04: No external documentation was needed for parser location tracking; implementation carries lexer-produced 1-based line/column metadata through `LocatedToken` in `src/lib.rs` and formats parser diagnostics at the consumed or current token.
- 2026-05-04: No external documentation was needed for lexer source context snippets; implementation formats the offending source line plus a caret under the 1-based lexer column in `src/lib.rs`.
- 2026-05-04: C block-scope semantics reference: cppreference documents compound statements/block scope and that identifiers declared in a block are visible from declaration until block end: https://en.cppreference.com/w/c/language/scope. Implemented nested interpreter scopes in `src/lib.rs` with inner shadowing and nearest-scope assignment lookup.
- 2026-05-04: C logical operator semantics reference: cppreference documents `!`, `&&`, and `||` as using scalar truth values where zero is false and nonzero is true; `&&` and `||` short-circuit and return `1` or `0`: https://en.cppreference.com/w/c/language/operator_logical. Implemented this precedence/short-circuit behavior in `src/lib.rs`.
- 2026-05-04: C `for` statement reference: cppreference documents `for (init-clause; expression-2; expression-3) statement`, optional controlling expression interpreted as nonzero/zero truth, and init-clause scope lasting until loop end: https://en.cppreference.com/w/c/language/for. Implemented declaration/assignment initializers, optional condition/increment clauses, and loop-local initializer scope in `src/lib.rs`.
- 2026-05-04: C `break` and `continue` statements did not require additional external research beyond the existing C control-flow roadmap. Implemented loop-local consumption of these control-flow signals for `while` and `for`, with propagation through nested blocks/conditionals and errors at top level in `src/lib.rs`.
- 2026-05-04: Attempted to re-check cppreference statement pages (`https://en.cppreference.com/w/c/language/statements` and `/for`) for empty/expression statement details, but direct HTTP access returned 403 in the cron environment. Implementation followed existing roadmap/C semantics: a bare `;` is a no-op statement, `expr;` evaluates and discards the value, and `for` initializer/increment clauses may use expression statements in addition to existing declarations/assignments.
- 2026-05-04: No additional external documentation was needed for ordinary integer function definitions/calls. Implementation follows the existing C-subset shape: every function has return type `int`, calls evaluate integer arguments before entering a per-call parameter scope, undefined calls and argument-count mismatches are interpreter errors, and a 1,000-call-depth limit guards runaway recursion in `src/lib.rs`.
- 2026-05-04: No additional external documentation was needed for recursive calls. Tightened the existing recursive call guard from 1,000 to 256 active calls because the new depth-limit regression exposed Rust test-process stack overflow before the old limit could return a Cust error; `src/lib.rs` now checks before entering the next function call and reports the callee name.
- 2026-05-04: No external documentation was needed for parser separator diagnostics. Implementation remains within Cust's existing parser design and adds local checks after each parsed function parameter/call argument so malformed lists report the missing separator at the unexpected token instead of falling through to generic `expected RParen`/expression errors.
- 2026-05-04: No additional external documentation was needed for first-pass `char` support. Implementation treats `char` declarations and parameters as integer-valued storage in Cust's existing `i64` interpreter model and lexes ordinary/escaped character literals into numeric codepoint values while preserving lexer line/caret diagnostics for malformed literals.
- 2026-05-04: No additional external documentation was needed for one-dimensional array support. Implementation uses zero-initialized `int`/`char` arrays backed by shared interpreter storage (`Rc<RefCell<ArrayValue>>`), supports indexed reads/writes and array parameters with declared length checks, and reports runtime negative/out-of-bounds accesses from `src/lib.rs`.
- 2026-05-04: Attempted to check cppreference string literal semantics at https://en.cppreference.com/w/c/language/string_literal, but direct HTTP access returned 403 in the cron environment. Implementation follows the C-subset roadmap and standard C behavior needed here: ordinary string literals are NUL-terminated byte arrays, support the same simple escapes as character literals plus `\"`, can be indexed and passed to `char` array parameters, and are read-only in Cust's interpreter storage model.
- 2026-05-04: No external documentation was needed for targeted parser missing-semicolon diagnostics. Implementation uses the parser's existing `LocatedToken` line/column data and adds context-specific semicolon expectations after declarations, assignments, expression statements, and return statements in `src/lib.rs`.
- 2026-05-04: No external documentation was needed for targeted parser missing-`]` diagnostics. Implementation uses the parser's existing `LocatedToken` line/column data and adds context-specific bracket expectations after array declaration lengths, array parameter lengths, indexed array assignments/reads, and string-literal indexing in `src/lib.rs`.
- 2026-05-04: No external documentation was needed for targeted parser missing-`)` diagnostics. Implementation uses the parser's existing `LocatedToken` line/column data and adds context-specific parenthesis expectations after grouped expressions, function call arguments, function definition parameters, and `if`/`while`/`for` headers in `src/lib.rs`.
- 2026-05-04: No external documentation was needed for targeted parser missing-`{` diagnostics. Implementation uses the parser's existing `LocatedToken` line/column data and adds context-specific opening-brace expectations after function headers and `if`/`else`/`while`/`for` control-flow headers in `src/lib.rs`.
- 2026-05-04: No external documentation was needed for additional parser delimiter diagnostics. Implementation adds a shared `expect_opening_paren_after()` helper for function/control-flow headers, reuses `expect_semicolon_after()` for `break`, `continue`, and `for` condition clauses, reports unmatched stray closing delimiters, and rejects `break`/`continue` in non-body `for` clauses in `src/lib.rs`.
