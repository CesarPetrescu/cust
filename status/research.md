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
- Add the date, URL, and concise finding.
- If a researched detail affects implementation, mention the file/function changed.
- Keep notes short; link out instead of copying large docs.

## Findings

- 2026-05-04: No external documentation was needed for lexer line/column tracking; implementation used simple 1-based position accounting over Rust `char`s in `src/lib.rs`.
- 2026-05-04: `docker compose run --rm test` initially reused the existing `cust-test:v0.1` image and showed only the previous 4 integration tests. Running `docker compose build test cust` first rebuilt images, after which `docker compose run --rm test` showed all 6 tests. Next Docker ergonomics work should prevent stale-image verification.
- 2026-05-04: Docker Compose service reference documents `pull_policy: build` as forcing Compose to build the image and rebuild it if already present: https://docs.docker.com/reference/compose-file/services/#pull_policy. Applied to both `cust` and `test` services in `docker-compose.yml` so required cron verification commands rebuild from the current checkout.
- 2026-05-04: No external documentation was needed for parser location tracking; implementation carries lexer-produced 1-based line/column metadata through `LocatedToken` in `src/lib.rs` and formats parser diagnostics at the consumed or current token.
- 2026-05-04: No external documentation was needed for lexer source context snippets; implementation formats the offending source line plus a caret under the 1-based lexer column in `src/lib.rs`.
- 2026-05-04: C block-scope semantics reference: cppreference documents compound statements/block scope and that identifiers declared in a block are visible from declaration until block end: https://en.cppreference.com/w/c/language/scope. Implemented nested interpreter scopes in `src/lib.rs` with inner shadowing and nearest-scope assignment lookup.
- 2026-05-04: C logical operator semantics reference: cppreference documents `!`, `&&`, and `||` as using scalar truth values where zero is false and nonzero is true; `&&` and `||` short-circuit and return `1` or `0`: https://en.cppreference.com/w/c/language/operator_logical. Implemented this precedence/short-circuit behavior in `src/lib.rs`.
