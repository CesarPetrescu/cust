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
