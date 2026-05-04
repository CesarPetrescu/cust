# Cust

Cust is a tiny C interpreter written in Rust. It reads a safe subset of C, interprets it directly, and prints the integer value returned by `main()`.

> Status: **v0.1** — minimal, tested, Dockerized foundation.

## Why Cust?

Cust is meant as a clean starting point for experimenting with language implementation in Rust:

- C-like syntax
- Small lexer/parser/interpreter pipeline
- Automated tests
- Docker-based safe execution
- No native C compilation or system execution

## Quick start

### Run locally

```bash
cargo run -- examples/sum.c
```

Expected output:

```text
10
```

### Run tests locally

```bash
cargo test
```

### Run inside Docker

Build and run the example:

```bash
docker compose run --rm cust
```

Run the test suite in a container:

```bash
docker compose run --rm test
```

The Docker Compose setup is intentionally locked down for safer automated testing:

| Setting | Purpose |
|---|---|
| `network_mode: "none"` | no network access during execution/tests |
| `read_only: true` on runtime | interpreter runtime filesystem is read-only |
| `cap_drop: [ALL]` | removes Linux capabilities |
| `no-new-privileges:true` | blocks privilege escalation |
| non-root runtime user | avoids running interpreted code as root |
| read-only `examples` volume | sample C inputs cannot be modified by the runtime container |

Both Compose services use `pull_policy: build`, so `docker compose run --rm test` and `docker compose run --rm cust` rebuild from the current checkout instead of silently reusing stale local images.

The `test` service keeps a writable container overlay so Cargo can update `target/`, but it has no host source mount, no network, dropped capabilities, and no privilege escalation.

## Supported v0.1 language

Cust currently supports this C subset:

```c
int main() {
    int i = 0;
    int sum = 0;

    while (i < 5) {
        sum = sum + i;
        i = i + 1;
    }

    if (sum == 10) {
        return sum;
    } else {
        return 0;
    }
}
```

Features:

- `int main() { ... }` plus additional `int` function definitions
- function calls with integer arguments and local function parameters
- integer literals and variables
- declarations: `int x = 1;`
- assignment: `x = x + 1;`
- nested block scopes with inner shadowing
- `return`
- `if` / `else`
- `while`
- `for`
- `break` / `continue`
- empty statements (`;`) and expression statements (`expr;`)
- arithmetic: `+ - * / %`
- comparisons: `== != < <= > >=`
- logical operators with C-style truth values and short-circuiting: `&& || !`
- unary plus: `+expr`
- `//` comments

## CLI

```bash
cust <file.c>
```

Output is the integer returned from `main()`.

Example:

```bash
cargo run -- examples/sum.c
# 10
```

## Project layout

```text
.
├── src/
│   ├── lib.rs        # lexer, parser, AST, interpreter
│   └── main.rs       # CLI wrapper
├── tests/
│   ├── fixtures/      # valid/invalid C fixture programs
│   └── interpreter.rs
├── examples/
│   └── sum.c
├── docs/
│   ├── autonomous-agent-prompt.md
│   ├── plans/autonomous-agent.md
│   └── v0.1.md
├── status/           # autonomous maintainer state/backlog/blockers/research
├── Dockerfile
└── docker-compose.yml
```

## Autonomous maintenance

Cust includes a `status/` workspace for an autonomous Hermes maintainer:

| File | Purpose |
|---|---|
| `status/current-state.md` | current project snapshot |
| `status/missing-features.md` | prioritized backlog |
| `status/todo.md` | next tasks and every-run checklist |
| `status/stuck.md` | blockers and failed attempts |
| `status/research.md` | links/findings from docs research |

See `docs/plans/autonomous-agent.md` and `docs/autonomous-agent-prompt.md` for the cron-based maintenance loop.

## Development

Useful commands:

```bash
cargo fmt
cargo test
cargo clippy -- -D warnings
cargo run -- examples/sum.c
```

Docker commands:

```bash
docker compose build
docker compose run --rm test
docker compose run --rm cust
```

## v0.1 limitations

Cust is not a full C implementation yet. Missing features include pointers, arrays, structs, preprocessor support, includes, standard library calls, floating-point values, and many richer C compatibility rules.

See [docs/v0.1.md](docs/v0.1.md) for implementation details and safety notes.

## Roadmap

- v0.2: better diagnostics with source spans and expanded test fixtures
- v0.3: recursive calls with an explicit bounded-depth regression suite
- v0.4: arrays and strings
- v0.5: richer C compatibility tests

## License

MIT or project-owner choice. Add a `LICENSE` file before publishing releases.
