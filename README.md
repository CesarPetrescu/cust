# Cust

Cust is a tiny C interpreter written in Rust. It reads a safe subset of C, interprets it directly, and prints the integer value returned by `main()`.

> Status: **v0.1** — tested, Dockerized C-subset interpreter foundation.

## License

Cust is licensed under the GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`). See `LICENSE` for the full text. This strong copyleft license is intended to keep distributed and network-served modified versions open-source.

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

- `int main() { ... }` or `int main(void) { ... }` plus additional `int`, `char`, `void`, supported `struct`, or supported `union` function definitions/prototypes; prototypes may use C-style unnamed parameter declarations such as `int add(int, int);` or `void use(int [], struct Point *);`
- function calls with scalar/struct/union/pointer arguments, local function parameters, and C-style empty `void` parameter lists
- integer, character, and string literals
- declarations: initialized or zero/default-initialized `int`/`char` scalars, arrays, supported pointer variables, first-pass `const int` / `const char` scalars and arrays, typedef aliases, structs, unions, and enum constants, such as `int x = 1;`, `int y;`, `char c;`, `const int limit = 5;`, `int xs[3];`, `char text[4];`, `int *p;`, `typedef int Count;`, `struct Point { int x; char y; };`, `typedef struct Pair { int left; int right; } Pair;`, `typedef enum { READY = 1, RUNNING } State;`, block-local aggregate typedef definitions that may shadow outer tags, and `enum StateTag { READY_TAG = 1, RUNNING_TAG };`
- assignment statements and assignment expressions for scalar, array-index, field, and dereferenced pointer lvalues, such as `x = x + 1;`, `y = (x = 4);`, `xs[0] = (xs[1] = 7);`, `point.x += 1;`, and `*p = value;`
- scalar cast expressions for supported scalar types and typedef aliases, such as `(int)expr`, `(char)expr`, and `(Count)expr`
- one-dimensional `int`/`char` arrays with indexed reads/writes
- scalar pointers such as `int *p = &x;`, `*p`, and `*p = value;`
- pointer parameters with array/string decay, pointer indexing (`p[i]`), and array-element addresses such as `&values[1]`
- array parameters such as `char text[4]` and C-style unsized parameter spellings such as `int values[]`, `char text[]`, and `struct Point points[]`, which behave like pointer parameters; string literals are read-only NUL-terminated byte arrays and can be passed to matching array or pointer parameters
- nested block scopes with inner shadowing
- `return expr;` for `int` functions and `return;` for `void` functions
- `if` / `else`
- `while`
- `for`
- `break` / `continue`
- `switch`, `case`, and `default` with C-style fallthrough
- empty statements (`;`) and expression statements (`expr;`)
- arithmetic and bitwise operators: `+ - * / %`, unary `~`, binary `& ^ |`, and shifts `<< >>`
- comparisons: `== != < <= > >=`
- logical operators with C-style truth values and short-circuiting: `&& || !`
- unary plus: `+expr`
- comments: `// line comments` and `/* block comments */`
- `sizeof` for supported types/expressions, including aggregate type spellings such as `sizeof(struct Point)` / `sizeof(union Number)` and const-qualified type contexts such as `sizeof(const int)`, with Cust-defined sizes (`int = 8`, `char = 1`, pointer = `8`, no native struct padding)

## CLI

```bash
cust <file.c>
cust --max-steps N <file.c>
cust --tokens <file.c>
cust --ast <file.c>
cust --version
```

Default output is the integer returned from `main()`. `--max-steps N` runs the program with an explicit total loop-iteration budget, which is useful for bounding runaway programs from the CLI without changing the library default. `--tokens` prints the lexer token stream with source locations, and `--ast` prints the parsed syntax tree without interpreting the program, which is useful for inspecting parse results even when the program would fail at runtime.

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

Cust is not a full C implementation yet. Missing features include preprocessor support, includes, standard library calls, floating-point values, multiple pointer levels, richer casts (including pointer and aggregate casts), native ABI layout compatibility, function pointers, and many broader C-subset compatibility rules. Cust is an interpreter: it executes user programs itself. Native compilers such as GCC/Clang may be used only as an optional test oracle to compare expected behavior for supported fixtures, never as Cust's execution path or as an implementation shortcut.

See [CHANGELOG.md](CHANGELOG.md) for release notes and [docs/v0.1.md](docs/v0.1.md) for implementation details and safety notes.

## Roadmap

- Near term: continue parser recovery/error-message expansion only for newly discovered malformed programs that are not already covered by exact-output diagnostics tests.
- Next language design: choose the next larger C-subset area from `status/todo.md` and define acceptance fixtures before implementation.
- Product quality: refresh release-oriented docs when the implemented language surface changes.
- Longer term: consider preprocessor support, includes, standard-library calls, floating-point values, multiple pointer levels, and broader C conformance fixtures.

## License

GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`). See [LICENSE](LICENSE).
