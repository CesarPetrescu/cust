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
- [x] Function prototypes/declarations at top level (`int helper(int);`, `void mutate(int *);`) with semicolon-terminated parser support, signature compatibility checks against definitions/earlier declarations, invalid conflicting-prototype diagnostics, and C compiler-oracle coverage
- [x] `char` function return types and prototypes (`char first(char *text);`) with scalar return-shape diagnostics, `sizeof` call-size support, interpreter fixtures, and C compiler-oracle coverage
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

- [x] Enum constant declarations: `enum Tag { A = 1, B, C = -1 };` at top level and inside blocks, optional tags, implicit incrementing values, trailing commas, scoped/shadowable integer constants, read-only assignment diagnostics for enum constants, malformed-value diagnostics, and C compiler-oracle coverage
- [x] Preprocessor-free struct first milestone: top-level `struct Name { int x; char y; };` declarations, top-level/local zero-initialized `struct Name value;` variables, scalar member reads/writes with `.`, targeted unknown-field diagnostics, deterministic Cust `sizeof` for structs/fields, design notes in `docs/plans/struct-model.md`, and C compiler-oracle coverage
- [x] Struct copy assignment and field lvalue expressions: same-type `b = a;` value-copy semantics with mismatched-type diagnostics, field assignment expressions (`return p.x = 3;`), field compound assignments (`p.x += 1` and related supported operators), prefix/postfix field increment/decrement, interpreter fixtures, invalid diagnostics, and C compiler-oracle coverage
- [x] Struct by-value function parameters: parser accepts `struct Point p` parameters in definitions/prototypes after prior struct declarations, calls copy same-type struct arguments into callee scope, callee field writes do not mutate caller structs, mismatched/non-struct arguments report targeted diagnostics, and interpreter plus C compiler-oracle fixtures cover the supported subset
- [x] Struct return types: parser accepts `struct Point make(...)` definitions/prototypes after a prior struct declaration, returns clone same-type struct values by value so local return variables remain valid after callee exit, struct-returning calls can be assigned to same-type struct variables, mismatched/empty struct returns report targeted diagnostics, `sizeof(struct_return_call())` uses deterministic Cust struct sizes, and interpreter plus C compiler-oracle fixtures cover the supported subset
- [x] Struct pointers and `->`: parser accepts one-level `struct Point *p` declarations and parameters/prototypes after prior struct declarations, `&point` creates interpreter-owned struct pointers, `p->x` and `(*p).x` read/write scalar fields, field assignment/compound/increment lvalues work through pointers, null/out-of-scope diagnostics are targeted, and interpreter plus C compiler-oracle fixtures cover the supported subset
- [x] Top-level and block-scoped `typedef` aliases: `typedef int Count;`, `typedef char Byte;`, `typedef struct Point Point;`, named enum aliases such as `typedef enum Status Status;` after a prior named enum declaration, and one-level pointer aliases such as `typedef int *IntPtr;`, `typedef char *CharPtr;`, and `typedef struct Point *PointPtr;` after a prior struct declaration resolve at parse time without runtime storage changes; aliases work in globals, locals, arrays, one-level pointer declarations, function prototypes/definitions/parameters/returns, scalar struct fields, pointer-alias declarations/parameters, enum-alias declarations/parameters/returns, and `sizeof(alias)` / `sizeof(pointer_alias)`, with block-local alias and enum-tag shadowing/scope expiry, exact malformed typedef diagnostics including pointer-to-pointer alias rejection, unknown enum tag rejection, and block enum tag expiry, and C compiler-oracle coverage
- [x] First-pass `const` qualifiers for leading `const int` / `const char` scalar declarations, zero-initialized arrays, and scalar function parameters; const variables remain readable in normal expressions but direct assignment, compound assignment, increment/decrement, parameter mutation, and writes through scalar pointers are rejected, while const arrays use read-only array storage and reject indexed/pointer writes with interpreter plus C compiler-oracle coverage
- [x] `sizeof(const type)` parsing for supported scalar, typedef-alias, and pointer type forms; acceptance: `sizeof(const int) == sizeof(int)`, `sizeof(const char) == sizeof(char)`, aliases such as `Count`/`Pair` preserve their underlying size, pointer spellings such as `sizeof(const int *)` and pointer typedef aliases report deterministic pointer size, `sizeof(const void)` preserves the targeted void diagnostic, and native compiler-oracle coverage avoids ABI-sensitive exact sizes
- [x] Const-qualified pointer semantics beyond size/type contexts; acceptance: `const int *p` / `const char *p` create read-only pointer views that reject writes through `*p`, `p[i]`, compound assignment, and increment/decrement while allowing pointer reassignment; `int * const p` / `char * const p` create non-reassignable pointer slots that may still mutate mutable targets; `const int * const p` combines both; parser/interpreter metadata is documented in `docs/plans/const-pointer-model.md`; valid/invalid interpreter fixtures and C compiler-oracle coverage verify the supported subset

## P2 — data types

- [x] `char` literals, declarations, and function parameters
- [x] arrays
- [x] string literals as read-only byte arrays
- [x] pointer model design document before implementation
- [x] first scalar pointer milestone: pointer declarations, scalar address-of/dereference, dereference assignment, reassignment, null diagnostics, and out-of-scope scalar diagnostics
- [x] pointer parameters, array/string decay to pointer arguments, and pointer indexing for array-base pointers
- [x] `&array[index]` array-element pointers and dereference assignment/read support
- [x] Scoped array-backed pointer arithmetic: `p + n`, `n + p`, `p - n`, same-storage pointer difference, `p += n`, `p -= n`, and pointer-variable `++`/`--` with scalar/null/out-of-bounds diagnostics plus interpreter and C compiler-oracle fixtures

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
