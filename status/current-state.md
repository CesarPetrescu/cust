# Cust Current State

Last updated: 2026-05-05

## Repository

- Path: `/root/hermes-workspace/cust`
- Remote: `git@github.com-cust:CesarPetrescu/cust.git`
- Default branch: `main`
- Current version: `v0.1`

## Implemented

- Rust binary + library crate named `cust`
- Tiny C-subset interpreter with pipeline:
  - lexer
  - parser
  - AST
  - interpreter
  - CLI wrapper
- CLI command shape: `cust <file.c>`
- CLI supports `cust --version`, printing the Cargo package version without requiring a source file.
- CLI supports `cust --tokens <file.c>`, printing the lexer token stream with 1-based line/column locations without evaluating the program.
- CLI supports `cust --ast <file.c>`, printing a deterministic parsed AST view without evaluating the program.
- CLI supports `cust --max-steps N <file.c>`, running with an explicit total loop-iteration budget so runaway programs can be bounded from the CLI without changing library defaults.
- Example: `examples/sum.c`
- Docs:
  - `README.md`
  - `CHANGELOG.md`
  - `docs/v0.1.md`
- Docker:
  - `Dockerfile`
  - `docker-compose.yml`
  - safe runtime service with no network, non-root user, read-only FS, dropped capabilities
  - Compose services force source rebuilds with `pull_policy: build` to avoid stale-image test/runtime runs

## Supported language subset

- `int main() { ... }` plus additional `int name(...) { ... }` function definitions
- function calls as expressions with integer arguments, local parameter scopes, direct/mutual recursion support, arity diagnostics, undefined-function diagnostics, and a 64-call-depth safety limit with function-name context
- integer literals, character literals, string literals, variables, and one-dimensional `int`/`char` arrays
- declarations: `int x = expr;`, `char x = expr;`, `int xs[N];`, or `char cs[N];`
- `int` and `char` function parameters (stored as integer values in the current interpreter model)
- one-dimensional array parameters such as `int values[3]`, passed by reference to the same array storage; string literals can be passed to `char` array parameters as read-only NUL-terminated byte arrays
- First-pass scalar pointer support from `docs/plans/pointer-model.md`: `int *p = &x;`, `char *p = &c;`, `p = &y;`, `p = 0;`, `*p` reads, and `*p = expr;` writes through interpreter-owned scalar references. Null dereferences report `null pointer dereference`; pointers to scalar variables whose block/function scope has ended report `pointer to out-of-scope variable '<name>'`.
- Pointer parameters are supported for scalar addresses (`inc(&x)`), array-to-pointer decay (`sum(values)` for `int *`/`char *` parameters), string-literal decay to read-only `char *` arguments, and array-element pointers (`&values[1]` and `&p[1]` when `p` is an array-backed pointer). Pointer indexing `p[i]` reads/writes array-base and array-element pointer storage with deterministic null/read-only/negative/out-of-bounds diagnostics; array-element pointer indexing is relative to the addressed element. Pointer values also participate in C-style truthiness (`if (p)`, `!p`, `&&`, `||`) and equality/inequality comparisons against null, matching scalar targets, and array-decay/array-element targets such as `values == &values[0]`; unsupported pointer arithmetic, pointer ordering, and pointer-vs-nonzero-integer comparisons now report explicit diagnostics.
- assignments: `x = expr;`, `xs[index] = expr;`, pointer reassignment (`p = &x;`/`p = 0;`/`p = &xs[index];`), scalar/array-element dereference assignment (`*p = expr;`), grouped dereference assignment such as `*(&xs[1]) = expr;`, right-associative assignment expressions for scalar, array-index, and dereferenced pointer lvalues such as `return x = 1;`, `xs[0] = (xs[1] = 7);`, and `(*p = 6) != 0`, plus compound assignment expressions/statements `+=` and `-=` for scalar, array-index/pointer-index, and dereferenced pointer lvalues
- conditional operator `cond ? then_expr : else_expr` with C-style truthiness for scalar, array-decay, and pointer-valued conditions; the middle operand is a full expression, nested conditionals are right-associative, and only the selected branch is evaluated
- array indexing expressions `xs[index]`, pointer indexing expressions `p[index]` for array-base or array-element pointers, string literal indexing expressions `"text"[index]`, and scalar/array-element pointer dereference expressions `*p` with runtime negative/out-of-bounds/null/out-of-scope/read-only diagnostics as applicable
- `return expr;`
- nested block statements `{ ... }` with per-block variable scopes, inner shadowing, and outer-scope assignment lookup
- `if (...) { ... } else { ... }`
- `while (...) { ... }`
- `do { ... } while (...);` with guaranteed first body execution, C-style truthiness, `break`/`continue` handling, and the same loop-iteration safety/budget accounting as `while`/`for`
- `for (init; condition; increment) { ... }` with optional clauses, declaration/assignment initializers, assignment increments, loop-local initializer scope, and the shared 1,000,000-iteration safety limit
- `break;` and `continue;` in `while`, `do-while`, and `for` loop bodies, including propagation through nested blocks/conditionals and diagnostics when used outside loops
- empty statements (`;`) and expression statements (`expr;`) in block bodies and C-style `for` initializer/increment clauses
- `+ - * / %`, unary `+`, unary `-`, unary `*` for scalar pointer dereference, and unary `&` for scalar address-of
- `== != < <= > >=`
- logical operators `&&`, `||`, and `!` with C-style integer truth values and short-circuit evaluation for `&&`/`||`
- `//` comments

## Test/tooling coverage

- Cust is an interpreter. The implementation and runtime path must execute via `cust::interpret`/the `cust` CLI. Native compilers (`$CC`, `gcc`, `clang`, or `cc`) are allowed only inside tests as external oracles that compile supported fixtures and compare native exit codes against Cust results; they must not be used as implementation helpers or as Cust's execution engine. `clangd` is editor/LSP-only and is not part of verification.
- `tests/fuzz_safety.rs` adds deterministic generated malformed-program and arbitrary-byte/lossy-UTF-8 smoke properties that assert `cust::interpret` does not panic on lexer/parser/interpreter setup inputs; normal `CustError`s are accepted.

## Diagnostics

- Lexer errors include 1-based line and column plus a source-line/caret context snippet for unexpected characters and out-of-range integer literals.
- Parser errors include 1-based line and column plus token context for expected-token, identifier, expression, statement, and unterminated-block failures.
- Parser diagnostics now include targeted separator messages for malformed function parameter lists and function call argument lists, including missing commas and trailing commas.
- Parser diagnostics now include targeted missing-semicolon messages after variable declarations, array declarations, scalar/indexed assignments, expression statements, and return statements.
- Parser diagnostics now include targeted missing-`]` messages for array declaration lengths, array parameter lengths, indexed assignments, indexed array expressions, and string-literal indexing expressions.
- Parser diagnostics now include targeted missing-`)` messages for grouped expressions, function call arguments, function definition parameters, and `if`/`while`/`for` headers.
- Parser diagnostics now include targeted missing-`{` messages after function headers and `if`/`else`/`while`/`for` control-flow headers.
- Parser diagnostics now include targeted missing-`(` messages after function names and `if`/`while`/`for` keywords, targeted missing-semicolon messages after `break`, `continue`, and `for` conditions, targeted missing-`=` messages after variable/pointer declarations and scalar/indexed/dereference assignments, targeted missing-name/type messages for function names, variable/pointer declarations, and parameter lists, unmatched closing delimiter messages for stray `)`/`]` in statements and extra `}` at top level, context-aware unterminated-block messages (for example after a function header or `if` condition), explicit empty-array-length diagnostics before `]`, negative array-length diagnostics, explicit rejection of `break`/`continue` in non-body `for` clauses, pointer-parameter malformed-list coverage, explicit unsupported pointer-return/pointer-array declaration/parser diagnostics, explicit unsupported pointer-to-pointer parameter/declaration diagnostics, and delimiter-aware trailing-comma diagnostics for function parameter/call lists.

## Verified commands

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-05 autonomous compound-assignment run. This run added lexer/parser/interpreter support for `+=` and `-=` as right-associative compound assignment expressions/statements over scalar, indexed array/pointer, and dereferenced pointer lvalues; compound assignments return the assigned scalar value and unsupported pointer arithmetic remains explicit. Regression coverage includes valid and invalid interpreter fixtures plus a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: the autonomous do-while run added lexer/parser/interpreter support for `do { ... } while (...);`, including guaranteed first execution, `break`/`continue` behavior, shared loop-step budgeting, exact missing-semicolon diagnostics after the do-while condition, and a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: the autonomous conditional-operator run added lexer/parser/interpreter support for `?:`, including branch short-circuiting, pointer-truthiness conditions, assignment-expression operands, a missing-colon exact diagnostic, and a C compiler-oracle fixture.

Previous verified state: the autonomous assignment-expressions run added right-associative assignment expressions for scalar variables, array/pointer-index lvalues, and dereferenced pointer lvalues, plus an exact invalid-lvalue diagnostic (`invalid assignment target`) and a C compiler-oracle fixture. Parser index expressions intentionally remain delimiter-oriented so malformed `array[0 = ...` continues to report the established missing-`]` diagnostic.

Previous verified state: the autonomous parser-diagnostics continuation run added exact parser coverage for unsupported pointer-to-pointer parameters/declarations and delimiter-aware missing function parameter/call argument diagnostics after commas before `{`/`;`. Unsupported `int **param` now reports `pointer-to-pointer parameters are not supported`, unsupported local `int **value` reports `pointer-to-pointer declarations are not supported`, and malformed lists such as `int f(int x, { ... }` or `call(1,;` now report the missing parameter/argument at the delimiter instead of falling through to generic type/expression errors.

Previous verified state: the autonomous pointer-parser diagnostics run added exact parser coverage for malformed pointer-specific unsupported forms and list boundaries: pointer return types, pointer array parameters/declarations, missing pointer parameter names after `*`, missing commas after pointer parameters, and trailing commas after pointer parameters. Unsupported pointer return types now report `pointer return types are not supported`, while unsupported pointer arrays in parameter/declaration grammar report `pointer array parameters are not supported` / `pointer array declarations are not supported` at the offending `[` token.

Previous verified state: the repository includes v0.1 release notes in `CHANGELOG.md`, an updated README release-notes link/current roadmap, and refreshed `docs/v0.1.md` implementation notes that include arrays, strings, and the current safe pointer subset instead of the older no-pointer limitation. The parser reports exact contextual diagnostics for EOF inside unterminated blocks (`unterminated block after ...`), empty array lengths before `]`, and negative array lengths in declarations/parameters, in addition to missing function names after return types, missing variable names after declaration types, missing pointer names after `*`, missing parameter names after types, and missing parameter types before parameter names, while preserving valid expression statements. The interpreter also reports explicit errors for unsupported pointer arithmetic (`pointer arithmetic is not supported`), pointer ordering comparisons (`pointer ordering comparisons are not supported`), pointer-vs-nonzero-integer equality/inequality (`cannot compare pointer with nonzero integer`), and negative pointer-array indices. The suite includes `tests/c_compat.rs`, which compiles supported fixtures with a native C compiler only as an oracle and compares native exit codes to Cust interpreted results, including pointer scalar, pointer parameter, array-decay, array-element pointer, pointer truthiness/equality, and mixed pointer/string/array fixtures. It also includes deterministic fuzz/property-style safety tests for generated malformed source and arbitrary bytes decoded lossily to UTF-8. CLI integration tests use per-process atomic temp-file suffixes so parallel Docker test runs cannot collide on temporary source paths.

## Operating rule for autonomous agent

The autonomous agent must keep this `status/` directory current every run, even if no code changes are made.
