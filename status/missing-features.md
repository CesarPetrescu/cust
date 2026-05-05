# Cust Missing Features

Prioritized backlog for autonomous implementation.

## P0 — correctness and developer trust

- [x] Parser errors with source spans: line, column, token context
- [x] Add token context snippets to lexer errors
- [x] Block scoping rules for variables inside `{ ... }`
- [ ] Better parser recovery/error messages (broad track largely expanded; missing-`=` diagnostics, missing-name/type diagnostics, context-aware unterminated-block diagnostics, array length edge diagnostics, pointer-parameter malformed-list coverage, explicit unsupported pointer-return/pointer-array/pointer-to-pointer diagnostics, delimiter-aware parameter/call trailing-comma diagnostics, and duplicate `switch` case/default label diagnostics are covered; remaining work should be driven by newly discovered malformed programs beyond current exact-error coverage)
- [x] Initial test fixtures for valid and invalid programs
- [x] Improve local Docker test automation for repeatable cron runs

## P1 — C subset expansion

- [x] Function definitions and function calls
- [x] Local function parameters
- [x] Recursive calls with a bounded call-depth limit regression suite
- [x] Boolean/logical operators: `&&`, `||`, `!`
- [x] Unary plus
- [x] `for` loops
- [x] `break` and `continue`
- [x] Empty statements and expression statements
- [x] Assignment expressions for scalar, array-index, and dereferenced pointer lvalues, including right-associative chained assignment and C compiler-oracle coverage
- [x] Conditional operator `?:` with C-style truthiness, right-associative nesting, branch short-circuiting, assignment-expression operands, pointer-truthiness conditions, and compiler-oracle coverage
- [x] `do { ... } while (condition);` loops with guaranteed first body execution, C-style truthiness, `break`/`continue` handling, loop-step budgeting, exact missing-semicolon diagnostics, and compiler-oracle coverage
- [x] Increment/decrement operators (`++`/`--`) or compound assignment (`+=`, `-=`) once selected with precise parser/interpreter acceptance fixtures and C compiler-oracle coverage
- [x] Increment/decrement operators (`++`/`--`) with prefix/postfix semantics for scalar, indexed array/pointer, and dereferenced pointer lvalues plus `for` clause use, non-lvalue diagnostics, and C compiler-oracle coverage
- [x] Bitwise and shift operators: unary `~`, binary `&`, `^`, `|`, `<<`, and `>>` with C precedence, interpreter diagnostics for pointer operands, and C compiler-oracle coverage
- [x] Bitwise/shift compound assignments: `&=`, `|=`, `^=`, `<<=`, and `>>=` as right-associative assignment-precedence expressions/statements for scalar, indexed array/pointer, and dereferenced pointer lvalues, with invalid pointer diagnostics and C compiler-oracle coverage
- [x] Comma operator `left, right` with lowest-precedence parsing, side-effecting left-expression evaluation, right-expression result propagation, pointer/truthiness support, call-argument separator preservation, exact malformed-RHS diagnostic, and C compiler-oracle coverage
- [x] Switch statements with integer/character `case` labels, `default`, C-style fallthrough, `break` consumption, `continue` propagation to enclosing loops, exact missing-colon diagnostics, and C compiler-oracle coverage
- [x] Single-statement control-flow bodies and `else if` chains for `if`/`else`/`while`/`do`/`for`, including nearest-`if` dangling-`else` binding, loop control propagation, and C compiler-oracle coverage
- [x] C-style block comments `/* ... */` treated as lexer whitespace across lines/inline token boundaries, with source-line/caret diagnostics for unterminated block comments and C compiler-oracle coverage
- [x] `void` helper functions and empty `return;` statements, including side-effect-only void calls, int/void return-shape diagnostics, scalar-use diagnostics for void calls, and C compiler-oracle coverage
- [x] Top-level global variables for scalar `int`/`char`, arrays, and supported pointer globals, initialized before `main()` in a persistent outer scope, mutable from helper functions, with duplicate-global diagnostics and C compiler-oracle coverage
- [x] `sizeof` over supported types and expressions, with Cust-defined sizes (`int=8`, `char=1`, pointer `=8`), declared scalar/array/pointer element-type tracking, non-evaluating operand semantics, exact `sizeof(void)` diagnostics, interpreter fixtures, and stable C compiler-oracle coverage for char/string/char-array sizes
- [x] Uninitialized scalar and pointer declarations: `int x;` / `char c;` default to `0`, supported pointer declarations such as `int *p;` default to null, existing array declarations remain zero-initialized, exact missing-`=` diagnostics are preserved for malformed declarations like `int x 1;`, and stable global zero-initialization has C compiler-oracle coverage

## P2 — data types

- [x] `char` literals, declarations, and function parameters
- [x] arrays
- [x] string literals as read-only byte arrays
- [x] pointer model design document before implementation
- [x] first scalar pointer milestone: pointer declarations, scalar address-of/dereference, dereference assignment, reassignment, null diagnostics, and out-of-scope scalar diagnostics
- [x] pointer parameters, array/string decay to pointer arguments, and pointer indexing for array-base pointers
- [x] `&array[index]` array-element pointers and dereference assignment/read support

## P3 — C-subset conformance tooling

- [x] Compare Cust output against `gcc`/`clang`/`cc` for supported programs as an external test oracle only
- [x] Add an initial corpus under `tests/fixtures/`
- [x] Add more conformance fixtures with explicit expected results and optional compiler-oracle comparisons where supported (pointer scalar/parameter/array-decay/array-element/truthiness/equality coverage plus invalid negative pointer-index and unsupported pointer/integer diagnostics have been added; continue expanding mixed supported-subset fixtures as new features land)
- [x] Add fuzz/property tests for lexer/parser safety

## P4 — product quality

- [x] CLI flag: `--tokens`
- [x] CLI flag: `--ast`
- [x] CLI flag: `--max-steps`
- [x] CLI flag: `--version`
- [x] Better README examples
- [x] Release notes and tags (`CHANGELOG.md` and annotated `v0.1` tag)
- [ ] License file (blocked on project-owner license choice)
