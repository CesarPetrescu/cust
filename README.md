# Cust

Cust is a tiny C interpreter written in Rust. It reads a safe subset of C, interprets it directly, and prints the integer value returned by `main()`.

> Status: **v0.40.0** — tested, Dockerized deterministic C-subset interpreter.

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

The v0.40.0 executable inventory is 2,079 tests: 1,943 interpreter tests, 99 deterministic fuzz-safety tests, 33 CLI tests, 2 Docker metadata tests, 1 compiler-oracle harness, and 1 repository-license test.

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

## Current language subset

The v0.40.0 package gives supported standalone scalar and one-dimensional-array `double` storage a deterministic eight-byte IEEE-754 binary64 little-endian object representation across exactly prototyped bounded `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr`. Full and partial reads/writes preserve destination-aware typed coercion, zero count, const/lifetime/capacity/overlap/owner safety, aligned/interior `memchr` identity, one-time ordered argument evaluation, and direct/nested non-evaluating validation without exposing host addresses or invoking host libc. Raw-memory views over aggregate-field, two-dimensional, union-backed, pointer-object, or unprovable helper-returned double storage remain unsupported. Release procedure publishes annotated tags only after independent review, the canonical gate, and release-commit acceptance on `origin/main`. Cust currently supports this C subset:

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

- bounded object-like, named-parameter, and variadic function-like `#define` macros with balanced argument collection, raw/prescanned substitution, stringification (`#` / `%:`), token pasting (`##` / `%:%:`), rescanning, and nested expansion; bounded `#undef`; bounded nested `#ifdef`/`#ifndef`/`#if`/`#elif`/`#else`/`#endif`; direct-source C11 digraph punctuators (`<:`, `:>`, `<%`, `%>`, `%:`, `%:%:`) with spelling preservation; global LF/CRLF physical-line splicing; and project-relative quoted headers on Linux, including object/function-macro operands that expand to exactly one ordinary string-literal header name, with shared macro state plus expansion/depth/source-size/path-containment bounds; comments and string/character literal contents are not macro-expanded
- `int main() { ... }` or `int main(void) { ... }` plus additional `int`, `char`, `double`, `void`, supported `struct`, supported `union`, or direct named-`enum` function definitions/prototypes; prototypes may use C-style unnamed parameter declarations such as `int add(int, int);`, `double scale(double);`, or `void use(int [], struct Point *);`
- function calls with scalar/struct/union/pointer arguments, local function parameters, C-style empty `void` parameter lists, and by-value scalar/aggregate return types including top-level `const` spellings such as `const int f(void)` / `const struct Point make(void)`
- integer, character, string, and bounded decimal `double` literals
- manually declared C11 integer library calls: `abs`, `labs`, and `llabs` over Cust integers; `atoi`, `atol`, and `atoll` plus base-aware `strtol`, `strtoll`, `strtoul`, and `strtoull` over interpreter-owned NUL-terminated `char` storage; deterministic ASCII/C-locale `isalnum`/`isalpha`/`isblank`/`iscntrl`/`isdigit`/`isgraph`/`islower`/`isprint`/`ispunct`/`isspace`/`isupper`/`isxdigit` classification plus `tolower`/`toupper` conversion; bounded unsigned-byte lexical `strcmp`/`strncmp` plus C-locale `strcoll` and bounded `strxfrm`; bounded `strlen`; pointer-preserving bounded `strchr`/`strrchr`/`strpbrk`/`strstr`; initial-segment `strspn`/`strcspn`; capacity-checked mutable `strcpy`/`strcat`/`strncat`/`strncpy`; stateful in-place `strtok` over tracked mutable storage; bounded overlap-rejecting `memcpy`, overlap-safe `memmove`, unsigned-byte lexical `memcmp`, byte-normalizing `memset`, and unsigned-byte first-match `memchr` over standalone character storage, character scalars/one-dimensional arrays embedded in named, anonymous, or nested struct fields, row-local views of supported two-dimensional character arrays, deterministic little-endian object bytes of eight-byte `int` and one-byte canonical `_Bool` scalars/arrays/selected two-dimensional rows, deterministic eight-byte IEEE-754 binary64 little-endian object bytes of standalone scalar and one-dimensional-array `double` storage, and complete supported non-union structs whose layouts contain no double or pointer fields, including supported integer/character/`_Bool` scalar fields, nested structs, and embedded struct arrays; deterministic per-interpreter `rand`/`srand`; and interpreter-owned `exit`/`_Exit`/`abort` unwinding, with one-time ordered argument evaluation, non-evaluating `sizeof`, deterministic bounds/state, exact diagnostics, safe absent/null/non-null `endptr` handling, and no host libc termination path
- deterministic scalar spellings for `_Bool`, `char`, `short`, `int`, `long`, signed/unsigned permutations, and typedef aliases; Cust intentionally normalizes these onto its own fixed scalar model rather than host ABI widths
- bounded direct `double` scalar objects, one-dimensional arrays, and fixed two-dimensional objects at local, file-global, and block-static scope plus scalar and one-dimensional-array fields in supported structs and unions, including fixed/inferred one-dimensional lengths, nested two-dimensional initialization, positional/designated initialization and zero fill, direct and typedef-backed one-dimensional array compound literals with scoped hidden storage and ordinary decay, direct/reverse indexed and direct/indexed/arrow/nested field replacement/compound/prefix/postfix updates, double-index two-dimensional updates, aggregate pointer-field element routes, mixed arithmetic/comparison, conditional/comma/`_Generic` forwarding, truthiness, scalar compound literals, `int`/`double`/`_Bool` casts, integer-constant casts, direct scalar parameter/return declarations and call results, aggregate copies and function boundaries, recursive const protection, shared scalar-union bits, one-time index evaluation, non-evaluating full-object/row/field/element `sizeof`, deterministic eight-byte size/alignment relationships, and typedef aliases for each supported scalar, one-dimensional-array, function-boundary, aggregate-field, and fixed two-dimensional object form
- safe one-level direct `double *` objects and typedef aliases over standalone and supported struct-field scalar/one-dimensional-array storage, including local/file-global/block-static declarations, direct/indexed/reverse/arrow/nested field address and array decay, indexed reads and updates, bounded arithmetic, same-array comparison, truthiness/equality, pointer fields, parameter/return forwarding, qualification-preserving `void *` conversion, pointer-slot versus pointee const preservation, and non-evaluating `sizeof`/`_Alignof`/`_Generic` classification with interpreter-owned owner/path/lifetime/const metadata
- named and anonymous structs/unions, named and typedef-backed enums, nested aggregates, scalar/aggregate array fields, pointer fields, aggregate arrays, by-value parameters/returns/copies, designated initializers, and scalar/array/aggregate compound literals
- declarations: initialized or zero/default-initialized `int`/`char` scalars, arrays, supported pointer variables, first-pass `const int` / `const char` scalars and arrays, direct named-`enum` variables, typedef aliases, structs, unions, and enum constants, such as `int x = 1;`, `int y;`, `char c;`, `const int limit = 5;`, `enum StateTag state = READY_TAG;`, `const enum StateTag saved = RUNNING_TAG;`, `int xs[3];`, `char text[4];`, `int *p;`, `typedef int Count;`, `struct Point { int x; char y; };`, anonymous object declarations such as `struct { int x; int y; } point = {1, 2};`, `typedef struct Pair { int left; int right; } Pair;`, `typedef enum { READY = 1, RUNNING } State;`, block-local aggregate typedef definitions that may shadow outer tags, and `enum StateTag { READY_TAG = 1, RUNNING_TAG };`
- assignment statements and assignment expressions for scalar, array-index, field, and dereferenced pointer lvalues, such as `x = x + 1;`, `y = (x = 4);`, `xs[0] = (xs[1] = 7);`, `point.x += 1;`, and `*p = value;`
- scalar cast expressions for supported scalar types and typedef aliases, such as `(int)expr`, `(char)expr`, and `(Count)expr`
- one-dimensional scalar and aggregate arrays with fixed or initializer-inferred lengths, indexed reads/writes, reverse subscripting, array designators, string initializers for `char` arrays, and C array-to-pointer adjustment for function parameters
- fixed two-dimensional `int[R][C]` and `char[R][C]` objects and aggregate fields with nested initialization, typedef aliases, comma-separated declarators, double-index scalar lvalues, deterministic type queries, and C-style parameter adjustment
- safe pointer-to-row forms for fixed two-dimensional scalar arrays, including `T (*row)[C]` objects/parameters, pointer-to-row typedef aliases and function returns, row-scaled arithmetic/comparison, and double indexing through direct, call, conditional, comma, and supported aggregate-field decay expressions
- safe one-level typed pointers such as `int *p = &x;`, `struct Point *point = points`, dereference/address-of, pointer-returning functions, bounded arithmetic, same-array difference/ordering, pointer truthiness/equality, and const-preserving scalar/aggregate conversions
- bounded one-level `void *` objects at local, file-global, block-static, `for`-initializer, and parameter scope plus explicit function return types, prototypes, definitions, and call results, with null/equality/truthiness, ordinary assignment, conditional/comma forwarding, `_Generic`, compatible object-pointer conversions, qualification preservation, and constraint-aware non-evaluating pointer-size queries while retaining interpreter-owned owner/lifetime/read-only identity
- narrowly typed unqualified `char **` objects at local, file-global, and block-static scope plus standard-library-style output parameters, preserving mutable `char *` slot identity, pointee owner/lifetime/read-only metadata, null/default state, ordinary reassignment from null, compatible mutable unqualified `char *` slot addresses, or tracked object values, indirect reads/writes, conditional/comma and assignment-result forwarding, equality/truthiness, `_Bool` normalization, and branch-compatible non-evaluating type checks without exposing host addresses; qualified slot addresses, compound updates, address-taking, arithmetic, and relational ordering remain exact boundaries
- pointer parameters with scalar/aggregate array and string decay, pointer indexing (`p[i]`), supported field-array decay, and element/field addresses such as `&values[1]`, `&points[1]`, and `&point->x`
- array parameters such as `char text[4]` and C-style unsized parameter spellings such as `int values[]`, `char text[]`, and `struct Point points[]`, which behave like pointer parameters; string literals are read-only NUL-terminated byte arrays and can be passed to matching array or pointer parameters
- nested block scopes with inner shadowing
- `return expr;` for supported non-void scalar and aggregate functions, including direct `double` returns, and `return;` for `void` functions
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
- assignment/compound-assignment expressions, prefix/postfix `++`/`--`, conditional `?:`, comma expressions, scalar/pointer/void casts, and C-style scalar or pointer result classification
- comments: `// line comments` and `/* block comments */`
- `sizeof` and `_Alignof` for supported type names and expressions, including aggregate, enum, pointer, array, qualified, conditional, comma, and assignment-result forms, with non-evaluating operand semantics and Cust-defined sizes (`int = 8`, `char = 1`, pointer = `8`, no native struct padding)
- C11 `_Static_assert`, storage-class/function-specifier syntax, supported `const`/`volatile`/`restrict`/`_Atomic` qualification, and per-function read-only `__func__` arrays
- bounded C11 `_Generic` selections over deterministic scalar, one-level pointer, and named aggregate association types, including optional `default`, selected-expression value/type propagation, non-evaluation of the controlling and unselected expressions, integer-constant-expression use, and exact duplicate/default/type/no-match diagnostics


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

## Current limitations

Cust is not a full C implementation. On current `main`, the bounded raw-memory slice supports `memcpy`, `memmove`, `memcmp`, `memset`, and `memchr` over interpreter-owned standalone character storage, character scalars/one-dimensional arrays embedded in struct fields, one selected row of supported two-dimensional character storage, scalar/one-dimensional-array/selected-two-dimensional-row eight-byte `int` plus canonical one-byte `_Bool` storage both standalone and embedded in supported struct fields, standalone scalar and one-dimensional-array `double` storage under Cust's deterministic eight-byte IEEE-754 binary64 little-endian representation, and complete supported non-union structs whose layouts contain no double or pointer fields, including supported integer/character/`_Bool` scalar fields, one- and two-dimensional scalar arrays, nested structs, and embedded struct arrays under Cust's deterministic field-order no-padding model; every range remains within its selected storage root. Union-backed storage, aggregate-field or two-dimensional `double` storage, whole pointer-object storage, and structs containing pointer fields remain unsupported because Cust does not model shared union bytes or pointer object encodings and does not flatten aggregate/row double storage. Evaluated helper calls can return supported standalone double pointers to bounded-memory intrinsics, but unprovable helper-returned double targets remain conservatively rejected in non-evaluating contexts. Fixed two-dimensional `int` and `char` arrays, their aggregate fields, adjusted parameters, and safe pointer-to-row forms are supported; direct and typedef-backed fixed two-dimensional `double` objects are supported only at local, file-global, and block-static scope. Variable-length arrays, arrays with more than two dimensions, aggregate-valued multidimensional elements, and two-dimensional controlling expressions in `_Generic` remain unsupported. Preprocessing supports bounded object-like, named-parameter, and variadic function-like `#define`, function-like macro stringification and token pasting, `#undef`, C11 null `#` / `%:` directives, active `#error` / `%:error` diagnostics, dynamic predefined `__FILE__`/`__LINE__`, C11 `#line`/`%:line` presumed source locations, physical-line splicing, nested conditionals, direct-source C11 digraph punctuators, and project-relative quoted headers on Linux with direct or exactly-one-string-literal macro-expanded operands plus shared expansion/depth/source-size/path-containment bounds. File-aware Linux preprocessing also supports active `#pragma once` / `%:pragma once` and direct or macro-produced `_Pragma("once")`, keyed by opened file identity across repeated, symlinked, hard-linked, and recursive header paths. Other pragma names remain unsupported; direct string-only library entry points reject file-identity-dependent pragmas and includes. System headers remain unsupported, and quoted inclusion fails closed on non-Linux platforms. Other unsupported areas include standard-library calls beyond explicitly prototyped `abs`/`labs`/`llabs`, `atoi`/`atol`/`atoll`, `strtol`/`strtoll`/`strtoul`/`strtoull`, `strcmp`/`strncmp`/`strcoll`, `strlen`/`strxfrm`, `strchr`/`strrchr`/`strpbrk`/`strstr`, `strspn`/`strcspn`, `strcpy`/`strcat`/`strncat`/`strncpy`, `strtok`, `memcpy`/`memmove`/`memcmp`/`memset`/`memchr`, the twelve C11 `is*` character classifiers, `tolower`/`toupper`, `rand`/`srand`, and `exit`/`_Exit`/`abort`; locale-sensitive behavior outside Cust's fixed ASCII/C-locale model; floating types and forms beyond bounded direct and typedef-backed `double` scalar/function-boundary/one-dimensional-array/scalar-and-one-dimensional-array-aggregate-field support, direct and typedef-backed fixed two-dimensional object storage, and one-level pointers to standalone and supported struct-field scalar/one-dimensional-array storage (`float`, `long double`, hexadecimal/suffixed/non-finite literals, deeper double pointers, arrays of double pointers, pointer-to-row double forms such as `double (*)[N]`, whole-array or row addresses, union-backed double field addresses/decay, atomic double-pointer aliases, direct one-dimensional double-array parameter declarators, direct and alias-spelled two-dimensional double array parameter declarators, array returns, two-dimensional double aggregate fields/compound literals, three-dimensional double arrays, and raw-memory operations over aggregate-field, two-dimensional, or union-backed double storage) plus complex runtime values; general multiple pointer levels beyond narrow tracked unqualified `char **` objects and output parameters; `void *` dereference, indexing, arithmetic, ordering, deeper pointers, pointer arrays, pointer-to-array forms, and other memory intrinsics; function pointers and variadic function calls; flexible array members and bit-fields; `goto`; general aggregate casts; and native ABI layout/promotion compatibility. The bounded termination intrinsics do not implement `atexit`, stdio flushing, signal delivery, or host-process termination: `exit` and `_Exit` return the supplied status through Cust's library/CLI result surface, while `abort` produces the recoverable `program aborted` interpreter error. Cust executes programs itself; GCC/Clang may be used only as optional test oracles for supported fixtures.

See [CHANGELOG.md](CHANGELOG.md) for current release notes and [docs/v0.1.md](docs/v0.1.md) for the historical v0.1 foundation notes.

## Roadmap

- Near term: continue parser recovery/error-message expansion only for newly discovered malformed programs that are not already covered by exact-output diagnostics tests.
- Next milestone: extend deterministic binary64 object-byte access across all five bounded raw-memory intrinsics to scalar and one-dimensional-array `double` fields in supported non-union structs, with field-local capacity/identity, recursive const/lifetime safety, partial/full typed writes, non-evaluating validation, and exact two-dimensional/union/pointer-field boundaries.
- Product quality: keep release-oriented docs and exact package/Docker/CLI version assertions synchronized.
- Longer term: extend standard-library calls cautiously, then reconsider multiple pointer levels and broader C conformance fixtures.

## License

GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`). See [LICENSE](LICENSE).
