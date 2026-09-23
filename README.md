# Cust

[![Rust implementation](https://img.shields.io/badge/implementation-Rust-DEA584?logo=rust)](#how-it-is-organized) [![Language: C subset](https://img.shields.io/badge/language-bounded%20C%20subset-2563EB)](docs/ROADMAP.md) [![Docker](https://img.shields.io/badge/Docker-runnable-2496ED?logo=docker)](#try-it) [![License: AGPL-3.0-or-later](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue)](LICENSE)

Cust is a Rust interpreter for a **bounded subset of C**. It preprocesses, parses, and executes supported source itself, then prints the integer result of `main()`. It does not compile or run the input with a host C compiler. Cust is useful for exploring C semantics and interpreter design, not as a drop-in C compiler or a general-purpose sandbox for hostile code.

## Try it

From a checkout with a Rust toolchain and Cargo (no C compiler is needed to interpret a program):

```sh
cargo run -- examples/sum.c
# 10
cargo run -- --help
```

[`examples/sum.c`](examples/sum.c) sums the integers below five. Pass your own supported `.c` file in its place. For a bounded loop budget, use `cargo run -- --max-steps 1000 examples/sum.c`; `--tokens` and `--ast` print the token stream and parsed syntax tree without executing the program. `--version` reports the package version. Use `--` before a source filename that begins with `-` (also after a mode's arguments). The CLI accepts one source path; see `--help` for the exact forms.

With Docker and Compose, from the same checkout:

```sh
docker compose run --rm cust  # runs examples/sum.c; prints 10
docker compose run --rm test  # builds and runs the Rust tests
```

For another container input, place the supported source under `examples/` and run `docker compose run --rm cust /workspace/examples/your-program.c`. To use plain Docker instead of Compose:

```sh
docker build -t cust-local .
docker run --rm --network none --read-only -v "$PWD/examples:/workspace/examples:ro" cust-local /workspace/examples/sum.c
# 10
```

Run these from the repository root; the read-only bind mount gives Cust access to the example file. Compose builds from the checkout. The runtime service mounts `examples/` read-only and has no network, a read-only root filesystem, a non-root user, dropped capabilities, and no-new-privileges. The test service builds and runs `cargo test --locked` in the image with no network and a writable container layer; it does **not** mount the host source tree. These settings reduce exposure, but do not make Docker or Cust a security boundary for untrusted C programs.

## What the interpreter covers

- Control flow and expressions: functions/prototypes, local/global/static objects, block scopes, `if`/`else`, loops, `switch`, `break`/`continue`, arithmetic, bitwise, comparison, assignment, conditional and comma expressions, casts, `sizeof`, `_Alignof`, and bounded C11 `_Generic`/`_Static_assert` forms.
- Values and storage: deterministic scalar `int`/`char`/`_Bool` spellings and bounded `double`; fixed and inferred arrays (including supported two-dimensional scalar arrays), strings, structs/unions/enums, typedefs, designated initializers, aggregate copies/returns, and scoped compound literals. Supported typed pointers carry interpreter-owned identity, bounds, const, and lifetime metadata rather than host addresses. For fixed two-dimensional `double` objects, `typedef double (*Row)[C]` and direct row-pointer function parameters/returns also support row-scaled access/arithmetic, qualified views, and compatible signatures; recursive pointer derivatives remain unsupported.
- Preprocessing: bounded macros, conditional directives, line splicing and digraphs. File-aware quoted includes and file-identity-dependent `#pragma once` are supported on Linux for project-relative headers, **not** system headers.
- Selected C library functions work only with supported explicit prototypes and interpreter-owned storage, including bounded string/byte operations, integer conversions, character classification, deterministic `rand`/`srand`, and interpreter-owned termination behavior. GCC/Clang/`cc` are used only by compiler-oracle **tests**, never to execute Cust input. The full `cargo test` suite requires an available C compiler (`gcc`, `clang`, or `cc`).

The exact supported shapes matter: a supported scalar, pointer, or array does **not** imply all C declarators or combinations of them work. Look at [`status/missing-features.md`](status/missing-features.md) and the valid/invalid [`tests/fixtures`](tests/fixtures) for the detailed, evolving boundary.

### Explicit limits

Cust is not ISO C conforming or ABI compatible. Its sizes and layout are deterministic interpreter choices (for example, `int` and pointers are 8 bytes, `char` is 1 byte, and structs have no native padding). It does not implement system headers, general host libc/stdio, arbitrary variadic calls, function pointers, `goto`, bit-fields, flexible array members, VLAs, or arrays above the supported two-dimensional shapes. Pointer support is not general recursive pointer support: ordinary typed pointers are scoped to supported storage, while the `T **` families for `char`, `int`, `_Bool`, and `double` are narrow tracked output-slot forms with further restrictions on decay, address-taking, arithmetic, casts, and qualifiers. `void *` cannot be dereferenced. `float`, `long double`, complex values, and many `double` declarator/storage combinations are unsupported. Raw-memory operations are limited to modeled object-byte roots; they do not expose host addresses or arbitrary object representations. Unsupported programs may be rejected with diagnostics rather than interpreted as C would execute them.

## How it is organized

`src/main.rs` implements the CLI; `src/lib.rs` contains preprocessing/lexing, parsing and the interpreter. The parser builds an AST, and the evaluator tracks Cust-owned values and storage instead of invoking native C. `tests/interpreter.rs` exercises behavior and rejection paths; `tests/c_compat.rs` compares explicitly registered supported fixtures with a native compiler where the expected behavior is portable; other integration tests cover CLI, parser safety and metadata. [`docs/ROADMAP.md`](docs/ROADMAP.md) records product scope and acceptance gates. [`CHANGELOG.md`](CHANGELOG.md) has releases; [`status/`](status/) holds the detailed maintainer backlog/history, not a feature guarantee for every C program.

## Develop and contribute

The full local test suite requires a C compiler (`gcc`, `clang`, or `cc`) as well as Rust and Cargo; `tests/c_compat.rs` invokes it for portable comparisons. Docker builds the test environment with its own compiler.

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
# Optional container verification (requires Docker Compose):
docker compose run --rm test
docker compose run --rm cust
```

When adding a language feature, first define a narrow supported shape and its adjacent rejected shapes. Add focused interpreter tests and valid/invalid fixtures; where C behavior is portable, register a warning-free compiler-oracle fixture in `tests/c_compat.rs`. Run the local checks and, when available, the Compose checks. Avoid host-ABI assumptions in tests. Prefer a semantic C feature or a conformance gap with a concrete counterexample over speculative diagnostic sweeps; see the [roadmap](docs/ROADMAP.md) for phase gates. The [autonomous development plan](docs/plans/autonomous-agent.md) describes maintainer operations, not Cust's public contract.

Cust is licensed under [AGPL-3.0-or-later](LICENSE).
