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

- `int main() { ... }`
- integer literals and variables
- declarations: `int x = 1;`
- assignment: `x = x + 1;`
- `return`
- `if` / `else`
- `while`
- arithmetic: `+ - * / %`
- comparisons: `== != < <= > >=`
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
│   └── interpreter.rs
├── examples/
│   └── sum.c
├── docs/
│   └── v0.1.md
├── Dockerfile
└── docker-compose.yml
```

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

Cust is not a full C implementation yet. Missing features include pointers, arrays, structs, function calls, preprocessor support, includes, standard library calls, floating-point values, and full C scoping rules.

See [docs/v0.1.md](docs/v0.1.md) for implementation details and safety notes.

## Roadmap

- v0.2: block scopes and better diagnostics with source spans
- v0.3: function definitions and calls
- v0.4: arrays and strings
- v0.5: richer C compatibility tests

## License

MIT or project-owner choice. Add a `LICENSE` file before publishing releases.
