# Cust Current State

Last updated: 2026-05-07

## Repository

- Path: `/root/hermes-workspace/cust`
- Remote: `git@github.com-cust:CesarPetrescu/cust.git`
- Default branch: `main`
- Current version: `v0.1`
- License: GNU Affero General Public License v3.0 or later (`AGPL-3.0-or-later`), documented in `LICENSE`, `Cargo.toml`, and `README.md`.

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
  - `LICENSE`
  - `docs/v0.1.md`
- Docker:
  - `Dockerfile`
  - `docker-compose.yml`
  - safe runtime service with no network, non-root user, read-only FS, dropped capabilities
  - Compose services force source rebuilds with `pull_policy: build` to avoid stale-image test/runtime runs

## Supported language subset

- Top-level `static` storage-class specifiers are accepted for supported global variables, function prototypes, and function definitions (linkage remains irrelevant inside Cust's single-file interpreter); local `static` declarations for supported scalar, pointer, array, and struct locals initialize once and persist with interpreter-owned static lifetime while keeping lexical/block visibility.
- Top-level/local supported aggregate declarations can use brace initializers, same-type aggregate-returning call expressions such as `struct Point p = make_point(1, 2);` and `union Number n = make_number(5);`, or aggregate-valued conditional/comma expressions such as `struct Point p = flag ? a : b;` and `union Number n = (side_effect(), right);`, preserving branch short-circuiting, by-value copies, and const binding enforcement after initialization.
- Top-level `int`/`char` scalar globals, one-dimensional array globals, supported pointer globals (including one-level pointers to supported structs after the struct type is declared), leading `const int` / `const char` scalar and array globals/locals/parameters for a first-pass read-only qualifier milestone, top-level/local enum constant declarations with optional tags, explicit integer values, implicit incrementing values, trailing commas, and block-scope shadowing, top-level and block-scoped `typedef` aliases for `int`, `char`, prior named `struct` types, prior named enum tags as integer aliases, and one-level scalar/struct pointer aliases (usable in globals/locals/arrays/pointers/function signatures/`sizeof` without changing runtime storage, with inner-block shadowing and scope expiry), plus preprocessor-free `struct` type declarations (`struct Point { int x; char y; };`, `struct Packet { int values[3]; char tag[2]; };`, `struct Rect { struct Point origin; int width; };`, `struct Node { int value; struct Node *next; int *external; };`) and top-level/local zero-initialized or brace-initialized struct variables and one-dimensional arrays of supported structs (`struct Point points[2] = {{1, 2}, {3, 4}};`) with scalar/array/nested/pointer member reads/writes, same-type copy assignment including array and nested struct field copies plus C-style pointer-value field copies, scalar and array-element field lvalue expressions, struct-array element field lvalues such as `points[i].x += 1`, by-value struct function parameters including array/nested struct-field and struct-array-element arguments, by-value struct function return types, and struct pointer declarations/parameters, plus `int main() { ... }` / `int main(void) { ... }` and additional `int name(...) { ... }` / `char name(...) { ... }` / `void name(...) { ... }` / `struct Name name(...) { ... }` function definitions
- function calls as expressions with scalar, pointer, and supported struct arguments; local parameter scopes including by-value `struct Name param` and by-reference `struct Name *param` parameters after a prior struct declaration; C-style empty `void` parameter lists in definitions and prototypes such as `int main(void)` / `int helper(void);`; direct/mutual recursion support, top-level function prototypes such as `int helper(int value);` / `char first(char *text);` / `void mutate(int *slot);` / `int sum(struct Point p);` / `void set(struct Point *p);` / `struct Point make_point(int x);` with signature compatibility checks, arity diagnostics, undefined-function diagnostics, empty `return;` support for `void` functions, diagnostics for value returns from `void` functions / empty returns from scalar- or struct-returning functions / scalar use of `void` or struct calls, targeted mismatched/non-struct argument diagnostics for struct parameters, targeted mismatched struct-return diagnostics, `sizeof` on function calls respecting `int` vs `char` return sizes and deterministic Cust struct sizes, and a 64-call-depth safety limit with function-name context
- integer literals, character literals, string literals, variables, and one-dimensional `int`/`char` arrays
- declarations: top-level or local initialized `int x = expr;` / `char x = expr;`, default-initialized scalar `int x;` / `char c;`, first-pass const-qualified scalar declarations such as `const int limit = 5;` / `const char marker = 'A';`, supported pointer declarations such as `int *p = &x;`, `int *p;` (defaulting to null), or pointer-typedef declarations such as `IntPtr p = &x;` after `typedef int *IntPtr;`, persistent local `static` declarations for supported scalar/pointer/array/struct forms such as `static int counter = 0;`, `static int values[3] = {1, 2};`, and `static struct Point point = {3, 4};`, zero-initialized or brace-initialized arrays `int xs[N];` / `char cs[N];` / `int xs[N] = {expr, ...};` including designated entries such as `int xs[4] = {[2] = 5, [0] = 1};`, read-only const arrays such as `const int table[N] = {8, 9};`, one-dimensional supported struct arrays such as `struct Point points[3] = {{1, 2}, {3, 4}};` with omitted trailing elements zero-filled, enum constant declarations such as `enum State { READY = 1, RUNNING };`, scalar, array-field, nested struct, pointer-field, and designated struct brace initializers and path designators such as `struct Point p = {1, 2};` / `struct Point p = {.y = 2, .x = 1};` / `struct Packet packet = {.values = {[1] = 2}, .anchor = {.y = 4, .x = 3}};` / `struct Packet packet = {.anchor.x = 3, .values[1] = 2};` / `struct Node head = {3, &tail, 0};` / `const struct Config c = {7, 8};` with declaration-order or field-designated initialization, zero-filled omitted trailing fields/elements/pointers defaulting to null, and typedef aliases such as `typedef int Count;`, `typedef char Byte;`, `typedef struct Point Point;`, `typedef enum State State;`, and one-level pointer aliases such as `typedef int *IntPtr;` / `typedef struct Point *PointPtr;` at top level or in block scopes, with inner aliases shadowing outer aliases until block exit; globals initialize before `main()` and remain visible/mutable from helper functions
- `int` and `char` function parameters and `char` function return types (stored as integer values in the current interpreter model)
- one-dimensional array parameters such as `int values[3]`, passed by reference to the same array storage; string literals can be passed to `char` array parameters as read-only NUL-terminated byte arrays
- First-pass scalar pointer support from `docs/plans/pointer-model.md`: `int *p = &x;`, `char *p = &c;`, `p = &y;`, `p = 0;`, `*p` reads, and `*p = expr;` writes through interpreter-owned scalar references. Null dereferences report `null pointer dereference`; pointers to scalar variables whose block/function scope has ended report `pointer to out-of-scope variable '<name>'`.
- Pointer parameters are supported for scalar addresses (`inc(&x)`), struct addresses (`set(&point)` for `struct Point *` parameters), struct-array element addresses (`set(&points[i])`), array-to-pointer decay (`sum(values)` for `int *`/`char *` parameters), string-literal decay to read-only `char *` arguments, and array-element pointers (`&values[1]` and `&p[1]` when `p` is an array-backed pointer). Pointer indexing `p[i]` reads/writes array-base and array-element pointer storage with deterministic null/read-only/negative/out-of-bounds diagnostics; array-element pointer indexing is relative to the addressed element. Struct pointers support `p->field` and `(*p).field` scalar field reads/writes plus field lvalue assignment, compound assignment, and increment/decrement; null struct pointers report `null pointer dereference` and ended-scope targets report `pointer to out-of-scope variable '<name>'`. Scalar struct fields can be addressed with `&point.x`, nested field paths such as `&packet.anchor.y`, and struct-array element fields such as `&points[i].x`; dereferencing those pointers aliases the original field storage and preserves const-discard diagnostics. Array-backed pointer arithmetic is supported for `p + n`, `n + p`, `p - n`, pointer difference between two pointers to the same array/string storage, `p += n`, `p -= n`, and pointer-variable `++`/`--`, with bounds checks and explicit null/scalar/different-array diagnostics. Pointer values also participate in C-style truthiness (`if (p)`, `!p`, `&&`, `||`) and equality/inequality comparisons against null, matching scalar/struct targets, and array-decay/array-element targets such as `values == &values[0]`; unsupported pointer ordering and pointer-vs-nonzero-integer comparisons now report explicit diagnostics.
- assignments: `x = expr;`, `xs[index] = expr;`, same-type struct copy assignment (`b = a;`) with value semantics, struct field assignment statements and expressions (`p.x = expr;` / `return p.x = 3;`), pointer reassignment (`p = &x;`/`p = 0`/`p = &xs[index]`/`p = p + n`), scalar/array-element dereference assignment (`*p = expr;`), grouped dereference assignment such as `*(&xs[1]) = expr;`, right-associative assignment expressions for scalar, struct-field, array-index, and dereferenced pointer lvalues such as `return x = 1;`, `xs[0] = (xs[1] = 7);`, and `(*p = 6) != 0`, compound assignment expressions/statements `+=`, `-=`, `&=`, `|=`, `^=`, `<<=`, and `>>=` for scalar, struct-field, array-index/pointer-index, and dereferenced pointer lvalues, `+=`/`-=` for array-backed pointer variables, plus prefix/postfix increment/decrement expressions/statements (`++x`, `x++`, `--x`, `x--`) for scalar, struct-field, array-index/pointer-index, dereferenced pointer lvalues, and array-backed pointer variables; const-qualified scalar/parameter bindings reject direct assignment, assignment expressions, compound assignment, increment/decrement, and scalar pointer writes, while const arrays reject indexed/pointer writes via read-only storage
- comma operator `left, right` at the lowest expression precedence, evaluating the left expression for side effects and yielding the right expression; supported in grouped expressions, conditions/truthiness contexts, `for` clauses, pointer-valued expressions, and call arguments (where top-level commas still separate arguments)
- conditional operator `cond ? then_expr : else_expr` with C-style truthiness for scalar, array-decay, and pointer-valued conditions; the middle operand is a full expression, nested conditionals are right-associative, and only the selected branch is evaluated
- array indexing expressions `xs[index]`, pointer indexing expressions `p[index]` for array-base or array-element pointers, string literal indexing expressions `"text"[index]`, and scalar/array-element pointer dereference expressions `*p` with runtime negative/out-of-bounds/null/out-of-scope/read-only diagnostics as applicable
- `sizeof` expressions for supported Cust types and expressions: `sizeof(int)`, `sizeof(char)`, `sizeof(const int)`, `sizeof(const char)`, `sizeof(int *)`/`sizeof(char *)` and const-qualified pointer type spellings such as `sizeof(const int *)`, scalar variables, arrays (using declared element type and length), pointer variables/address-of expressions including struct-field address-of expressions, first-pass struct variables, struct-array variables/elements, and scalar/array/nested struct fields (using deterministic Cust field-size sums without native ABI padding), typedef aliases including const-qualified aliases in size contexts, string literals (including the NUL terminator), and indexed string/array/pointer expressions. Cust defines `sizeof(int) == 8`, `sizeof(char) == 1`, and pointer size `8`; `sizeof(void)` and `sizeof(const void)` are rejected with a targeted parser diagnostic.
- Const-qualified pointer declarations and parameters support a scoped subset documented in `docs/plans/const-pointer-model.md`: `const int *p` / `const char *p` / `const struct Point *p` mark writes through that pointer binding as read-only while still allowing pointer reassignment; `int * const p` / `char * const p` / `struct Point * const p` mark the pointer slot read-only while allowing writes to mutable targets; `const int * const p` / `const char * const p` / `const struct Point * const p` combine both. Pointer conversions preserve pointee constness: mutable pointer expressions may flow into const pointer targets, but assigning or passing `const int *` / `const char *` / `const struct Point *` expressions to mutable `int *` / `char *` / `struct Point *` targets reports `cannot discard const qualifier from pointer target`.
- Const-qualified struct variables and by-value parameters are supported for the existing scalar-field struct subset: `const struct Point p;` and `const Point p;` after a typedef create zero-initialized read-only struct bindings, `int f(const struct Point p)` receives a by-value read-only parameter copy, direct field/copy assignment to const struct bindings reports `cannot assign to const variable '<name>'`, and writes through const struct pointers or direct pointers to const struct targets report `cannot assign through pointer to const`.
- Const-qualified scalar fields inside struct definitions are supported for `const int` and `const char` fields. Field reads work through direct variables and struct pointers, mutable sibling fields remain writable, writes to const fields report `cannot assign to const struct field '<field>'`, and whole-struct copy assignment into struct types containing const fields reports `cannot assign to struct '<Type>' with const fields`.
- One-level pointer fields inside structs are supported for scalar and struct pointees, including self-referential links such as `struct Node *next;`, scalar pointer fields such as `int *external;`, pointer-field initializer entries, pointer-field reassignment, chained struct-pointer field access (`head.next->value`), and dereference of scalar pointer fields (`*head.external`). Pointer fields copy pointer values by value during struct copy/parameter/return flows, preserve pointee const metadata for `const T *field`, and reject unsupported pointer-to-pointer or pointer-array fields with targeted diagnostics.
- First-pass scalar-field `union` support is documented in `docs/plans/union-model.md`: named top-level unions such as `union Number { int value; char tag; };` can be declared as variables, one-dimensional arrays, by-value parameters/returns, direct union-returning prototypes/definitions, nested fields inside supported structs, and one-level pointer targets/fields; zero-initialized or one-entry brace initialization is supported; scalar field reads/writes through root variables, array elements, nested field paths, and union pointers share one logical interpreter value; self-referential union pointer fields and scalar pointer fields inside unions use Cust's safe pointer metadata while pointer-to-pointer union fields are rejected; and deterministic Cust `sizeof` reports max field size while native ABI byte layout and padding remain intentionally out of scope.
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
cargo test --test interpreter supports_conditional_and_comma_expressions_for_aggregates -- --nocapture
cargo test --test interpreter aggregate_conditional -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-conditional-expression run. This run extended aggregate-valued expression evaluation so supported structs and unions can flow through the conditional operator and comma operator in aggregate contexts: declarations such as `struct Point chosen = flag ? high : low;`, copy assignments such as `other = cond ? left : right;`, and comma expressions such as `union Number n = (side_effect(), right);` now evaluate through `eval_struct_expr`, preserve conditional branch short-circuiting and comma left-side effects, and return by-value aggregate copies through the existing safe return/copy machinery. Coverage includes `tests/fixtures/valid/aggregate_conditional_expressions.c`, `tests/fixtures/invalid/aggregate_conditional_type_mismatch.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_conditional_expressions.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter aggregate_initializer -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-initializer-expression run. This run closed a declaration initializer gap for aggregate-returning functions: same-type `struct Point p = make_point(...);` and `union Number n = make_number(...);` now initialize supported aggregate variables directly from returned aggregate values instead of requiring declaration plus later copy assignment. The parser distinguishes brace aggregate initializers from expression initializers; the interpreter evaluates expression initializers through the existing safe aggregate-return path, preserves by-value field copies and `const struct` binding enforcement after initialization, and reports mismatched aggregate result types with the same `cannot assign struct '<Rhs>' to struct '<Lhs>'` diagnostic used by copy assignment. Coverage includes `tests/fixtures/valid/aggregate_initializer_expressions.c`, `tests/fixtures/invalid/aggregate_initializer_type_mismatch.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_initializer_expressions.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_union_return_functions_and_prototypes -- --nocapture
cargo test --test interpreter union -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous union-return-function run. This run closed a direct aggregate-function spelling gap for unions: `union Number make_number(int value);` prototypes and `union Number make_number(...) { ... }` definitions now route through the same safe aggregate-by-value return machinery as structs instead of being parsed as malformed union variable declarations. Side-effect-only assignment expressions such as `n = make_number(5);` now delegate aggregate variable assignment to `assign_struct_copy()` during discard evaluation, so returned union values can be copied into local union variables without scalar evaluation. Coverage includes `tests/fixtures/valid/union_return_functions.c` and native C compiler-oracle fixture `tests/fixtures/compat/valid/union_return_functions.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter nested_union -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous nested-union-field run. This run expanded the first-pass scalar union model so nested union fields inside supported structs and one-dimensional union arrays preserve the same logical shared-scalar semantics as root union variables. Recursive aggregate initialization now synchronizes nested union scalar fields, scalar field writes through `holder.number.value` and `numbers[i].value` update sibling scalar views, by-value union copies/parameters continue to isolate caller storage, and deterministic nested `sizeof` remains Cust-defined. Coverage includes `tests/fixtures/valid/nested_union_fields.c` and native C compiler-oracle fixture `tests/fixtures/compat/valid/nested_union_fields.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter union -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous first-pass union run. This run added `docs/plans/union-model.md`, lexer/parser/runtime support for named scalar-field `union` declarations and variables, deterministic max-field `sizeof`, one first-field brace initializer, scalar member reads/writes over shared logical interpreter storage, and targeted excess-initializer diagnostics. Coverage includes `tests/fixtures/valid/unions.c`, `tests/fixtures/invalid/union_initializer_too_long.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/unions.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test repository_license
cargo test --test interpreter path_designator -- --nocapture
cargo test --test interpreter supports_path_designated_struct_initializers -- --nocapture
cargo test --test interpreter designator -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous path-designated-initializer run. This run extended designated initializers with path forms for supported aggregate fields: nested struct fields such as `.inner.x = 5` and one-dimensional scalar array fields such as `.values[2] = 7` can now be initialized directly inside struct initializer lists. Runtime struct initialization now applies field initializers in source order over zero/default storage, so repeated nested path entries merge sibling fields instead of replacing the whole nested aggregate. Invalid path designators include targeted diagnostics for unknown nested fields and out-of-bounds array-field indices. Coverage includes `tests/fixtures/valid/path_designated_initializers.c`, invalid `struct_path_designator_unknown_field.c` / `struct_array_path_designator_out_of_bounds.c`, compiler-oracle fixture `tests/fixtures/compat/valid/path_designated_initializers.c`, and updated `docs/plans/designated-initializers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter designated -- --nocapture
cargo test --test interpreter designator -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous designated-initializer run. This run added C-style designated initializers for supported one-dimensional scalar arrays and supported structs: array designators such as `{[2] = 5, [0] = 1}` initialize specific indices with omitted zero-fill and mixed positional continuation; struct field designators such as `{.y = 2, .x = 1}` initialize fields out of order; nested aggregate brace lists can use their own array/struct designators; and invalid array designator bounds or unknown struct fields have targeted diagnostics. Coverage includes `tests/fixtures/valid/designated_initializers.c`, invalid `array_designator_out_of_bounds.c` / `struct_designator_unknown_field.c`, compiler-oracle fixture `tests/fixtures/compat/valid/designated_initializers.c`, and `docs/plans/designated-initializers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter address_of_struct -- --nocapture
cargo test --test interpreter const_struct_field_address -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous address-of-struct-fields recovery run. This run completed the interrupted pointer/aggregate follow-up: `&point.x`, nested field paths such as `&packet.anchor.y`, `&points[i]`, and `&points[i].x` produce interpreter-owned pointers that alias original scalar fields or struct-array elements without host addresses, can be passed to existing scalar/struct pointer parameters, and preserve const-discard diagnostics for const struct fields. Coverage includes `tests/fixtures/valid/address_of_struct_fields.c`, `tests/fixtures/invalid/const_struct_field_address_discard.c`, and native compiler-oracle fixture `tests/fixtures/compat/valid/address_of_struct_fields.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_arrays_of_structs -- --nocapture
cargo test --test interpreter rejects_struct_array_variable_initializers_longer_than_declared_length -- --nocapture
cargo test --test interpreter struct_array -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous arrays-of-structs run. This run completed the interrupted aggregate milestone: supported struct arrays such as `struct Point points[3] = {{1, 2}, {3, 4}};` now store zero-filled/deep-cloned struct elements, support field-path reads/writes/lvalues such as `points[i].x`, field compound assignment and increment/decrement, struct-array elements as by-value struct arguments/assignment RHS values, nested array-field access such as `packets[i].values[j]`, deterministic Cust `sizeof(points)` / `sizeof(points[i])` / `sizeof(points[i].field)`, and targeted excess-initializer diagnostics for struct arrays. Coverage includes `tests/fixtures/valid/struct_arrays.c`, `tests/fixtures/invalid/struct_array_variable_initializer_too_long.c`, and native compiler-oracle fixture `tests/fixtures/compat/valid/struct_arrays.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter struct_pointer_field -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous struct-pointer-field recovery run. This run completed the interrupted pointer-field milestone: struct definitions can contain one-level scalar and struct pointer fields such as `int *external;`, `const int *view;`, and self-referential `struct Node *next;`; pointer-field initializers/reassignment, `head.next->value`, and `*head.external` work through interpreter-owned pointer metadata; pointer fields copy pointer values by value during struct copies/parameters/returns; `const T *field` preserves pointee constness without making the field slot const; and unsupported pointer-to-pointer fields plus const-discarding pointer-field assignments have targeted regressions. Coverage includes `tests/fixtures/valid/struct_pointer_fields.c`, `tests/fixtures/valid/struct_pointer_field_const_pointee.c`, `tests/fixtures/invalid/struct_pointer_to_pointer_field.c`, `tests/fixtures/invalid/struct_pointer_field_const_discard.c`, and native C compiler-oracle fixtures for the supported pointer-field behavior. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter struct_array -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous struct-array-field run. This run added one-dimensional scalar array fields inside structs, so declarations such as `struct Packet { int values[3]; char tag[2]; };` store interpreter-owned array field storage, support `packet.values[i]` reads/writes, element assignment/compound/increment lvalues, recursive array-field brace initializers with omitted-element zero-fill, and deterministic Cust `sizeof(packet.values)` / `sizeof(packet.values[i])`. Same-type struct copy and by-value struct parameters now deep-clone struct fields so embedded array storage keeps C value semantics instead of sharing `Rc` storage. Coverage includes `tests/fixtures/valid/struct_array_fields.c`, `tests/fixtures/invalid/struct_array_initializer_too_long.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_array_fields.c` while avoiding ABI-sensitive native struct-size comparisons. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter nested_struct_initializer -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous nested-struct-initializer run. This run added recursive brace initializers for nested struct fields, so declarations such as `struct Rect r = {{1, 2}, 3};` initialize nested struct values in declaration order, zero-fill omitted nested fields, preserve static/const declaration behavior, and report excess nested entries as `too many initializers for struct '<Nested>'`. Coverage includes `tests/fixtures/valid/nested_struct_initializers.c`, `tests/fixtures/invalid/nested_struct_initializer_too_long.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/nested_struct_initializers.c` while avoiding ABI-sensitive native struct-size comparisons. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter struct -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous nested-struct-field run. This run added nested struct fields for prior named struct types, field-path reads/writes such as `rect.origin.x`, same-type copy and by-value parameter passing from nested struct fields, recursive deterministic Cust `sizeof` for nested struct fields, and targeted innermost unknown-field diagnostics. Coverage includes `tests/fixtures/valid/nested_struct_fields.c`, `tests/fixtures/invalid/nested_struct_unknown_field.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/nested_struct_fields.c` while avoiding ABI-sensitive native struct-size comparisons. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter struct_initializers -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-06 autonomous struct-initializer run. This run added scalar brace initializers for supported struct variables in top-level, local, `static` local, and `const` struct declaration contexts. Initializer expressions are evaluated in field declaration order, omitted trailing fields remain zero-filled, const fields can receive initial values but remain read-only afterward, and excess entries report `too many initializers for struct '<Type>'`. Coverage includes `tests/fixtures/valid/struct_initializers.c`, `tests/fixtures/invalid/struct_initializer_too_long.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_initializers.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter array_initializers -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-06 autonomous array-initializer run. This run added scalar brace initializers for one-dimensional `int` and `char` arrays in global, local, `static` local, and `const` array declarations. Initializer expressions are evaluated left-to-right at declaration/static-initialization time, missing elements remain zero-filled, trailing commas are accepted, and too many initializers report `too many initializers for array '<name>'`. Coverage includes `tests/fixtures/valid/array_initializers.c`, `tests/fixtures/invalid/array_initializer_too_long.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/array_initializers.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous void-parameter-list run. This run added C-style empty `void` parameter lists for function definitions and prototypes, so `int main(void)` and `int helper(void);` parse as zero-argument signatures while malformed named `void` parameters report `void parameter lists must be empty`. Coverage includes `tests/fixtures/valid/void_parameter_lists.c`, `tests/fixtures/invalid/void_parameter_named.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/void_parameter_lists.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous local-static-storage run. This run added persistent local `static` storage for supported block/function-scope scalar, pointer, array, and struct declarations. Static locals initialize once, persist across calls in interpreter-owned storage, remain lexically scoped to their declaring block while active, and can be safely addressed through Cust pointers after the declaring function returns. Coverage includes `tests/fixtures/valid/static_local_storage.c`, `tests/fixtures/invalid/static_local_out_of_scope.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/static_local_storage.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous static-storage-class run. This run added lexer/parser support for `static` as a top-level storage-class specifier on supported global variables, function prototypes, and function definitions, treating it as linkage metadata only in Cust's single-file interpreter. Coverage includes `tests/fixtures/valid/static_storage_class.c` and native C compiler-oracle fixture `tests/fixtures/compat/valid/static_storage_class.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous const-struct-field run. This run added support for `const int`/`const char` fields in scalar-field struct definitions, preserving deterministic zero initialization and field reads while rejecting field mutation through direct variables or struct pointers with `cannot assign to const struct field '<field>'` and rejecting whole-struct copy assignment into such structs with `cannot assign to struct '<Type>' with const fields`. Coverage includes `tests/fixtures/valid/const_struct_fields.c`, invalid const-field assignment/pointer-write/copy-assignment fixtures, and native C compiler-oracle fixture `tests/fixtures/compat/valid/const_struct_fields.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous pointer-typedef run. This run extended parser-only typedef aliases to one-level scalar/struct pointer aliases (`typedef int *IntPtr;`, `typedef char *CharPtr;`, `typedef struct Point *PointPtr;`), with use in declarations, parameters/prototypes, calls, struct-pointer field access, and `sizeof(pointer_alias)`. It preserves the existing no-pointer-to-pointer boundary with exact diagnostics for both `typedef int **...` and `typedef IntPtr *...`, and adds interpreter plus native C compiler-oracle fixtures. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous enum-typedef run. This run extended parser-only typedef aliases to named enum tags (`typedef enum Status Status;`) after a prior enum declaration, resolving them to Cust's existing integer scalar type for globals, locals, arrays, parameters/prototypes, return types, and `sizeof(alias)`, while preserving enum constants as scoped read-only integer identifiers and keeping enum tag lookup block-scoped. Coverage includes `tests/fixtures/valid/enum_typedef_aliases.c`, `tests/fixtures/invalid/typedef_unknown_enum.c`, `tests/fixtures/invalid/block_enum_tag_out_of_scope.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/enum_typedef_aliases.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous `sizeof(const type)` run. This run added parser support for const-qualified type names in `sizeof(...)` type contexts, including `sizeof(const int)`, `sizeof(const char)`, const-qualified typedef aliases, and const-qualified scalar/char/struct pointer size spellings, while preserving the exact `sizeof(void)` diagnostic for `sizeof(const void)`. Coverage includes `tests/fixtures/valid/sizeof_const_types.c`, `tests/fixtures/invalid/sizeof_const_void.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/sizeof_const_types.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous const-struct-qualifier run. This run extended const-qualified pointer semantics to struct pointers (`const struct Point *p`, `struct Point * const p`, and `const struct Point * const p`), added const-qualified struct variables and by-value struct parameters, enforced direct field/copy assignment rejection for const struct bindings, rejected field writes through const struct pointers or pointers to const struct targets, and preserved const-discard conversion diagnostics for struct pointers. Coverage includes `tests/fixtures/valid/const_struct_qualifiers.c`, invalid const struct field/write/discard fixtures, and native C compiler-oracle fixture `tests/fixtures/compat/valid/const_struct_qualifiers.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous const-pointer-conversion run. This run tightened const-qualified pointer conversions: Cust now rejects pointer declarations, assignment expressions/statements, and function arguments that would discard pointee constness from `const int *` / `const char *` expressions into mutable pointer targets, while preserving valid mutable-to-const conversions and pointer arithmetic over const pointer views. Coverage includes `tests/fixtures/valid/const_pointer_conversions.c`, invalid const-discard declaration/assignment/argument fixtures, and native C compiler-oracle fixture `tests/fixtures/compat/valid/const_pointer_conversions.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous const-pointer-qualifier run. This run added `docs/plans/const-pointer-model.md`, parser/runtime support for split pointer-slot versus pointee const metadata on scalar pointer declarations and parameters (`const int *p`, `int * const p`, and `const int * const p` plus `char` equivalents), and write/reassignment diagnostics for the supported subset. Coverage includes `tests/fixtures/valid/const_pointer_qualifiers.c`, invalid write/index-write/reassignment fixtures, and native C compiler-oracle fixture `tests/fixtures/compat/valid/const_pointer_qualifiers.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous const-qualifier run. This run added lexer/parser/interpreter support for leading `const int` / `const char` scalar declarations, zero-initialized const arrays, and const scalar function parameters; runtime scopes now track const bindings, reject scalar/parameter mutation through direct assignment, assignment expressions, compound assignment, increment/decrement, and scalar pointer writes, and reuse read-only array storage for const arrays. Coverage includes `tests/fixtures/valid/const_qualifiers.c`, invalid const assignment/array/parameter fixtures, and native C compiler-oracle fixture `tests/fixtures/compat/valid/const_qualifiers.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous block-scoped typedef run. This run generalized parser-only typedef storage from one global alias table into lexical alias scopes, so block-local aliases shadow outer aliases and expire at block exit. Coverage includes `tests/fixtures/valid/block_scoped_typedefs.c`, invalid `tests/fixtures/invalid/block_typedef_alias_out_of_scope.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/block_scoped_typedefs.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous typedef-alias run. This run added `docs/plans/typedef-model.md`, lexer/parser support for top-level parser-only aliases (`typedef int Count;`, `typedef char Byte;`, `typedef struct Point Point;`), alias use in globals, locals, arrays, one-level pointer declarations, function prototypes/definitions/parameters/returns, scalar struct fields, and `sizeof(alias)`, plus explicit unsupported pointer-alias and missing-alias-name diagnostics. Coverage includes `tests/fixtures/valid/typedef_aliases.c`, `tests/fixtures/invalid/typedef_missing_alias_name.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/typedef_aliases.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous struct-pointer run. This run extended `docs/plans/struct-model.md` and `docs/plans/pointer-model.md`, added parser/interpreter support for one-level `struct Name *` local/global declarations and parameters/prototypes, `&struct_var`, `p->field`, `(*p).field`, field assignment/compound/increment through struct pointers, pointer equality/truthiness for struct targets, and null/out-of-scope diagnostics. Coverage includes `tests/fixtures/valid/struct_pointers.c`, `tests/fixtures/invalid/struct_pointer_null_dereference.c`, `tests/fixtures/invalid/struct_pointer_out_of_scope.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_pointers.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous struct-return run. This run extended `docs/plans/struct-model.md`, added parser/interpreter support for `struct Name f(...)` definitions and prototypes after a prior struct declaration, carried return flow as scalar or cloned struct values so returned local structs remain valid by value, allowed struct-returning calls in same-type struct assignment, added deterministic `sizeof(struct_return_call())`, and added mismatched/empty struct-return diagnostics plus interpreter and native C compiler-oracle fixtures. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-05-06 autonomous struct-parameter run. This run extended `docs/plans/struct-model.md`, added parser/interpreter support for by-value `struct Name param` function parameters in definitions and prototypes, cloned same-type struct arguments into callee scope so field writes do not mutate caller values, added targeted mismatched/non-struct argument diagnostics, and added interpreter plus native C compiler-oracle fixtures. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous struct-copy-and-field-lvalue run. This run extended `docs/plans/struct-model.md`, added parser/interpreter support for same-type struct copy assignment with value semantics, mismatched struct-copy diagnostics, struct field assignment expressions, field compound assignments, and field prefix/postfix increment/decrement. Coverage includes `tests/fixtures/valid/struct_lvalues_and_copy.c`, `tests/fixtures/invalid/struct_assignment_type_mismatch.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_lvalues_and_copy.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous verified state: all passed after the 2026-05-06 autonomous struct-first-milestone run. This run added `docs/plans/struct-model.md`, lexer/parser/interpreter support for top-level scalar-field struct declarations, top-level/local zero-initialized struct variables, scalar member reads/writes with `.`, deterministic `sizeof` for struct variables/fields, an exact unknown-field diagnostic, and interpreter plus native C compiler-oracle fixtures. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

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
