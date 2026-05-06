# Cust Current State

Last updated: 2026-05-06

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

- Top-level `int`/`char` scalar globals, one-dimensional array globals, supported pointer globals, top-level/local enum constant declarations with optional tags, explicit integer values, implicit incrementing values, trailing commas, and block-scope shadowing, plus first-pass preprocessor-free `struct` type declarations (`struct Point { int x; char y; };`) and top-level/local zero-initialized struct variables with scalar member reads/writes, plus `int main() { ... }` and additional `int name(...) { ... }` / `char name(...) { ... }` / `void name(...) { ... }` function definitions
- function calls as expressions with integer arguments for `int`/`char` functions, side-effect-only expression statements for `void` functions, local parameter scopes, direct/mutual recursion support, top-level function prototypes such as `int helper(int value);` / `char first(char *text);` / `void mutate(int *slot);` with signature compatibility checks, arity diagnostics, undefined-function diagnostics, empty `return;` support for `void` functions, diagnostics for value returns from `void` functions / empty returns from scalar-returning functions / scalar use of `void` calls, `sizeof` on function calls respecting `int` vs `char` return sizes, and a 64-call-depth safety limit with function-name context
- integer literals, character literals, string literals, variables, and one-dimensional `int`/`char` arrays
- declarations: top-level or local initialized `int x = expr;` / `char x = expr;`, default-initialized scalar `int x;` / `char x;`, supported pointer declarations such as `int *p = &x;` or `int *p;` (defaulting to null), zero-initialized arrays `int xs[N];` / `char cs[N];`, and enum constant declarations such as `enum State { READY = 1, RUNNING };`, with globals initialized before `main()` and visible/mutable from helper functions
- `int` and `char` function parameters and `char` function return types (stored as integer values in the current interpreter model)
- one-dimensional array parameters such as `int values[3]`, passed by reference to the same array storage; string literals can be passed to `char` array parameters as read-only NUL-terminated byte arrays
- First-pass scalar pointer support from `docs/plans/pointer-model.md`: `int *p = &x;`, `char *p = &c;`, `p = &y;`, `p = 0;`, `*p` reads, and `*p = expr;` writes through interpreter-owned scalar references. Null dereferences report `null pointer dereference`; pointers to scalar variables whose block/function scope has ended report `pointer to out-of-scope variable '<name>'`.
- Pointer parameters are supported for scalar addresses (`inc(&x)`), array-to-pointer decay (`sum(values)` for `int *`/`char *` parameters), string-literal decay to read-only `char *` arguments, and array-element pointers (`&values[1]` and `&p[1]` when `p` is an array-backed pointer). Pointer indexing `p[i]` reads/writes array-base and array-element pointer storage with deterministic null/read-only/negative/out-of-bounds diagnostics; array-element pointer indexing is relative to the addressed element. Array-backed pointer arithmetic is supported for `p + n`, `n + p`, `p - n`, pointer difference between two pointers to the same array/string storage, `p += n`, `p -= n`, and pointer-variable `++`/`--`, with bounds checks and explicit null/scalar/different-array diagnostics. Pointer values also participate in C-style truthiness (`if (p)`, `!p`, `&&`, `||`) and equality/inequality comparisons against null, matching scalar targets, and array-decay/array-element targets such as `values == &values[0]`; unsupported pointer ordering and pointer-vs-nonzero-integer comparisons now report explicit diagnostics.
- assignments: `x = expr;`, `xs[index] = expr;`, pointer reassignment (`p = &x;`/`p = 0`/`p = &xs[index]`/`p = p + n`), scalar/array-element dereference assignment (`*p = expr;`), grouped dereference assignment such as `*(&xs[1]) = expr;`, right-associative assignment expressions for scalar, array-index, and dereferenced pointer lvalues such as `return x = 1;`, `xs[0] = (xs[1] = 7);`, and `(*p = 6) != 0`, compound assignment expressions/statements `+=`, `-=`, `&=`, `|=`, `^=`, `<<=`, and `>>=` for scalar, array-index/pointer-index, and dereferenced pointer lvalues, `+=`/`-=` for array-backed pointer variables, plus prefix/postfix increment/decrement expressions/statements (`++x`, `x++`, `--x`, `x--`) for scalar, array-index/pointer-index, dereferenced pointer lvalues, and array-backed pointer variables
- comma operator `left, right` at the lowest expression precedence, evaluating the left expression for side effects and yielding the right expression; supported in grouped expressions, conditions/truthiness contexts, `for` clauses, pointer-valued expressions, and call arguments (where top-level commas still separate arguments)
- conditional operator `cond ? then_expr : else_expr` with C-style truthiness for scalar, array-decay, and pointer-valued conditions; the middle operand is a full expression, nested conditionals are right-associative, and only the selected branch is evaluated
- array indexing expressions `xs[index]`, pointer indexing expressions `p[index]` for array-base or array-element pointers, string literal indexing expressions `"text"[index]`, and scalar/array-element pointer dereference expressions `*p` with runtime negative/out-of-bounds/null/out-of-scope/read-only diagnostics as applicable
- `sizeof` expressions for supported Cust types and expressions: `sizeof(int)`, `sizeof(char)`, `sizeof(int *)`/`sizeof(char *)`, scalar variables, arrays (using declared element type and length), pointer variables/address-of expressions, first-pass struct variables and scalar struct fields (using deterministic Cust field-size sums without native ABI padding), string literals (including the NUL terminator), and indexed string/array/pointer expressions. Cust defines `sizeof(int) == 8`, `sizeof(char) == 1`, and pointer size `8`; `sizeof(void)` is rejected with a targeted parser diagnostic.
- `return expr;`
- nested block statements `{ ... }` with per-block variable scopes, inner shadowing, and outer-scope assignment lookup
- `if (...) statement else statement` with braced blocks, single-statement control bodies, `else if` chains, and C dangling-`else` binding to the nearest unmatched `if`
- `while (...) statement` with braced blocks or single-statement bodies
- `do statement while (condition);` with braced blocks or single-statement bodies, guaranteed first body execution, C-style truthiness, `break`/`continue` handling, and the same loop-iteration safety/budget accounting as `while`/`for`
- `switch (expression) { case constant: ... default: ... }` statements with integer/character case labels, optional default labels, C-style fallthrough, `break` consumption at the switch boundary, and `continue` propagation to enclosing loops
- `for (init; condition; increment) statement` with braced blocks or single-statement bodies, optional clauses, declaration/assignment initializers, assignment increments, loop-local initializer scope, and the shared 1,000,000-iteration safety limit
- `break;` and `continue;` in `while`, `do-while`, and `for` loop bodies, including propagation through nested blocks/conditionals and diagnostics when used outside loops
- empty statements (`;`) and expression statements (`expr;`) in block bodies and C-style `for` initializer/increment clauses
- `+ - * / %`, unary `+`, unary `-`, unary `~`, unary `*` for scalar pointer dereference, unary `&` for scalar/array-element address-of, bitwise `&`, `^`, `|`, and shifts `<<`/`>>` with C precedence
- `== != < <= > >=`
- logical operators `&&`, `||`, and `!` with C-style integer truth values and short-circuit evaluation for `&&`/`||`
- comments: `//` line comments and `/* ... */` block comments; unterminated block comments report a lexer source-line/caret diagnostic at the opening `/*`.

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
- Parser diagnostics now include targeted missing-`(` messages after function names and `if`/`while`/`for` keywords, targeted missing-semicolon messages after `break`, `continue`, and `for` conditions, targeted missing-`=` messages after variable/pointer declarations and scalar/indexed/dereference assignments, targeted missing-name/type messages for function names, variable/pointer declarations, and parameter lists, unmatched closing delimiter messages for stray `)`/`]` in statements and extra `}` at top level, context-aware unterminated-block messages (for example after a function header or `if` condition), explicit empty-array-length diagnostics before `]`, negative array-length diagnostics, explicit rejection of `break`/`continue` in non-body `for` clauses, pointer-parameter malformed-list coverage, explicit unsupported pointer-return/pointer-array/parser diagnostics, explicit unsupported pointer-to-pointer parameter/declaration diagnostics, delimiter-aware trailing-comma diagnostics for function parameter/call lists, and duplicate `switch` case/default label diagnostics.

## Verified commands

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-06 autonomous struct-first-milestone run. This run added `docs/plans/struct-model.md`, lexer/parser/interpreter support for top-level scalar-field struct declarations, top-level/local zero-initialized struct variables, scalar member reads/writes with `.`, deterministic `sizeof` for struct variables/fields, an exact unknown-field diagnostic, and interpreter plus native C compiler-oracle fixtures. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous pointer-arithmetic run. This run added a scoped array-backed pointer arithmetic milestone: `p + n`, `n + p`, `p - n`, pointer difference for pointers to the same array/string storage, `p += n`, `p -= n`, and pointer-variable `++`/`--`, while preserving explicit diagnostics for scalar/null/different-array/out-of-bounds cases. Coverage includes a valid interpreter fixture, invalid scalar-pointer and out-of-bounds fixtures, and a native C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous char-return-function run. This run added parser/interpreter support for `char name(...)` function definitions and prototypes, preserved scalar return-shape diagnostics for empty returns from `char` functions, made `sizeof(char_return_call())` report Cust's char size, and added interpreter plus native C compiler-oracle coverage. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous function-prototype run. This run added parser support for top-level function prototypes ending in `;`, signature compatibility checks against later definitions or earlier declarations, valid interpreter coverage for `int`, `void`, pointer, and string-decay prototype signatures, an invalid conflicting-prototype diagnostic, and native C compiler-oracle coverage for the supported prototype subset. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous enum-constant run. This run added lexer/parser/interpreter support for C-style enum constant declarations (`enum Tag { A = 1, B, C = -1 };`) at top level and inside blocks, with optional tags, implicit incrementing values, trailing commas, scoped/shadowable integer constants, duplicate-name diagnostics within an enum declaration, read-only assignment diagnostics for enum constants, and native C compiler-oracle coverage. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous uninitialized-declaration run. This run added parser/interpreter support for declarations without explicit initializers: scalar `int`/`char` declarations default to `0`, supported pointer declarations default to null, and existing array declarations remain zero-initialized. Coverage includes an interpreter fixture for local/global scalar and pointer defaults plus a native C compiler-oracle fixture for stable global zero initialization. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous `sizeof` run. This run added lexer/parser/interpreter support for `sizeof` over supported Cust types and expressions, introduced declared scalar/array/pointer element-type tracking for size queries, defined Cust sizes as `int=8`, `char=1`, and pointer `=8`, rejected `sizeof(void)` with an exact diagnostic, and added interpreter plus stable native C compiler-oracle fixture coverage for char/string/char-array sizes. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous global-variable run. This run added parser/interpreter support for top-level `int`/`char` scalar globals, array globals, and pointer globals; globals are initialized in a persistent outer scope before `main()` and can be read/written by helper functions. Regression coverage includes a valid interpreter fixture, duplicate-global invalid fixture, and native C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous void-function run. This run added lexer/parser/interpreter support for `void` helper functions, `return;`, side-effect-only void call statements, and diagnostics for returning a value from a void function, returning no value from an int function, and using a void function call as a scalar expression. Regression coverage includes valid/invalid interpreter fixtures and a native C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous block-comment run. This run added lexer support for C-style `/* ... */` block comments as whitespace across lines and inline token boundaries, plus source-line/caret diagnostics for unterminated block comments. Regression coverage includes valid/invalid interpreter fixtures and a native C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous duplicate-switch-label diagnostic run. This run added parser validation for duplicate `case` constants and duplicate `default` labels inside a `switch`, reporting exact source locations before interpretation. Regression coverage includes focused exact-output interpreter tests plus invalid fixtures for duplicate case/default labels. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous single-statement control-body run. This run added parser/interpreter support for braced or single-statement bodies after `if`/`else`/`while`/`do`/`for`, including `else if` chains and nearest-`if` dangling-`else` binding. Regression coverage includes focused interpreter tests, a valid fixture covering loops/continue/break/do-while/else-if, and a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous switch-statement run. This run added lexer/parser/interpreter support for `switch (expr) { case constant: ... default: ... }`, including C-style fallthrough, `break` consumption at the switch boundary, `continue` propagation to enclosing loops, exact missing-colon diagnostics after `case` labels, and a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous comma-operator run. This run added comma operator parsing/evaluation at the lowest expression precedence, with left-to-right side-effect evaluation and right-expression result propagation for scalar, pointer, and truthiness contexts. Regression coverage includes a valid interpreter fixture covering assignments, pointers, loops, and call-argument separation, an invalid missing-RHS exact parser diagnostic, and a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous bitwise-compound-assignment run. This run added lexer/parser/interpreter support for `&=`, `|=`, `^=`, `<<=`, and `>>=` as right-associative assignment-precedence expressions/statements over scalar, indexed array/pointer, and dereferenced pointer lvalues. Regression coverage includes a valid interpreter fixture, invalid pointer-bitwise-compound and shift-count diagnostic fixtures, and a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous bitwise-operator run. This run added lexer/parser/interpreter support for unary bitwise complement `~`, binary bitwise `&`, `^`, `|`, and shifts `<<`/`>>` with C precedence (`shift` between additive and comparison, bitwise tiers between equality and logical-and). Regression coverage includes valid interpreter fixtures covering precedence and array/pointer-index scalar operands, an invalid pointer-bitwise diagnostic fixture, and a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous increment/decrement run. This run added lexer/parser/interpreter support for prefix and postfix `++`/`--` over scalar variables, indexed array/pointer elements, and dereferenced pointer lvalues; prefix expressions return the updated value while postfix expressions return the prior value, and `for`/`while` clause usage is covered. Regression coverage includes valid and invalid interpreter fixtures plus a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-05 autonomous compound-assignment run. This run added lexer/parser/interpreter support for `+=` and `-=` as right-associative compound assignment expressions/statements over scalar, indexed array/pointer, and dereferenced pointer lvalues; compound assignments return the assigned scalar value and unsupported pointer arithmetic remains explicit. Regression coverage includes valid and invalid interpreter fixtures plus a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: the autonomous do-while run added lexer/parser/interpreter support for `do { ... } while (...);`, including guaranteed first execution, `break`/`continue` behavior, shared loop-step budgeting, exact missing-semicolon diagnostics after the do-while condition, and a C compiler-oracle fixture. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: the autonomous conditional-operator run added lexer/parser/interpreter support for `?:`, including branch short-circuiting, pointer-truthiness conditions, assignment-expression operands, a missing-colon exact diagnostic, and a C compiler-oracle fixture.

Previous verified state: the autonomous assignment-expressions run added right-associative assignment expressions for scalar variables, array/pointer-index lvalues, and dereferenced pointer lvalues, plus an exact invalid-lvalue diagnostic (`invalid assignment target`) and a C compiler-oracle fixture. Parser index expressions intentionally remain delimiter-oriented so malformed `array[0 = ...` continues to report the established missing-`]` diagnostic.

Previous verified state: the autonomous parser-diagnostics continuation run added exact parser coverage for unsupported pointer-to-pointer parameters/declarations and delimiter-aware missing function parameter/call argument diagnostics after commas before `{`/`;`. Unsupported `int **param` now reports `pointer-to-pointer parameters are not supported`, unsupported local `int **value` reports `pointer-to-pointer declarations are not supported`, and malformed lists such as `int f(int x, { ... }` or `call(1,;` now report the missing parameter/argument at the delimiter instead of falling through to generic type/expression errors.

Previous verified state: the autonomous pointer-parser diagnostics run added exact parser coverage for malformed pointer-specific unsupported forms and list boundaries: pointer return types, pointer array parameters/declarations, missing pointer parameter names after `*`, missing commas after pointer parameters, and trailing commas after pointer parameters. Unsupported pointer return types now report `pointer return types are not supported`, while unsupported pointer arrays in parameter/declaration grammar report `pointer array parameters are not supported` / `pointer array declarations are not supported` at the offending `[` token.

Previous verified state: the repository includes v0.1 release notes in `CHANGELOG.md`, an updated README release-notes link/current roadmap, and refreshed `docs/v0.1.md` implementation notes that include arrays, strings, and the current safe pointer subset instead of the older no-pointer limitation. The parser reports exact contextual diagnostics for EOF inside unterminated blocks (`unterminated block after ...`), empty array lengths before `]`, and negative array lengths in declarations/parameters, in addition to missing function names after return types, missing variable names after declaration types, missing pointer names after `*`, missing parameter names after types, and missing parameter types before parameter names, while preserving valid expression statements. The interpreter also reports explicit errors for unsupported pointer arithmetic (`pointer arithmetic is not supported`), pointer ordering comparisons (`pointer ordering comparisons are not supported`), pointer-vs-nonzero-integer equality/inequality (`cannot compare pointer with nonzero integer`), and negative pointer-array indices. The suite includes `tests/c_compat.rs`, which compiles supported fixtures with a native C compiler only as an oracle and compares native exit codes to Cust interpreted results, including pointer scalar, pointer parameter, array-decay, array-element pointer, pointer truthiness/equality, and mixed pointer/string/array fixtures. It also includes deterministic fuzz/property-style safety tests for generated malformed source and arbitrary bytes decoded lossily to UTF-8. CLI integration tests use per-process atomic temp-file suffixes so parallel Docker test runs cannot collide on temporary source paths.

## Operating rule for autonomous agent

The autonomous agent must keep this `status/` directory current every run, even if no code changes are made.
