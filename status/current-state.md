# Cust Current State

Last updated: 2026-07-12

## Latest autonomous verification

All required verification passed after the 2026-07-12 multidimensional-array designator diagnostic run. A fresh parser-trust probe found that a second designator suffix—such as `int values[2] = {[0][1] = 3}`, `(int[2]){[0][1] = 3}`, aggregate-array `[0][1]`, and aggregate-field `.values[0][1]` paths—fell through to `expected '=' after array designator, found LBracket`. TDD RED captured that generic fallback. GREEN now rejects the second `[` at its source location with `multidimensional array designators are not supported` from the shared bounded/unbounded designator-index paths, while preserving supported one-dimensional designators, integer-constant indexes, range-designator diagnostics, and designator bounds checks.

Verified commands: baseline `cargo test`; focused RED/GREEN `cargo test --test interpreter rejects_multidimensional_array_designators_with_context -- --nocapture`; focused `cargo test --test interpreter designator -- --nocapture`; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test`; `docker compose run --rm test`; and `docker compose run --rm cust`.

All required verification passed after the 2026-07-12 multidimensional-array cast diagnostic run. A parser-trust probe found that unsupported multidimensional array type names in cast/compound-literal syntax—such as `(int[2][3]){1}`, `(struct Point[2][3]){...}`, anonymous aggregate arrays, and an extra array suffix after an array typedef—fell through to `expected ')' after cast type, found LBracket`. TDD RED captured the generic error. GREEN now rejects the second `[` at its source location with `multidimensional array casts are not supported`, while preserving one-dimensional array compound literals, function-array diagnostics, and existing declaration/type-query multidimensional boundaries.

Verified commands: baseline `cargo test`; focused RED/GREEN `cargo test --test interpreter rejects_multidimensional_array_cast_type_names_with_context -- --nocapture`; focused multidimensional, function-array, and array-typedef compound-literal regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test`; `docker compose run --rm test`; and `docker compose run --rm cust`.

All required verification passed after the 2026-07-12 pointer-function type-name diagnostic run. A fresh parser-trust probe found that abstract function type names returning a one-level pointer—`(int *(void))0`, `sizeof(int *(void))`, and `_Alignof(int *(void))`—reached generic closing-parenthesis errors because pointer branches returned or expected a delimiter without checking a following function suffix. TDD RED captured `expected ')' after cast type, found LParen`. GREEN applies the existing function-suffix guard after every supported pointer route in casts and type queries: direct scalar/named/anonymous aggregate pointers plus pointer typedef aliases. Exact coverage preserves the established parenthesized-pointer boundary and earlier function-array diagnostics.

Verified commands: baseline `cargo test`; focused RED/GREEN `cargo test --test interpreter rejects_function_type_cast_and_type_query_names_with_context -- --nocapture`; focused function-array, parenthesized-pointer, and pointer-array regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test` (609 interpreter tests); `docker compose run --rm test`; and `docker compose run --rm cust` (printed `10`).

All required verification passed after the 2026-07-12 function-array type-name diagnostic run. A fresh parser-trust probe found that unsupported function-array spellings such as `(int[2](void))0`, `sizeof(int[2](void))`, and `_Alignof(int[2](void))` parsed the array suffix but then fell through to generic closing-parenthesis diagnostics. TDD RED captured the generic cast error. GREEN applies the existing function-suffix guard immediately after parsed scalar and aggregate array type suffixes in casts and type queries, so errors are source-located at the function declarator `(`. Exact coverage includes scalar, named aggregate, and typedef-array paths; valid array compound literals remain unchanged.

Verified commands: focused RED/GREEN `cargo test --test interpreter rejects_function_array_cast_and_type_query_names_with_context -- --nocapture`; focused direct-function and pointer-array type-query regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test` (609 interpreter tests); `docker compose run --rm test`; and `docker compose run --rm cust` (printed `10`).

All required verification passed after the 2026-07-11 function-type cast/type-query diagnostic run. A fresh parser-trust probe found that abstract function type spellings such as `(int(void))0`, `sizeof(int(void))`, and `_Alignof(int(void))` fell through to generic closing-parenthesis errors after parsing the base scalar type. TDD RED captured the generic cast diagnostic. GREEN adds a narrow suffix guard after supported type parsing so function type names report `function casts/sizeof/_Alignof types are not supported` at the function declarator `(`, while parenthesized pointer forms such as `(int (*)[3])0` retain their established diagnostics.

Verified commands: clean baseline `cargo test`; focused RED/GREEN `cargo test --test interpreter rejects_function_type_cast_and_type_query_names_with_context -- --nocapture`; focused pointer-array, parenthesized-pointer, and type-query regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test` (608 interpreter tests); `docker compose run --rm test`; and `docker compose run --rm cust` (printed `10`).

All required verification passed after the 2026-07-11 pointer-array `sizeof`/`_Alignof` diagnostic run. A parser-trust probe found that direct scalar, named/anonymous aggregate, and typedef-spelled pointer-array type names such as `sizeof(int *[2])` fell through to `expected ')' after sizeof type, found LBracket`, even though pointer typedefs already had a narrow diagnostic. TDD RED captured the generic fallback. GREEN makes every supported type-query route reject pointer-array type names at the `[` token with `pointer array sizeof/_Alignof types are not supported`, preserving ordinary array type queries, one-level pointer sizes, and pointer-to-pointer diagnostics.

Verified commands: clean baseline `cargo test`; focused RED/GREEN `cargo test --test interpreter rejects_pointer_array_sizeof_and_alignof_type_names_with_context -- --nocapture`; focused `pointer_array`, `sizeof`, and post-star qualifier regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test`; `docker compose run --rm test`; and `docker compose run --rm cust`.

All required verification passed after the 2026-07-11 pointer-array cast diagnostic run. A fresh parser-trust probe found that unsupported array-of-pointer cast type names such as `(int *[2])0` fell through to `expected ')' after cast type, found LBracket`, even though declaration, typedef, `sizeof`, and `_Alignof` routes already identify Cust's pointer-array boundary. TDD RED added exact-output coverage and captured that generic error. GREEN reports `pointer array casts are not supported` at the `[` token for scalar, named aggregate, anonymous aggregate, and pointer-typedef type names, while retaining the established one-level-pointer and pointer-to-pointer diagnostics.

Verified commands: clean baseline `cargo test`; focused RED `cargo test --test interpreter rejects_pointer_array_cast_type_names_with_context -- --nocapture` (failed as expected with the generic cast-delimiter diagnostic); focused GREEN plus pointer-cast and post-star qualifier regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test` (606 interpreter tests); `docker compose run --rm test`; and `docker compose run --rm cust` (printed `10`).

All required verification passed after the 2026-07-11 qualified pointer type-name diagnostic run. A fresh parser-trust probe found that legal C type names such as `sizeof(int * const *)`, `_Alignof(int * const *)`, and `(int * const *)0` reached generic closing-parenthesis errors instead of Cust's explicit one-level-pointer boundary. TDD RED added exact-output interpreter coverage and observed the generic `found Const` / `found Star` errors. GREEN consumes post-star qualifiers in scalar/named/anonymous aggregate cast and type-query routes, preserves valid one-level `const`/`volatile` pointer type names, and reports `pointer-to-pointer ... types/casts are not supported` at the second star. Native `cc -std=c11 -Wall -Wextra -Werror` accepted the supported one-level qualifier probe and all rejected pointer-to-pointer spellings as valid C syntax; Cust deliberately rejects the latter as outside its safe subset.

Verified commands: focused RED `cargo test --test interpreter rejects_qualified_pointer_to_pointer_type_names_with_context -- --nocapture` (failed as expected with the generic `expected ')' ...` diagnostic); focused GREEN plus pointer-cast/`sizeof`/`_Alignof` regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test` (605 interpreter tests); `docker compose run --rm test`; and `docker compose run --rm cust`.

All required verification passed after the 2026-07-11 restrict type-name correctness run. Ideation evaluated the clean baseline (all tests passing), active blockers (none), the parser-trust continuation, conformance coverage, and CLI/docs work. A narrow C conformance probe found that Cust accepted invalid leading `restrict` in `sizeof(restrict int)`, `_Alignof(restrict int)`, and `(restrict int)0`, returning Cust values although `cc -std=c11 -Wall -Wextra -Werror` rejects each use. TDD RED added exact-output interpreter coverage and observed the `sizeof` case return `Ok(8)`. GREEN reuses the existing leading-restrict guard before type-query qualifier consumption and before cast qualifier consumption, so all three type-name contexts reject non-pointer `restrict` at the source token while valid pointer-declarator restrict coverage stays green.

Verified commands: clean-baseline `cargo test`; focused RED `cargo test --test interpreter rejects_restrict_on_non_pointer_type_names_with_context -- --nocapture` (failed as expected because invalid source returned `Ok(8)`); focused GREEN plus restrict/type-query/cast regressions; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test`; `docker compose run --rm test`; and `docker compose run --rm cust`.

All required verification passed after the 2026-07-11 array-range-designator diagnostic run. Ideation evaluated clean-baseline failures (none), active blockers (none), the parser-trust continuation, conformance/fuzz coverage, and CLI/documentation work. A focused malformed-source probe found that unsupported GNU-style array range designators (`[0 ... 2] = 1`) were tokenized as three dots and fell through to `expected ']' after array designator, found Dot`. TDD RED added an exact-output interpreter regression and observed that fallback. GREEN adds a shared post-index guard that reports `array range designators are not supported` at the first dot for declared arrays, unsized array compound literals, and aggregate scalar-array field path designators; ordinary supported single-index designators remain unchanged.

Verified commands: clean-baseline `cargo test`; focused RED/GREEN `cargo test --test interpreter rejects_range_array_designators_with_context -- --nocapture`; focused designator regression `cargo test --test interpreter designator -- --nocapture`; `cargo fmt --check`; `cargo clippy -- -D warnings`; `cargo test` (602 interpreter tests, all suites); `docker compose run --rm test`; and `docker compose run --rm cust` (printed `10`).

All passed after the 2026-07-11 autonomous `sizeof` invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the standing parser-trust continuation item, broader C-subset conformance coverage, fuzz/property tests, CLI flags, README/release polish, and a narrow newly probed expression-boundary gap: `sizeof([)`, `sizeof(?)`, `sizeof(:)`, `sizeof(.)`, `sizeof(->field)`, and `sizeof({1})` were less contextual than adjacent `_Alignof`, `_Alignas`, `_Atomic(type-name)`, integer-constant, and expression-boundary routes. Selected this slice because it closes the existing `sizeof` missing-operand guard without changing valid `sizeof(type-name)` or `sizeof expression` semantics. TDD RED updated the exact-output test first and confirmed the previous generic `expected sizeof operand, found LBracket`; GREEN now reports `expected sizeof operand before '<token>'` for bracket/question/colon/selector/brace invalid starts while preserving delimiter-only missing-operand diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # clean pulled-tree baseline passed before selecting work
cargo test --test interpreter rejects_missing_sizeof_and_alignof_operands_with_context -- --nocapture  # RED first: generic `expected sizeof operand, found LBracket`; GREEN passed after invalid-start guard routing
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-11 autonomous `_Atomic(type-name)` selector/colon invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the standing parser-trust continuation item, broader C-subset conformance coverage, fuzz/property tests, CLI flags, README/release polish, and a narrow newly probed type-name gap: `_Atomic(:)`, `_Atomic(.)`, and `_Atomic(->field)` still reported generic `expected _Atomic type, found ...` messages even though adjacent `_Alignof`, `_Alignas`, integer-constant, and expression-boundary routes use contextual `before '<token>'` diagnostics. Selected this slice because it completes the existing `_Atomic(type-name)` invalid-start guard without changing valid `_Atomic` qualifier/specifier behavior. TDD RED captured `_Atomic(:)` first with the generic `found Colon`; GREEN routes `_Atomic(type-name)` through `integer_constant_invalid_start_or_selector_label()` so colon/dot/arrow now report `expected _Atomic type before ':'`, `before '.'`, and `before '->'`.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_atomic_type_arguments_with_context -- --nocapture  # RED first: generic `expected _Atomic type, found Colon`; GREEN passed after selector/colon guard routing
cargo test --test interpreter atomic -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-11 autonomous `_Alignas` / `_Static_assert` colon invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the standing parser-trust continuation item, broader C-subset conformance coverage, fuzz/property tests, CLI flags, README/release polish, and a freshly probed narrow parser-trust gap: `_Alignas(:)` and `_Static_assert(:, "msg")` still fell through to generic `expected expression, found Colon` even though neighboring brace, selector, bracket, keyword, and delimiter starts already had contextual diagnostics. Selected this slice because it closes two declaration/assertion operand boundaries with a shared parser helper and low regression risk. TDD RED captured both generic colon failures first; GREEN routes both guards through `integer_constant_invalid_start_or_selector_label()` so colon reports `expected _Alignas argument before ':'` and `expected _Static_assert condition before ':'` while preserving valid `_Alignas(type-name/expression)` and `_Static_assert(condition, message)` parsing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_alignas_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Colon`; GREEN passed after guard helper routing
cargo test --test interpreter rejects_missing_static_assert_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Colon`; GREEN passed after guard helper routing
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-10 autonomous colon-token parameter/integer-constant diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the standing parser-trust continuation item, broader conformance coverage, fuzz/property tests, CLI flags, README/release polish, and a concrete newly probed diagnostic slice: colon/selector/semicolon malformed function-parameter starts and colon starts in integer-constant boundaries still fell through to generic `expected type` / `expected ... found Colon` diagnostics. Selected this work because it closes route-local parser trust gaps without broad grammar changes. TDD RED captured `int add(; int b)` first with generic `expected type, found Semi`; GREEN extends parameter-list guards for semicolon/colon/selector starts and routes colon through shared integer-constant invalid-start labelling for array lengths, designator indexes, enum constants, and switch case labels.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_function_parameters_around_commas_with_context -- --nocapture  # RED first: generic `expected type, found Semi`; GREEN passed after parameter guard expansion
cargo test --test interpreter rejects_invalid_start_array_designator_indexes_with_keyword_context -- --nocapture
cargo test --test interpreter rejects_delimiter_only_array_lengths_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_start_enum_constant_values_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_start_switch_case_values_with_context -- --nocapture
cargo test --test interpreter rejects_variadic_function_parameters_with_context -- --nocapture  # Regression guard after parameter Dot/Arrow starts were added; preserves `...` diagnostics
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust  # printed 10 and exited 0
```

All passed after the 2026-07-10 autonomous subscript comma-expression run. Ideation considered failing tests/builds (clean startup inspection found no local changes and no active blockers), the standing parser-trust continuation item, broader conformance coverage, fuzz/property tests, CLI flags, README/release polish, and a concrete C-subset conformance gap discovered by probing: direct comma expressions inside subscript indices (`values[i++, 2]`) were rejected with `expected ']' after array index, found Comma` even though Cust already supports the comma operator generally and C subscript brackets take an expression. Selected this work because it is a compact behavior feature with interpreter and compiler-oracle coverage. TDD RED captured the rejected direct subscript comma first; GREEN extends `parse_index_expr()` to fold comma tails into `Expr::Comma` while preserving the existing first-operand parser path and legacy missing-bracket diagnostics such as `values[0 = 3;`.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_comma_expressions_in_subscript_indices -- --nocapture  # RED first: `expected ']' after array index, found Comma`; GREEN passed after subscript comma parsing
cargo test --test interpreter reports_missing_closing_brackets_after_array_assignment_indices -- --nocapture
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-10 autonomous colon-token parser-trust diagnostic run. Ideation considered failing tests/builds (clean startup inspection found no local changes and no active blockers), the remaining parser recovery continuation item in `status/todo.md`, broader conformance coverage, fuzz/property tests, CLI flags, README examples, release/docs polish, and newly probed malformed expression-boundary starts. Selected colon-token invalid-start parser diagnostics because probing showed several route-local expression boundaries (`callee(:)`, `callee(1, :)`, `values[:]`, `int values[2] = {:}`, `return :;`, `if (:)`, `switch (:)`, and `return (:);`) still fell through to generic `expected expression, found Colon` parsing even though neighboring delimiter, bracket, selector, brace, and keyword starts already had contextual diagnostics. TDD RED captured the generic function-call failure first; GREEN adds narrow `Token::Colon` guards at call-argument, array-index, braced-initializer, return/control/switch, for-clause, and grouped-expression boundaries while preserving label/case/default and valid conditional-operator colon handling.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_colon_invalid_starts_in_expression_boundaries_with_context -- --nocapture  # RED first for `callee(:)`: generic `expected expression, found Colon`; GREEN passed after colon boundary guards
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All required local and Docker verification passed. `docker compose run --rm cust` printed the sample result `10`.

All passed after the 2026-07-10 autonomous top-level invalid-start parser-trust diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the standing parser-trust continuation item in `status/todo.md`, remaining C-subset/conformance gaps (mostly complete), pointer negative diagnostics (complete), fuzz/property tests, CLI flags, README examples, and release/docs polish. Selected top-level declaration invalid-start diagnostics because probing showed malformed file-scope starts such as `{ int value; }`, `,`, `?`, `.field`, and `->field` still used the generic `unexpected token at top level: ...` fallback while neighboring unmatched delimiters and stray file-scope control flow already had contextual diagnostics. TDD RED captured the generic fallback first; GREEN adds a narrow top-level declaration-start guard while preserving existing unmatched `)`, `]`, `}`, stray control-flow, and valid declaration routing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_invalid_start_top_level_declarations_with_context -- --nocapture  # RED first for `{ int value; }`: generic `unexpected token at top level: LBrace`; GREEN passed after top-level guard
cargo test --test interpreter top_level -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-10 autonomous brace-token `_Atomic`/parameter parser-trust diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the standing parser-trust continuation item in `status/todo.md`, broader conformance coverage, pointer negative diagnostics, fuzz/property testing, CLI flags, README examples, and release/docs polish. Selected less-traveled declaration/type-name brace boundaries because probing showed `_Atomic({)` type arguments and leading `{` in function parameter lists still fell through to generic `expected type, found LBrace` diagnostics even after the prior expression/type/integer-constant brace sweep. TDD RED captured `_Atomic({)` at top level and in `sizeof(_Atomic({))`, plus `int add({ int b)`; GREEN adds narrow route-local `Token::LBrace` guards in `_Atomic(type-name)` parsing and initial function-parameter parsing while preserving statement-leading blocks, valid braced initializers, and already-contextual after-comma parameter diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_atomic_type_arguments_with_context -- --nocapture  # RED first for `_Atomic({)`: generic `expected type, found LBrace`; GREEN passed after `_Atomic` guard
cargo test --test interpreter rejects_missing_function_parameters_around_commas_with_context -- --nocapture  # RED first for `int add({ int b)`: generic `expected type, found LBrace`; GREEN passed after parameter-start guard
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-10 autonomous brace-token parser-trust diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance coverage, pointer negative diagnostics, fuzz/property testing, CLI flags, README examples, and release/docs polish. Selected malformed `{` expression-start boundaries because probing and the exact-error suite showed several expression/type/integer-constant routes treated `Token::LBrace` inconsistently: some already reported contextual missing-expression diagnostics, while `_Alignas`, `_Static_assert`, `sizeof`, `_Alignof`, grouped expressions, array lengths, enum values, switch labels, control clauses, comma/assignment/binary/conditional operands, and array indexes could still fall through to generic expression/type/integer-constant parsing. TDD RED first captured the array-index brace fallback; GREEN added narrow route-local `LBrace` guards and coverage across the affected parser-trust boundaries while leaving statement-leading `{ ... }` block parsing and valid braced initializers unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_array_index_expressions_with_context -- --nocapture  # RED first for `values[{1}]`: generic `expected expression, found LBrace`; GREEN passed after route-local guards
cargo test --test interpreter rejects_delimiter_only_array_lengths_with_context -- --nocapture
cargo test --test interpreter rejects_missing_alignas_arguments_with_context -- --nocapture
cargo test --test interpreter rejects_missing_static_assert_arguments_with_context -- --nocapture
cargo test --test interpreter rejects_missing_sizeof_and_alignof_operands_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_start_enum_constant_values_with_context -- --nocapture
cargo test --test interpreter rejects_missing_grouped_expression_operands_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_start_switch_case_values_with_context -- --nocapture
cargo test --test interpreter rejects_missing_switch_expressions_with_context -- --nocapture
cargo test --test interpreter rejects_missing_control_flow_condition_expressions_with_context -- --nocapture
cargo test --test interpreter rejects_misplaced_for_increment_expressions_with_context -- --nocapture
cargo test --test interpreter rejects_missing_rhs_after_comma_operator -- --nocapture
cargo test --test interpreter rejects_missing_rhs_after_binary_operators -- --nocapture
cargo test --test interpreter rejects_missing_rhs_after_assignment_operators -- --nocapture
cargo test --test interpreter rejects_missing_return_expressions_with_context -- --nocapture
cargo test --test interpreter rejects_missing_conditional_operator_operands_with_context -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-10 autonomous selector-token integer-constant diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance coverage, pointer negative diagnostics, fuzz/property testing, CLI flags, README examples, and release/docs polish. Selected selector-token malformed integer-constant boundaries because probing showed array declarator lengths, array designator indexes, enum constant values, and switch case labels such as `int values[.field];`, `{[->field] = 1}`, `enum Bad { A = .field };`, and `case ->field:` still fell through to generic `expected ..., found Dot/Arrow` diagnostics while neighboring parenthesized integer-constant and ordinary expression routes were already contextual. TDD RED first captured those generic fallbacks; GREEN added a narrow integer-constant selector-start helper used only by array length/designator/enum/switch routes so existing expression-context selector diagnostics remain stable.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_delimiter_only_array_lengths_with_context -- --nocapture  # RED first: generic `expected array length, found Dot`; GREEN passed after selector-aware integer-constant guard
cargo test --test interpreter rejects_malformed_path_designators_with_context -- --nocapture  # RED first: generic `expected array designator index, found Dot`; GREEN passed after selector-aware designator guard
cargo test --test interpreter rejects_invalid_start_enum_constant_values_with_context -- --nocapture  # RED first: generic `expected integer constant..., found Dot`; GREEN passed after selector-aware enum constant guard
cargo test --test interpreter rejects_invalid_start_switch_case_values_with_context -- --nocapture  # RED first: generic `expected integer constant..., found Dot`; GREEN passed after selector-aware switch case guard
cargo test --test interpreter invalid_start -- --nocapture
cargo test --test interpreter array_length -- --nocapture
cargo test --test interpreter designator -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-10 autonomous selector-token array/string index diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance coverage, pointer negative diagnostics, fuzz/property testing, CLI flags, README examples, and release/docs polish. Selected selector-token malformed array/string index boundaries because probing showed `values[.field]` and `"hi"[->field]` still fell through to generic `expected expression, found Dot/Arrow` diagnostics while neighboring statement, return/operator, control, type-query, grouped/unary, call, and initializer routes were already contextual. TDD RED first captured those generic fallbacks; GREEN added narrow Dot/Arrow guards to `parse_index_expr()` while preserving valid postfix selector usage after real expressions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_array_index_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after index selector guards
cargo test --test interpreter index -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous selector-token grouped/unary diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance coverage, pointer negative diagnostics, fuzz/property testing, and CLI/product polish. Selected selector-token malformed grouped-expression and unary-operand boundaries because probing showed `(.field)`, `(->field)`, `!.field`, and `*->field` still fell through to generic `expected expression, found Dot/Arrow` diagnostics while adjacent return/operator/control/type-query/call/initializer routes were contextual. TDD RED first captured those generic fallbacks; GREEN added narrow Dot/Arrow guards in grouped-expression and unary-operator operand parsing while preserving valid postfix selector usage after real expressions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_grouped_expression_operands_with_context -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after grouped-expression selector guards
cargo test --test interpreter rejects_missing_operands_after_unary_operators -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after unary-operand selector guards
cargo test --test interpreter grouped -- --nocapture
cargo test --test interpreter unary -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous selector-token function-call/initializer diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance coverage, pointer negative diagnostics, and CLI/product polish. Selected selector-token malformed list/initializer boundaries because probing showed `f(.field)`, `f(1,->field)`, scalar array initializers like `{.field}` / `{->field}`, and aggregate initializers like `{->field = 1}` still fell through to generic `expected expression, found Dot/Arrow` diagnostics while adjacent return/operator/control/type-query routes were contextual. TDD RED first captured those generic fallbacks; GREEN added narrow function-call argument guards and braced-initializer selector-start guards that preserve valid struct `.field` designators.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_initial_function_call_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after call-argument selector guards
cargo test --test interpreter rejects_missing_braced_initializer_elements_with_context -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after braced-initializer selector guards
cargo test --test interpreter initializer -- --nocapture
cargo test --test interpreter function_call -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous selector-token control/conditional diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance coverage, pointer negative diagnostics, and product/CLI polish. Selected selector-token invalid starts in control/conditional expression boundaries because probing showed `for (.field; ; )`, `if (.field)`, `while (->field)`, `for (...; .field; ...)`, `for (...; ...; ->field)`, `return 1 ? .field : 2`, and `return 1 ? 2 : ->field` still fell through to generic `expected expression, found Dot/Arrow` or `unexpected token in for increment` diagnostics while adjacent return/operator/sizeof/cast routes were contextual. TDD RED first captured those generic fallbacks; GREEN added narrow Dot/Arrow guards to for-initializer, control-condition, for-increment, and conditional-operator operand parsing while preserving valid postfix selector usage after real expressions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_invalid_start_for_initializer_expressions_with_context -- --nocapture  # RED first: generic `unexpected token in for initializer: Dot`; GREEN passed after selector-token guard
cargo test --test interpreter rejects_missing_control_flow_condition_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after control-condition guard
cargo test --test interpreter rejects_misplaced_for_increment_expressions_with_context -- --nocapture  # RED first: generic `unexpected token in for increment: Dot`; GREEN passed after for-increment guard
cargo test --test interpreter rejects_missing_conditional_operator_operands_with_context -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after conditional-operand guard
cargo test --test interpreter missing_ -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous qualifier-only declaration-type diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected qualifier-only declaration contexts because `const global;`, `volatile local;`, `_Atomic value` parameters, aggregate fields, typedef aliases, and top-level `const make(...)`-shaped starts still fell through to route-specific generic type errors such as `expected struct type name` / `expected parameter type` instead of reporting the missing declaration type after the consumed qualifier. TDD RED first captured the generic `expected struct type name`; GREEN records the last consumed qualifier and rejects non-type starts with contextual `expected <context> after type qualifier '<qualifier>'...` diagnostics while preserving valid const/volatile/restrict/_Atomic qualified declarations and existing cast/sizeof/_Alignof qualifier diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_qualifier_only_declaration_types_with_context -- --nocapture  # RED first: generic `expected struct type name`; GREEN passed after qualifier-only declaration guard
cargo test --test interpreter qualifier -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous function-parameter keyword-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected malformed function parameter-list starts because delimiter, bracket, and question-token forms were contextual, but statement/control keywords such as `return` at the beginning of a parameter list or after a comma still fell through to the generic type parser (`expected type, found Return`). TDD RED first captured that generic parser error; GREEN added narrow parameter-list keyword-start guards while preserving valid type-name starts, old-style identifier-list diagnostics, variadic diagnostics, and ordinary function parameter parsing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_function_parameters_around_commas_with_context -- --nocapture  # RED first: generic `expected type, found Return`; GREEN passed after function-parameter keyword-start guards
cargo test --test interpreter function_parameter -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous parenthesized integer-constant operand diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected malformed parenthesized integer-constant expressions because enum initializers, array lengths, and switch case labels already had contextual cast/unary and invalid-start diagnostics, but forms such as `enum Bad { VALUE = () };`, `int values[(,)];`, `case (:`, `int values[(?)];`, and `(return)` still fell through to the generic `expected integer constant in parenthesized integer constant expression, found ...` message. TDD RED first captured the generic `found RParen`; GREEN added a narrow parenthesized integer-constant start guard while preserving valid parenthesized constants, comma-operator rejection after a parsed left operand, and existing cast/unary integer-constant diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_parenthesized_integer_constant_operands_with_context -- --nocapture  # RED first: generic `expected integer constant..., found RParen`; GREEN passed after parenthesized integer-constant guard
cargo test --test interpreter integer_constant -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous `_Alignof` selector-token invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected `_Alignof` selector-token malformed type operands because adjacent `_Alignas`, `_Static_assert`, `sizeof`, cast, and `_Atomic` routes already had contextual selector-token diagnostics, but `_Alignof(.)` and `_Alignof(->field)` still fell through to the generic non-type fallback. TDD RED first captured `expected _Alignof type, found Dot`; GREEN added narrow selector-token guards in `reject_missing_alignof_type()` while preserving valid `_Alignof(type-name)` operands and integer-constant `_Alignof` folding.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_sizeof_and_alignof_operands_with_context -- --nocapture  # RED first: generic `expected _Alignof type, found Dot`; GREEN passed after _Alignof selector-token guards
cargo test --test interpreter alignof -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

## Previous autonomous verification

All passed after the 2026-07-09 autonomous `_Atomic(...)` type-argument invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected `_Atomic(type-name)` malformed argument diagnostics because empty/comma `_Atomic()` / `_Atomic(,)` forms were contextual, but invalid starts such as `_Atomic([)`, `_Atomic(?)`, `_Atomic(return)`, `_Atomic(.)`, and `_Atomic(->field)` still delegated to generic type parsing on less-traveled declaration, parameter, `sizeof`, cast, and aggregate-field routes. TDD RED first captured `expected type, found LBracket`; GREEN added narrow `_Atomic` argument invalid-start guards while preserving valid bare `_Atomic` qualifiers and `_Atomic(type-name)` specifier behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_atomic_type_arguments_with_context -- --nocapture  # RED first: generic `expected type, found LBracket`; GREEN passed after _Atomic argument guards
cargo test --test interpreter atomic -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-09 autonomous selector-token type-query/assertion diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected malformed selector tokens in less-traveled operand routes because `_Alignas(.)`, `_Static_assert(., ...)`, `sizeof(.)`, and `(int).` (plus arrow variants) still delegated to generic expression parsing even though adjacent statement/return/operator expression-start routes already had contextual dot/arrow diagnostics. TDD RED first captured generic `expected expression, found Dot`; GREEN added dot/arrow to the route-local `_Alignas`, `_Static_assert`, cast-operand, and `sizeof` operand guards while preserving valid `_Alignas`, `_Static_assert`, cast, and `sizeof` behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter missing_ -- --nocapture  # RED first: generic `expected expression, found Dot`; GREEN passed after selector-token guards
cargo test --test interpreter sizeof -- --nocapture
cargo test --test interpreter cast -- --nocapture
cargo test --test interpreter alignas -- --nocapture
cargo test --test interpreter static_assert -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-09 autonomous braced scalar initializer invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected braced scalar initializer invalid-start diagnostics because scalar braced initializer contexts (`int value = {...}`, aggregate scalar-field braced initializers, and scalar compound literals) still delegated malformed starts such as `{[}`, `{.bad}`, and `{->field}` to generic primary-expression parsing, unlike adjacent braced aggregate/array initializer value diagnostics. TDD RED first captured `expected expression, found LBracket`; GREEN routed scalar braced initializer operands through the contextual invalid-start initializer guard while preserving valid scalar braced initializer behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_braced_scalar_initializer_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after scalar braced initializer guard reuse
cargo test --test interpreter rejects_missing_scalar_compound_literal_initializers_with_context -- --nocapture
cargo test --test interpreter initializer -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous integer-constant unary operand diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected integer-constant unary operand diagnostics because enum initializers, array lengths, and switch case labels use parser-folded integer constant expressions, and malformed unary constants such as `enum Bad { VALUE = + };`, `int values[-];`, and `case !:` still fell through to generic integer-constant errors after consuming the unary operator. TDD RED first captured `expected integer constant in integer constant expression`; GREEN added a narrow post-unary integer-constant operand guard while preserving supported unary `+`, `-`, `~`, and `!` integer-constant-expression operands.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_integer_constant_unary_operands_with_context -- --nocapture  # RED first: generic integer-constant error; GREEN passed after integer-constant unary operand guard
cargo test --test interpreter integer_constant -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous integer-constant cast operand diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, broader conformance fixtures, negative pointer-arithmetic coverage, and CLI/product polish. Selected integer-constant cast operand diagnostics because enum initializers, array lengths, and switch case labels use parser-folded integer constant expressions, and malformed scalar casts such as `enum Bad { VALUE = (int) };`, `int values[(int)];`, and `case (int):` still fell through to generic integer-constant errors. TDD RED first captured `expected integer constant in integer constant expression`; GREEN added a narrow post-cast integer-constant operand guard while preserving supported scalar casts in integer-constant-expression contexts.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_integer_constant_cast_operands_with_context -- --nocapture  # RED first: generic integer-constant error; GREEN passed after integer-constant cast operand guard
cargo test --test interpreter rejects_invalid_start_enum_constant_values_with_context -- --nocapture
cargo test --test interpreter rejects_invalid_start_switch_case_values_with_context -- --nocapture
cargo test --test interpreter rejects_non_constant_array_lengths_with_context -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous dot/arrow invalid-start expression diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, larger C-subset conformance fixtures, pointer-arithmetic negative coverage, and CLI/product polish. Selected dot/arrow expression-start diagnostics because malformed selector tokens in statement, return, comma, assignment, and binary RHS expression positions still fell through to generic statement/primary-expression parser errors while adjacent `[`/`?` and keyword-start routes were contextual. TDD RED first captured `unexpected token in statement: Dot`; GREEN added dot/arrow to the focused invalid-start guards while preserving valid selector use after postfix aggregate/pointer expressions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_invalid_start_expression_statements_with_context -- --nocapture  # RED first: generic `unexpected token in statement: Dot`; GREEN passed after statement-start guard expansion
cargo test --test interpreter rejects_missing_return_expressions_with_context -- --nocapture
cargo test --test interpreter rejects_misplaced_operator_rhs_operands_with_context -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous typedef keyword invalid-start expression diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, larger C-subset conformance fixtures, pointer-arithmetic negative coverage, and CLI/product polish. Selected typedef-keyword expression-context diagnostics because grouped expressions and declaration initializers still delegated malformed starts such as `(typedef)` and `int value = typedef;` to generic expression parsing even though adjacent expression/list contexts had keyword-start guards. TDD RED first captured generic `expected expression, found Typedef`; GREEN reused the shared invalid-start keyword classifier in grouped-expression and declaration-initializer guards while preserving delimiter, punctuation, and valid initializer/grouped-expression behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_grouped_expression_operands_with_context -- --nocapture  # RED first: generic `expected expression, found Typedef`; GREEN passed after grouped-expression keyword guard
cargo test --test interpreter rejects_missing_declaration_initializer_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Typedef`; GREEN passed after declaration-initializer keyword guard
cargo test --test interpreter grouped -- --nocapture
cargo test --test interpreter initializer -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous expression-list keyword invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, larger C-subset conformance fixtures, pointer-arithmetic negative coverage, and CLI/product polish. Selected expression-list keyword invalid starts because function-call arguments, array/string indexes, braced initializer elements, comma/assignment/binary RHS operands, and conditional branches still delegated statement/type keywords such as `return` and `int` to generic expression parsing. TDD RED first captured generic `expected expression, found Return/Int`; GREEN added contextual keyword-start guards while preserving delimiter/`[`/`?` diagnostics and valid expressions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_misplaced_function_call_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Return`; GREEN passed after call-argument keyword guard
cargo test --test interpreter rejects_missing_array_index_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Int`; GREEN passed after index keyword guard
cargo test --test interpreter rejects_missing_braced_initializer_elements_with_context -- --nocapture
cargo test --test interpreter rejects_missing_conditional_operator_operands_with_context -- --nocapture
cargo test --test interpreter rejects_misplaced_operator_rhs_operands_with_context -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous `_Alignas` / `_Static_assert` keyword invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, declaration-specifier/assertion malformed starts not already covered, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected `_Alignas` and `_Static_assert` keyword invalid starts because delimiter and `[`/`?` forms were contextual, but `_Alignas(return) int value;` and `_Static_assert(return, "condition required");` still delegated to generic expression parsing. TDD RED first captured `expected expression, found Return`; GREEN added keyword-start guards while preserving valid `_Alignas(type-name)`, `_Alignas(expression)`, and `_Static_assert` conditions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_alignas_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Return`; GREEN passed after _Alignas keyword guard
cargo test --test interpreter rejects_missing_static_assert_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Return`; GREEN passed after _Static_assert keyword guard
cargo test --test interpreter alignas -- --nocapture
cargo test --test interpreter static_assert -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous `_Alignof` invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed type-query starts not already covered, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected malformed `_Alignof` type operands because empty/comma-leading `_Alignof()` / `_Alignof(,)` and qualifier-only `_Alignof(volatile)` were contextual, but invalid type-start tokens such as `_Alignof([)`, `_Alignof(?)`, and `_Alignof(return)` still fell through to generic `_Alignof` type parsing. TDD RED first captured `expected _Alignof type, found LBracket`; GREEN added a shared `_Alignof` type-start guard used by ordinary expression parsing and integer-constant `_Alignof` folding while preserving valid `_Alignof(type-name)` operands.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_sizeof_and_alignof_operands_with_context -- --nocapture  # RED first: generic `expected _Alignof type, found LBracket`; GREEN passed after _Alignof type-start guard
cargo test --test interpreter alignof -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous `sizeof` invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed expression/type-query operands not already covered, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected malformed `sizeof` operands because delimiter and `[`/`?` starts were contextual, but keyword starts such as `sizeof return`, `sizeof int`, and `sizeof(return)` still delegated to generic expression parsing. TDD RED first captured `expected expression, found Return`; GREEN split the `sizeof` operand guard so parenthesized type names remain valid while unparenthesized type keywords and statement/control keywords report contextual `expected sizeof operand before '<token>'` diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_sizeof_and_alignof_operands_with_context -- --nocapture  # RED first: generic `expected expression, found Return`; GREEN passed after sizeof keyword-start guard
cargo test --test interpreter sizeof -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous cast-operand invalid-start diagnostic run. Ideation considered failing tests/builds (none on clean startup status), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed expression/list boundaries not already covered, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected malformed cast operands because delimiter-leading casts such as `(int);` were contextual, but invalid operand starts such as `(int)[`, `(char)?`, and `(int)return` still delegated to generic primary-expression parsing. TDD RED first captured `expected expression, found LBracket`; GREEN expanded `reject_missing_cast_operand()` to guard the same delimiter, invalid-start, and keyword tokens used by other expression operand diagnostics while preserving valid scalar, pointer, void, and compound-literal casts.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_cast_operands_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after cast-operand guard expansion
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-08 autonomous grouped-expression invalid-start diagnostic run. Ideation considered failing tests/builds (none on clean startup status), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed expression/list boundaries not already covered, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected malformed grouped-expression operands because delimiter-leading grouped expressions such as `()` and `(]` were contextual, but invalid operand starts such as `([)` and `(?)` still delegated to the generic primary-expression parser. TDD RED first captured `expected expression, found LBracket`; GREEN added `[` and `?` to the grouped-expression operand guard while preserving valid grouped expressions, casts, and existing missing-closing-paren diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_grouped_expression_operands_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after grouped-expression guard expansion
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous `for` initializer invalid-start diagnostic run. Ideation considered failing tests/builds (none on clean startup status), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed expression/list boundaries not already covered, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected malformed `for` initializer starts because `for ([; ; )`, `for (?; ; )`, and `for (,; ; )` still reported the generic `unexpected token in for initializer: ...` fallback while adjacent condition/increment and statement-expression routes had contextual diagnostics. TDD RED first captured `unexpected token in for initializer: LBracket`; GREEN added a narrow `for` initializer guard for invalid expression-start tokens while preserving declaration initializers, assignment/expression initializers, statement-only control-flow diagnostics, and valid `for` loops.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_invalid_start_for_initializer_expressions_with_context -- --nocapture  # RED first: generic `unexpected token in for initializer: LBracket`; GREEN passed after parser guard
cargo test --test interpreter for_initializer -- --nocapture
cargo test --test interpreter for_clauses -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous return/control keyword invalid-start diagnostic run. Ideation considered failing tests/builds (none on clean startup status), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed expression operand routes, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected keyword invalid-start diagnostics for return/control expressions because delimiter/`[`/`?` forms were contextual, but malformed starts such as `return int;`, `if (return)`, `while (int)`, `do ... while (struct)`, `for (...; return; ...)`, and `for (...; ...; struct)` still delegated to generic `expected expression` / `unexpected token in for increment` fallbacks. TDD RED first captured `expected expression, found Int`; GREEN added a shared keyword-start expression guard for return/control conditions and a matching for-increment guard while preserving existing delimiter, bracket/question, statement-only control-flow, and valid expression behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_keyword_start_return_and_control_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Int`; GREEN passed after keyword-start expression guards
cargo test --test interpreter expressions_with_context -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous array-designator keyword invalid-start diagnostic run. Ideation considered failing tests/builds (none on clean startup status), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed integer-constant routes, less-traveled declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected bounded/unbounded array designator keyword invalid-start diagnostics because designator indexes already handled delimiter starts and `[`/`?`, but malformed starts such as `[int]`, `[return]`, and `[struct]` still delegated to generic integer-constant parsing. TDD RED first captured `expected array designator index, found Int`; GREEN reused the shared integer-constant invalid-start keyword label guard for both bounded and unbounded designator index parsers while preserving existing delimiter, comma-expression, bounds, and valid constant-expression behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_invalid_start_array_designator_indexes_with_keyword_context -- --nocapture  # RED first: generic `expected array designator index, found Int`; GREEN passed after designator keyword guard
cargo test --test interpreter array_designator -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous expression-statement invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, less-traveled statement-start routes, malformed declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected expression-statement invalid-start diagnostics because statement-leading malformed tokens such as `[;`, `?;`, and `,;` still reported generic `unexpected token in statement: ...` instead of a contextual expression-statement diagnostic. TDD RED first captured the generic `unexpected token in statement: LBracket`; GREEN added a narrow statement-start guard for comma/colon/bracket/question tokens while preserving unmatched `)` / `]` diagnostics and block-ending `}` behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_invalid_start_expression_statements_with_context -- --nocapture  # RED first: generic `unexpected token in statement: LBracket`; GREEN passed after statement-start guard
cargo test --test interpreter statement -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous integer-constant keyword invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, less-traveled integer-constant-expression routes, malformed declaration/type-name starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected enum constant and switch `case` label keyword invalid-start diagnostics because `[`/`?` were contextual, but malformed values such as `enum Bad { FIRST = int };`, `enum Bad { FIRST = return };`, `case int:`, and `case return:` still delegated to generic integer-constant parsing. TDD RED first captured generic `expected integer constant..., found Int`; GREEN added a shared integer-constant invalid-start keyword label guard reused by enum constants, switch case labels, and existing array-length diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter invalid_start -- --nocapture  # RED first: generic `found Int`; GREEN passed after integer-constant invalid-start guard reuse
cargo test --test interpreter enum -- --nocapture
cargo test --test interpreter switch -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous unary-operator invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, less-traveled expression operand routes, malformed type-name/declaration starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected unary-operator invalid-start diagnostics because delimiter-only missing operands after prefix unary operators were contextual, but malformed starts such as `return ![;`, `return *?;`, and `return &return;` still delegated into generic expression parsing. TDD RED first captured `expected expression, found LBracket`; GREEN expanded the shared unary operand guard for invalid operand-start tokens and statement/type keywords while preserving valid unary operands such as `sizeof(...)` and `_Alignof(...)`.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_operands_after_unary_operators -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after unary invalid-start guard expansion
cargo test --test interpreter unary -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous array-length keyword invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, less-traveled declaration/type-name routes, malformed array length starts beyond delimiter/`?`/nested-`[` coverage, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected keyword/statement-token array length diagnostics because malformed fixed array lengths such as `int values[int];`, `char text[char]`, aggregate fields with `[struct]`, typedef aliases with `[return]`, and type-name lengths like `sizeof(int[while])` still delegated to generic integer-constant parsing. TDD RED first captured `expected array length, found Int`; GREEN added a shared array-length invalid-start label guard while preserving valid integer-constant-expression starts such as `sizeof(...)` and `_Alignof(...)`.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_delimiter_only_array_lengths_with_context -- --nocapture  # RED first: generic `expected array length, found Int`; GREEN passed after keyword/statement-token array-length guard
cargo test --test interpreter array_length -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous switch case-label invalid-start diagnostic run. Ideation considered failing tests/builds (clean pulled tree and focused switch suite available), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, less-traveled integer-constant-expression routes, additional type-query/compound-literal operand starts, pointer-arithmetic negative coverage, and broader conformance fixture work. Selected switch `case` label invalid-start diagnostics because enum value and array-length routes had contextual invalid-start guards, but malformed case labels such as `case [:` and `case ?:` still delegated to generic integer-constant parsing. TDD RED first captured `expected integer constant after switch case, found LBracket`; GREEN added switch-case value guards so these forms report source-located `expected integer constant after switch case before '['/'?'` diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_invalid_start_switch_case_values_with_context -- --nocapture  # RED first: generic `expected integer constant after switch case, found LBracket`; GREEN passed after switch-case invalid-start guards
cargo test --test interpreter switch -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous enum constant invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, less-traveled integer-constant-expression routes, type-query/compound-literal operand starts, additional pointer-arithmetic negative coverage, and broader conformance fixture work. Selected enum constant value invalid-start diagnostics because missing/delimiter enum values were contextual, but malformed starts such as `enum Bad { FIRST = [?] };` and `enum Bad { FIRST = ? };` still delegated to generic integer-constant parsing. TDD RED first captured `expected integer constant after enum constant '=', found LBracket/Question`; GREEN added enum-value guards so these forms report source-located `expected integer constant after enum constant '=' before '['/'?'` diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_invalid_start_enum_constant_values_with_context -- --nocapture  # RED first: generic `found LBracket/Question`; GREEN passed after enum-value invalid-start guards
cargo test --test interpreter enum -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-07 autonomous array-designator invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, less-traveled designator path variants, type-query/compound-literal operand starts, additional pointer-arithmetic negative coverage, and broader conformance-only fixture work. Selected invalid operand-start array designator indexes because delimiter-only designators (`[]`, `[;]`, `[,]`, `[}`) were contextual, but malformed starts such as `.values[[] = 1`, `.values[?] = 1`, `(int[]){[[] = 1}`, and `(int[]){[?] = 1}` still delegated to generic integer-constant parsing. TDD RED first captured `expected array designator index, found LBracket`; GREEN extended both bounded and unbounded array-designator index guards so `[` and `?` report source-located `expected array designator index before ...` diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_malformed_path_designators_with_context -- --nocapture  # RED first: generic `expected array designator index, found LBracket`; GREEN passed after array-designator invalid-start guard expansion
cargo test --test interpreter designator -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous function-parameter invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed function parameter starts, malformed declaration/type-name starts, additional expression/list boundaries, and broader conformance/tooling work. Selected invalid-start function parameter tokens because comma/`]` list-boundary forms already had contextual parameter diagnostics, but malformed starts such as `int add([ int b)` and `int add(int a,? int b)` still delegated to generic type parsing. TDD RED first captured `expected type, found LBracket`; GREEN extended the function-parameter list guards so `[` and `?` report source-located `expected function parameter...` diagnostics at the initial and after-comma positions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_function_parameters_around_commas_with_context -- --nocapture  # RED first: generic `expected type, found LBracket`; GREEN passed after parameter invalid-start guard expansion
cargo test --test interpreter function_parameter -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous declaration-specifier assertion invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed declaration-specifier/assertion arguments, malformed array designator indexes, generic parameter/type starts, and broader conformance/tooling work. Selected `_Alignas(...)` and `_Static_assert(...)` invalid operand-start diagnostics because delimiter-only forms already had contextual messages, but malformed starts such as `_Alignas([)` and `_Static_assert(?, "msg")` still delegated to generic expression parsing. TDD RED first captured the generic `expected expression, found LBracket`; GREEN extended both route-local guards so `[` and `?` report source-located contextual diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_alignas_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after _Alignas guard expansion
cargo test --test interpreter rejects_missing_static_assert_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after _Static_assert guard expansion
cargo test --test interpreter missing -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous array-length invalid-start diagnostic run. Ideation considered failing tests/builds (clean startup inspection and no active blocker), the remaining parser-trust continuation item in `status/todo.md`, malformed array-length tokens in declaration/type-name routes, designator path invalid starts, unsupported near-future C forms, and broader conformance fixture work. Selected invalid operand-start array lengths because delimiter-only forms such as `int values[);`, `int values[;`, and `sizeof(int[)` were contextual, but malformed starts such as `int values[?];` and `sizeof(int[[)` still delegated to generic integer-constant parsing. TDD RED first captured `expected array length, found Question`; GREEN extended the shared array-length helper so `?` and nested `[` starts report source-located `expected array length before '?'/'['` diagnostics across every array-length route.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_delimiter_only_array_lengths_with_context -- --nocapture  # RED first: generic `expected array length, found Question`; GREEN passed after guard expansion
cargo test --test interpreter array_lengths -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous operator RHS invalid-start diagnostic run. Ideation considered failing tests/builds (clean startup inspection and no active blocker), the remaining parser-trust continuation item in `status/todo.md`, invalid-start operator RHS operands, additional nested declaration/type-name delimiters, unsupported near-future C forms, and broader conformance/tooling work. Selected invalid operand-start tokens after comma, assignment, binary, and conditional operators because delimiter-only missing RHS forms were contextual, but malformed starts such as `return 1 + [;`, `value = [;`, `value += ?;`, `return 1, [;`, `return 1 ? [ : 2;`, and `return 1 ? 2 : [;` still delegated to generic expression parsing. TDD RED first captured the generic `expected expression, found LBracket`; GREEN extended the operator RHS guards so these malformed operator operands report source-located contextual diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_misplaced_operator_rhs_operands_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after operator-RHS guard expansion
cargo test --test interpreter rejects_missing_conditional_operator_operands_with_context -- --nocapture
cargo test --test interpreter operator -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous return invalid-start expression diagnostic run. Ideation considered failing tests/builds (clean startup inspection and no active blocker), the remaining parser-trust continuation item in `status/todo.md`, invalid-start return expressions, additional expression-list boundaries, unsupported near-future C forms, and broader conformance/tooling work. Selected malformed `return` value starts because delimiter-leading value returns such as `return ,;`, `return );`, and `return }` were contextual, but invalid operand-start tokens such as `return [;` and `return ?;` still delegated to generic expression parsing. TDD RED first captured the generic `expected expression, found LBracket`; GREEN extended the return-expression guard so these malformed returns report source-located `expected expression after return...` diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_return_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after return-expression guard expansion
cargo test --test interpreter return -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous braced-initializer designator-value invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed braced initializer value boundaries, additional declaration/type-name delimiters, and unsupported near-future C forms. Selected invalid-start initializer values after array/field designators because delimiter-only forms such as `[0] = ,` and `.x = ,` were contextual, but invalid operand-start tokens such as `int values[2] = {[0] = ?};` and `struct Point point = {.x = [?]};` still delegated to generic expression parsing. TDD RED first captured the generic `expected expression, found LBracket`; GREEN added a value-position braced-initializer guard that rejects `[`/`?` after designator `=` without breaking valid array designator starts.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_braced_initializer_elements_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after value-position initializer guard
cargo test --test interpreter initializer -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous `for` clause invalid-start diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining parser-trust continuation item in `status/todo.md`, malformed `for` initializer/condition/increment boundaries, additional list/delimiter routes, and unsupported near-future C forms. Selected malformed `for` condition/increment expressions because recent control-flow condition coverage included `if`/`while`/`do while`/`switch`, but `for (init; [; inc)` still delegated to generic expression parsing and `for (init; cond; [)` reported an uncontextual `unexpected token in for increment`. TDD RED first captured the generic `LBracket` failure; GREEN reused the shared control-condition guard for `for` conditions and added a narrow `for` increment invalid-start guard for `[`/`?`.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_control_flow_condition_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after for-condition guard
cargo test --test interpreter rejects_misplaced_for_increment_expressions_with_context -- --nocapture  # RED first: uncontextual for-increment failure; GREEN passed after increment guard
cargo test --test interpreter control -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous control-flow invalid-start condition diagnostic run. Ideation considered failing tests/builds (clean pull and no active blocker), the remaining parser-trust continuation item in `status/todo.md`, malformed expression-start tokens in control-flow headers, additional list/delimiter routes, and unsupported near-future C forms. Selected invalid-start control-flow conditions because delimiter-only `if`/`while`/`do while`/`switch` conditions were already contextual, but malformed starters such as `if ([)`, `while (?)`, `do { } while ([);`, and `switch ([)` still delegated to generic expression parsing and reported `expected expression, found ...`. TDD RED first captured the generic `LBracket` failure; GREEN extended the shared control-condition and switch-expression guards to report contextual `expected expression after <keyword>...` diagnostics while preserving valid control-flow parsing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_control_flow_condition_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after control-condition guard expansion
cargo test --test interpreter rejects_missing_switch_expressions_with_context -- --nocapture
cargo test --test interpreter control_flow -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous declaration-initializer invalid-start diagnostic run. Ideation considered failing tests/builds (clean pull and no active blocker), the remaining parser-trust continuation item in `status/todo.md`, nearby initializer/list delimiter diagnostics, malformed nested type-name routes, and unsupported near-future C forms. Selected invalid-start declaration initializer expressions because delimiter-only forms such as `int value = ;` and pointer/array/aggregate initializer separators already had contextual diagnostics, but invalid operand-start tokens such as `int value = [?];` and `int *slot = ?;` still delegated to generic expression parsing and reported `expected expression, found ...`. TDD RED first captured the generic `LBracket` failure; GREEN extended the shared declaration-initializer guard to report `expected initializer expression after '=' in ... declaration, found LBracket/Question` at the offending token while preserving existing scalar, pointer, array, and aggregate initializer routes.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_declaration_initializer_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after declaration-initializer guard expansion
cargo test --test interpreter initializer -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-06 autonomous misplaced array-index diagnostic run. Ideation considered failing tests/builds (clean pull and no active blocker), the checked broad C-subset backlog, the remaining parser-trust continuation item in `status/todo.md`, malformed nested declaration/type-name routes, invalid operand-start tokens in other expression contexts, and unsupported near-future C forms. Selected invalid-start array/string index operands because delimiter-only index expressions such as `values[]`, `values[;`, and `values[}` already had contextual diagnostics, but `values[[)` and `"hi"[?]` still delegated to generic primary-expression parsing and reported `expected expression, found ...`. TDD RED first captured the generic `LBracket` failure; GREEN extended the shared `parse_index_expr()` guard to report `expected array index expression, found LBracket/Question` at the offending token while preserving valid array, pointer, string, and aggregate-array indexing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_array_index_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after parse_index_expr guard expansion
cargo test --test interpreter array_index -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous comma-leading array-designator diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), already-completed broad C-subset backlog items, parser-trust continuation slices, malformed nested declaration/type-name routes, and unsupported near-future C forms. Selected comma-leading array designator indexes because the prior delimiter-only array-designator work covered empty `[]`, semicolon, and brace starts, but `[ , ]` in struct path designators and scalar array compound literals still delegated to generic integer-constant parsing. TDD RED first captured `.values[,]` reporting `expected array designator index, found Comma`; GREEN added comma guards in both bounded and unbounded array-designator index parsers.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_malformed_path_designators_with_context -- --nocapture  # RED first: generic `expected array designator index, found Comma`; GREEN passed after comma designator-index guards
cargo test --test interpreter designator -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous array-designator delimiter diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining generic parser-trust continuation item in `status/todo.md`, additional malformed nested declaration/type-name routes, unsupported near-future C forms, and a fresh malformed designator-path gap. Selected delimiter-only array designator indexes because empty `[ ]` already had a contextual `expected array designator index before ']'` diagnostic, but `[;]` and `[}` forms in struct path designators and scalar array compound literals still delegated to generic integer-constant parsing. TDD RED first captured `.values[;]` reporting `expected array designator index, found Semi`; GREEN added semicolon/brace guards in both bounded and unbounded array-designator index parsers.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_malformed_path_designators_with_context -- --nocapture  # RED first: generic `expected array designator index, found Semi`; GREEN passed after designator-index guards
cargo test --test interpreter designator -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous misplaced function-call argument diagnostic run. Ideation considered failing tests/builds (clean pull and no active blocker), the remaining parser-trust continuation item in `status/todo.md`, malformed declaration/type-name routes, compound-literal initializer gaps, and an adjacent function-call parser diagnostic gap. Selected misplaced function-call arguments because `take([)` and `take(?)`, including after-comma forms, still delegated to generic expression parsing even though delimiter-leading call arguments already had contextual diagnostics. TDD RED first captured `take([)` reporting `expected expression, found LBracket`; GREEN extended `parse_call_args()` guards so misplaced `[` and `?` report contextual function-call-argument diagnostics before generic primary-expression parsing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_misplaced_function_call_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after call-argument guard expansion
cargo test --test interpreter function_call -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous missing grouped-expression operand diagnostic run. Ideation considered failing tests/builds (clean pull and no active blocker), the remaining parser-trust continuation item in `status/todo.md`, malformed nested declaration/type-name routes, and a fresh expression parser diagnostic gap. Selected missing grouped-expression operands because `return ();`, `return (, 1);`, `return (;`, and `return (];` still delegated to generic primary-expression parsing after `(` was consumed, even though adjacent parser routes already provide contextual missing-operand diagnostics. TDD RED first captured `return ();` reporting `expected expression, found RParen`; GREEN added a narrow grouped-expression delimiter guard in `parse_primary()`.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_grouped_expression_operands_with_context -- --nocapture  # RED first: generic `expected expression, found RParen`; GREEN passed after grouped-expression guard
cargo test --test interpreter grouped -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous sizeof misplaced-operand diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining broad parser-trust continuation item in `status/todo.md`, malformed nested declaration/type-name routes, and less-traveled conformance fixtures. Selected a focused parser-trust slice for malformed `sizeof` expression operands because delimiter-only operands such as `sizeof()` were already contextual, but misplaced operand-start tokens such as `sizeof([)` and `sizeof(?)` still fell through to generic expression parsing. TDD RED first captured `sizeof([)` reporting `expected expression, found LBracket`; GREEN extended the shared `reject_missing_sizeof_operand()` guard so both `[` and `?` now report source-located `expected sizeof operand, found ...` before generic primary-expression parsing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_sizeof_and_alignof_operands_with_context -- --nocapture  # RED first: generic `expected expression, found LBracket`; GREEN passed after sizeof operand guard expansion
cargo test --test interpreter sizeof -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous aggregate-array field path-designator run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining open C-subset closure item in `status/todo.md`, parser-trust diagnostic gaps, and a more substantial supported-C initializer parity gap. Selected aggregate-array field path designators because Cust already supported scalar array-field path designators such as `.values[1] = 2` and aggregate-array variables, but rejected C-compatible nested aggregate-array field initializer paths such as `.points[1].x = 7` with `field 'points' is not an array for path designator`. TDD RED first captured that runtime/parser gap; GREEN added parser lowering for `StructFieldType::StructArray` path designators, including whole aggregate-array element designators (`.points[1] = {2, 3}`), nested scalar field designators (`.points[2].x = 5`), union aggregate-array field paths, and out-of-bounds diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter aggregate_array_field_path_designators -- --nocapture  # RED first: `field 'points' is not an array for path designator`; GREEN passed after StructArray path-designator support
cargo test --test interpreter rejects_struct_array_path_designators_outside_declared_length -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous semicolon/brace array-length diagnostic run. Ideation considered failing tests/builds (clean pulled tree, no active blockers), the now-complete broad feature backlog, and the open parser-trust continuation item in `status/todo.md`. Selected a narrow continuation of the shared array-declarator/type-name diagnostic work because `[` followed by `;` or `}` still fell through to the generic integer-constant parser as `expected array length, found Semi/RBrace`, while the previous run had only covered `)` and the older empty-`]` path. TDD RED first captured the generic semicolon diagnostic; GREEN added source-located `expected array length before ';'` and `expected array length before '}'` guards in the shared `expect_array_len()` helper while preserving existing `[]`, `[)`, `[*]`, non-constant identifier, comma-expression, and positive-length diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_delimiter_only_array_lengths_with_context -- --nocapture  # RED first: generic `expected array length, found Semi`; GREEN passed after shared semicolon/brace array-length guards
cargo test --test interpreter array_length -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous delimiter-only array-length diagnostic run. Ideation considered failing tests/builds (clean pulled tree, no active blockers), the completed broad C-subset backlog, and the open parser-trust queue item in `status/todo.md`. Selected a narrow but cross-route parser diagnostic work package for array declarators/type-name lengths where `[` was followed by `)`: local arrays, function parameters, aggregate fields, typedef aliases, and `sizeof(int[)` previously fell through to the generic integer-constant parser as `expected array length, found RParen`. TDD RED first captured the generic diagnostic; GREEN added a source-located `expected array length before ')'` guard in the shared `expect_array_len()` helper while preserving existing `[]`, `[*]`, non-constant identifier, comma-expression, and positive-length diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_delimiter_only_array_lengths_with_context -- --nocapture  # RED first: generic `expected array length, found RParen`; GREEN passed after shared array-length guard
cargo test --test interpreter array_length -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous control-flow missing-condition diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), completed roadmap/backlog status, and the next useful parser-trust gap. Selected a tightly scoped diagnostics work package for missing `if`/`while`/`do while`/`switch` controlling expressions because those routes still delegated delimiter-only conditions to generic expression parsing. TDD RED first showed `switch ()` and `if ()` reported generic `expected expression` messages; GREEN added source-located contextual guards and focused parser regression coverage.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_switch_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found RParen`; GREEN passed after parser guard
cargo test --test interpreter rejects_missing_control_flow_condition_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found RParen`; GREEN passed after shared control-condition guard
cargo test --test interpreter control_flow -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-05 autonomous `_Atomic(...)` missing-type diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the open C-subset closure queue item in `status/todo.md`, and parser-trust gaps around qualifier/specifier forms. Selected missing `_Atomic(type-name)` argument diagnostics as the best finishable work package because `_Atomic()` / `_Atomic(,)` were still falling through to generic type parsing across declaration, parameter, field, cast/type-query, and static-assert contexts. TDD RED first showed the generic `expected type, found RParen` diagnostic for `_Atomic()` globals; GREEN added a guarded `_Atomic(...)` type-argument parser and regression coverage for every shared route.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_atomic_type_arguments_with_context -- --nocapture  # RED first: generic `expected type, found RParen`; GREEN passed after parser guard
cargo test --test interpreter atomic -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous qualifier-only `sizeof`/`_Alignof` type diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the now-open C-subset closure queue item in `status/todo.md`, malformed nested declaration/type-name routes, unsupported near-future C forms, negative pointer/storage-root diagnostics, less-traveled inline enum/aggregate conformance contexts, and adjacent parser-trust gaps around type queries. The selected work package closes qualifier-only type-query diagnostics: `sizeof(volatile)`, `sizeof(_Atomic)`, `_Alignof(volatile)`, and `_Alignof(_Atomic)` now report `expected sizeof/_Alignof type after type qualifier '<qualifier>', found RParen` at the missing type location instead of falling through to the generic `expected type, found RParen`, while valid qualified type queries remain unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_qualifier_only_sizeof_and_alignof_types_with_context -- --nocapture  # RED first: generic `expected type, found RParen`; GREEN passed after parser guard
cargo test --test interpreter sizeof -- --nocapture
cargo test --test interpreter alignof -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous qualifier-only cast diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the checked C-subset/product backlog, malformed nested declaration/type-name routes, unsupported near-future C forms, negative pointer/storage-root diagnostics, less-traveled inline enum/aggregate conformance contexts, and adjacent parser-trust gaps around casts/type queries. The selected work package closes qualifier-only cast type diagnostics: `(const);`, `(volatile);`, and `(_Atomic);` now report `expected cast type after type qualifier '<qualifier>', found RParen` at the missing type location instead of falling through to the generic `expected type, found RParen`, while valid qualified casts and existing missing-cast-operand diagnostics remain unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_cast_types_after_qualifiers_with_context -- --nocapture  # RED first: generic `expected type, found RParen`; GREEN passed after parser guard
cargo test --test interpreter cast -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous return missing-expression diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the now-checked C-subset backlog, adjacent malformed statement/expression parser-trust gaps, unsupported near-future C forms, negative pointer/storage-root diagnostics, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes malformed `return` expressions: `return ,;`, `return );`, and `return }` now report `expected expression after return, found ...` at the offending delimiter instead of falling through to the generic primary-expression parser, while valid empty `return;` in void functions and existing return-shape diagnostics remain unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_return_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Comma`; GREEN passed after parser guard
cargo test --test interpreter return -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous void object/typedef diagnostic run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the remaining unchecked C-subset closure item in `status/todo.md`, malformed nested declaration/type-name routes, unsupported near-future C forms, negative pointer/storage-root diagnostics, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes unsupported `void` object and typedef forms: file-scope `void value;`, block-local `void value;`, aggregate fields `void value;`, and `typedef void Void;` now report targeted source-located diagnostics instead of function/type parser fallthroughs, while existing empty `void` parameter lists, named-void-parameter diagnostics, `void` functions, `(void)expr` casts, and `void *` diagnostics remain unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_void_object_and_typedef_forms_with_context -- --nocapture  # RED first: global `void value;` reported `expected '(' after function name`; GREEN passed after parser guards
cargo test --test interpreter void -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous scalar compound-literal missing-initializer diagnostic coverage run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the remaining unchecked C-subset closure item in `status/todo.md`, malformed nested declaration/type-name delimiter routes, unsupported near-future C forms, negative pointer/storage-root diagnostics, less-traveled inline enum/aggregate conformance contexts, and a narrow compound-literal parser-trust gap. The selected work package locks in missing initializer diagnostics for scalar compound literals: `(int){}`, `(char){,}`, and `(int){;}` now have exact regression coverage for `expected initializer element in braced scalar initializer for scalar compound literal, found ...` at the offending delimiter. Focused coverage passed immediately because the shared braced-scalar initializer guard already handled scalar compound literals, so this is parser-trust coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_scalar_compound_literal_initializers_with_context -- --nocapture  # immediate GREEN; existing shared guard already produced exact diagnostics
cargo test --test interpreter compound_literal -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous cast missing-operand diagnostic run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the remaining unchecked C-subset closure item in `status/todo.md`, malformed nested declaration/type-name delimiter routes, unsupported near-future C forms, negative pointer/storage-root diagnostics, less-traveled inline enum/aggregate conformance contexts, and a narrow cast-expression parser-trust gap. The selected work package closes missing operands after scalar, pointer, and void casts: `(int);`, `(int *) ;`, and `(void);` now report `expected expression after cast, found ...` at the offending delimiter instead of falling through to the generic primary-expression parser.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_cast_operands_with_context -- --nocapture  # RED first: generic `expected expression, found Semi`; GREEN passed after parser guard
cargo test --test interpreter cast -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous `_Alignas` missing-argument diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining unchecked C-subset closure item in `status/todo.md`, malformed nested declaration/type-name delimiter routes, unsupported near-future C forms, negative pointer/storage-root diagnostics, less-traveled inline enum/aggregate conformance contexts, and a narrow C11 `_Alignas` declaration-specifier parser gap. The selected work package closes the `_Alignas` malformed-argument slice: `_Alignas()`, `_Alignas(,)`, and `_Alignas(;` now report `expected _Alignas argument, found ...` at the offending delimiter instead of falling through to generic assignment-expression parsing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_alignas_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found RParen`; GREEN passed after parser guard
cargo test --test interpreter alignas -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous `_Static_assert` missing-argument diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining unchecked C-subset closure item in `status/todo.md`, malformed nested declaration/type-name delimiter routes, additional unsupported near-future C forms, negative pointer/storage-root diagnostics, less-traveled inline enum/aggregate conformance contexts, and a narrow C11 `_Static_assert` argument-list parser gap. The selected work package closes the `_Static_assert` malformed-argument slice: delimiter-leading conditions such as `_Static_assert(, "msg")` now report `expected _Static_assert condition, found ...` at the offending delimiter instead of the generic expression parser, while missing message expressions such as `_Static_assert(1, )` are locked in with existing exact string-literal diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_static_assert_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Comma`; GREEN passed after parser guard
cargo test --test interpreter static_assert -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous non-char string initializer type-mismatch diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked C-subset closure item in `status/todo.md`, nested declaration/type-name delimiter routes, non-`char` string initializer type-mismatch source locations, malformed designator variants, additional negative pointer/storage-root diagnostics, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes the string-initializer type-mismatch source-location slice: fixed non-char arrays (`int values[3] = "nope"`), inferred non-char arrays (`int values[] = "nope"`), and non-char array compound literals (`(int[]){"nope"}`) now report their existing `string literal initializer requires char array ...` diagnostics with exact `at line ..., column ...` metadata at the offending string literal token instead of returning unlocated parser errors.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter reports_non_char_array_string_initializer_type_mismatches_with_source_locations -- --nocapture  # RED first: unlocated non-char string initializer diagnostics; GREEN passed after preserving LocatedToken
cargo test --test interpreter string_initializer -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous inferred-array missing-initializer diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, inferred `[]` declaration-list tails, nested declaration/type-name delimiter routes, non-`char` string initializer source locations, malformed designator variants, negative pointer/storage-root diagnostics, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes the inferred-array delimiter slice: scalar inferred array declarations (`int values[] = ;`), scalar declaration-list tails (`int first[] = {1}, second[] = ;`), aggregate inferred array declarations (`struct Point points[] = ;`), and aggregate declaration-list tails now report `expected initializer expression after '=' in inferred array/inferred aggregate array declaration, found ...` at the delimiter instead of falling through to generic compound-literal brace diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_inferred_array_initializer_expressions_with_context -- --nocapture  # RED first: generic compound-literal brace diagnostics; GREEN passed after parser guard
cargo test --test interpreter inferred_array -- --nocapture
cargo test --test interpreter inferred_aggregate_array -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-04 autonomous char-array string initializer source-location diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, inferred-array initializer delimiter routes, nested declaration/type-name delimiter gaps, non-`char` string initializer type-mismatch source locations, malformed designator path variants, negative pointer/storage-root diagnostics, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes a string-initializer diagnostic source-location gap: fixed char array declarations (`char short_text[2] = "abc"`), aggregate char-array field initializers (`struct Label label = {"toolong"}`), and fixed-size char array compound literals (`(char[3]){"four"}`) now report their existing too-long string diagnostics with exact `at line ..., column ...` metadata at the offending string literal token instead of returning unlocated parser errors.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter string_initializer_too_long -- --nocapture  # RED first: unlocated too-long string diagnostics; GREEN passed after preserving LocatedToken
cargo test --test interpreter array_compound_literal_string_initializers_that_are_too_long -- --nocapture  # RED first corrected expected string column to 28; GREEN passed
cargo test --test interpreter initializer -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous array/aggregate-array missing-initializer diagnostic run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the next unchecked parser/runtime parity gap in `status/todo.md`, malformed nested declaration/type-name routes, declaration-list edge cases after comma-separated pointer/aggregate declarators, remaining compound-literal/string-initializer source-location gaps, negative pointer/storage-root coverage, and less-traveled enum/inline aggregate conformance contexts. The selected work package closes a declaration-list parser-trust slice: fixed scalar array declarations (`int values[2] = ;`), array declaration-list tails (`int a[1] = {1}, b[2] = ;`), fixed aggregate-array declarations (`struct Point points[1] = ;`), and aggregate-array declaration-list tails now report `expected initializer expression after '=' in array/struct array declaration, found ...` at the delimiter instead of the less-specific `expected '{' after ... initializer` diagnostic.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_declaration_initializer_expressions_with_context -- --nocapture  # RED first: fixed arrays reported `expected '{' after array initializer`; GREEN passed after parser guard
cargo test --test interpreter initializer -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous malformed path-designator diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked parser/runtime parity gap in `status/todo.md`, malformed nested declaration/type-name routes, declaration-list edge cases after comma-separated pointer/aggregate declarators, remaining compound-literal/string-initializer source-location gaps, negative pointer/storage-root coverage, and less-traveled enum/inline aggregate conformance contexts. The selected work package closes a path-designator parser-trust slice: malformed struct initializer designators such as `.inner. = 1`, `.values[] = 1`, and `.values[;] = 1` now have exact regression coverage, and empty array path designators now report `expected array designator index before ']' at line ..., column ...` instead of the generic integer-constant diagnostic.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_malformed_path_designators_with_context -- --nocapture  # RED first: `.values[]` reported generic `expected array designator index, found RBracket`; GREEN passed after parser guard
cargo test --test interpreter designator -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous compound-literal excess-initializer diagnostic source-location run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the next unchecked parser/runtime parity gap in `status/todo.md`, malformed designator paths, declaration-list edge cases after comma-separated declarators, negative pointer/storage-root diagnostics, less-traveled inline enum/aggregate contexts, and the candidate compound-literal excess-initializer source-location gap. The selected work package completes that parser-trust slice: fixed-size scalar array compound literals and aggregate-array compound literals now report `too many initializers for ... compound literal at line ..., column ...` at the first excess initializer token instead of returning unlocated diagnostics. Existing invalid compound-literal tests were tightened to assert exact locations.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter compound_literals_longer_than_declared_length -- --nocapture  # RED first: unlocated too-many-initializers diagnostics; GREEN passed after parser fix
cargo test --test interpreter initializers -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous excess braced-initializer diagnostic source-location run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, malformed designator paths, declaration-list edge cases, scalar braced initializer excess-entry delimiter variants, negative pointer/storage-root diagnostics, and remaining less-traveled inline enum/aggregate conformance contexts. The selected work package closes a source-location gap for excess braced initializer entries: scalar braced initializers (`int value = {1, 2}`), scalar arrays, scalar struct/union initializers, nested struct-field initializers, and struct-array initializers now report `too many initializers for ... at line ..., column ...` at the first excess initializer token instead of returning an unlocated parser error. Existing too-many-initializer tests were tightened to assert exact locations.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter reports_too_many_braced_initializer_entries_with_source_locations -- --nocapture  # RED first: unlocated `too many initializers for variable 'value'`; GREEN passed
cargo test --test interpreter initializers -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous braced scalar initializer missing-expression diagnostic run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, malformed scalar braced initializer delimiters adjacent to the previous braced aggregate/array initializer work, malformed designator paths, declaration-list edge cases, negative pointer/storage-root diagnostics, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes a scalar initializer parser diagnostic gap: `int value = {,};`, `int value = {};`, and aggregate scalar-field forms such as `struct Point point = { .x = {,} };` now report `expected initializer element in braced scalar initializer for ...` at the offending delimiter instead of falling through to generic expression parsing. The parser-local guard runs immediately after the scalar initializer opening brace and preserves valid braced scalar initializers plus optional trailing commas.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_braced_scalar_initializer_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Comma`; GREEN passed
cargo test --test interpreter braced -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous braced-initializer missing-element diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, additional malformed declaration/type-name delimiter routes, negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes a newly discovered braced initializer parser diagnostic gap: scalar arrays (`int values[2] = {, 1}`), array designators (`[0] = ,`), struct/union initializers (`struct Point p = {, 1}` and `.x = ,`), and aggregate-array initializers (`struct Point points[2] = {, {1, 2}}`) now report `expected initializer element in ... initializer, found ...` at the offending delimiter instead of falling through to generic expression parsing. The parser-local guard is shared across array, array-compound-literal, struct/union, struct-array, and aggregate-array-compound-literal initializer loops while preserving empty `{}` and trailing-comma initializer forms.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_braced_initializer_elements_with_context -- --nocapture  # RED first: generic `expected expression, found Comma`; GREEN passed
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous declaration-initializer missing-expression diagnostic run. Ideation considered failing tests/builds (clean tree after pull; focused test suite compiled before the selected regression), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, fresh malformed declaration/type-name delimiter routes, negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes a newly discovered declaration initializer diagnostic gap: `int value = ;`, declaration-list tails like `int value = 1, other = ;`, pointer declarations like `int *slot = ,`, and aggregate declarations like `struct Point point = ;` now report `expected initializer expression after '=' in ... declaration, found ...` at the delimiter instead of falling through to the generic primary-expression parser. The fix is parser-local and preserves valid zero-initialized omitted declarations plus braced aggregate initializers.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_declaration_initializer_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found Semi/Comma`; GREEN passed
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous missing struct-field-name diagnostic coverage run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, whether missing field-name diagnostics after direct `.` and pointer `->` selectors needed stronger exact-output coverage, additional malformed declaration/type-name delimiter routes, negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package locks in exact parser diagnostics for malformed field selectors: `p.;` / `p.];` now have regression coverage for `expected struct field name after '.', found ...`, and `q->;` / `q->];` now have regression coverage for `expected struct field name after '->', found ...`. Focused coverage passed immediately because the existing shared `expect_ident_after(...)` route already preserved the contextual diagnostic, so this is parser-trust coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_struct_field_names_after_dot_and_arrow_with_context -- --nocapture  # immediate GREEN; existing parser helper already produced exact contextual diagnostics
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous misplaced closing-bracket call/parameter diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked C-subset parser-trust closure item in `status/todo.md`, field-name diagnostics after `.`/`->` (current `expect_ident_after` routes already preserve exact output), negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes a newly discovered malformed delimiter gap adjacent to the recent function call/parameter comma work: `first(]` now reports `expected function call argument, found RBracket`, `add(1,]` reports `expected function call argument after ',', found RBracket`, `int add(] int b)` reports `expected function parameter, found RBracket`, and `int add(int a,] int b)` reports `expected function parameter after ',', found RBracket` instead of falling through to generic expression/type diagnostics. Existing valid calls/definitions and trailing-comma diagnostics stayed green in focused regression checks.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_initial_function_call_arguments_with_context -- --nocapture  # RED first for `first(]`: generic `expected expression, found RBracket`; GREEN passed
cargo test --test interpreter rejects_missing_function_parameters_around_commas_with_context -- --nocapture  # RED first for `int add(] int b)`: generic `expected type, found RBracket`; GREEN passed
cargo test --test interpreter reports_trailing_commas -- --nocapture
cargo test --test interpreter supports_function_definitions_calls_and_parameters -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-03 autonomous malformed function-parameter comma diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked C-subset closure item in `status/todo.md`, remaining malformed-source parser trust gaps, missing field-name diagnostics after `.`/`->` (already targeted in current parser routes), negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes a fresh function-parameter list gap: leading parameter commas (`int add(, int b)`) now report `expected function parameter, found Comma`, and doubled parameter separators (`int add(int a,, int b)`) now report `expected function parameter after ',', found Comma`, instead of falling through to the generic parameter type parser. Existing trailing-comma diagnostics and valid function definitions/calls stayed green in focused regression checks.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_function_parameters_around_commas_with_context -- --nocapture  # RED first for leading comma: generic `expected type, found Comma`; GREEN passed
cargo test --test interpreter reports_trailing_commas_in_function_parameter_lists -- --nocapture
cargo test --test interpreter reports_trailing_commas_after_pointer_parameters -- --nocapture
cargo test --test interpreter supports_function_definitions_calls_and_parameters -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-02 autonomous comma-leading function-call argument diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the next unchecked parser-trust closure item in `status/todo.md`, malformed function-call separators adjacent to the previous missing-initial-argument work, missing field-name coverage for `.`/`->` (already targeted by `expect_ident_after` in smoke checks), negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes a fresh malformed-source parser gap for comma-leading call arguments: `first(,)` and `add(, 2)` now report `expected function call argument, found Comma` at the comma instead of falling through to the generic primary-expression diagnostic. Valid function calls, empty zero-argument calls, and existing trailing-comma diagnostics remained unchanged in focused regression checks.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_initial_function_call_arguments_with_context -- --nocapture  # RED first for `first(,)`: generic `expected expression, found Comma`; GREEN passed
cargo test --test interpreter reports_trailing_commas_in_function_call_argument_lists -- --nocapture
cargo test --test interpreter supports_function_definitions_calls_and_parameters -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-02 autonomous function-call missing-initial-argument diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining generic parser-trust closure item in `status/todo.md`, malformed-source diagnostics adjacent to the recent expression/delimiter work, negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes the next parser-trust gap for malformed function calls: `first(;` and `first(}` now report `expected function call argument, found ...` at the delimiter/terminator instead of falling through to the generic primary-expression diagnostic. Valid function-call behavior and existing trailing-comma call diagnostics remained unchanged in focused regression checks.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_initial_function_call_arguments_with_context -- --nocapture  # RED first: generic `expected expression, found Semi`; GREEN passed
cargo test --test interpreter supports_function_definitions_calls_and_parameters -- --nocapture
cargo test --test interpreter reports_trailing_commas_in_function_call_argument_lists -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-02 autonomous array-index missing-expression diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining generic parser-trust closure item in `status/todo.md`, malformed-source diagnostics adjacent to the recent comma/conditional/assignment/binary/unary/`sizeof` missing-operand work, negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes the next parser-trust gap for malformed array, string, and postfix index expressions: `values[]`, `values[;`, `"hi"[]`, and `values[}` now report `expected array index expression, found ...` at the delimiter/terminator instead of falling through to the generic primary-expression diagnostic. Valid array, pointer, string, and aggregate-array indexing behavior remained unchanged in focused regression checks.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_array_index_expressions_with_context -- --nocapture  # RED first: generic `expected expression, found RBracket`; GREEN passed
cargo test --test interpreter array -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-02 autonomous `sizeof`/`_Alignof` missing-operand diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the remaining generic parser-trust closure item in `status/todo.md`, malformed-source diagnostics adjacent to the recent comma/conditional/assignment/binary/unary missing-operand work, unsupported `sizeof`/`_Alignof` delimiter forms, negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes the next parser-trust gap for malformed `sizeof` operands: `sizeof()`, `sizeof(,)`, and unparenthesized `sizeof;` now report `expected sizeof operand, found ...` at the delimiter/terminator instead of falling through to the generic primary-expression diagnostic. Adjacent `_Alignof()` / `_Alignof(,)` delimiter coverage was locked in with its existing targeted type diagnostic, and valid `sizeof`/`_Alignof` behavior remained unchanged in focused regression checks.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_sizeof_and_alignof_operands_with_context -- --nocapture  # RED first for sizeof(): generic `expected expression, found RParen`; GREEN passed
cargo test --test interpreter supports_sizeof_operator_for_types_scalars_arrays_strings_and_pointers -- --nocapture
cargo test --test interpreter supports_alignof_type_names -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test  # passed; non-fatal missing-buildx warning emitted
docker compose run --rm cust  # passed; non-fatal missing-buildx warning emitted, printed 10
```

All passed after the 2026-07-02 autonomous unary-operator missing-operand diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the already-completed high-level roadmap, the only unchecked generic C-subset closure item in `status/todo.md`, malformed-source parser diagnostics adjacent to the recent comma/conditional/assignment/binary missing-operand work, negative pointer/storage-root coverage, and less-traveled enum/aggregate conformance contexts. The selected work package closes the next parser-trust gap for malformed prefix unary expressions: `++`, `--`, unary `+`, unary `-`, `~`, `!`, dereference `*`, and address-of `&` now detect delimiter/terminator tokens immediately after the operator and report `expected expression after unary operator '<op>', found ...` at the offending token instead of falling through to generic primary-expression diagnostics. Valid unary, bitwise, logical, and prefix/postfix increment behavior remained unchanged in focused regression checks.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_operands_after_unary_operators -- --nocapture  # RED first: generic `expected expression, found Semi`; GREEN passed
cargo test --test interpreter supports_logical_operators_short_circuiting_and_unary_plus -- --nocapture
cargo test --test interpreter supports_bitwise_and_shift_operators_with_c_precedence -- --nocapture
cargo test --test interpreter supports_increment_decrement_for_scalar_indexed_and_deref_lvalues -- --nocapture
cargo test --test interpreter reports_line_and_column_for_parser_expression_errors -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test  # passed; non-fatal missing-buildx warning emitted
docker compose run --rm cust  # passed; non-fatal missing-buildx warning emitted, printed 10
```

All passed after the 2026-07-02 autonomous binary-operator missing-RHS diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the already-completed high-level roadmap, the only unchecked generic C-subset closure item in `status/todo.md`, malformed-source parser diagnostics adjacent to the recent comma/conditional/assignment missing-operand work, negative pointer/storage-root coverage, and less-traveled enum/aggregate conformance contexts. The selected work package closes a parser-trust gap for malformed binary expressions: logical, bitwise, equality/comparison, shift, additive, multiplicative, and remainder operators now detect delimiter/terminator tokens immediately after the operator and report `expected expression after binary operator '<op>', found ...` at the offending token instead of falling through to the generic primary-expression diagnostic. Valid operator precedence and parsing behavior remained unchanged in focused regression checks.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_rhs_after_binary_operators -- --nocapture  # RED first: generic `expected expression, found Semi`; GREEN passed
cargo test --test interpreter supports_bitwise_and_shift_operators_with_c_precedence -- --nocapture
cargo test --test interpreter supports_logical_operators_short_circuiting_and_unary_plus -- --nocapture
cargo test --test interpreter respects_arithmetic_precedence_with_unary_and_remainder -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test  # passed; non-fatal missing-buildx warning emitted
docker compose run --rm cust  # passed; non-fatal missing-buildx warning emitted, printed 10
```

All passed after the 2026-07-02 autonomous assignment-operator missing-RHS diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, malformed-source parser diagnostics adjacent to recent comma/conditional missing-operand work, negative pointer/storage-root coverage, and remaining less-traveled enum/aggregate conformance contexts. The selected work package closes a parser-trust gap for malformed assignment expressions and assignment statements: `=`, compound assignment operators such as `+=`, and shift/bitwise compound operators such as `<<=` now detect delimiter/terminator tokens immediately after the assignment operator and report `expected expression after assignment operator '<op>', found ...` at the offending token instead of a generic primary-expression diagnostic. The check covers expression-level assignments and statement-level scalar, dereference, array, struct-field, and embedded-array assignment routes.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_rhs_after_assignment_operators -- --nocapture  # RED first: generic `expected expression, found Semi`; GREEN passed
cargo test --test interpreter assignment -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test  # passed; non-fatal missing-buildx warning emitted
docker compose run --rm cust  # passed; non-fatal missing-buildx warning emitted, printed 10
```

All passed after the 2026-07-02 autonomous conditional-operator missing-operand diagnostic run. Ideation considered failing tests/builds (`cargo test` passed on the clean pulled tree), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, malformed-source parser diagnostics adjacent to recent comma/delimiter work, negative pointer/storage-root coverage, and less-traveled enum/aggregate conformance contexts. The selected work package closes a parser-trust gap for malformed conditional expressions: missing `?:` then-operands and missing else-operands now report `expected expression after '?' in conditional operator, found ...` or `expected expression after ':' in conditional operator, found ...` at the delimiter/terminator instead of a generic primary-expression diagnostic, while valid conditional-expression parsing and existing missing-colon diagnostics remain unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_missing_conditional_operator_operands_with_context -- --nocapture  # RED first: generic `expected expression, found Colon`; GREEN passed
cargo test --test interpreter conditional -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-02 autonomous comma-operator missing-RHS diagnostic run. Ideation considered failing tests/builds (none in the clean pulled tree), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, malformed-source parser diagnostics adjacent to recent delimiter/control-flow work, negative pointer/storage-root coverage, and less-traveled enum/aggregate conformance contexts. The selected work package closes a parser-trust gap for malformed comma expressions: when the comma operator is followed by a delimiter or statement terminator, Cust now reports `expected expression after comma operator, found ...` at the delimiter instead of the generic primary-expression diagnostic, while valid comma-expression behavior remains unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_missing_rhs_after_comma_operator -- --nocapture  # RED first: generic `expected expression, found RParen`; GREEN passed
cargo test --test interpreter supports_conditional_and_comma_expressions_for_aggregates -- --nocapture
cargo test --test interpreter supports_sizeof_comma_expression_rhs_types_without_evaluating_operands -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-02 autonomous top-level unmatched-delimiter diagnostic run. Ideation considered failing tests/builds (none in the clean pulled tree), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, parser-trust gaps adjacent to recent top-level/control-flow diagnostic work, additional malformed-source exact diagnostics, negative pointer/storage-root coverage, and less-traveled enum/aggregate conformance contexts. The selected work package closes a fresh malformed-source parser gap: unmatched top-level `)` and `]` now report targeted source-located diagnostics (`unmatched ')' at top level`, `unmatched ']' at top level`) instead of generic `unexpected token at top level: RParen/RBracket`, while the existing unmatched top-level `}` and valid control-flow diagnostics remain unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_unmatched_top_level_delimiters_with_context -- --nocapture  # RED first: generic `unexpected token at top level: RParen`; GREEN passed
cargo test --test interpreter rejects_unhandled_control_flow_with_context -- --nocapture
cargo test --test interpreter rejects_statement_only_control_flow_in_for_clauses_with_context -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-02 autonomous `for`-clause control-flow diagnostic run. Ideation considered failing tests/builds (none in the clean pulled tree), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, parser-trust gaps adjacent to the recent top-level/stray control-flow work, additional malformed-source exact diagnostics, negative pointer/storage-root coverage, and less-traveled enum/aggregate conformance contexts. The selected work package closes the next malformed-control-flow parser gap: statement-only control-flow keywords in `for` initializer/increment clauses now report targeted source-located diagnostics (`return is not allowed in for initializer`, `if statement is not allowed in for increment`, `case label outside switch`, etc.) instead of generic `unexpected token in for initializer/increment: ...`, while valid `for` loop `break`/`continue` behavior and prior top-level/function-body diagnostics remain unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_statement_only_control_flow_in_for_clauses_with_context -- --nocapture  # RED first: generic `unexpected token in for initializer: Return`; GREEN passed
cargo test --test interpreter supports_break_and_continue_in_while_and_for_loops -- --nocapture
cargo test --test interpreter rejects_unhandled_control_flow_with_context -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-07-01 autonomous stray control-flow diagnostic run. Ideation considered failing tests/builds (none in the clean pulled tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, parser-trust gaps adjacent to the just-completed top-level `return`/`break`/`continue` work, additional malformed-source exact diagnostics, negative pointer/storage-root coverage, and less-traveled inline enum/aggregate conformance contexts. The selected work package closes the next control-flow parser diagnostic gap: file-scope `if`, `else`, `while`, `do`, `for`, `switch`, `case`, and `default` now report targeted source-located diagnostics instead of generic `unexpected token at top level`, and unmatched `else` inside function bodies now reports `else without matching if at line ..., column ...`. Valid loop/switch control-flow behavior remains unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_unhandled_control_flow_with_context -- --nocapture  # RED first: generic `unexpected token at top level: If`; GREEN passed
cargo test --test interpreter supports_break_and_continue_in_while_and_for_loops -- --nocapture
cargo test --test interpreter supports_switch_statements_with_cases_default_fallthrough_break_and_continue -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: Ideation considered failing tests/builds (`cargo fmt --check` passed during the run and no active blocker was present), the only unchecked generic C-subset closure item in `status/todo.md`, more malformed-source exact diagnostics, negative pointer/storage-root coverage, and remaining less-traveled inline enum/aggregate contexts. The selected work package closes a parser-trust gap for control-flow keywords at file scope: top-level `return;`, `break;`, and `continue;` previously fell through to generic `unexpected token at top level: ...` errors even though Cust has targeted runtime diagnostics for the same unhandled flows. Cust now reports `return outside function at line ..., column ...`, `break outside loop or switch at line ..., column ...`, and `continue outside loop at line ..., column ...` directly from top-level parsing, while existing function-body invalid and valid loop/switch control-flow behavior remains unchanged.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_unhandled_control_flow_with_context -- --nocapture  # RED first: generic `unexpected token at top level: Return`; GREEN passed
cargo test --test interpreter rejects_break_outside_loops -- --nocapture
cargo test --test interpreter rejects_continue_outside_loops -- --nocapture
cargo test --test interpreter supports_break_and_continue_in_while_and_for_loops -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous break/continue source-location diagnostic run. Ideation considered failing tests/builds (focused baseline was clean), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, additional unsupported type-specifier diagnostics, less-traveled inline enum/aggregate conformance contexts, negative pointer/storage-root diagnostics, and malformed-source exact diagnostics. The selected work package closes a runtime diagnostic trust gap for unhandled control-flow statements: `break;` and `continue;` parsed with source locations but the runtime `ExecFlow` dropped them before reporting `break outside loop` / `continue outside loop`. Cust now carries `LocatedToken` metadata through `Stmt::Break`/`Stmt::Continue` and `ExecFlow`, reports `break outside loop or switch at line ..., column ...` and `continue outside loop at line ..., column ...`, and still consumes valid loop/switch `break`/`continue` normally.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_unhandled_break_and_continue_with_context -- --nocapture  # RED first: unlocated `break outside loop`; GREEN passed
cargo test --test interpreter rejects_break_outside_loops -- --nocapture
cargo test --test interpreter rejects_continue_outside_loops -- --nocapture
cargo test --test interpreter supports_break_and_continue_in_while_and_for_loops -- --nocapture
cargo test --test interpreter supports_switch_statements_with_cases_default_fallthrough_break_and_continue -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous unsupported complex type diagnostic run. Ideation considered failing tests/builds (none in the clean pulled worktree after focused baseline), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, additional negative pointer/storage-root diagnostics, and unsupported C numeric type specifiers adjacent to the just-completed floating-point diagnostics. The selected work package closes a parser-trust gap around unsupported C `_Complex` and `_Imaginary` type specifiers: they previously lexed as identifiers and produced generic top-level/type/cast errors. Cust now lexes them as dedicated tokens and reports `complex types are not supported` at the offending keyword in top-level declarations, block locals, parameters, `sizeof` type names, casts, and aggregate fields, without adding complex/imaginary runtime semantics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_complex_type_specifiers_with_context -- --nocapture  # RED first: generic unexpected token at top level for `_Complex`; GREEN passed
cargo test --test interpreter rejects_floating_point_type_specifiers_with_context -- --nocapture
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous unsupported floating-point type diagnostic run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, additional negative pointer/storage-root diagnostics, and remaining less-traveled direct enum/inline aggregate contexts. The selected work package closes a parser-trust gap around unsupported C floating-point type specifiers: `float` and `double` previously lexed as identifiers and produced generic top-level/statement/type-query/cast errors. Cust now lexes them as dedicated tokens and reports `floating-point types are not supported` at the offending keyword in top-level declarations, block locals, parameters, `sizeof` type names, casts, and aggregate fields, without adding floating-point runtime semantics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_floating_point_type_specifiers_with_context -- --nocapture  # RED first: generic unexpected token at top level for `float`; GREEN passed
cargo fmt --check
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous parenthesized pointer cast/type-query diagnostic run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, additional negative pointer/storage-root diagnostics, and remaining less-traveled direct enum/inline aggregate contexts. The selected work package closes another parser-trust gap around unsupported parenthesized pointer type syntax in expression/type-query contexts: `(int (*)[3])0`, `sizeof(int (*)[3])`, and `_Alignof(int (*)[3])` previously fell through to generic closing-delimiter errors at the inner `(`; Cust now reports targeted parenthesized-pointer unsupported diagnostics at the `*` token while preserving supported ordinary pointer casts and `sizeof`/`_Alignof` type-name forms.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_parenthesized_pointer_cast_and_type_query_forms_with_context -- --nocapture  # RED first: generic `expected ')' after cast type, found LParen`; GREEN passed
cargo test --test interpreter supports_pointer_cast_expressions -- --nocapture
cargo test --test interpreter rejects_pointer_casts_that_discard_const_pointees -- --nocapture
cargo test --test interpreter supports_sizeof_array_type_names -- --nocapture
cargo test --test interpreter supports_alignof_type_names -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous unsupported `void *` diagnostics run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the only unchecked generic C-subset closure item in `status/todo.md`, more malformed-source exact diagnostics, negative pointer/storage-root diagnostics, less-traveled inline/direct enum and aggregate contexts, and the documented pointer-model non-goal `void *`. The selected work package closes a parser-trust gap: unsupported `void *` forms previously fell through to route-specific generic diagnostics such as `expected function name after return type`, `void parameter lists must be empty`, `(void)` cast delimiter errors, or `sizeof(void)` before recognizing the unsupported pointer. Cust now reports `void pointers are not supported` at the `*` token for return types, parameters, local declarations, casts, `sizeof(void *)`, and `_Alignof(void *)` while preserving supported `void` functions, empty `void` parameter lists, and `(void)expr` casts.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_void_pointer_forms_with_context -- --nocapture  # RED first: generic function-name/sizeof(void) diagnostics; GREEN passed
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest:

All passed after the 2026-07-01 autonomous aggregate field-assignment result field-access run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, more malformed-source exact diagnostics, additional negative pointer/storage-root diagnostics, less-traveled direct enum/inline aggregate contexts, and an adjacent aggregate-expression parity gap discovered while extending recent aggregate assignment-result field-access coverage: `(line.start = replacement).x` failed in parsing with `invalid struct field access target` even though aggregate field assignment expressions should return the assigned by-value aggregate. The selected work package fixes parser, runtime, and metadata routing for field access on direct and struct-pointer aggregate field assignment results while preserving non-evaluating `sizeof((marker = marker + 1, (line.end = other).tag))` behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_field_access_on_aggregate_field_assignment_results -- --nocapture  # RED first: invalid struct field access target; GREEN passed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/aggregate_field_assignment_field_access.c -o /tmp/aggregate_field_assignment_field_access && /tmp/aggregate_field_assignment_field_access  # exit=23
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous aggregate element-assignment result field-access run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, more malformed-source exact diagnostics, additional negative pointer/storage-root diagnostics, less-traveled direct enum/inline aggregate contexts, and an adjacent aggregate-expression parity gap discovered while probing recent `sizeof` aggregate element-assignment work: `(points[0] = replacement).x` failed in parsing with `invalid struct field access target` even though aggregate assignment expressions return by-value aggregate copies. The selected work package fixes parser and metadata routing for field access on aggregate-array element assignment results, including root aggregate arrays, embedded aggregate-array fields, and struct-pointer embedded aggregate-array fields, while preserving non-evaluating `sizeof((marker = marker + 1, (points[1] = replacement).tag))` behavior.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_field_access_on_aggregate_element_assignment_results -- --nocapture  # RED first: invalid struct field access target; GREEN passed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/aggregate_element_assignment_field_access.c -o /tmp/aggregate_element_assignment_field_access && /tmp/aggregate_element_assignment_field_access  # exit=11
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous `sizeof` embedded aggregate-array element-field metadata run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, more malformed-source exact diagnostics, additional negative pointer/storage-root diagnostics, and nearby non-evaluating type-query gaps after the recent aggregate assignment and element-assignment `sizeof` work. The selected work package fixes `sizeof` metadata for fields selected from embedded aggregate-array elements: `sizeof(line.points[0].tag)` and comma-wrapped variants now report the selected field type (for example `char`) without evaluating indexes or comma-left side effects instead of falling back to Cust `int` size.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_sizeof_embedded_aggregate_array_element_field_types_without_evaluating_operands -- --nocapture  # RED first: returned 2 instead of 4 because char fields were sized as int; GREEN passed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/sizeof_embedded_aggregate_array_element_fields.c -o /tmp/sizeof_embedded_aggregate_array_element_fields && /tmp/sizeof_embedded_aggregate_array_element_fields  # exit=4
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-07-01 autonomous `sizeof` aggregate element-assignment metadata run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, additional negative pointer/storage-root diagnostics, and concrete non-evaluating type-query gaps adjacent to the recent `sizeof` aggregate assignment work. The selected work package fixes `sizeof` metadata for aggregate element assignment expressions: `sizeof((points[0] = replacement))` and `sizeof((line.points[1] = replacement))` now report the selected aggregate element type without evaluating or mutating the target, matching already-supported `*slot = replacement` and struct-pointer field-array forms.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_sizeof_aggregate_element_assignment_expressions_without_evaluating_operands -- --nocapture  # RED first: struct variable 'points' is not an array / struct field 'points' is a struct array; GREEN passed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/sizeof_aggregate_element_assignment_expressions.c -o /tmp/sizeof_aggregate_element_assignment_expressions && /tmp/sizeof_aggregate_element_assignment_expressions  # exit=12
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous `sizeof` aggregate assignment-expression metadata run. Ideation considered failing tests/builds (clean pulled tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, pointer negative diagnostics, additional less-traveled inline type-definition contexts, and a concrete non-evaluating type-query parity gap discovered adjacent to the recent `sizeof` comma/conditional work: `sizeof((aggregate_var = rhs))` rejected supported aggregate assignment expressions with `struct variable '<name>' assignment is not supported` instead of reporting the assigned aggregate type size. The selected work package fixes `sizeof` metadata for aggregate assignment expressions by deriving `Value::Struct` assignment result size from the aggregate type table while preserving non-evaluation of direct assignment and comma-left side effects.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_sizeof_aggregate_assignment_expressions_without_evaluating_operands -- --nocapture  # RED first: struct variable 'left' assignment is not supported; GREEN passed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/sizeof_aggregate_assignment_expressions.c -o /tmp/sizeof_aggregate_assignment_expressions && /tmp/sizeof_aggregate_assignment_expressions  # exit=7
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous `sizeof` aggregate conditional-expression metadata run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, additional malformed-source exact diagnostics, less-traveled inline type-definition contexts, pointer negative diagnostics, and a concrete non-evaluating type-query parity gap discovered in `sizeof_expr`: `sizeof(cond ? aggregate_a : aggregate_b)` reported Cust `int` size instead of the common aggregate type. The selected work package fixes `sizeof` metadata for aggregate-valued conditional expressions while preserving non-evaluation of the condition and both branches, including side-effecting assignments nested in branches or comma-left operands.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_sizeof_aggregate_conditional_expressions_without_evaluating_operands -- --nocapture  # RED first: returned 2/6; GREEN passed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/sizeof_aggregate_conditional_expressions.c -o /tmp/sizeof_aggregate_conditional_expressions && /tmp/sizeof_aggregate_conditional_expressions  # exit=6
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous `sizeof` comma-expression type inference run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, more inline aggregate conformance contexts, malformed-source exact diagnostics, and a concrete runtime parity gap discovered during probing: `sizeof((side_effect, rhs))` was non-evaluating but reported `int` size instead of the comma expression RHS type. The selected work package fixes `sizeof` metadata for comma expressions so the left operand remains unevaluated and the result size follows the right operand for scalar, pointer, and array-index RHS forms. Focused RED reproduced the bug with `sizeof((marker = marker + 1, (char){7}))`; GREEN routes `Expr::Comma` through RHS `sizeof_expr` metadata.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_sizeof_comma_expression_rhs_types_without_evaluating_operands -- --nocapture  # RED first: returned 32/18 instead of RHS-type relationships; GREEN passed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/sizeof_comma_expression_types.c -o /tmp/sizeof_comma_expression_types && /tmp/sizeof_comma_expression_types  # exit=4
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous restrict non-pointer diagnostic run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, additional inline aggregate conformance contexts, malformed-source exact-diagnostic fuzzing, and C11 qualifier syntax gaps. The selected work package closes a standards-conformance/parser-trust gap where Cust accepted `restrict int` on scalar declarations, parameters, and aggregate fields even though the supported `restrict` subset is pointer-declarator-only. Focused RED reproduced acceptance of `restrict int value`; GREEN rejects leading `restrict` qualifiers before base types while preserving post-star pointer declarators such as `int * restrict p`.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cc -std=c11 -Wall -Wextra -Werror /tmp/restrict_scalar.c -o /tmp/restrict_scalar  # native oracle rejected leading scalar restrict as invalid use of 'restrict'
cargo test --test interpreter rejects_restrict_on_non_pointer_declarations_with_context -- --nocapture  # RED first: accepted scalar restrict; GREEN after parser diagnostic
cargo test --test interpreter supports_restrict_pointer_qualifiers -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous inline aggregate `sizeof(*pointer_expr)` type-definition conformance run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, function-parameter type-definition native pitfalls, and a less-traveled inline aggregate context adjacent to the recent pointer arithmetic/comparison coverage: inline named aggregate definitions inside non-evaluating `sizeof` operands that dereference pointer expressions. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct`/`union` definitions inside `sizeof(*(values + ...))`, `sizeof(*(points + ...))`, and `sizeof(*(&((struct Inline){...}).field))` pointer-expression operands. Focused coverage passed after correcting the fixture to avoid assuming that tags introduced inside `sizeof` expression operands leak to later native declarations, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_aggregate_sizeof_pointer_expression_type_definitions -- --nocapture  # coverage GREEN after fixture-scope correction; no production-code change needed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_aggregate_sizeof_pointer_expression_type_definitions.c -o /tmp/inline_aggregate_sizeof_pointer_expression_type_definitions && /tmp/inline_aggregate_sizeof_pointer_expression_type_definitions  # exit=52
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous inline aggregate pointer-comparison type-definition conformance run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, further direct enum/inline enum contexts, and a less-traveled inline aggregate context adjacent to the prior pointer arithmetic/index coverage: inline named aggregate definitions inside pointer equality, relational, and difference expressions. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside pointer comparison/difference operands, plus an adjacent aggregate compound-literal field-address initializer that leaves its inline tag available for a later same-block declaration. Focused coverage passed immediately because shared expression/type-name parsing and existing pointer comparison/difference evaluation already handle the scenario, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_aggregate_pointer_comparison_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_aggregate_pointer_comparison_type_definitions.c -o /tmp/inline_aggregate_pointer_comparison_type_definitions && /tmp/inline_aggregate_pointer_comparison_type_definitions  # exit=65
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous inline aggregate pointer-arithmetic type-definition conformance run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, further direct enum/inline enum contexts, and a less-traveled inline aggregate context adjacent to the prior pointer-initializer and compound-literal coverage: inline named aggregate definitions inside pointer arithmetic/index expressions. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside pointer arithmetic offsets, pointer indexing offsets, aggregate compound-literal field-address initializers, and array index expressions used by pointer dereference writes, then declares objects of those inline tags later in the same block. Focused coverage passed immediately because shared pointer expression initializer/index parsing already installs inline aggregate tags through existing type-name and compound-literal paths, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_aggregate_pointer_arithmetic_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_aggregate_pointer_arithmetic_type_definitions.c -o /tmp/inline_aggregate_pointer_arithmetic_type_definitions && /tmp/inline_aggregate_pointer_arithmetic_type_definitions  # exit=168
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous inline aggregate aggregate-array compound-literal type-definition conformance run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, further direct enum/inline enum contexts, and a less-traveled inline aggregate context adjacent to the prior scalar array compound-literal coverage: inline named aggregate definitions inside aggregate-array compound-literal initializer expressions and designator indexes. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside `(struct Anchor[]){ ... }` initializer values, a compound-literal aggregate-array designator index, and an inline aggregate compound-literal field read, then declares objects of those inline tags later in the same block. Focused coverage passed immediately because shared aggregate-array compound-literal initializer/designator parsing already installs inline aggregate tags through existing type-name and compound-literal paths, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_aggregate_aggregate_array_compound_literal_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_aggregate_aggregate_array_compound_literal_type_definitions.c -o /tmp/inline_aggregate_aggregate_array_compound_literal_type_definitions && /tmp/inline_aggregate_aggregate_array_compound_literal_type_definitions  # exit=105
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous inline aggregate array-compound-literal type-definition conformance run. Ideation considered failing tests/builds (`cargo test` baseline passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, further direct enum/inline enum contexts, and a less-traveled inline aggregate context adjacent to recent initializer/pointer-initializer coverage: inline named aggregate definitions inside scalar array compound-literal initializer expressions and designator indexes. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside `(int[]){ ... }` initializer values, a compound-literal array designator index, and an inline aggregate compound-literal field read, then declares objects of those inline tags later in the same block. Focused coverage passed immediately because shared scalar-array compound-literal initializer/designator parsing already installs inline aggregate tags through existing type-name and compound-literal paths, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cc -std=c11 -Wall -Wextra -Werror /tmp/cust-inline-array-compound-*.c -o /tmp/cust-inline-array-compound && /tmp/cust-inline-array-compound  # local smoke fixture exited 103
cargo test --test interpreter supports_inline_aggregate_array_compound_literal_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-30 autonomous inline aggregate pointer-initializer type-definition conformance run. Ideation considered failing tests/builds (none in the clean pulled worktree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, and less-traveled inline aggregate contexts not yet explicitly covered. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside pointer-valued declaration initializer and assignment RHS expressions that take addresses of aggregate compound-literal fields, then declares objects of those inline tags later in the same block. Focused coverage passed immediately because shared aggregate compound-literal type-name parsing already installs inline aggregate tags in the enclosing block scope and existing address-of aggregate compound-literal field support provides safe scalar pointers, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_inline_aggregate_pointer_initializer_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_aggregate_pointer_initializer_type_definitions.c -o /tmp/inline_aggregate_pointer_initializer_type_definitions && /tmp/inline_aggregate_pointer_initializer_type_definitions  # exit=56
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate initializer type-definition conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, less-traveled direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, and an initializer edge adjacent to recent inline aggregate declaration/assignment, conditional, static-assert, call-argument, expression-statement, and return-expression coverage: inline named aggregate definitions inside aggregate initializer expressions and designator indexes. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside positional aggregate initializer expressions, aggregate compound-literal initializer entries, array designator indexes, and field designator values, then declares objects of those inline tags later in the same block. Focused coverage passed immediately because shared initializer/designator expression parsing already installs inline aggregate tags through existing type-name and compound-literal parsing, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cc -std=c11 -Wall -Wextra -Werror /tmp/inline_aggregate_initializer_type_definitions.c -o /tmp/inline_aggregate_initializer_type_definitions && /tmp/inline_aggregate_initializer_type_definitions  # exit=36
cargo test --test interpreter supports_inline_aggregate_initializer_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate return-expression type-definition conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, less-traveled direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, and a return-statement edge adjacent to the recent inline aggregate declaration/assignment, conditional, static-assert, call-argument, and expression-statement coverage: inline named aggregate definitions inside return expressions. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside return-expression type-query and compound-literal contexts. Focused coverage passed immediately because shared type-name and compound-literal parsing already installs inline aggregate tags in the enclosing function block while parsing return expressions, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_aggregate_return_expression_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate expression-statement type-definition conformance run. Ideation considered failing tests/builds (none in the clean pulled worktree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, less-traveled direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, and an expression-statement edge adjacent to the recent inline aggregate declaration/assignment, conditional, static-assert, and call-argument coverage: inline named aggregate definitions inside discarded expression statements. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside `(void)` expression statements, including a type-query expression and an aggregate compound-literal field expression, followed by same-block declarations using those tags. Focused coverage passed immediately because shared type-name and compound-literal parsing already installs inline aggregate tags in the enclosing block scope while parsing these expressions, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_inline_aggregate_expression_statement_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate declaration/assignment type-definition conformance run. Ideation considered failing tests/builds (none in the clean pulled worktree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, less-traveled direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, and an expression/declaration edge adjacent to the recent inline aggregate conditional/static-assert/call-argument coverage: inline named aggregate definitions inside declaration-list initializer expressions and assignment RHS expressions. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` and `union` definitions inside declaration-list initializers, scalar assignment RHS, and compound-assignment RHS, followed by same-block declarations using those tags. Focused coverage passed immediately because shared type-name parsing already installs inline aggregate tags in the enclosing block scope while parsing these expressions, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_inline_aggregate_declaration_assignment_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate conditional/short-circuit type-definition conformance run. Ideation considered failing tests/builds (none in the clean pulled worktree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, less-traveled direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, and an expression-control edge adjacent to the existing inline enum conditional coverage: inline named aggregate definitions inside `?:`, `&&`, and `||` operands. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` definitions in selected/unselected conditional branches and short-circuited `&&` operands plus an inline `union` definition in a short-circuited `||` operand, followed by same-block declarations using those tags. Focused coverage passed immediately because shared type-name parsing already installs inline aggregate tags in the enclosing block scope while parsing all operands, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_inline_aggregate_conditional_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate static-assert type-definition conformance run. Ideation considered failing tests/builds (none in the clean pulled worktree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, less-traveled direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, inline aggregate call-argument coverage from the prior run, and an adjacent declaration/assertion edge not yet locked in: inline named aggregate definitions inside C11 `_Static_assert` conditions. The selected work package adds interpreter and warning-free native compiler-oracle coverage for `_Static_assert(sizeof(struct AssertBox { ... }) == sizeof(struct AssertBox), ...)` and matching `union` coverage, followed by same-block declarations using those tags. Focused coverage passed immediately because shared `_Static_assert` condition/type-name parsing already installs inline aggregate tags in the enclosing block scope, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_inline_aggregate_static_assert_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate call-argument type-definition conformance run. Ideation considered failing tests/builds (none in the clean pulled worktree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, less-traveled direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, and a call-expression edge adjacent to the existing inline enum call-argument coverage: inline named aggregate definitions inside function-call argument subexpressions. The selected work package adds interpreter and warning-free native compiler-oracle coverage for a function call whose arguments define `struct ArgBox`, `union ArgChoice`, and `struct LitBox` via `sizeof(type-name)` and aggregate compound-literal type-name contexts, then declares objects of those tags later in the same block. Focused coverage passed immediately because shared type-name/compound-literal parsing already installs inline aggregate tags in the enclosing block scope, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_inline_aggregate_call_argument_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate for-clause type-definition conformance run. Ideation considered failing tests/builds (none in the clean pulled worktree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic storage roots, remaining direct enum/inline enum/inline aggregate contexts, function parameter type-definition coverage, and an adjacent less-traveled control-flow edge not covered by the prior `if`/`while`/`switch` fixture: inline named aggregate definitions inside `for` initializer, condition, and increment clauses. The selected work package adds interpreter and warning-free native compiler-oracle coverage for `struct` definitions in a `for` initializer and condition plus a `union` definition in a `for` increment expression, then declares objects of those inline tags in the loop bodies. Focused coverage passed immediately because shared type-name parsing already installs inline aggregate tags in the enclosing statement/block scope, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_inline_aggregate_for_clause_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline union control-expression type-definition conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic roots, remaining less-traveled direct enum/inline enum and aggregate contexts, function parameter type-definition coverage, and the closest unverified edge next to the prior inline `struct` control-expression coverage: inline named `union` definitions in `if`, `while`, and `switch` controlling expressions should be locked in explicitly. The selected work package expands the existing inline aggregate control fixture with warning-free `union` definitions in those control expressions and same-body declarations of the new union tags. Focused coverage passed immediately because shared `sizeof(type-name)` parsing already installs inline union tag definitions in the enclosing block scope, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_aggregate_control_type_definitions -- --nocapture  # union coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline aggregate control-expression type-definition conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic roots, remaining less-traveled direct enum/inline enum and aggregate contexts, extending function parameter type-definition coverage if a warning-free native pattern appears, and a type-name/control-flow edge adjacent to the existing inline aggregate return/expression/parameter coverage: inline named aggregate definitions inside control-flow controlling expressions should be locked in explicitly. The selected work package adds interpreter and warning-free native compiler-oracle coverage for inline `struct` definitions inside `if`, `while`, and `switch` controlling expressions, then declares objects of those inline tags in the corresponding statement bodies. Focused coverage passed immediately because shared `sizeof(type-name)` parsing already installs inline aggregate tag definitions in the enclosing block scope, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_aggregate_control_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-29 autonomous inline type-definition array-length conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic roots, remaining less-traveled direct enum/inline enum contexts, function parameter type-definition coverage, mixed supported-subset conformance, and a type-query/array-length edge adjacent to the recent parser-folded integer-constant-expression work: inline enum and aggregate definitions inside array declarator lengths and array type-name length expressions should be locked in explicitly. The selected work package adds interpreter and warning-free native compiler-oracle coverage for `sizeof(enum Tag { ... })` in object array lengths, `sizeof(struct Tag { ... })` in typedef array lengths, inline enum definitions inside array type-name length expressions, and inline aggregate definitions inside aggregate array type-name operands with ABI-independent native relationships. Focused coverage passed immediately because the shared integer-constant-expression and type-name paths already preserve pending inline enum declarations and aggregate tag definitions, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_type_definitions_in_array_lengths -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous array type-name integer-constant-expression conformance run. Ideation considered failing tests/builds (no baseline failures known and the pulled worktree was clean), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic roots, remaining less-traveled direct enum/inline enum contexts, function parameter type-definition coverage, and a small type-query conformance gap adjacent to the already-completed array-length integer-constant-expression work: array type-name operands in `sizeof(...)` and `_Alignof(...)` should explicitly lock in parser-folded integer constant expression lengths. The selected work package adds interpreter and warning-free native compiler-oracle coverage for enum constants, `sizeof` operands, conditional expressions, typedef element names, direct aggregate typedefs, and ABI-independent `_Alignof(T[N]) == _Alignof(T)` relationships. Focused coverage passed immediately because the existing shared `expect_array_len()` path already handled these type-name array suffixes, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter supports_integer_constant_expressions_for_array_type_lengths -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous aggregate compound-literal array-field negative pointer-arithmetic coverage run. Ideation considered failing tests/builds (no baseline failures known and the focused path compiled cleanly), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, more negative pointer-arithmetic paths through field-backed storage, remaining direct enum/inline enum contexts, function parameter type-definition coverage, and mixed supported-subset conformance. The selected work package locks in diagnostics for pointer subtraction and relational ordering between scalar array fields and embedded aggregate-array fields selected from separately evaluated aggregate compound literals. This complements earlier coverage for separate string literals, separate scalar/aggregate array compound literals, and distinct field-backed array roots. Focused coverage passed immediately because existing hidden aggregate compound-literal field storage already carries distinct array identity metadata, so this run records conformance/diagnostic coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter aggregate_compound_literal -- --nocapture  # coverage GREEN immediately; no production-code change needed
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous star-VLA array-length diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic paths, remaining direct enum/inline enum contexts, function parameter type-definition coverage, and a fresh parser-trust gap adjacent to the fixed-size array-length work: C variable-length-array star declarators such as `int values[*]` fell through to the generic `expected array length, found Star` diagnostic. The selected work package keeps VLA star declarators outside Cust's deterministic fixed-size array subset but adds targeted diagnostics for function prototype parameters, local/object arrays, aggregate fields, and typedef array aliases by handling `Token::Star` in the shared array-length parser.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_star_vla_array_lengths_with_context -- --nocapture  # RED: generic expected array length, found Star; GREEN after targeted diagnostic
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous inline-enum conditional/short-circuit conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic paths, extending function parameter type-definition coverage, mixed supported-subset conformance, and a less-traveled inline enum context not yet locked by fixtures: inline enum type definitions nested inside conditional (`?:`) branches and short-circuit logical operands. The selected work package adds interpreter and warning-free native compiler-oracle coverage proving generated enumerators from both selected and unselected conditional branches, plus unevaluated `&&`/`||` operands, are emitted before runtime evaluation and remain visible to later same-block expressions. Focused coverage passed immediately because existing enclosing-statement pending-inline-enum routing was already correct, so no production-code change was needed.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_enum_conditional_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous inline-enum aggregate-initializer conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional negative pointer-arithmetic coverage, extending function parameter type-definition coverage, and a less-traveled inline enum runtime-routing context not yet locked by fixtures: inline enum type definitions inside nested aggregate initializer expressions and designator indexes. The selected work package adds interpreter and warning-free native compiler-oracle coverage for `sizeof(enum InitX { ... })`, enum casts, `_Alignof(enum TailValue { ... })`, and an inline-enum-derived designator index inside struct/array aggregate initializers. Focused coverage initially failed only because the expected fixture arithmetic was miscomputed (`116` vs the correct `122`), then passed after correcting the test expectation; no production-code change was needed because existing initializer pending-enum routing was already correct.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_enum_aggregate_initializer_type_definitions -- --nocapture  # coverage GREEN after correcting expected arithmetic to 122
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous local function-definition diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, fuzz/property expansion, more pointer-arithmetic negative coverage, function parameter type-definition coverage, mixed conformance fixtures, and a small parser-trust gap found in the unsupported local function-definition path. The selected work package adds exact invalid fixture coverage for a nested block-scope function definition and changes the parser to report the existing unsupported-feature message at the declaration start (`line 2, column 5`) instead of returning an unlocated error after parsing the nested body. Focused RED failed with the unlocated `function definitions are not supported inside blocks`; focused GREEN passed after capturing the local declaration start token before parsing the function header/body.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_local_function_definitions_with_context -- --nocapture  # RED: unlocated diagnostic; GREEN after source-location fix
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous inline-enum pointer/aggregate expression-statement conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional pointer-arithmetic negative coverage, extending function parameter type-definition coverage, and a less-traveled inline-enum runtime-routing context explicitly called out in the backlog: inline enum type definitions nested inside pointer-valued and aggregate-valued expression statements. The selected work package adds interpreter and warning-free native compiler-oracle fixtures covering a pointer assignment expression statement, an aggregate assignment expression statement with aggregate compound-literal initializers, and an aggregate compound literal passed through a call expression. Focused coverage passed immediately because the existing expression-statement pending-inline-enum wrapper already emits generated enumerators before runtime evaluation, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter inline_enum_pointer_aggregate_expression_statements -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous integer-constant-expression designator index run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional pointer-arithmetic negative coverage, remaining direct enum/inline enum contexts, and a higher-impact parser parity gap adjacent to the existing parser-folded array-length work: C array/designator indexes such as `[SLOT_INDEX]`, `[sizeof(char)]`, and `[BASE + 2]` were rejected because Cust's designator parser consumed only numeric literal tokens. The selected work package now routes fixed and inferred array designator indexes through the integer-constant-expression folder, preserving non-negative/too-large bounds checks and comma-expression rejection, and adds interpreter plus native compiler-oracle fixtures covering scalar arrays, fixed and inferred array compound literals, scalar array fields, aggregate-array designators, and struct-array designators.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter integer_constant_expressions_for_designator_indexes -- --nocapture  # RED: expected array designator index, found Ident("SLOT_INDEX"); GREEN after parser-folded designator indexes
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous anonymous aggregate-array field pointer ordering negative coverage run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through additional embedded/anonymous aggregate/string/compound-literal field paths, remaining less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and more mixed supported-subset conformance fixtures. The selected work package locks in `cannot compare pointers to different arrays` for relational ordering between anonymous aggregate-array fields belonging to distinct aggregate objects, complementing the existing same-path pointer-difference diagnostic coverage for `right.items - left.items`. Focused coverage passed immediately because existing `PointerValue::StructFieldElement` owner/path metadata already distinguishes anonymous aggregate-array field storage roots for ordering just as it did for subtraction, so this run records conformance/diagnostic coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter different_anonymous_aggregate_array_fields -- --nocapture  # coverage GREEN immediately; no production-code change needed
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-28 autonomous aggregate-array-compound-literal pointer negative coverage run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through additional embedded/anonymous aggregate/string/compound-literal paths, remaining less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and more mixed supported-subset conformance fixtures. The selected work package locks in diagnostics for pointer subtraction and ordering between distinct aggregate-array compound literal storage roots: `((struct Point[]){{...}}) - ((struct Point[]){{...}})` reports `cannot subtract pointers to different arrays`, while relational ordering between separate aggregate-array compound literals reports `cannot compare pointers to different arrays`. Focused coverage passed immediately because existing aggregate-array compound-literal storage metadata already allocates distinct hidden array roots and routes through cross-array pointer arithmetic checks, so this run records conformance/diagnostic coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter different_aggregate_array_compound_literals -- --nocapture  # coverage GREEN immediately; no production-code change needed
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous array-compound-literal pointer negative coverage run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through field-backed, string-literal, and compound-literal storage paths, remaining less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and more mixed supported-subset conformance fixtures. The selected work package locks in diagnostics for pointer subtraction and ordering between distinct array compound literal storage roots: `((int[]){1, 2, 3}) - ((int[]){4, 5, 6})` reports `cannot subtract pointers to different arrays`, while relational ordering between separate array compound literals reports `cannot compare pointers to different arrays`. Focused coverage passed immediately because existing array-compound-literal storage metadata already allocates distinct hidden array roots and routes through the cross-array pointer arithmetic checks, so this run records conformance/diagnostic coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter different_array_compound_literals -- --nocapture  # coverage GREEN immediately; no production-code change needed
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous string-literal pointer negative coverage run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through field-backed and string-literal storage paths, remaining less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and more mixed supported-subset conformance fixtures. The selected work package locks in diagnostics for pointer subtraction and ordering between different string literal arrays: `"dog" - "cat"`-style pointer difference reports `cannot subtract pointers to different arrays`, while relational ordering between distinct string literals reports `cannot compare pointers to different arrays`. Focused coverage passed immediately because existing read-only string-literal array metadata already keeps distinct literal storage roots separate, so this run records conformance/diagnostic coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter different_string_literals -- --nocapture  # coverage GREEN immediately; no production-code change needed
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous pointer-difference negative field-path coverage run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, remaining less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and more mixed supported-subset conformance fixtures. The selected work package locks in `cannot subtract pointers to different arrays` diagnostics for less-traveled field-backed array pointer difference paths: embedded aggregate-array fields from different fields, scalar array fields from different aggregate objects, and anonymous aggregate-array fields from different aggregate objects. Focused coverage passed immediately because the existing pointer owner/path metadata already distinguishes the different storage roots, so this run records conformance/diagnostic coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_pointer_difference_between_different -- --nocapture  # coverage GREEN immediately; no production-code change needed
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous array-length comma-expression diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, remaining less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and a parser-trust gap adjacent to the recent array-length integer-constant-expression work: `int values[1, 2];` parsed the leading constant expression and then fell through to the generic bracket helper with `expected ']' after array length, found Comma`. The selected work package keeps comma expressions outside Cust's integer constant expression subset and now reports `comma operator is not allowed in integer constant expression` at the comma token for array declarator lengths, matching the existing enum/switch integer-constant-expression boundary while preserving non-constant identifier and empty/negative/zero diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter rejects_comma_operator_in_array_length_integer_constant_expressions -- --nocapture  # RED: generic expected-closing-bracket diagnostic; GREEN after array-length comma diagnostic
cargo test --test interpreter rejects_non_constant_array_lengths_with_context -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous non-constant array-length diagnostic run. Ideation considered failing tests/builds (none in the pulled clean tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, remaining less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and a parser-trust gap adjacent to the just-completed array-length integer-constant-expression work: declarations such as `int values[n];` fell through to `expected array length, found Ident("n")` even though Cust's supported subset requires parser-folded integer constant expressions for fixed-size storage. The selected work package keeps variable-length array declarators unsupported but now reports `array length must be an integer constant expression` at the non-constant identifier before preserving existing empty, negative, zero, and flexible aggregate-field diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter rejects_non_constant_array_lengths_with_context -- --nocapture  # RED: generic expected-array-length diagnostic; GREEN after targeted non-constant array-length diagnostic
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous flexible-array aggregate-field diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, less-traveled direct enum/inline enum pointer/aggregate expression contexts, function parameter type-definition oracle feasibility, and a small parser-trust gap adjacent to the just-expanded array-length parser: C flexible array members such as `struct Packet { int data[]; };` fell through to the generic `expected array length before ']'` diagnostic. The selected work package keeps flexible array aggregate fields outside Cust's fixed-size deterministic aggregate model but now reports `flexible array aggregate fields are not supported` at the `]` token before the generic array-length helper runs.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter flexible_array_aggregate_fields -- --nocapture  # RED: generic expected-array-length diagnostic; GREEN after aggregate-field [] diagnostic
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous array-length integer constant expression run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, remaining less-traveled direct enum/inline enum contexts, and a higher-impact parser parity gap: C array declarator lengths should accept integer constant expressions rather than only literal number tokens. The selected work package implemented parser-folded integer constant expression lengths for ordinary scalar arrays, typedef array declarators, aggregate fields, aggregate arrays, array parameters, and fixed-size array compound literal type names. The RED fixture failed at `typedef int Scores[TYPEDEF_LEN];` with `expected array length, found Ident("TYPEDEF_LEN")`; after the parser change it passes and compiler-oracle coverage verifies the same warning-free program under native C. Existing empty/negative/zero array-length diagnostics are preserved.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test
cargo test --test interpreter supports_integer_constant_expressions_for_array_lengths -- --nocapture  # RED before implementation; GREEN after parser-folded array lengths
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous inline enum function-call argument type-definition conformance run. Ideation considered failing tests/builds (none in the pulled clean tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, remaining less-traveled direct enum/inline enum contexts, and mixed supported-subset conformance fixtures. The selected work package targeted inline enum definitions nested inside function-call argument subexpressions because it is adjacent to recent pending-inline-enum statement wrapping work and was explicitly called out as a less-traveled context. Interpreter and warning-free native compiler-oracle fixtures now cover `sizeof(enum ArgSize { ... })`, `_Alignof(enum ArgAlign { ... })`, and `(enum ArgCast { ... })0` inside call arguments, plus later same-block enumerator visibility. Focused interpreter coverage passed immediately because the existing pending-inline-enum wrapper path already emits generated enumerators before evaluating the enclosing statement, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter inline_enum_call_argument_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous inline enum assignment-lvalue type-definition run. Ideation considered failing tests/builds (none in the pulled clean tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate paths, less-traveled direct enum/inline enum contexts, function parameter type-definition oracle feasibility, and an audit of assignment-like statement paths that might still miss pending inline enum declarations. The selected work package targeted lvalue-specific assignment statements after the prior `_Alignof` assignment fix; the RED fixture exposed a real runtime gap where array, struct-field, struct-array-field, and dereference assignment/compound-assignment statement parsers returned early without emitting pending inline enum declarations, so RHS references such as `values[0] = _Alignof(enum ArrayAlign { ARRAY_ALIGN = 3 }) + ARRAY_ALIGN;` failed with `undefined variable 'ARRAY_ALIGN'`. Those statement paths now wrap pending inline enum declarations before runtime assignment evaluation, matching the existing expression/return/static-assert/control/declaration-list/plain-assignment behavior. Interpreter and warning-free native compiler-oracle fixtures cover scalar array lvalues, direct struct fields, struct array fields, pointer dereference lvalues, and compound assignment variants.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter inline_enum_assignment_lvalue_type_definitions -- --nocapture  # RED: undefined variable 'ARRAY_ALIGN'; GREEN passed after wrapping all assignment-lvalue early returns
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-27 autonomous inline enum `_Alignof` type-definition and assignment-statement wrapping run. Ideation considered failing tests/builds (none observed in the pulled tree), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, remaining less-traveled direct enum/inline enum contexts, assignment-like statement paths that might miss pending inline enum declarations, function parameter type-definition native-oracle feasibility, and compact mixed supported-subset conformance fixtures. The selected work package targeted `_Alignof(enum Tag { ... })` expression contexts adjacent to the prior `sizeof` work; the RED fixture exposed a real runtime gap where plain assignment statements parsed inline enum definitions on the RHS but returned `Stmt::Assign` without emitting pending enum declarations, so the next statement failed with `undefined variable 'LOCAL_ALIGN'`. Plain scalar assignments and scalar compound-assignment statements now wrap pending inline enum declarations before runtime evaluation, and interpreter plus warning-free native compiler-oracle fixtures cover `_Alignof` in assignments, declaration-list initializers, and return expressions.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
date +%F
cargo test --test interpreter inline_enum_alignof_type_definitions -- --nocapture  # RED: undefined variable 'LOCAL_ALIGN'; GREEN passed after assignment-statement pending enum wrapper
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous inline enum `sizeof` type-definition conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing for newly discovered gaps, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, less-traveled direct enum and inline enum contexts, warning-free function parameter type-definition native-oracle feasibility, and compact mixed supported-subset conformance fixtures. The selected work package locks in inline enum definitions in ordinary `sizeof(enum Tag { ... })` type-name expressions because this is adjacent to the recent inline enum cast/control/static-assert/switch work and verifies generated enumerators are emitted before later statements in expression, declaration-list, and return contexts. Focused interpreter coverage passed immediately because the audited parser/runtime wrapper path was already supported, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter inline_enum_sizeof_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous inline aggregate expression type-definition conformance run. Ideation considered failing tests/builds (none in the pulled clean tree), active blockers (none), the remaining C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate field paths, less-traveled direct enum/inline enum contexts, function parameter aggregate-definition native-oracle feasibility, and a compact high-value conformance gap adjacent to recent inline aggregate return/parameter work: named `struct`/`union` definitions in expression type contexts. The selected work package added interpreter and native compiler-oracle fixtures proving expression-local named aggregate definitions work in compound literal type names, leave the tag visible for later same-block declarations, and work in `sizeof(struct Tag { ... })` type queries without relying on native aggregate layout byte counts. Focused interpreter coverage passed immediately because the audited parser path was already supported, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter inline_aggregate_expression_type_definitions -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous line-comment conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted negative pointer-arithmetic coverage through embedded/anonymous aggregate field paths, less-traveled direct enum/inline enum contexts, function parameter type-definition native-oracle feasibility, and compact supported-subset conformance gaps. The selected work package closed a documentation/test coverage gap for already-supported C99/C++-style `//` line comments: fixtures now prove line comments are lexer whitespace before statements and after trailing code while `//` inside string literals and `/` character literals remain ordinary literal content. Focused interpreter coverage and the native C compiler-oracle fixture passed immediately, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter line_comments -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous inline enum switch-case-label run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted negative pointer-arithmetic coverage through embedded/anonymous aggregate field paths, function parameter type-definition native-oracle feasibility, additional mixed conformance fixtures, and another inline-enum runtime gap adjacent to recent cast/control/static-assert work: `case sizeof(enum CaseSize { CASE_SIZE = 7 }):` parsed and folded the case value but did not emit the generated enum constant before executing that switch section, so case body references failed with `undefined variable 'CASE_SIZE'`. Switch sections now prepend pending inline enum declarations produced by case-label integer constant expressions, and fixtures cover interpreter behavior plus warning-free C compiler-oracle comparison.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter inline_enum_switch_case_label -- --nocapture  # RED: undefined variable 'CASE_SIZE'; GREEN passed after case-section enum hoisting
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous inline enum `_Static_assert` run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic diagnostics through embedded/anonymous aggregate paths, less-traveled direct enum and inline enum contexts, function parameter type-definition native-oracle feasibility, and a concrete inline-enum runtime gap adjacent to recent inline enum expression/control work: `_Static_assert(sizeof(enum E { A = 1 }) == sizeof(int), "...");` parsed the inline enum type definition but returned a bare `StaticAssert` statement, so generated enum constants were never emitted before the assertion or later statements. Static assertions now prepend pending inline enum declarations before runtime assertion evaluation, and fixtures cover top-level and block-scope inline enum type definitions inside `_Static_assert` conditions with C compiler-oracle comparison.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_static_assertions -- --nocapture  # RED: undefined variable 'TOP_ASSERT_VALUE'; GREEN passed after static-assert pending enum wrapper
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous inline enum control-expression run. Ideation considered failing tests/builds (none known after the previous verified run), active blockers (none), the remaining C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic negative coverage through embedded/anonymous aggregate paths, less-traveled direct enum contexts, additional mixed conformance fixtures, and a concrete inline-enum runtime gap adjacent to the previous cast type-definition work: inline enum definitions in control expressions parsed and made enumerators visible to later parser phases, but runtime execution did not emit pending enum constants before evaluating `if`/`while`/`for`/`switch` headers or loop bodies. Control statements now hoist pending inline enum declarations into the appropriate runtime statement sequence, and fixtures cover `if`, `while`, `for` init/condition/increment, and `switch` expression/case-label contexts with C compiler-oracle comparison.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test --test interpreter inline_enum_control_expr_definitions -- --nocapture  # RED: undefined variable 'WHILE_LIMIT'; GREEN passed after control-expression enum hoisting
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous inline enum cast type-definition run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted negative pointer-arithmetic diagnostics through embedded/anonymous aggregate paths, more mixed supported-subset conformance fixtures, and less-traveled inline enum contexts. The selected work package closed a real parser/runtime parity gap for inline enum definitions in cast type specifiers: `(enum Inline { A = 4 })0` parsed and made `A` visible to the parser, but expression and return statements did not emit the pending inline enum constants before runtime evaluation, producing `undefined variable 'A'`. Expression and return statements now reuse the pending-inline-enum wrapper already used by declarations, and fixtures cover expression statements, declaration-list initializers, return expressions, and warning-free native compiler-oracle comparison.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter inline_enum_cast_type_definitions -- --nocapture  # RED: undefined variable 'EXPR_VALUE'; GREEN passed after expression/return statement enum wrapper
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-26 autonomous enum typedef declaration-list conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic diagnostics through embedded/anonymous aggregate field paths, direct enum/inline enum edge contexts, function parameter aggregate-definition native-oracle feasibility, and a compact high-value conformance package for an uncovered ordinary-C enum typedef declaration-list form: `typedef enum State { ... } State, *StatePtr, StateArray[4];`. The selected work package added interpreter and warning-free native compiler-oracle fixtures that combine inline named enum typedef definitions, pointer aliases, array aliases, array-typedef parameter decay, enum compound literals, scalar enum casts, enum-array pointer arithmetic/indexing, and ABI-independent `sizeof`/`_Alignof` relationships. Focused coverage passed immediately, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter enum_typedef_declaration_lists -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-26 autonomous named aggregate typedef declaration-list conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted pointer-arithmetic diagnostics through embedded/anonymous aggregate field paths, direct enum/inline enum edge contexts, and a compact high-value conformance package for an uncovered ordinary-C declaration form: comma-separated aliases after inline named aggregate typedef definitions, such as `typedef struct Point { ... } Point, *PointPtr, PointArray[3];` and matching union aliases. The selected work package added interpreter and warning-free native compiler-oracle fixtures that combine inline named aggregate typedef definitions, pointer aliases, array aliases, array-typedef parameter decay, aggregate-pointer arithmetic, `->` mutation, indexed union pointer mutation, and ABI-independent `sizeof(array typedef) == N * sizeof(element)` checks. Focused coverage passed immediately, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter named_aggregate_typedef_declaration_lists -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-26 autonomous mixed aggregate-field type-query conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, targeted negative pointer arithmetic through embedded/anonymous aggregate paths, direct enum and inline enum edge contexts, and a safe high-value conformance package that combines less-traveled supported surfaces: nested named aggregate field declaration lists, anonymous aggregate fields, typedef-backed pointer fields with const pointer-slot/pointee metadata, struct-pointer field access, array-field type queries, and aggregate `_Alignof` relationships. The selected work package added an interpreter fixture and warning-free native compiler-oracle fixture for that mixed supported-subset program; focused coverage passed immediately, so this run records conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter mixed_aggregate_field_type_query -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-26 autonomous inline aggregate parameter type-definition run. Ideation considered failing tests/builds (none known after the previous verified run), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, more mixed supported-subset conformance fixtures, pointer/const/storage/type-query audits through nested and anonymous aggregate paths, and a concrete parser-scope gap adjacent to the recent inline aggregate return-type work: named `struct`/`union` definitions in function parameter type specifiers were accepted by the shared declaration-type parser but their tags leaked into file scope. The selected work package now parses parameter-list aggregate definitions in a temporary function-parameter aggregate scope that remains visible while parsing the function body and is popped before later file-scope declarations.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
date +%F
cargo test --test interpreter inline_aggregate_parameter -- --nocapture  # RED: tag leak fixture returned Ok(3); GREEN passed after parameter aggregate scope fix
cargo test --test interpreter inline_enum_parameter -- --nocapture
cargo test --test interpreter inline_aggregate_return_type_definitions -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-06-25 autonomous inline aggregate return-type definition run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the generic C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional mixed supported-subset conformance fixtures, pointer/const/storage-class/type-query audits through nested and anonymous aggregate paths, named aggregate definition declarator extensions, direct enum/inline enum edge contexts, and a concrete C declaration parity gap adjacent to the recent named aggregate definition work: top-level function return type specifiers that define a named aggregate inline, such as `struct Pair { int x; int y; } make_pair(int);`, were routed as aggregate variable declarations and failed with `expected ';' after struct variable declaration, found LParen`. The selected work package now lets function lookahead skip full inline `struct`/`union` definition bodies, parses named aggregate definitions in return-type contexts, registers the tag before the function declarator/body, and preserves existing pending inline enum handling for aggregate fields inside the return-type definition.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter inline_aggregate_return_type_definitions -- --nocapture  # RED: expected ';' after struct variable declaration, found LParen; GREEN passed after inline aggregate return-type routing
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-25 autonomous unparenthesized `sizeof` integer-constant-expression run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the generic C-subset closure queue item in `status/todo.md`, malformed-source exact-diagnostic fuzzing, additional mixed supported-subset conformance fixtures, pointer/const/storage-class/type-query audits through nested and anonymous aggregate paths, named aggregate definition declarator extensions, and a concrete parser-folding parity gap: enum initializers and switch case labels rejected unparenthesized `sizeof "abc"` with `expected integer constant in sizeof expression` and hard-coded unparenthesized `sizeof` results to `INT_SIZE` instead of reusing operand-size metadata. The selected work package now parses unparenthesized `sizeof` operands as non-evaluating unary expressions in integer constant contexts and folds supported operand sizes for string literals, unary scalar expressions, and nested `sizeof` forms.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter unparenthesized_sizeof -- --nocapture  # RED: expected integer constant in sizeof expression for sizeof "abc"; GREEN passed after parser folding change
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-25 autonomous named aggregate definition declarator run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure queue item in `status/todo.md`, malformed-source fuzzing for fresh exact diagnostics, additional mixed supported-subset conformance fixtures, pointer/const/storage-class/type-query audits through nested and anonymous aggregate paths, targeted pointer-arithmetic negative coverage through embedded aggregate field paths, and a concrete C declaration parity gap adjacent to the previous block-scope aggregate work: Cust accepted standalone `struct Pair { ... };` definitions and anonymous aggregate object declarations, but `struct Pair { ... } pair = {1, 2}, *slot = &pair;` failed by treating the definition prefix as standalone and requiring a semicolon immediately after the closing brace. The selected work package now distinguishes standalone named aggregate definitions from named aggregate definitions with declarators, then routes the declarator form through the existing aggregate declaration-list machinery for globals, locals, static locals, pointer declarators, unions, and inline enum fields.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter named_aggregate_definition_declarators -- --nocapture  # RED: expected ';' after struct declaration, found Ident("global_point"); GREEN passed after parser routing
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-25 autonomous direct enum pointer-field indexing parity follow-up. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure queue item in `status/todo.md`, malformed-source fuzzing for fresh exact diagnostics, additional mixed supported-subset conformance fixtures, pointer/const/storage-class/type-query audits through nested and anonymous aggregate paths, targeted pointer-arithmetic negative coverage through embedded aggregate field paths, and a concrete parity gap adjacent to the prior enum aggregate-field run: direct aggregate pointer fields such as `job.cursor[1]` / `&job.cursor[1]` still routed through scalar array-field address-of and reported `struct field 'cursor' is not an array`, even though the struct-pointer `job->cursor[1]` path had been fixed. The selected work package now routes address-of direct pointer-field indexes through `direct_struct_pointer_field_index_pointer`, so direct `.` and `->` aggregate pointer-field indexing/address-of share the same pointer-valued-field semantics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter direct_enum_aggregate -- --nocapture  # RED: struct field 'cursor' is not an array for &job.cursor[1]; GREEN passed after direct pointer-field address-of routing
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-06-25 autonomous direct-enum aggregate-field and pointer-field indexing run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure queue item in `status/todo.md`, malformed-source fuzzing for fresh exact diagnostics, additional mixed supported-subset conformance fixtures, pointer/const/storage-class/type-query audits through nested and anonymous aggregate paths, targeted pointer-arithmetic negative coverage through embedded aggregate field paths, and direct enum aggregate contexts not covered by the prior direct-enum pointer/type-query run. The selected work package added direct named-enum aggregate-field conformance coverage (`enum State` scalar fields, `const enum State` fields, `enum State[N]` fields, and `enum State *` pointer fields) and closed a real runtime parity gap for indexing pointer-valued aggregate fields through `job->cursor[1]` / `&job->cursor[1]`, which previously fell into the embedded-array path and reported `struct field 'cursor' is not an array`.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter direct_enum_aggregate -- --nocapture  # RED: struct field 'cursor' is not an array; GREEN passed after pointer-field indexed routing
cargo test --test interpreter const_enum_aggregate -- --nocapture
cargo test --test interpreter struct_pointer_array_field_decay -- --nocapture
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-25 autonomous direct-enum conformance run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source fuzzing for fresh exact diagnostics, additional mixed supported-subset conformance fixtures, pointer/const/storage-class/type-query audits through nested and anonymous aggregate paths, targeted pointer-arithmetic negative coverage through embedded aggregate field paths, and direct enum type-name parity in less-traveled expression/type-query contexts. The selected work package locks in direct `enum Tag` pointer parameters, `const enum Tag` array-parameter decay, enum pointers, enum scalar casts, enum compound literals, `sizeof(enum Tag[N])`, `_Alignof(enum Tag[N])`, and `sizeof(enum Tag *)` with interpreter and native compiler-oracle fixtures. Focused RED passed immediately because the parser/runtime already supported the audited path; this was recorded as conformance coverage rather than a production-code fix.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter direct_enum_pointer -- --nocapture  # coverage GREEN immediately; no production-code change needed
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-25 autonomous inline enum parameter-definition run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, additional mixed supported-subset conformance fixtures, pointer/const/storage/type-query negative coverage, direct enum type-name parity in less-traveled contexts, and a concrete enum-scope gap adjacent to the previous inline-enum return work: inline enum definitions in function parameter type specifiers parsed, but their enumerators were cleared after parameter parsing, so function-body references such as `int f(enum Mode { MODE_READY = 3 } value) { return MODE_READY; }` failed with `undefined variable 'MODE_READY'`. The selected work package now captures parameter-list pending enum declarations and inserts them as the first statement of function definitions only, preserving the existing no-file-scope-leak behavior for prototypes and return-type inline enum declarations.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter inline_enum_parameter -- --nocapture  # RED: undefined variable 'MODE_READY'; GREEN passed after function-body EnumDecl insertion
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-25 autonomous inline enum return-type run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, additional mixed supported-subset conformance fixtures, pointer/const/storage/type-query negative coverage, direct enum type-name parity in less-traveled contexts, and a concrete parser-routing gap: top-level function return type specifiers that define an enum inline, such as `enum Status { READY = 5 } choose(void);`, were being treated as variable declarations and failed with `expected '=' after variable declaration, found LParen`. The selected work package now lets function lookahead skip inline enum definition bodies, emits return-type inline enum constants as file-scope `EnumDecl`s before function registration, and clears parameter-list pending enum constants to avoid accidental leakage.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inline_enum_return_type_definitions -- --nocapture  # RED: expected '=' after variable declaration, found LParen; GREEN passed after enum-return lookahead and pending EnumDecl routing
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source parser-trust gaps, additional mixed supported-subset conformance fixtures, pointer/const/storage-class/type-query audits, targeted pointer-arithmetic negative coverage through embedded and anonymous aggregate paths, and a concrete C99 expression parity gap: direct named enum type spellings such as `(enum State){READY}` were already valid declaration/type-query spellings but not recognized by cast/compound-literal lookahead. The selected work package now accepts direct named enum type names in scalar and one-dimensional array compound literals by routing `Token::Enum` through the existing cast parser, reusing Cust's deterministic integer-backed enum storage model.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter enum_compound_literals -- --nocapture  # RED: expected expression, found Enum; GREEN passed after enum cast lookahead routing
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-25 autonomous scalar array-field pointer-expression run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, additional mixed supported-subset conformance fixtures, malformed-source parser-trust gaps, and a concrete pointer/array parity gap: scalar array fields already decayed in declarations/parameters but not in ordinary pointer expressions such as `packet.values < &packet.values[3]`, `packet.values + 1`, `&packet.values[3] - packet.values`, or field-array truthiness. The selected work package now classifies direct struct scalar-array fields, struct-array element scalar-array fields, and struct-pointer arrow scalar-array fields as pointer-valued in expression contexts, while preserving the narrower pointer-field assignment routing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter scalar_array_field -- --nocapture  # RED: struct field 'values' is an array; GREEN passed after pointer-expression classification and truthiness routing
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous parenthesized pointer typedef diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, additional mixed supported-subset conformance fixtures, pointer/const/storage-class audits through nested and anonymous aggregate paths, and a fresh parser-trust gap adjacent to the recent parenthesized declarator work: unsupported pointer-to-array typedef spellings such as `typedef int (*Row)[3];` were reported as unsupported function-pointer typedef aliases. The selected work package keeps parenthesized pointer typedef aliases outside Cust's supported subset but now distinguishes function-shaped `(*name)(...)` aliases from pointer-to-array-style `(*name)[N]` aliases with `parenthesized pointer typedef aliases are not supported` at the `(` token.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter parenthesized_pointer_typedef -- --nocapture  # RED: got function-pointer typedef diagnostic; GREEN passed after parenthesized-pointer typedef lookahead split
cargo test --test interpreter function_pointer_typedef -- --nocapture  # regression: existing function-pointer typedef diagnostic preserved
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous parenthesized typedef declarator run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, additional mixed supported-subset conformance fixtures, pointer/const/storage-class audits through nested and anonymous aggregate paths, and a concrete ordinary-C parser parity gap adjacent to the previous parenthesized declarator work: typedef alias declarators such as `typedef int (Count);`, `typedef int *(IntPtr);`, and `typedef int (Scores)[3];`. The selected work package now accepts parenthesized alias names for supported scalar, pointer, array, aggregate, aggregate-pointer, and aggregate-array typedef declarators while preserving the existing unsupported function-pointer typedef diagnostic for `(*name)` forms.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_parenthesized_typedef_declarators -- --nocapture  # RED: expected typedef alias name after type, found LParen; GREEN passed after reusing the parenthesized declarator-name helper in typedef parsing
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous parenthesized variable declarator run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, fresh malformed-source exact diagnostics, additional mixed supported-subset conformance fixtures, pointer/const/storage-class audits through nested and anonymous aggregate paths, and a concrete ordinary-C parser parity gap adjacent to the previous function-declarator work: parenthesized ordinary declarator names such as `int (x)`, `int *(p)`, `int (values)[3]`, and `struct Point (point)`. The selected work package now accepts parenthesized declarator names for supported parameters, local/global declaration lists, pointer declarations, arrays, aggregate variables, aggregate pointers, and aggregate fields while preserving the existing `(*name)` unsupported function-pointer/pointer-to-array diagnostics.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_parenthesized_variable_declarators -- --nocapture  # RED: expected parameter name after type, found LParen; GREEN passed after shared declarator-name parser
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous array-return diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, additional mixed supported-subset conformance fixtures, pointer/const/storage-class audits through nested and anonymous aggregate paths, targeted pointer-arithmetic negative coverage through embedded aggregate field paths, and a fresh parser-trust gap: direct array-return declarator suffixes such as `int make(void)[2]` fell through to the generic function-header block diagnostic. The selected work package keeps array return types outside Cust's supported subset but now reports `array return types are not supported` at the `[` token before prototype/body parsing.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_array_return_types_with_context -- --nocapture  # RED: expected '{' after function header, found LBracket; GREEN passed after post-parameter-list '[' diagnostic
cargo test --test interpreter reports_missing_pointer_parameter_names_after_stars -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous parenthesized function declarator run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, additional mixed supported-subset conformance fixtures, targeted pointer/const negative coverage through embedded and anonymous aggregate paths, and a concrete ordinary-C parser parity gap: parenthesized function declarator names such as `int (add)(int, int);` and `int (main)(void)`. The selected work package now accepts parenthesized names for supported top-level function prototypes/definitions and block-scope function prototypes while preserving the existing unsupported function-pointer declarator diagnostic for `(*name)` forms.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter parenthesized_function_declarators -- --nocapture  # RED: expected function name after return type, found LParen; GREEN passed after function-name helper and lookahead update
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

All passed after the 2026-06-24 autonomous const array typedef compound literal run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the remaining generic C-subset closure item in `status/todo.md`, malformed-source exact diagnostics, additional mixed conformance fixtures, targeted pointer-arithmetic negative coverage, and a concrete const-correctness gap in the just-added array typedef compound literal path: `typedef const int Scores[2]; (Scores){1, 2}` and `typedef const struct Point Points[2]; (Points){{...}}` evaluated as mutable array compound literals. The selected work package now carries read-only metadata on scalar and aggregate array compound-literal expressions, preserves explicit `const` and const-qualified array typedef aliases during pointer-conversion metadata checks, rejects mutable pointer flows that would discard const, and stores the evaluated compound literal as read-only array/aggregate-array storage.

Commands verified so far:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter const_array_typedef_compound_literal -- --nocapture  # RED: mutable pointer const-discard fixture returned Ok(1); GREEN passed after read_only expression metadata
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous array-typedef compound literal run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, additional parser-trust diagnostics, mixed conformance fixtures, pointer/const/storage-class audits through nested/anonymous aggregate paths, targeted pointer-arithmetic negative coverage, and a concrete C99 type-name parity gap: compound literals whose type name is a one-dimensional array typedef alias. The selected work package now lets `typedef int Scores[3]; (Scores){1, 2, 3}`, `typedef char Word[4]; (Word){"cat"}`, and `typedef struct Point Points[2]; (Points){{...}}` lower through Cust's existing scalar-array and aggregate-array compound literal machinery while preserving the unsupported non-literal array-cast diagnostic.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter array_typedef_compound_literals -- --nocapture  # RED: pointer casts are not supported; GREEN passed after DeclType::Array compound-literal routing
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous aggregate-field parenthesized pointer diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, additional mixed supported-subset conformance fixtures, pointer/const/storage-class audits through nested/anonymous aggregate field paths, and a fresh parser-trust gap: unsupported parenthesized pointer declarators inside aggregate field lists. The selected work package keeps Cust's current no-function-pointer/no-parenthesized-pointer field boundary but reports targeted diagnostics for `struct Hooks { int (*callback)(int); };` and `struct Matrix { int (*row)[3]; };` instead of the misleading generic `expected struct field name after type, found LParen` fallback. Implementation reuses the existing parenthesized pointer declarator lookahead before aggregate field-name parsing.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_parenthesized_pointer_aggregate_fields_with_context -- --nocapture  # RED: generic missing field-name diagnostic; GREEN passed after aggregate-field parenthesized-pointer lookahead
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous inline enum aggregate field run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source fuzzing for another exact diagnostic, additional mixed supported-subset conformance fixtures, pointer/const diagnostic coverage through embedded/anonymous aggregate paths, and a concrete C declaration parity gap: inline `enum` specifiers used as fields inside supported aggregate definitions. The selected work package lets declarations such as `struct Flags { enum State { STATE_READY = 3, STATE_DONE = 7 } state; enum { MODE_FAST = 11 } mode; };` and `typedef struct { enum { TYPE_VALUE = 17 } code; } TypeHolder;` parse field-local enum definitions as scalar integer fields while installing their enumerators in the enclosing runtime scope before later global initializers/functions use them. Multiple inline enum field definitions now append pending constants instead of overwriting them, and standalone aggregate definitions, anonymous aggregate object declarations, and aggregate typedef declarations all flush pending inline enum constants without leaking stale metadata into the next declaration.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter inline_enum_aggregate_fields -- --nocapture  # RED: undefined variable 'LOCAL_BASE' / 'TYPE_VALUE'; GREEN passed after pending enum constants flush through aggregate definitions/typedefs
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-24 autonomous nested named aggregate field definition run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source fuzzing for another exact diagnostic, additional mixed supported-subset conformance fixtures, pointer/const diagnostic coverage through embedded/anonymous aggregate paths, and a concrete C aggregate declaration parity gap: field-local named `struct`/`union` definitions inside supported aggregate definitions. The selected work package lets declarations such as `struct Scene { struct Point { int x; int y; } origin, cursor; union Number { int value; char tag; } primary, secondary; struct Segment { struct Point start; struct Point end; } segments[2]; };` parse by recursing through the aggregate definition parser for `struct`/`union` followed by either `{` or `Ident {`, then reusing the existing field declarator-list loop. This preserves reusable nested tag metadata for later fields in the same definition and relies on existing nested field, array-field, initializer, access, mutation, and compiler-oracle paths.

Commands verified:

```bash
git checkout main && git pull --ff-only
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_nested_named_aggregate_fields -- --nocapture  # RED: undefined struct type 'Point'; GREEN passed after field-definition parser routing
cargo test --test c_compat -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous const embedded aggregate-array element copy-assignment run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source fuzzing for another exact diagnostic, additional mixed supported-subset conformance fixtures, and a concrete const/aggregate-array assignment gap from the backlog. The selected work package closes const enforcement for aggregate-array element copy assignment through struct pointers: `slot->points[0] = replacement` now rejects writes to `const struct Point points[2];` with `cannot assign to const struct field 'points'` instead of mutating through the address-of-struct-pointer-array-field path. Direct `line.points[1] = replacement` coverage was also added and already passed, proving the direct variable-backed path was safe. The implementation extends metadata-only const-origin tracing for `StructPtrArrayGet` / `AddressOfStructPtrArrayField` so pointer expressions derived from struct-pointer aggregate-array fields recover the parent const field label before assignment.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_const_embedded_aggregate_array_element_copy_assignment -- --nocapture  # RED: pointer path returned Ok(5); GREEN passed after metadata-only const field tracing for struct-pointer array-field expressions
cargo test --test interpreter embedded_aggregate_array_element_assignment -- --nocapture
cargo test --test interpreter const_aggregate -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous const aggregate-array field compound-literal diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source fuzzing for a fresh parser-trust diagnostic, additional mixed supported-subset conformance fixtures, and a targeted const/aggregate edge from the current backlog. The selected work package tightens const diagnostics for nested scalar writes through const aggregate-array fields selected from aggregate compound literals: `((struct Box){{...}}).points[1].x = 9` now reports `cannot assign to const struct field 'points'` instead of the generic `cannot assign through pointer to const`. Direct variable-backed const aggregate-array field writes such as `box.points[1].x = 9` are also locked in with coverage. The implementation keeps the existing pointer-to-const safety gate but adds metadata-only origin tracing for `AggregateFieldGet` pointer expressions and pointer arithmetic over those expressions, avoiding evaluation of compound-literal initializers while recovering the const parent field label.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_assignment_to_nested_fields_of_const_aggregate_array_fields -- --nocapture  # passed immediately as existing direct-field behavior coverage
cargo test --test interpreter rejects_assignment_to_nested_fields_of_const_aggregate_array_fields_on_compound_literals -- --nocapture  # RED failed with generic pointer-to-const diagnostic; GREEN passed after metadata-only field-label tracing
cargo test --test interpreter const_aggregate -- --nocapture
cargo test --test interpreter aggregate_compound_literal -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous const aggregate field nested-write run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source fuzzing for another exact diagnostic, additional mixed conformance fixtures, anonymous aggregate pointer declaration-list type/const edge cases, and a concrete const-enforcement audit for aggregate fields. The selected work package fixes a correctness gap for const-qualified aggregate fields: nested writes through named and anonymous const aggregate fields such as `box.point.x = 3` and `anon.point.y = 4` now report `cannot assign to const struct field 'point'` instead of mutating the nested scalar. Initializers and read paths remain unchanged. Implementation is a recursive const guard in `assign_scalar_field_in_map()` before descending into a nested `StructFieldValue::Struct`.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_assignment_to_nested_fields_of_const -- --nocapture  # RED failed with Ok(3)/Ok(4); GREEN passed after recursive const guard
cargo test --test interpreter const_aggregate -- --nocapture
cargo test --test interpreter const_struct -- --nocapture
cargo test --test interpreter supports_anonymous_aggregate_fields -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous anonymous aggregate field run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source fuzzing for fresh exact diagnostics, additional mixed conformance fixtures, anonymous aggregate pointer declaration-list const/type edge cases, and a concrete anonymous aggregate declaration parity gap. The selected work package adds anonymous `struct { ... }` / `union { ... }` field definitions inside supported aggregate definitions: `struct Box { struct { int x; int y; } point; union { int value; char tag; } number; struct { int value; } items[2]; };` now parses through the shared anonymous aggregate definition body, creates unique internal type identities, and reuses existing nested aggregate field, array-field, initializer, field access, and copy semantics. A negative regression also locks in that separately spelled anonymous aggregate pointer types remain distinct and incompatible.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_anonymous_aggregate_fields -- --nocapture  # RED failed with `expected struct field type, found LBrace`; GREEN passed after aggregate-field parser routing
cargo test --test interpreter rejects_distinct_anonymous_aggregate_pointer_assignments -- --nocapture  # passed immediately as negative coverage for distinct anonymous type identities
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous anonymous aggregate pointer-cast run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, more malformed-source parser-trust diagnostics, additional mixed supported-subset conformance fixtures, anonymous aggregate pointer declaration-list edge cases, and a concrete C99 type-name pointer expression parity gap. The selected work package adds pointer casts whose pointee type is an expression-local anonymous `struct`/`union` type name: forms such as `(struct { int x; } *)0`, `(const union { char tag; } *)0`, and non-evaluating metadata queries like `sizeof(*(struct { char tag; } *)0)` now parse through the shared anonymous aggregate definition body, create a unique internal anonymous aggregate type identity, and lower to the existing safe `Expr::PointerCast` path. No source-level tag, typedef alias, or anonymous aggregate pointer-object compatibility across distinct spellings was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_anonymous_aggregate_pointer_casts -- --nocapture  # RED failed with `expected ')' after cast type, found Star`; GREEN passed after anonymous aggregate pointer-cast parser support
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous anonymous aggregate compound-literal run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source parser-trust diagnostics, additional mixed conformance fixtures, anonymous aggregate pointer declaration-list edge cases, and a concrete C99 type-name expression parity gap. The selected work package adds anonymous `struct`/`union` type-name support in aggregate compound literals: forms such as `((struct { int x; int y; }){.x = 2}).x`, `((union { int value; char tag; }){7}).value`, and anonymous aggregate-array compound literals such as `((struct { int x; int y; }[]){{1, 2}, {.y = 9}})[1].y` now parse through the shared anonymous aggregate definition body, create a unique internal type identity, and reuse existing aggregate/aggregate-array compound literal evaluation. No source-level tag or typedef alias is installed for these expression-local anonymous types.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_anonymous_aggregate_compound_literals -- --nocapture  # RED failed with `expected cast type, found LBrace`; GREEN passed after cast/type-name parser support
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous anonymous aggregate type-query run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, additional malformed-source diagnostics, more mixed supported-subset conformance fixtures, and a concrete remaining C type-query parity gap. The selected work package adds `sizeof` / `_Alignof` support for anonymous aggregate type-name operands: `sizeof(struct { int x; char tag; })`, `sizeof(union { int value; char tag; })`, `sizeof(const struct { ... })`, `sizeof(struct { ... } *)`, `sizeof(struct { ... }[N])`, and `_Alignof(struct/union { ... })` now parse through the shared anonymous aggregate definition body and evaluate with Cust's deterministic no-padding size/alignment model. No source-level tag or typedef alias is installed for these anonymous type operands.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_sizeof_and_alignof_anonymous_aggregate_type_names -- --nocapture  # RED failed with `expected sizeof struct type name, found LBrace`; GREEN passed after type-query parser support
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous anonymous aggregate parenthesized pointer declarator diagnostic run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, additional mixed supported-subset conformance fixtures, storage-class/anonymous-aggregate routing audits, and targeted negative coverage for anonymous aggregate pointer declaration-list edge cases. The selected work package closes a parser-trust gap for unsupported anonymous aggregate parenthesized pointer declarators: `struct { int x; } (*slot);` now reports `parenthesized pointer declarations are not supported` at the parenthesized declarator instead of the generic `expected struct variable name, found LParen` fallback. The fix is parser-local in `parse_aggregate_var_decl_after_type()` and mirrors the existing named/typedef-backed declaration guard; no parenthesized pointer declarator or function-pointer runtime support was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter anonymous_aggregate_parenthesized_pointer -- --nocapture  # RED failed with generic missing-name diagnostic; GREEN passed after parser lookahead fix
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous storage/alignment anonymous aggregate declaration-context run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source parser-trust diagnostics, more mixed supported-subset conformance fixtures, and the storage/alignment declaration-context audit called out by the backlog. The selected work package closes a concrete parser routing gap for qualifier-leading anonymous aggregate object declarations behind local storage/alignment specifiers: `static _Alignas(8) const struct { ... } value`, `_Alignas(8) volatile union { ... } scratch`, and `static _Thread_local const struct { ... } local_shape` now route through `parse_aggregate_var_decl()` instead of the scalar declaration parser, preserving anonymous type identity, same-declaration pointer declarators, static-local wrapping, and const pointer-view metadata. Global `_Thread_local const struct { ... }` coverage was also added. No new runtime storage model was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_alignas_specifiers -- --nocapture  # RED failed with `expected struct type name, found LBrace`; GREEN passed after declaration-context routing fix
cargo test --test interpreter supports_thread_local_storage_class_specifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest:

All passed after the 2026-06-23 autonomous qualified anonymous aggregate `for` initializer run. Ideation considered failing tests/builds (none from the inherited clean state), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source parser-trust diagnostics, additional mixed supported-subset conformance fixtures, storage/alignment declaration-context audits, and targeted anonymous aggregate pointer declaration-list type/const edge cases. The selected work package closes the qualifier-leading follow-up to anonymous aggregate `for` initializer support: Cust now accepts `for (const struct { int limit; } cfg = {...}, *view = &cfg; ...)` and `for (volatile union { int value; char tag; } number = {...}; ...)` by routing qualified aggregate starts through `parse_aggregate_var_decl()` before the scalar declaration parser in `parse_for`. The new invalid regression proves same-declaration const anonymous aggregate pointer views in `for` initializers still reject `slot->x = ...` with `cannot assign through pointer to const`. No new runtime storage model was added.

Commands verified:

```bash
cargo test --test interpreter qualified_anonymous_aggregate_for -- --nocapture  # RED failed with `expected struct type name, found LBrace`; GREEN passed after parse_for routing fix
cargo test --test interpreter const_anonymous_aggregate_for_initializer_pointer -- --nocapture  # RED failed with the same parser routing error; GREEN passed and preserved const pointer-view diagnostics
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt
# Full required gate was run after this status update; see final run report for exact pass/fail output.
```

Previous latest: All passed after the 2026-06-23 autonomous anonymous aggregate `for` initializer run. Ideation considered failing tests/builds (baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source parser-trust diagnostics, additional mixed supported-subset conformance fixtures, storage/alignment declaration-context audits, and targeted anonymous aggregate pointer declaration-list type/const edge cases. The selected work package closes a concrete declaration-context parity gap: Cust now accepts anonymous aggregate object declarations directly in `for` initializers, including `for (struct { int x; } point = {1}; ...)` and `for (union { int value; char tag; } number = {5}; ...)`. `parse_for` now routes `Token::Struct | Token::Union` through `parse_aggregate_var_decl()` in initializer position, reusing existing unique anonymous aggregate type identities, initializer semantics, field lvalues, and loop scoping. No new runtime storage model was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_anonymous_aggregate_for_initializers -- --nocapture  # RED failed with `unexpected token in for initializer: Struct`; GREEN passed after parser routing fix
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted the known non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

Previous: All passed after the 2026-06-22 autonomous anonymous aggregate pointer-declarator diagnostic run. Ideation considered failing tests/builds (none in the inherited verified state), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, malformed-source parser-trust diagnostics, additional mixed supported-subset conformance fixtures, storage/alignment declaration-context audits, and targeted anonymous aggregate pointer declaration-list type/const edge cases. The selected work package closes two concrete unsupported-form diagnostic gaps in anonymous aggregate object declarations: `struct { int x; } **slot;` now reports `pointer-to-pointer declarations are not supported` at the second `*`, and `struct { int x; } *slots[2];` now reports `pointer array declarations are not supported` at `[`, matching the named/typedef-backed pointer declaration boundary instead of falling through to generic missing-name/missing-`=` diagnostics. No pointer-to-pointer or pointer-array runtime support was added.

Commands verified:

```bash
cargo test --test interpreter anonymous_aggregate_pointer -- --nocapture  # RED failed with generic diagnostics; GREEN passed after parser checks
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted the known non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous const anonymous aggregate array negative-coverage run. Ideation considered failing tests/builds (none; baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, another malformed-source parser-trust diagnostic, additional mixed supported-subset conformance coverage, storage/alignment declaration-context audits, and targeted const anonymous aggregate array negative coverage. The selected work package locks in a previously unisolated const-safety edge for same-declaration anonymous aggregate arrays and pointer declarators: `const struct { int x; int y; } points[2] = ..., *slot = points + 1; slot->x = 9;` now has focused invalid fixture coverage proving the pointer view remains pointer-to-const and rejects writes with `cannot assign through pointer to const`. Focused RED initially failed because the first test fixture wrote directly through `points[1].x` and correctly produced the existing root-const diagnostic `cannot assign to const variable 'points'`; the fixture was narrowed to the intended pointer-view path and then passed. No production parser/runtime change was needed.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_assignment_to_const_anonymous_aggregate_array_elements -- --nocapture  # RED exposed incorrect first fixture expectation; GREEN passed after targeting pointer-view write
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous old-style function parameter diagnostic run. Ideation considered failing tests/builds (none; baseline `cargo test` passed), active blockers (none), the first unchecked C-subset closure item in `status/todo.md`, const anonymous aggregate negative coverage, more mixed conformance fixtures, and another parser-trust diagnostic. The selected work package closes a concrete malformed-source diagnostic gap for unsupported K&R/old-style function identifier-list definitions such as `int add(x, y) int x; int y; { ... }`: Cust now reports `old-style function parameter lists are not supported` at the first identifier-list parameter instead of the generic `expected parameter type` fallback. The detection is conservative so modern malformed definitions such as `int identity(value) { ... }` keep the existing missing-parameter-type diagnostic.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_old_style_function_parameter_lists_with_context -- --nocapture  # RED failed with old generic parameter-type diagnostic; GREEN passed after parser lookahead routing
cargo test --test interpreter reports_missing_parameter_types_before_parameter_names -- --nocapture
cargo test --test interpreter parameter -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous anonymous aggregate return-type diagnostic run. Ideation considered failing tests/builds (none; baseline `cargo test` passed), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, another malformed-source parser-trust diagnostic, additional mixed supported-subset conformance fixtures, storage/alignment declaration-context audits, and const anonymous aggregate negative coverage. The selected work package closes a concrete parser-trust gap adjacent to existing anonymous aggregate object/typedef support: function return types spelled directly as anonymous aggregate types such as `struct { int x; } make(void)` now report targeted `anonymous aggregate return types are not supported` at the `struct` keyword instead of falling through to the generic aggregate-variable semicolon diagnostic. No anonymous aggregate return runtime/type support was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_anonymous_aggregate_return_types_with_context -- --nocapture  # RED failed with old generic aggregate variable semicolon diagnostic; GREEN passed after parser lookahead routing
cargo test --test interpreter anonymous_aggregate -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous anonymous aggregate parameter diagnostic run. Ideation considered failing tests/builds (none; baseline `cargo test` passed), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, malformed-source fuzzing for a fresh exact diagnostic, additional mixed supported-subset conformance fixtures, auditing storage/alignment declaration contexts for anonymous aggregates, and const anonymous aggregate negative coverage. The selected work package closes a concrete parser-trust gap adjacent to existing anonymous aggregate object/typedef support: function parameters spelled directly as anonymous aggregate types such as `struct { int x; } point` now report targeted `anonymous aggregate parameters are not supported` at the `struct` keyword instead of falling through to the generic `expected parameter type, found LBrace` message. No anonymous aggregate parameter runtime/type support was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_anonymous_aggregate_parameters_with_context -- --nocapture  # RED failed with old generic LBrace diagnostic; GREEN passed after parser parameter routing
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous enum forward-declaration diagnostic run. Ideation considered failing tests/builds (none; baseline `cargo test` passed), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, malformed-source fuzzing for fresh exact diagnostics, additional const anonymous aggregate negative coverage, and more mixed supported-subset conformance fixtures. The selected work package closes a concrete parser-trust gap adjacent to the existing unsupported aggregate forward-declaration diagnostics: top-level `enum Color;` now reports targeted `forward enum declarations are not supported` at the semicolon instead of falling through to the misleading `undefined enum type 'Color'` message at the tag token. No incomplete enum runtime/type support was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_aggregate_forward_declarations_with_context -- --nocapture  # RED failed with old undefined-enum diagnostic; GREEN passed after parser diagnostic routing
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous mixed declaration-context conformance run. Ideation considered failing tests/builds (none after pull/status inspection), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, malformed-source fuzzing for fresh exact diagnostics, anonymous aggregate pointer-first/const-array negative coverage, and a mixed supported-subset conformance fixture. The selected work package adds a compact compiler-oracle fixture that combines several recently completed declaration/runtime surfaces in one warning-free C program: comma-separated typedef alias lists, inline enum object declarations in a `for` initializer and local declaration, pointer-first anonymous aggregate declaration lists, const anonymous aggregate arrays with pointer views, aggregate compound literal returns, and typedef-spelled aggregate return values. The focused interpreter coverage passed immediately, confirming this was a conformance-lock fixture over already-supported behavior rather than a production parser/runtime change; the native C compiler-oracle suite also passed.

Commands verified:

```bash
cargo test --test interpreter supports_mixed_declaration_context_conformance_fixture -- --nocapture  # coverage test passed immediately; no production behavior change required
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous inline enum declaration-context run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, malformed-source fuzzing for fresh diagnostics, mixed supported-subset conformance fixtures, anonymous aggregate pointer-first/const-array coverage, and the follow-up audit from the prior inline-enum run. The selected work package closes inline enum object declarations in ordinary declaration contexts beyond simple local/global statements: Cust now accepts inline enum object declarations in `for` initializers (`for (enum { START = 2 } i = START; ...)`), block-scope `static enum { SAVED = 4 } saved = SAVED;`, and local `auto enum` / `register enum` declarations. Parser routing now allows `enum` after these storage/context specifiers, and static-local wrapping preserves the generated runtime `EnumDecl` before wrapping only the actual variable declarations as `StaticLocal`, so same-statement enum constants are visible during first-time static initialization without assigning static storage ids to enum-constant declarations.

Commands verified:

```bash
cargo test --test interpreter inline_enum_object_declarations_in_storage -- --nocapture  # RED failed with `expected declaration after static, found Enum`; GREEN passed after parser/static-local wrapping changes
cc -std=c11 -Wall -Wextra -Werror tests/fixtures/compat/valid/inline_enum_declaration_contexts.c -o /tmp/cust-inline-enum-contexts && /tmp/cust-inline-enum-contexts; printf 'exit=%s\n' "$?"  # native oracle returned 41; corrected new test expectation before GREEN
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous inline enum object declaration run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, malformed-source fuzzing for fresh diagnostics, mixed supported-subset conformance fixtures, anonymous aggregate pointer/const-array coverage, and an audit of declaration contexts where parser-only enum/type metadata must survive into runtime initialization. The selected work package closes inline enum definition object declarations because it is a compact ordinary-C declaration gap adjacent to completed enum typedef/direct enum work: Cust now accepts `enum { A = 1 } value = A;`, `const enum { LOCKED = 7 } global = LOCKED;`, `enum Mode { IDLE = 11 } mode = IDLE;`, and same-declaration enum scalar lists such as `enum { ONE = 1 } first = ONE, second = first + 5;`. Inline enum constants are installed as runtime read-only enum constants before the associated variable declaration so same-statement initializers and later expressions can reference them, while const enum object declarations reuse existing scalar const assignment diagnostics.

Commands verified:

```bash
cargo test --test interpreter inline_enum -- --nocapture  # RED failed with old enum-declaration/enum-type-name routing; GREEN passed after parser/runtime statement wrapping
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-22 autonomous comma-separated typedef alias run. Ideation considered failing tests/builds (none; baseline `cargo test` passed), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, malformed-source fuzzing for fresh diagnostics, mixed supported-subset conformance fixtures, anonymous aggregate pointer/const-array coverage, and auditing typedef alias const metadata in declaration contexts. The selected work package closes ordinary C typedef declarator-list parity because it is a compact, high-impact declaration syntax gap adjacent to previous declaration-list work: Cust now accepts forms such as `typedef int Count, *CountPtr, Counts[3];`, `typedef const int ConstCount, *ConstCountView, ConstCounts[2];`, `typedef struct Point Point, *PointPtr, Points[2];`, and `typedef struct { int value; int extra; } Anon, *AnonPtr, Anons[2];`. Per-declarator pointer stars, post-star const pointer-slot metadata, and array suffixes are preserved; unsupported pointer-to-pointer, pointer-array, function typedef, and multidimensional typedef diagnostics still apply per declarator.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_comma_separated_typedef_aliases -- --nocapture  # RED failed with old missing-semicolon typedef diagnostic; GREEN passed after parser refactor
cargo test --test interpreter comma_separated_typedef -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous anonymous aggregate array-pointer declaration-list conformance run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, malformed-source fuzzing for fresh diagnostics, mixed conformance fixtures that combine declaration lists/aggregate pointers/const views/compound literals, and a typedef alias const-metadata audit. The selected work package locks in same-declaration anonymous aggregate arrays with pointer declarators, e.g. `struct { int x; int y; } points[3] = {...}, *slot = points + 1;` and matching anonymous union arrays/pointers. Focused RED initially failed due to an incorrect expected arithmetic total in the new test (Cust returned 31); after correcting the test expectation, the focused interpreter test and compiler-oracle suite passed. No production parser/runtime code was required because the previous anonymous aggregate declaration-list support already handled this supported C declarator shape.

Commands verified:

```bash
cargo test --test interpreter supports_anonymous_aggregate_array_pointer_declaration_lists -- --nocapture  # RED failed with incorrect new-test expectation; GREEN passed after correcting expected result to 31
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous anonymous aggregate const/pointer declaration-list run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` C-subset closure item, extending anonymous aggregate object coverage to const/pointer/address-of forms, mixed supported-subset conformance fixtures, malformed-source diagnostic fuzzing, and a typedef-const metadata audit. The selected work package extends anonymous `struct { ... }` / `union { ... }` object declarations to qualified aggregate specifiers and same-declaration pointer declarator lists: `const struct { ... } value = {...};` now parses through aggregate declaration routing, and declaration lists such as `struct { int x; int y; } point = {4, 5}, copy = point, *slot = &point;` preserve one unique anonymous type across all declarators so pointer initialization, `->` mutation, by-value copy, and const aggregate write diagnostics work without installing user-visible tags or aliases.

Commands verified:

```bash
cargo test --test interpreter supports_const_and_pointer_anonymous_aggregate_declaration_lists -- --nocapture  # RED failed with expected missing qualified-anonymous aggregate routing; GREEN passed after parser routing fix
cargo test --test interpreter anonymous_aggregate -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous anonymous aggregate object declaration run. Ideation considered failing tests/builds (none; `cargo test` passed after pull), active blockers (none), the first unchecked `status/todo.md` parser/runtime parity item, mixed supported-subset conformance fixtures, malformed-source exact diagnostics, and a typedef-const metadata audit. The selected work package adds ordinary C anonymous `struct`/`union` object declarations in supported aggregate object contexts: global/local anonymous aggregate variables, aggregate arrays, brace/designated initializers, scalar array fields, union fields, and field reads now reuse Cust's existing unique internal aggregate type identities without leaking source-level tags or aliases. Anonymous aggregate typedef definitions remain supported as before; no native ABI layout compatibility was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_anonymous_aggregate_object_declarations -- --nocapture  # RED failed with expected missing anonymous aggregate type-name parsing; GREEN passed after parser routing/helper split
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous aggregate bit-field diagnostic run. Ideation considered failing tests/builds (none; `cargo test` passed after pull), active blockers (none), the first unchecked `status/todo.md` parser/runtime parity item, mixed supported-subset conformance fixtures, malformed-source fuzzing, and an audit of declaration contexts around typedef const metadata. The selected work package closes a newly discovered parser-trust gap for unsupported C aggregate bit-fields: `struct Flags { unsigned ready : 1; };` and `union Bits { int value : 3; };` now report targeted `bit-field aggregate fields are not supported` diagnostics at the colon instead of falling through to the generic field-declaration semicolon helper. No bit-field storage/layout runtime support was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_aggregate_bit_fields_with_context -- --nocapture  # RED failed with the old generic semicolon diagnostic; GREEN passed after parser routing fix
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous aggregate forward declaration diagnostic run. Ideation considered failing tests/builds (none; `cargo test` passed after pull), active blockers (none), the first unchecked `status/todo.md` parser/runtime parity item, mixed supported-subset conformance fixtures, malformed-source fuzzing, and an audit of declaration contexts around typedef const metadata. The selected work package closes a newly discovered parser-trust gap for unsupported C incomplete aggregate declarations: top-level `struct Point;` and `union Number;` now report targeted `forward struct declarations are not supported` / `forward union declarations are not supported` diagnostics at the semicolon instead of falling through to misleading `undefined struct type` variable-declaration errors. No incomplete-type runtime support was added.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_aggregate_forward_declarations_with_context -- --nocapture  # RED failed with the old undefined-type diagnostic; GREEN passed after parser routing fix
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous aggregate-field typedef const metadata run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` parser/runtime parity item, malformed-source diagnostic fuzzing, and mixed supported-subset conformance fixtures. The selected work package closes the highest-impact compact parity gap in aggregate field declaration lists: fields spelled with typedef aliases now preserve alias-carried const metadata, including const pointer-slot aliases such as `typedef int * const ConstIntSlot; struct Cursor { ConstIntSlot fixed, backup; };`. Cust continues to support comma-separated typedef-backed scalar/pointer/aggregate/array fields, preserves pointee-const metadata for `typedef const int *ConstIntView;`, rejects assignment to const pointer-slot fields with `cannot assign to const struct field 'fixed'`, and verifies the supported subset against the native C compiler oracle.

Commands verified:

```bash
cargo test --test interpreter aggregate_field -- --nocapture  # RED exposed the const pointer-slot typedef field assignment gap; GREEN passed after parser metadata fix
cargo test --test interpreter rejects_assignment_to_const_pointer_slot_typedef_field_in_declaration_list -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous aggregate field declaration-list run. Ideation considered failing tests/builds (none after pull), active blockers (none), remaining P0 parser-trust diagnostics, mixed supported-subset conformance-only fixtures, and the first unchecked `status/todo.md` item for newly discovered parser/runtime parity gaps. The selected work package closes an ordinary C aggregate declaration gap: Cust now accepts comma-separated fields inside struct/union definitions such as `int x, y;`, `char tag, code;`, `struct Point start, end;`, `int weights[2], offsets[2];`, `int *head, *tail;`, `const int *view, *limit;`, and `union Number { int value, other; };`. Field duplicate diagnostics continue to point at the duplicate declarator in a declaration list, and native compiler-oracle coverage confirms the supported subset matches C exit behavior.

Commands verified:

```bash
cargo test --test interpreter aggregate_field_declaration_lists -- --nocapture  # RED failed with expected comma-after-field parser error; GREEN passed after implementation
cargo test --test interpreter rejects_duplicate_aggregate_fields_in_declaration_lists -- --nocapture  # RED failed with old semicolon diagnostic; GREEN passed with duplicate-field diagnostic
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous aggregate pointer declaration-list conformance run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` item for struct/union pointer declarator-list fixtures, additional malformed-source exact diagnostics, and broader tooling-only work. The selected work package completes the concrete unchecked conformance coverage item because mixed declaration-list parser/runtime support was already implemented but aggregate pointer declarators needed explicit interpreter and native-oracle coverage. Cust is now covered for ordinary C declaration lists such as `struct Point *p = points, *q = points + 1;` and `union Number *n = numbers, *m = numbers + 2;`, with `->`, indexed aggregate pointer reads/writes, pointer arithmetic, and function-argument flow verified against both Cust and the C compiler oracle. The focused interpreter test passed immediately, confirming this was a coverage/conformance closure item over already-supported runtime behavior rather than a production-code behavior change.

Commands verified:

```bash
cargo test --test interpreter supports_aggregate_pointer_declaration_lists -- --nocapture  # coverage test passed immediately; no production behavior change required
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous embedded aggregate-array element assignment run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` parity gap, struct/union pointer declaration-list fixture expansion, malformed-source diagnostic polish, and conformance/tooling-only work. The selected work package closes aggregate-array element copy assignment through embedded struct fields because it is the first concrete unchecked runtime parity item and directly extends the prior top-level aggregate-array assignment slice. Cust now accepts direct embedded aggregate-array writes such as `line.points[1] = replacement`, aggregate assignment expressions returning by-value copies such as `struct Point returned = (line.points[1] = replacement)`, and struct-pointer embedded aggregate-array writes such as `slot->points[0] = (struct Point){11, 12}`. The implementation reuses the interpreter-owned embedded aggregate-array pointer metadata for `->` paths and adds direct `StructArraySet` aggregate routing for field paths, preserving deep-copy isolation, same-type checks, const/read-only diagnostics, and scalar array-field assignment behavior.

Commands verified:

```bash
cargo test --test interpreter supports_embedded_aggregate_array_element_copy_assignment -- --nocapture  # RED failed with expected missing embedded aggregate StructArraySet routing; GREEN passed after implementation
cargo test --test interpreter rejects_embedded_aggregate_array_element_assignment_type_mismatch -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

All passed after the 2026-06-21 autonomous embedded aggregate-array element assignment run. Ideation considered failing tests/builds (none after pull), active blockers (none), the first unchecked `status/todo.md` parity gap, struct/union pointer declaration-list fixture expansion, malformed-source diagnostic polish, and conformance/tooling-only work. The selected work package closes aggregate-array element copy assignment through embedded struct fields because it is the first concrete unchecked runtime parity item and directly extends the prior top-level aggregate-array assignment slice. Cust now accepts direct embedded aggregate-array writes such as `line.points[1] = replacement`, aggregate assignment expressions returning by-value copies such as `struct Point returned = (line.points[1] = replacement)`, and struct-pointer embedded aggregate-array writes such as `slot->points[0] = (struct Point){11, 12}`. The implementation reuses the interpreter-owned embedded aggregate-array pointer metadata for `->` paths and adds direct `StructArraySet` aggregate routing for field paths, preserving deep-copy isolation, same-type checks, const/read-only diagnostics, and scalar array-field assignment behavior.

Commands verified:

```bash
cargo test --test interpreter supports_embedded_aggregate_array_element_copy_assignment -- --nocapture  # RED failed with expected missing embedded aggregate StructArraySet routing; GREEN passed after implementation
cargo test --test interpreter rejects_embedded_aggregate_array_element_assignment_type_mismatch -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous aggregate-array element copy assignment run. Ideation considered failing tests/builds (none; pre-change `cargo test` passed), active blockers (none), remaining parser diagnostics, embedded aggregate-array field assignment parity, struct/union pointer declaration-list fixture expansion, and the first concrete unchecked runtime parity gap in `status/todo.md`. The selected work package closes direct aggregate-array element copy assignment because it is compact, high-impact C-subset parity adjacent to existing aggregate pointer/indexed-value support. Cust now accepts `points[0] = replacement`, aggregate assignment expressions such as `struct Point returned = (points[0] = replacement)`, and indexed aggregate pointer writes such as `cursor[0] = (struct Point){11, 12}`. The runtime deep-clones same-type aggregate RHS fields, preserves copy isolation, routes statement/discard and aggregate-expression contexts through the same helper, keeps scalar array assignment behavior unchanged, and retains const/read-only aggregate target diagnostics.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_aggregate_array_element_copy_assignment -- --nocapture  # RED failed with expected missing aggregate ArraySet routing; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-21 autonomous comma-separated mixed declaration-list run. Ideation considered failing tests/builds (none; pre-change `cargo test` passed), active blockers (none), the remaining unchecked declaration-list parity item, parser diagnostics, pointer/aggregate expression edges, and product/tooling polish. The selected work package extends comma-separated declaration lists beyond the previous scalar-only slice because it was the first concrete unchecked roadmap item and closes ordinary C declaration syntax for pointer, array, and aggregate declarators without changing runtime storage semantics. Cust now accepts mixed declarator lists such as `int *p = values, *q = values + 2;`, `const int *view = values + 1, *start = values;`, `int values[3] = {7, 8, 9}, zeros[2];`, `struct Point point = {10, 11}, copy = point;`, `struct Point points[2] = {{1, 2}, {3, 4}}, empty_points[1];`, and `union Number number = {12}, other;`. Declaration-list initializers are parsed at assignment-expression precedence so the separating comma is preserved for subsequent declarators; parenthesized comma expressions remain available inside initializers.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_comma_separated_pointer_array_and_aggregate_declarations -- --nocapture  # RED failed with expected comma-after-array-declaration parser error before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-20 autonomous comma-separated scalar declaration run. Ideation considered failing tests/builds (none; pre-change `cargo test` passed), active blockers (none), newly discovered parser diagnostics, additional compound-literal edges, aggregate-kind diagnostics, remaining declaration-specifier syntax, pointer/aggregate parity gaps, and a foundational C declaration syntax gap. The selected work package closes comma-separated scalar declarators because it is high-impact, compact, and improves ordinary C source compatibility without changing pointer/aggregate storage semantics. Cust now accepts scalar declaration lists such as `int a = 1, b, c = a + 2;`, `const int x = 7, y = 11;`, `char first = 'A', next = 'B';`, `_Bool ok = expr, nope;`, and `for (int i = 0, j = 3; ... )` in global, local, static-local, and `for` initializer contexts. The parser lowers each declarator to same-scope `Stmt::Many` entries, preserving zero-initialization for omitted initializers and const metadata for every declared scalar. Pointer and array declarators in comma-separated lists were intentionally outside that slice and are now covered by the 2026-06-21 follow-up.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_comma_separated_scalar_declarations -- --nocapture  # RED failed with expected comma-after-declaration parser error before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-20 autonomous pointer cast expression run. Ideation considered failing tests/builds (none; pre-change `cargo test` passed), active blockers (none), newly discovered parser diagnostics, additional compound-literal edge cases, aggregate-kind diagnostics, standard-library-like builtins, and pointer/aggregate parity gaps. The selected work package closes one-level C pointer cast expressions because it is a compact high-impact pointer-conformance gap adjacent to existing scalar casts and pointer typedefs. Cust now accepts casts such as `(int *)0`, `(const int *)0`, `(IntPtr)(values + 1)`, `(ConstIntPtr)cursor`, and `sizeof(*(char *)0)` over the existing safe one-level scalar/aggregate pointer subset. Runtime values remain interpreter-owned pointers; assignment/argument boundaries still validate concrete pointee type compatibility, and explicit casts preserve source const-pointee safety instead of allowing unsafe const discard.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter pointer_cast -- --nocapture  # RED failed with expected "pointer casts are not supported" before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test

docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-06-20 autonomous inferred aggregate array declaration run. Ideation considered failing tests/builds (none from status; focused baseline was clean before implementation), active blockers (none), newly discovered parser diagnostics, compound-literal edge cases, aggregate-kind diagnostics, standard-library-like builtins, and the next concrete C declaration/initializer parity gap. The selected work package closes inferred-length aggregate array declarations because it is high-impact, scoped, and reuses already-verified aggregate-array initializer/runtime paths. Cust now accepts direct and typedef-spelled aggregate arrays with empty brackets when an initializer is present, including `struct Point points[] = {{1, 2}, {.y = 4}, [3] = {5, 6}};`, `const struct Point fixed[] = {{7, 8}, {.x = 9}};`, and `union Number numbers[] = {{3}, [2] = {.value = 5}};`. The inferred length feeds existing zero-fill, `sizeof`, pointer decay/arithmetic, const enforcement, and mutation aliasing; initializer-less aggregate arrays now get the targeted `expected '=' after inferred aggregate array declaration` diagnostic.

Commands verified:

```bash
cargo test --test interpreter inferred_aggregate_array -- --nocapture  # RED failed with expected empty-bracket aggregate-array parser errors before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-05-12 autonomous inferred scalar array declaration run. Ideation considered failing tests/builds (none; pre-change `cargo test` passed), active blockers (none), remaining parser-trust diagnostics for newly discovered malformed programs, additional compound-literal/aggregate edge cases, scoped standard-library-like builtins, CLI/product polish, and C declaration/initializer parity gaps. The selected work package closes C-style inferred-length scalar array declarations because it is a compact, high-impact conformance feature that reuses existing array initializer, designator, string literal, `sizeof`, const-array, and pointer-decay runtime paths. Cust now accepts `int values[] = {1, 2, [4] = 5, 6};`, `char word[] = "cat";`, and `const int table[] = {[1] = 3, [3] = 4};`, inferring the object length from positional/designated/string initializers while requiring an initializer for empty-bracket declarations.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_inferred_array_declarations -- --nocapture  # RED failed with expected empty-bracket array parser error before implementation; GREEN passed after implementation
cargo test --test interpreter inferred_array -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-05-12 autonomous local function prototype run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), the remaining P0 parser/lexer trust bucket for newly discovered malformed programs, additional C-compatible compound-literal edges, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, and C declaration syntax parity gaps. The selected work package closes C block-scope function prototype syntax parity: Cust now accepts no-op local prototypes such as `int add(int, int);`, `extern char pick(char, char);`, `extern struct Point make_point(int, int);`, `union Number make_number(int);`, and array-parameter prototype spellings inside function blocks. Runtime function lookup remains the existing top-level function table; local prototypes are parser-only declarations and local nested function definitions are still rejected.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_local_function_prototypes -- --nocapture  # RED failed with expected variable-declaration parser error before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-05-12 autonomous unsupported function typedef diagnostic run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), the remaining P0 parser/lexer trust bucket for newly discovered malformed programs, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, and unsupported function type declarator forms adjacent to existing function-pointer diagnostics. The selected work package closes a concrete parser-trust gap for unsupported C function typedef aliases: `typedef int Callback(int);` now reports `function typedef aliases are not supported` at the function declarator `(` instead of falling through to a generic missing-semicolon diagnostic. The fix is parser-local and intentionally does not add function type aliases or function pointer runtime support.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter function_typedef_aliases -- --nocapture  # RED failed with generic missing-semicolon diagnostic before implementation; GREEN passed after implementation
cargo fmt --check
cargo clippy -- -D warnings
cargo test --test interpreter function_typedef_aliases -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-05-12 autonomous unsupported C11 `_Generic` diagnostic run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), the remaining P0 parser/lexer trust bucket for newly discovered malformed programs, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, remaining parser-metadata declaration syntax, deliberately scoped builtins, and unsupported C11 expression forms. The selected work package closes a concrete parser-trust gap for unsupported C11 generic selections: `_Generic(1, int: 2, default: 3)` now reports `generic selections are not supported` at the `_Generic` keyword instead of falling through to a misleading association-list parser error. The fix is lexer/parser-local and intentionally does not add type-dispatch semantics.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_generic_selections_with_context -- --nocapture  # RED failed with downstream association-list parser error before implementation; GREEN passed after implementation
cargo fmt --check
cargo clippy -- -D warnings
cargo test --test interpreter rejects_generic_selections_with_context -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Previous latest: All passed after the 2026-05-12 autonomous unsupported preprocessor-directive diagnostic run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), remaining P0 parser/lexer trust gaps for newly discovered malformed programs, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, remaining declaration specifier syntax if any, deliberately scoped builtins, and unsupported C preprocessing forms. The selected work package closes a concrete preprocessor-free subset diagnostic gap: source beginning with `#include <stdio.h>` now reports `preprocessor directives are not supported` with the existing source-line/caret context instead of falling through to generic `unexpected character '#'`. The fix is lexer-local and intentionally does not add macro/include preprocessing.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_preprocessor_directives_with_context -- --nocapture  # RED failed with generic unexpected '#' diagnostic before implementation; GREEN passed after implementation
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-12 autonomous unsupported `goto`/label diagnostic run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), remaining P0 parser recovery for newly discovered malformed programs, additional compound-literal edge cases, aggregate-kind diagnostic polish, remaining declaration specifier syntax if any, deliberately scoped builtins, and unsupported C statement forms. The selected work package closes a concrete parser-trust gap for unsupported C jump labels: `goto done;` now reports `goto statements are not supported` at the `goto` keyword, and `done:` now reports `labels are not supported` at the label identifier instead of falling through to generic missing-assignment/missing-semicolon diagnostics. The fix is parser-local and intentionally does not add arbitrary jump execution.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter rejects_goto -- --nocapture  # RED failed with generic missing-assignment diagnostic before implementation; GREEN passed after implementation
cargo test --test interpreter rejects_label -- --nocapture  # RED failed with generic missing-semicolon diagnostic before implementation; GREEN passed after implementation
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-12 autonomous variadic-parameter diagnostic run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), remaining P0 parser recovery only for newly discovered malformed programs, additional compound-literal edge cases, aggregate-kind diagnostic polish, declaration specifier ordering gaps, deliberately scoped builtins, and unsupported C declarator forms. The selected work package closes a concrete parser-trust gap for unsupported C variadic function parameters: `int f(int count, ...)` now reports `variadic function parameters are not supported` at the ellipsis start instead of falling through to a generic `expected type, found Dot` parser error. The fix is parser-local and does not add C varargs runtime support.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter variadic_function_parameters -- --nocapture  # RED failed with generic Dot type error before implementation; GREEN passed after implementation
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-12 autonomous permuted scalar type-specifier run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), remaining P0 parser recovery only for newly discovered malformed programs, additional compound-literal edge cases, aggregate-kind diagnostic polish, standard-library-like builtins, and declaration specifier syntax parity gaps. The selected work package closes a concrete C declaration-specifier parity gap: Cust now accepts supported scalar specifier permutations such as `int unsigned`, `char signed`, `int const unsigned`, `int long signed`, `int short unsigned`, and `int long long unsigned` across globals, locals, typedef aliases, casts, sizeof type operands, function returns/prototypes/parameters, pointers, and `for` declarations. The parser shares scalar-specifier consumption with function lookahead so permuted return types route to function parsing, while simple invalid-combination validation avoids silently accepting forms such as `int char`. Coverage adds interpreter and C compiler-oracle fixtures plus implementation notes in `references/cust-permuted-scalar-type-specifiers.md`.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter supports_permuted_scalar_type_specifiers -- --nocapture  # RED failed with expected parser/function-lookahead errors before implementation; GREEN passed after implementation
cargo test --test interpreter type_spellings -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-12 autonomous block-scope `extern` object declaration run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), P0 parser recovery gaps, remaining compound-literal edges, aggregate-kind diagnostic polish, standard-library-like builtins, and declaration specifier syntax parity. The selected work package closes a concrete C declaration-syntax gap: Cust now accepts block-scope `extern` object declarations such as `extern int total;`, `extern int values[3];`, `extern struct Point origin;`, `extern union Number number;`, and `extern int *cursor;` as parser-only declarations that do not shadow existing global storage. Initialized block-scope extern declarations are rejected with `extern local declarations cannot have initializers` rather than silently discarding initializer side effects. Coverage adds valid, invalid, and C compiler-oracle fixtures plus implementation notes in `references/cust-extern-local-declarations.md`.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter extern_local -- --nocapture  # RED failed with unexpected Extern before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-12 autonomous C99 array-parameter qualifier run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), P0 parser recovery gaps, remaining compound-literal edge cases, aggregate-kind diagnostics, standard-library-like builtins, and declaration/parameter syntax parity gaps. The selected work package closes a concrete C99 parameter-declarator gap: Cust now accepts `static`, `const`, `restrict`, `volatile`, and `_Atomic` metadata inside one-dimensional array parameter brackets such as `int values[static 3]`, `int values[restrict 3]`, and `struct Point points[static 2]`, while preserving existing decay to pointer parameters. Bracket `const` is mapped to pointer-slot const metadata, so `int values[const 3]` rejects reassignment of the parameter name with the existing const-variable diagnostic; leading `const` still qualifies the pointee. Coverage adds valid, invalid, and C compiler-oracle fixtures plus implementation notes in `references/cust-array-parameter-qualifiers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter array_parameter -- --nocapture  # RED failed with expected array-length parser diagnostics before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-12 autonomous void-cast expression run. Ideation considered failing tests/builds (none; `cargo test` passed before changes), active blockers (none), P0 parser recovery gaps, remaining C-compatible compound-literal edges, pointer/aggregate parity gaps, standard-library-like builtins, and small C expression conformance items. The selected work package closes the C `(void)expr` cast gap because it is a compact, high-confidence expression feature with clear TDD coverage and native compiler-oracle parity. Cust now parses `(void)` as a cast type start, evaluates the operand in discard context so scalar/pointer/void-call side effects are preserved without scalar conversion, and rejects value use with `void expression used as scalar`. Coverage adds valid, invalid, and C compiler-oracle fixtures plus implementation notes in `references/cust-void-cast-expressions.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test  # pre-change baseline; passed
cargo test --test interpreter void_cast -- --nocapture  # RED failed with expected parser errors before implementation; GREEN passed after implementation
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-12 autonomous integer-constant comma diagnostic run. Ideation considered failing tests/builds (none), active blockers (none), P0 parser recovery/error-message gaps for newly discovered malformed programs, remaining C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, pointer/aggregate parity gaps, and standard-library-like builtins. The selected work package tightens parser diagnostics for comma expressions in enum initializer and `switch case` integer-constant-expression contexts, matching C's integer-constant-expression boundary instead of falling through to generic missing-`)`/missing-`:` messages. Cust now reports `comma operator is not allowed in integer constant expression` for parenthesized enum constants, parenthesized case labels, and unparenthesized case-label commas. Coverage adds three invalid fixtures and exact-output interpreter tests. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter comma_operator_in -- --nocapture  # RED failed with missing ')' / missing ':' diagnostics before implementation
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous function-pointer parameter/local declarator diagnostics run. Ideation considered failing tests/builds (none), active blockers (none), remaining newly discovered malformed-program diagnostics, comma-expression constant-expression expansion (deferred because warning-free C-oracle parity is unclear), additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, remaining pointer/aggregate parity gaps, and deliberately scoped standard-library-like builtins. The selected work package tightens unsupported C function-pointer declarator diagnostics in contexts that previously fell through to broader parenthesized-pointer messages: local `int (*callback)(int);` declarations now report `function pointer declarations are not supported`, and parameter declarators such as `int apply(int (*callback)(int), int value)` now report `function pointer parameters are not supported` at the opening parenthesis. Pointer-to-array parenthesized declarators retain their existing targeted diagnostics. Coverage adds two invalid fixtures, exact-output interpreter tests, and updated implementation notes in `references/cust-function-pointer-declarator-diagnostics.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter rejects_function_pointer_parameters_with_context -- --nocapture  # RED failed with parenthesized-pointer diagnostic before implementation
cargo test --test interpreter function_pointer -- --nocapture
cargo test --test interpreter parenthesized_pointer -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous function-pointer declarator diagnostics run. Ideation considered failing tests/builds (none), active blockers (none), parser recovery/error-message expansion for newly discovered malformed programs, additional compound-literal edge cases, aggregate-kind diagnostic polish, remaining declaration specifier syntax, pointer/aggregate parity gaps, and deliberately scoped standard-library-like builtins. The selected work package tightens unsupported C declarator diagnostics for function-pointer declarations and typedef aliases: `int (*callback)(int);` now reports `function pointer declarations are not supported` and `typedef int (*Callback)(int);` now reports `function pointer typedef aliases are not supported` at the opening parenthesis instead of falling through to misleading missing-name errors. Coverage adds two invalid fixtures and exact-output interpreter tests; implementation notes live in `references/cust-function-pointer-declarator-diagnostics.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter rejects_function_pointer_declarations_with_context -- --nocapture
cargo test --test interpreter rejects_function_pointer_typedef_aliases_with_context -- --nocapture
cargo test --test interpreter rejects_function_pointer -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous expression-form `sizeof` integer constant-expression run. Ideation considered newly discovered malformed-program parser diagnostics, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, remaining declaration specifier syntax, pointer/aggregate parity gaps, comma expressions in enum/switch integer constant expressions, and expression-form `sizeof` in enum/switch integer constant expressions. The selected work package extends the parser-side integer-constant-expression evaluator used for enum initializer values and `switch case` labels: Cust now accepts parenthesized expression-form `sizeof(...)` operands, infers supported expression sizes without runtime evaluation, and preserves type-name `sizeof(type-name)` handling. Coverage demonstrates non-evaluation with `sizeof(1 / 0) == sizeof(int)` in `tests/fixtures/valid/switch_enum_case_labels.c` plus the compiler-oracle twin `tests/fixtures/compat/valid/switch_enum_case_labels.c`; implementation notes live in `references/cust-sizeof-expression-integer-constant-expressions.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_enum_constants_as_switch_case_labels -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous scalar-cast integer constant-expression run. Ideation considered newly discovered malformed-program parser diagnostics, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, remaining declaration specifier syntax, pointer/aggregate parity gaps, expression-form `sizeof`/comma in enum/switch integer constant expressions, and scalar casts in enum/switch integer constant expressions. The selected work package extends the parser-side integer-constant-expression evaluator used for enum initializer values and `switch case` labels: Cust now accepts scalar type-name casts, including scalar typedef aliases, at unary precedence while preserving the existing deterministic scalar-cast value model. Pointer/array/aggregate casts remain outside the supported constant-expression subset with targeted unsupported-cast diagnostics. Coverage expands `tests/fixtures/valid/switch_enum_case_labels.c` plus the compiler-oracle twin `tests/fixtures/compat/valid/switch_enum_case_labels.c`, and implementation notes live in `references/cust-cast-integer-constant-expressions.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_enum_constants_as_switch_case_labels -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous `sizeof`/`_Alignof` integer constant-expression run. Ideation considered newly discovered malformed-program parser diagnostics, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, remaining declaration specifier syntax, pointer/aggregate parity gaps, casts/comma in enum/switch integer constant expressions, and type-query operators in enum/switch integer constant expressions. The selected work package extends the parser-side integer-constant-expression evaluator used for enum initializer values and `switch case` labels: Cust now accepts type-name `sizeof(...)` and `_Alignof(...)` forms at unary precedence, reusing the existing deterministic `SizeOfType` size/alignment metadata while remaining parser-only and non-evaluating. Coverage expands `tests/fixtures/valid/switch_enum_case_labels.c` plus the compiler-oracle twin `tests/fixtures/compat/valid/switch_enum_case_labels.c` with ABI-independent `char` array and `_Alignof(char)` forms, and implementation notes live in `references/cust-sizeof-alignof-integer-constant-expressions.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_enum_constants_as_switch_case_labels -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous comparison/logical/conditional integer constant-expression run. Ideation considered newly discovered malformed-program parser diagnostics, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, remaining declaration specifier syntax, pointer/aggregate parity gaps, and further broadening enum/switch constant-expression operators. The selected work package extends the parser-side integer-constant-expression evaluator used for enum initializer values and `switch case` labels: Cust now accepts relational (`<`, `<=`, `>`, `>=`), equality (`==`, `!=`), logical (`&&`, `||`), and conditional (`?:`) operators in addition to the existing arithmetic/shift/bitwise/unary operators and visible enum constants. Folded comparison/logical results use C-style `0`/`1` values, conditional expressions are right-associative, and the helper remains parser-only and non-evaluating so runtime variables are still rejected. Coverage expands `tests/fixtures/valid/switch_enum_case_labels.c` plus the compiler-oracle twin `tests/fixtures/compat/valid/switch_enum_case_labels.c`, with implementation notes in `references/cust-comparison-logical-conditional-integer-constant-expressions.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_enum_constants_as_switch_case_labels -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous switch/enum constant-expression parity run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, enum/switch constant-expression parity, and remaining declaration/initializer conformance gaps. The selected work package closes a concrete enum/switch gap: Cust now maintains parser-side enum constant scopes and accepts enum constants as `switch case` labels, including block-scoped enum constants and simple additive integer constant expressions such as `DONE = BUSY + 3` and `case DONE:`. Duplicate switch-case detection now resolves identifier-valued case labels before comparing values. Runtime enum constant behavior remains unchanged for ordinary expressions. Coverage includes `tests/fixtures/valid/switch_enum_case_labels.c`, invalid duplicate-case fixture `tests/fixtures/invalid/switch_duplicate_enum_case.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/switch_enum_case_labels.c`, and reference notes in `references/cust-switch-enum-case-labels.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter enum_constants_as_switch_case_labels -- --nocapture
cargo test --test interpreter switch_enum -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous multiplicative compound-assignment run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, enum/switch constant-expression parity, and closing the remaining arithmetic compound-assignment syntax gap. The selected work package completes C compound-assignment operator parity for Cust's existing scalar lvalue families: `*=`, `/=`, and `%=` now lex, parse at assignment precedence, and evaluate for scalar variables, indexed scalar array/pointer lvalues, and dereferenced pointer lvalues. Division and remainder compound assignments reuse the existing `division by zero` diagnostic, while pointer-valued compound-assignment contexts reject the new operators through the established pointer-arithmetic diagnostic path. Coverage extends `tests/fixtures/valid/compound_assignments.c` plus the C compiler-oracle fixture `tests/fixtures/compat/valid/compound_assignments.c`, with reference notes in `references/cust-multiplicative-compound-assignments.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_compound_assignment_expressions_for_scalar_array_and_deref_lvalues -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-11 autonomous C11 `_Atomic` type qualifier/specifier syntax run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, remaining declaration specifier syntax parity gaps, and C11 `_Atomic` syntax as a safe no-op metadata feature. The selected work package closes one concrete qualifier/specifier gap: Cust now lexes `_Atomic`, accepts bare `_Atomic` qualifier spellings such as `_Atomic int value;`, and accepts `_Atomic(type-name)` spellings such as `_Atomic(int) value;` across supported global/local declarations, typedef aliases, parameters, function return type parsing, `for` initializer declarations, pointer/array forms, and type-query contexts. Runtime remains Cust's deterministic single-thread interpreter model; `_Atomic` intentionally does not add native atomic operations, memory-order semantics, lock-free guarantees, layout changes, or new write restrictions. Coverage includes `tests/fixtures/valid/atomic_type_qualifiers.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/atomic_type_qualifiers.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-atomic-type-qualifiers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_atomic_type_qualifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous C11 `_Thread_local` storage-class syntax run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge cases, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, and remaining declaration specifier syntax parity gaps. The selected work package closes one concrete storage-class gap: Cust now lexes `_Thread_local` and accepts it as parser-level no-op metadata on supported top-level object declarations, `static _Thread_local` globals, and `static _Thread_local` local declarations. Runtime remains Cust's deterministic single-thread interpreter model; `_Thread_local` intentionally does not change storage partitioning, pointer identity, initialization order, `sizeof`, or `_Alignof`. Coverage includes `tests/fixtures/valid/thread_local_storage_class.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/thread_local_storage_class.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-thread-local-storage-class.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_thread_local_storage_class_specifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous C11 `_Alignas` declaration specifier run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge cases, aggregate-kind diagnostic polish, standard-library-like builtins, and remaining small declaration-specifier syntax parity gaps. The selected work package closes one concrete syntax gap: Cust now lexes `_Alignas` and accepts `_Alignas(type-name)` / `_Alignas(expression)` as parser-level no-op metadata on supported object declarations, static locals, `for` initializer declarations, and aggregate fields. Runtime remains Cust's deterministic interpreter storage model; requested alignment intentionally does not change layout, `sizeof`, `_Alignof`, pointer identity, or aggregate field offsets. Coverage includes `tests/fixtures/valid/alignas_specifiers.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/alignas_specifiers.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-alignas-specifiers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_alignas_specifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous C11 `_Alignof` type-name run. Ideation considered newly discovered malformed-program parser diagnostics, additional C-compatible compound-literal edge cases, aggregate-kind diagnostic polish, remaining declaration specifier syntax, deliberately scoped standard-library-like builtins, and small C11 type-query conformance items. The selected work package closes a concrete type-query syntax gap: Cust now accepts `_Alignof(type-name)` for supported scalar, qualified, one-level pointer, one-dimensional array, direct aggregate, and aggregate typedef type names. Runtime uses Cust's deterministic interpreter alignment model (`char`/`_Bool` = 1, `int` and integer aliases = 8, pointer = 8, arrays use element alignment, structs/unions use max field alignment) rather than native ABI padding. `_Alignof(void)` reports `_Alignof(void) is not supported`. Coverage includes `tests/fixtures/valid/alignof_type_names.c`, invalid fixture `tests/fixtures/invalid/alignof_void.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/alignof_type_names.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-alignof-type-names.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter alignof -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous C11 static assertion run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, pointer/aggregate parity gaps, remaining declaration specifier syntax parity, and small C conformance closure items. The selected work package closes a concrete declaration/statement syntax gap: Cust now accepts `_Static_assert(condition, "message");` at top level and inside blocks/functions. Parser support treats the assertion condition as an assignment-precedence expression so the comma remains the assertion argument separator, stores top-level assertions with the existing globals pre-`main()` execution path, decodes the string-literal message, and runtime evaluates the condition with existing Cust truthiness. False assertions report `static assertion failed: <message>`. Coverage includes `tests/fixtures/valid/static_assertions.c`, invalid failure fixture `tests/fixtures/invalid/static_assertion_failure.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/static_assertions.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-static-assertions.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter static_assert -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous postfix/interleaved const qualifier run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, pointer/aggregate parity gaps, and remaining declaration specifier syntax parity. The selected work package closes a concrete C declaration-specifier ordering gap: Cust now accepts `const`/`volatile`/`restrict` qualifiers after supported base type spellings as well as before them, including forms such as `int const local`, `unsigned const int value`, `long const int value`, `int const *view`, `struct Point const point`, `typedef int const ConstInt`, and `typedef struct Point const ConstPoint`. The function-definition lookahead now skips interleaved qualifiers so `unsigned const int helper(...)` is parsed as a function instead of a malformed variable declaration, while existing const enforcement metadata is preserved for scalar, aggregate, pointer-pointee, parameter, typedef, and array contexts. Coverage includes `tests/fixtures/valid/postfix_const_qualifiers.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/postfix_const_qualifiers.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-postfix-const-qualifiers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_postfix_const_qualifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous const pointer typedef alias run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, pointer/aggregate parity gaps, and remaining declaration-specifier/typedef const-correctness gaps. The selected work package closes a concrete pointer typedef const metadata gap: Cust now preserves separate pointee-const and pointer-slot-const metadata for one-level pointer typedef aliases such as `typedef const int *ConstIntView;`, `typedef const struct Point *ConstPointView;`, `typedef int * const ConstIntSlot;`, and `typedef struct Point * const PointSlot;`. Alias-spelled declarations, parameters, pointer-returning functions, and pointer fields now reuse the correct existing const enforcement paths: pointer-to-const aliases reject const-to-mutable conversions, while const pointer-slot aliases reject reassignment without making mutable pointees read-only. Coverage includes `tests/fixtures/valid/const_pointer_typedef_aliases.c`, invalid fixtures `tests/fixtures/invalid/const_pointer_typedef_alias_const_discard.c` and `tests/fixtures/invalid/const_pointer_typedef_alias_slot_assignment.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/const_pointer_typedef_aliases.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-const-pointer-typedef-aliases.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter const_pointer_typedef -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous const typedef alias run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, pointer/aggregate parity gaps, remaining declaration specifier syntax, and const-qualified typedef alias semantics. The selected work package closes a concrete typedef const-correctness gap: Cust now preserves leading `const` metadata on non-pointer typedef aliases such as `typedef const int ConstInt;`, `typedef const char ConstChar;`, `typedef const struct Point ConstPoint;`, and `typedef const int Scores[3];`. Alias-spelled declarations and parameters now reuse the existing const variable/parameter/array enforcement paths, so mutation through a const typedef alias reports the same targeted const diagnostics as direct `const` declarations. Coverage includes `tests/fixtures/valid/const_typedef_aliases.c`, invalid assignment fixture `tests/fixtures/invalid/const_typedef_alias_assignment.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/const_typedef_aliases.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-const-typedef-aliases.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter const_typedef -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous C function specifier run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, deliberately scoped standard-library-like builtins, pointer/aggregate parity gaps, and remaining declaration/function specifier syntax. The selected work package closes a safe function-declaration syntax gap: Cust now lexes `inline` and `_Noreturn`, consumes one or more function specifiers before and/or after top-level `static`/`extern`, and treats them as parser-level no-op metadata over existing function prototypes/definitions. Runtime behavior remains Cust's deterministic single-file function table/call model; `inline` does not add native inlining/linkage semantics, and `_Noreturn` does not alter return-shape checks or control flow. Coverage includes `tests/fixtures/valid/function_specifiers.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/function_specifiers.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-function-specifiers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_function_specifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous C99 `restrict` pointer qualifier run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, standard-library-like builtins, `inline`/`_Noreturn` declaration specifier metadata, and small remaining declaration syntax parity gaps. The selected work package closes a safe pointer qualifier syntax gap: Cust now lexes `restrict` and accepts it as parser-level no-op qualifier syntax in the shared qualifier paths used by supported pointer declarations, pointer parameters, function-signature lookahead, aggregate pointer fields, casts, and `sizeof` type parsing. Runtime pointer behavior remains Cust's deterministic interpreter-owned model; `restrict` intentionally does not add alias analysis or write restrictions, while existing `const` metadata is preserved in mixed forms such as `int * const restrict p`. Coverage includes `tests/fixtures/valid/restrict_pointer_qualifiers.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/restrict_pointer_qualifiers.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-restrict-pointer-qualifiers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_restrict_pointer_qualifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous `volatile` type qualifier run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, standard-library-like builtins, post-extern declaration cleanup, and small remaining C declaration syntax parity gaps. The selected work package closes a safe qualifier syntax gap: Cust now lexes `volatile` and accepts it as parser-level no-op syntax across supported declaration/type contexts, including globals, locals, static locals, `for` initializer declarations, function parameters/prototypes, pointer declarations and post-star qualifiers, aggregate fields, typedef aliases, casts, and `sizeof` type operands. Existing `const` enforcement is preserved when `const` appears with `volatile`; `volatile` alone intentionally adds no runtime write restrictions or native optimizer semantics. Coverage includes `tests/fixtures/valid/volatile_type_qualifiers.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/volatile_type_qualifiers.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-volatile-type-qualifiers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_volatile_type_qualifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous top-level `extern` global declaration run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, top-level `extern` object declaration parity, standard-library-like builtins, and post-array-typedef pointer/aggregate parity gaps. The selected work package closes the matching object-declaration side of the recent `extern` function syntax work: Cust now accepts uninitialized top-level `extern` global declarations for supported scalar, char, array, struct, struct-array, union, enum, and one-level pointer variables, treating those declarations as parser-only single-file linkage metadata so a later ordinary definition provides the runtime storage without duplicate-global diagnostics. Coverage includes `tests/fixtures/valid/extern_global_declarations.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/extern_global_declarations.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-extern-global-declarations.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_extern_global_declarations -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous top-level `extern` function storage-class run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, top-level `extern` syntax parity, standard-library-like builtins, and post-array-typedef pointer/aggregate parity gaps. The selected work package closes a small C function-declaration conformance gap: Cust now lexes and parses `extern` before top-level function prototypes and definitions, treating it as single-file linkage metadata like existing top-level `static` while preserving all existing prototype/definition compatibility checks. Coverage includes `tests/fixtures/valid/extern_function_storage_class.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/extern_function_storage_class.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-extern-function-storage-class.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_extern_function_storage_class_specifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous array typedef alias run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, top-level `extern` storage-class syntax, standard-library-like builtins, and array typedef aliases. The selected work package closes a concrete C declaration/type-alias gap: Cust now supports one-dimensional array typedef aliases such as `typedef int Scores[3];`, `typedef char Word[4];`, and aggregate aliases such as `typedef Point Points[2];`. Array aliases work for global/local/`const` variable declarations, scalar/char/aggregate array initializers including char-array string initializers, function parameters via the existing array-to-pointer decay path, and `sizeof(alias)` full array-object queries. Pointer-array, multidimensional-array, pointer-to-array, and array-return typedef forms remain outside the supported subset with targeted diagnostics. Coverage includes `tests/fixtures/valid/array_typedef_aliases.c`, invalid pointer-array typedef coverage, C compiler-oracle fixture `tests/fixtures/compat/valid/array_typedef_aliases.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-array-typedef-aliases.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_array_typedef_aliases -- --nocapture
cargo test --test interpreter array_typedef -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-11 autonomous `auto`/`register` local storage-class run. Ideation considered newly discovered malformed-program parser diagnostics, additional compound-literal edge fixtures, aggregate-kind diagnostic polish, array typedefs, top-level `extern`/storage-class work, and small C storage-class syntax parity. The selected work package closes a safe local declaration conformance gap: Cust now lexes and parses `auto` and `register` as block-scope/local declaration prefixes for ordinary automatic storage. The specifiers work for scalar, pointer, aggregate, and `for` initializer declarations by lowering to the existing local declaration paths; runtime behavior remains Cust's deterministic interpreter-owned local storage model and does not attempt native register allocation or address-taking restrictions. Coverage includes `tests/fixtures/valid/auto_register_storage_class.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/auto_register_storage_class.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-auto-register-storage-class.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_auto_and_register_local_storage_class_specifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous `_Bool` type-spelling run. Ideation considered newly discovered malformed-program parser diagnostics, additional C-compatible compound-literal edge fixtures, aggregate-kind-specific diagnostic polish, `_Bool` syntax parity after the recent scalar type-spelling work, and standard-library-like builtins. The selected work package closes another small scalar C syntax conformance gap from the roadmap: Cust now lexes and parses C99 `_Bool` as a deterministic scalar type with Cust-defined size 1. `_Bool` works across globals, locals, static locals, `for` declarations, function returns/prototypes/parameters, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` scalar/pointer/one-dimensional array type operands. This is syntax/type-spelling parity over Cust's deterministic scalar model, not a broader native C integer-conversion/range implementation. Coverage includes `tests/fixtures/valid/bool_type_spellings.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/bool_type_spellings.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-bool-type-spellings.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_bool_type_spellings -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous `long long` type-spelling run. Ideation considered newly discovered malformed-program parser diagnostics, additional C-compatible compound-literal edge fixtures, aggregate-kind-specific diagnostic polish, `long long` syntax parity, and standard-library-like builtins. The selected work package closes the next small scalar C syntax conformance gap from the roadmap: Cust now parses `long long`, `long long int`, `signed long long`, `signed long long int`, `unsigned long long`, and `unsigned long long int` as aliases for its existing deterministic integer storage. These spellings work across globals, locals, static locals, `for` declarations, function returns/prototypes/parameters, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` scalar/pointer/one-dimensional array type operands. Runtime storage remains Cust's `i64` integer model and `sizeof(long long)` reports Cust's deterministic integer size; this is syntax parity, not native width/range/wraparound semantics. Coverage includes `tests/fixtures/valid/long_long_type_spellings.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/long_long_type_spellings.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-long-long-type-spellings.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_long_long_type_spellings -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous long/short type-spelling run. Ideation considered newly discovered parser diagnostics, additional compound-literal parity fixtures, aggregate-kind-specific diagnostic polish, deterministic `long`/`short` type spellings, `long long` as a larger follow-up, and standard-library-like builtins. The selected work package closes a small scalar C syntax conformance gap: Cust now lexes and parses `long`, `long int`, `short`, `short int`, and signed/unsigned long/short forms such as `signed long int` and `unsigned short int` as aliases for its existing deterministic integer storage. These spellings work across globals, locals, static locals, `for` declarations, function returns/prototypes/parameters, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` scalar/pointer/one-dimensional array type operands. Runtime storage remains Cust's `i64` integer model and `sizeof(long)` / `sizeof(short)` report Cust's deterministic integer size; this is syntax parity, not native width/range/wraparound semantics. Coverage includes `tests/fixtures/valid/long_short_type_spellings.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/long_short_type_spellings.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-long-short-type-spellings.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_long_short_type_spellings -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous signed/unsigned char type-spelling run. Ideation considered newly discovered parser diagnostics, additional compound-literal parity fixtures, aggregate-kind-specific diagnostic polish, signed/unsigned char follow-up semantics, deterministic `long`/`short` type spellings, and standard-library-like builtins. The selected work package closes the safest concrete scalar conformance gap from the previous run: Cust now parses `signed char` and `unsigned char` as aliases for its existing deterministic `char` storage wherever ordinary `char` already participates in supported type syntax, including globals, locals, static locals, `for` declarations, function returns/prototypes/parameters, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` scalar/pointer/one-dimensional array type operands. Runtime scalar storage and `sizeof(char) == 1` remain unchanged; this is syntax parity, not native signedness/range/wraparound semantics. Coverage includes `tests/fixtures/valid/signed_unsigned_char_types.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/signed_unsigned_char_types.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, required Docker verification, and reference notes in `references/cust-signed-unsigned-char-types.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_signed_unsigned_char_type_spellings -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous signed/unsigned integer type-spelling run. This run closes a small C declaration/type-operand conformance gap: Cust now lexes and parses `signed`, `signed int`, `unsigned`, and `unsigned int` as parser-level spellings for the existing deterministic integer storage model. These spellings work across globals, locals, static locals, `for` initializer declarations, function returns, parameters/prototypes, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` type operands including one-dimensional array type names such as `sizeof(const unsigned int[2])`. Runtime scalar storage and Cust-defined integer size remain unchanged (`i64`, `sizeof(int) == 8`); this is syntax/conformance parity rather than native unsigned wraparound semantics. Coverage includes `tests/fixtures/valid/signed_unsigned_int_types.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/signed_unsigned_int_types.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, full local verification, and required Docker verification. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_signed_unsigned_int_type_spellings -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous `sizeof` array type-name run. This run closes a small C `sizeof` type-operand parity gap: one-dimensional scalar and aggregate array type names now parse in `sizeof(...)`, e.g. `sizeof(int[3])`, `sizeof(char[4])`, `sizeof(const int[2])`, `sizeof(struct Pair[2])`, and typedef-spelled aggregate arrays such as `sizeof(Number[3])`. Cust computes these sizes with its deterministic interpreter model by multiplying the element type size by the parsed positive length, without creating or evaluating runtime storage. Pointer-array and multidimensional array type operands remain outside the supported subset with targeted diagnostics. Coverage includes `tests/fixtures/valid/sizeof_array_types.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/sizeof_array_types.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, and a reference note in `references/cust-sizeof-array-type-names.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_sizeof_array_type_names -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous struct-array element aggregate-field address run. This run closes the aggregate counterpart to direct `&segments[i].field` addressability: aggregate-valued fields selected from ordinary struct-array elements can now be addressed directly, e.g. `struct Point *start = &segments[1].start;`, and the resulting struct pointer aliases the original array element field for `->` reads/writes and helper mutation. The runtime now lets `Interpreter::find_struct_element_field_pointer` return `PointerValue::StructField` for both scalar and aggregate-valued fields while preserving `element_index: Some(i)` metadata. The run also locks in direct embedded aggregate-array element aggregate-field syntax such as `&drawing.segments[1].start` and nested `&box.drawing.segments[0].end`, which was already supported through parser lowering. Coverage includes `tests/fixtures/valid/struct_array_element_aggregate_field_addresses.c`, `tests/fixtures/valid/struct_field_array_element_aggregate_field_addresses.c`, matching C compiler-oracle fixtures, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, and a reference note in `references/cust-struct-array-element-aggregate-field-addresses.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_addresses_of_struct_array_element_aggregate_fields -- --nocapture
cargo test --test interpreter supports_direct_addresses_of_embedded_aggregate_array_element_aggregate_fields -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous direct embedded aggregate-array element field address run. This run closes the direct syntax counterpart to the recent pointer-through-embedded-array field address work: scalar fields selected after indexing embedded aggregate-array fields can now be addressed directly, e.g. `&line.points[1].x` and nested `&box.line.points[2].y`. The parser rewrites address-of over `StructFieldArrayElementGet` through the existing `AddressOfStructArrayField` plus `AddressOfStructPtrField` path, preserving the established `StructFieldElement` owner/path/index metadata and avoiding a new pointer target. Coverage includes `tests/fixtures/valid/struct_field_array_element_field_addresses.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/struct_field_array_element_field_addresses.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, and a reference note in `references/cust-direct-embedded-aggregate-array-element-field-addresses.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_direct_addresses_of_embedded_aggregate_array_element_fields -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous embedded aggregate-array element field pointer-equality run. This run closes a pointer identity parity gap left after `&p->field` support for pointers into embedded aggregate-array fields: independently computed scalar field pointers targeting the same embedded element field now compare equal, while sibling fields and different embedded elements compare unequal. The interpreter now includes `PointerValue::StructFieldElementField` in `Interpreter::pointer_eq`, comparing the full owner/path/index/field identity metadata. Coverage includes `tests/fixtures/valid/struct_field_element_field_pointer_equality.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/struct_field_element_field_pointer_equality.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, and a reference note in `references/cust-embedded-aggregate-array-element-field-pointer-equality.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter embedded_aggregate_array_pointers -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous integer literal suffix run. This run closes a small C lexical conformance gap: decimal, octal, and hexadecimal integer constants now accept standard C suffix spellings (`u`/`U`, `l`/`L`, `ll`/`LL`, and unsigned-long combinations such as `UL`, `lu`, `uL`, and `LLU`) while preserving Cust's deterministic `i64` integer storage model. Coverage includes `tests/fixtures/valid/integer_literal_suffixes.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/integer_literal_suffixes.c`, focused RED/GREEN interpreter coverage, full C compiler-oracle verification, and a reference note in `references/cust-integer-literal-suffixes.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_c_integer_literal_suffixes -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous multidimensional array diagnostic run. This run expands the P0 parser recovery/error-message track for a newly discovered unsupported C declarator family: multidimensional scalar/aggregate array declarations, function parameter spellings, and aggregate fields now fail at the second `[` with targeted diagnostics instead of generic semicolon/list-boundary errors. Coverage includes invalid fixtures `tests/fixtures/invalid/multidimensional_array_declaration.c`, `tests/fixtures/invalid/multidimensional_array_parameter.c`, and `tests/fixtures/invalid/multidimensional_array_field.c`, plus focused RED/GREEN exact-diagnostic interpreter tests and a reference note in `references/cust-multidimensional-array-diagnostics.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter multidimensional_array -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous embedded aggregate-array element aggregate-field address run. This run closes the aggregate counterpart to the previous scalar-field pointer gap: aggregate-valued fields reached through pointers into embedded aggregate-array fields can now be addressed directly, e.g. `struct Segment *second = drawing.segments + 1; struct Point *start = &second->start;`, and the resulting struct pointer supports `->` reads/writes that alias the original containing struct storage. The interpreter now routes aggregate-valued `PointerValue::StructFieldElementField` targets through both immutable and mutable struct-pointer field resolution, with helper traversal returning owned type-name metadata to avoid borrowing temporary element type strings. Coverage includes `tests/fixtures/valid/struct_field_element_aggregate_field_addresses.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/struct_field_element_aggregate_field_addresses.c`, focused RED/GREEN interpreter coverage, C compiler-oracle verification, and a reference note in `references/cust-embedded-aggregate-array-element-aggregate-field-addresses.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_addresses_of_aggregate_fields_through_embedded_aggregate_array_pointers -- --nocapture
cargo test --test interpreter supports_addresses_of_fields_through_embedded_aggregate_array_pointers -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous embedded aggregate-array element scalar-field address run. This run closes the next pointer/aggregate parity gap: scalar fields reached through pointers into embedded aggregate-array fields can now be addressed directly, e.g. `struct Point *p = line.points + 1; int *x = &p->x;`, with the resulting pointer aliasing the original containing struct storage through helper calls and dereference writes. The interpreter adds a dedicated `PointerValue::StructFieldElementField` target to preserve the embedded aggregate-array owner/path/index metadata while exposing scalar-field pointer semantics. Coverage includes `tests/fixtures/valid/struct_field_element_field_addresses.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/struct_field_element_field_addresses.c`, focused RED/GREEN interpreter coverage, and a reference note in `references/cust-embedded-aggregate-array-element-field-addresses.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_addresses_of_fields_through_embedded_aggregate_array_pointers -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous struct-pointer scalar field address run. This run closes a C pointer/aggregate parity gap: scalar fields reached through struct pointers can now be addressed directly (`&point_ptr->x`, `&box_ptr->inner.y`), and the resulting pointer aliases the original struct storage for helper calls and dereference writes. The interpreter now lowers `&` over `StructPtrGet` into `AddressOfStructPtrField`, resolves the underlying struct pointer, and returns existing safe `PointerValue::StructField` targets for ordinary struct pointers, struct-array element pointers, and nested aggregate-field pointers. Coverage includes `tests/fixtures/valid/struct_pointer_field_addresses.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/struct_pointer_field_addresses.c`, focused RED/GREEN interpreter coverage, and a reference note in `references/cust-struct-pointer-field-addresses.md`.

Commands verified:

```bash
cargo test --test interpreter supports_addresses_of_struct_pointer_scalar_fields -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous aggregate compound-literal aggregate-field address run. This run closes the aggregate counterpart to the prior scalar-field addressability feature: aggregate-valued fields selected from aggregate compound literals can now be addressed directly (`&((struct Box){{5, 7}, 9}).inner`) and used as safe struct pointers through `->`, including mutation through helper functions. The interpreter now allows `AddressOfAggregateField` to return a `PointerValue::StructField` for `StructFieldValue::Struct` selections and teaches struct-pointer field resolution to treat those field pointers as aggregate targets in both read and write paths. Coverage includes `tests/fixtures/valid/aggregate_compound_literal_aggregate_field_addresses.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_compound_literal_aggregate_field_addresses.c`, focused RED/GREEN interpreter coverage, and a reference note in `references/cust-aggregate-compound-literal-aggregate-field-addresses.md`.

Commands verified:

```bash
cargo test --test interpreter supports_addresses_of_aggregate_compound_literal_aggregate_fields -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous aggregate compound-literal scalar-field address run. This run closes another compound-literal lvalue/addressability parity gap: scalar fields selected from aggregate compound literals can now be addressed directly (`&((struct Point){4, 8}).x`), including nested scalar fields (`&((struct Box){{2, 3}, 4}).inner.y`) and union scalar fields (`&((union Number){7}).value`). The parser now lowers address-of over aggregate field selections to a dedicated `AddressOfAggregateField` expression; the interpreter creates hidden current-scope compound-literal aggregate storage and returns a safe `PointerValue::StructField` to the selected scalar field, preserving pointer type and const metadata. Coverage includes `tests/fixtures/valid/aggregate_compound_literal_field_addresses.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_compound_literal_field_addresses.c`, focused RED/GREEN interpreter coverage, and a reference note in `references/cust-aggregate-compound-literal-field-addresses.md`.

Commands verified:

```bash
cargo test --test interpreter supports_addresses_of_aggregate_compound_literal_scalar_fields -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous parenthesized pointer declarator diagnostic run. This run expands the remaining parser recovery/error-message track for unsupported C declarator forms that previously fell through to generic missing-name diagnostics: function/parameter declarators such as `int sum(int (*row)[3])` now report `parenthesized pointer parameters are not supported`, and local declarations such as `int (*row)[3];` report `parenthesized pointer declarations are not supported` at the opening parenthesis. Coverage includes invalid fixtures `tests/fixtures/invalid/parenthesized_pointer_parameter.c` and `tests/fixtures/invalid/parenthesized_pointer_declaration.c` plus focused RED/GREEN exact-diagnostic interpreter tests. Docker Compose emitted a non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter parenthesized_pointer -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous aggregate compound-literal pointer-field lvalue run. This run closes a compound-literal/pointer lvalue parity gap: pointer-valued fields selected from aggregate compound literals now behave as pointer lvalues in pointer contexts, including assignment results (`(((struct Cursor){values}).p = values + 2)[-1]`), pointer compound assignment results (`((struct Cursor){values + 1}).p += 2`), and prefix/postfix increment/decrement results (`--((struct Cursor){values + 3}).p`, `((struct Cursor){values + 1}).p++`). Pointer field assignment preserves pointer-slot const diagnostics and pointee type/const conversion checks. Coverage includes `tests/fixtures/valid/aggregate_compound_literal_pointer_field_lvalues.c`, invalid fixture `tests/fixtures/invalid/aggregate_compound_literal_const_pointer_field_assignment.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_compound_literal_pointer_field_lvalues.c`, focused RED/GREEN interpreter tests, and a reference note in `references/cust-aggregate-compound-literal-pointer-field-lvalues.md`. Docker Compose emitted a non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_pointer_field_lvalues_on_aggregate_compound_literals -- --nocapture
cargo test --test interpreter aggregate_compound_literal -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous embedded aggregate-array field pointer-equality run. This run closes the equality counterpart to the prior embedded aggregate-array field pointer-ordering work: pointers such as `line.points`, `&line.points[0]`, `line.points + 2`, `&line.points[2]`, and nested `box.line.points` forms now compare equal/unequal by matching `StructFieldElement` owner scope/name, optional containing element, field path, and element index in `pointer_eq`. Coverage includes `tests/fixtures/valid/struct_field_pointer_equality.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/struct_field_pointer_equality.c`, focused RED/GREEN interpreter tests, and a reference note in `references/cust-embedded-aggregate-array-field-pointer-equality.md`. Docker Compose emitted a non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_pointer_equality_within_embedded_aggregate_array_fields -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous embedded aggregate-array field pointer-ordering run. This run closes the pointer-ordering parity gap for pointers into embedded aggregate-array fields: `line.points < &line.points[2]`, `line.points + 1 <= &line.points[2]`, and nested field paths such as `box.line.points` now compare through the same-array pointer-ordering path by teaching `pointer_difference` to recognize matching `StructFieldElement` owner/path/index metadata. Comparisons between different embedded aggregate-array fields report `cannot compare pointers to different arrays`. Coverage includes `tests/fixtures/valid/struct_field_pointer_ordering.c`, invalid fixture `tests/fixtures/invalid/struct_field_pointer_ordering_different_fields.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/struct_field_pointer_ordering.c`, and focused RED/GREEN interpreter tests. Docker Compose emitted a non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_pointer_ordering_within_embedded_aggregate_array_fields -- --nocapture
cargo test --test interpreter rejects_pointer_ordering_between_different_embedded_aggregate_array_fields -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous pointer-ordering run. This run closes a C pointer/conformance gap: Cust now supports relational pointer comparisons (`<`, `<=`, `>`, `>=`) for pointers into the same supported array storage, including scalar arrays, string literal storage, and struct/union array element pointers, by reusing interpreter-owned pointer-difference metadata. Comparisons between different arrays report `cannot compare pointers to different arrays`; scalar/null pointer ordering remains deliberately unsupported with the existing `pointer ordering comparisons are not supported` diagnostic. Coverage includes `tests/fixtures/valid/pointer_ordering.c`, invalid fixture `tests/fixtures/invalid/pointer_ordering_different_arrays.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/pointer_ordering.c`, and focused regression coverage for the pre-existing scalar-pointer ordering diagnostic. The run also added warning-free static-local union coverage (`tests/fixtures/valid/static_local_unions.c` plus compiler-oracle fixture) after discovering the behavior was already implemented but uncovered. Docker Compose emitted a non-fatal `Docker Compose requires buildx plugin to be installed` warning and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_pointer_ordering_within_same_array_storage -- --nocapture
cargo test --test interpreter rejects_pointer_ordering_between_different_arrays -- --nocapture
cargo test --test interpreter rejects_pointer_ordering_comparisons -- --nocapture
cargo test --test interpreter supports_static_local_unions -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-10 autonomous adjacent string-literal concatenation run. This run closed a C lexical/expression conformance gap: Cust now concatenates adjacent string literal tokens by removing the intermediate NUL terminator and preserving a single final NUL byte. Concatenation works for ordinary string literal pointer expressions, direct string indexing, `sizeof("..." "...")`, char-array string initializers, char-array compound literal string initializers, and pointer/array-parameter call paths. Coverage includes `tests/fixtures/valid/string_literal_concatenation.c`, C compiler-oracle fixture `tests/fixtures/compat/valid/string_literal_concatenation.c`, and the compat fixture list. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_adjacent_string_literal_concatenation -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous string-literal element address run. This run closed a concrete pointer/string parity gap: Cust now accepts address-of on direct string-literal indexed lvalues such as `&"cast"[2]`, lowering it to the same safe string array-base pointer plus offset used by grouped and reverse-subscript forms. The resulting pointer remains backed by read-only string storage, so writes through `char *middle = &"cat"[1]; middle[0] = 'u';` report `cannot modify read-only array through pointer`. Coverage includes direct `&"..."[i]`, reverse `&i["..."]`, grouped `&("...")[i]`, negative relative indexing from the produced pointer, an invalid read-only write fixture, and C compiler-oracle coverage. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_address_of_string_literal_elements -- --nocapture
cargo test --test interpreter rejects_writes_through_string_literal_element_addresses -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous reverse-subscript conformance run. This run closed a small but concrete C expression parity gap: Cust now accepts C's commutative subscript spelling where the integer offset appears before the pointer/array expression (`i[p]`), by lowering otherwise-unhandled postfix subscript targets through the existing `*(lhs + rhs)` pointer-arithmetic/dereference path. Coverage includes scalar array and pointer reads/writes (`0[p]`, `2[values] = 9`), string literal and `char *` indexing (`1["hi"]`, `2[text]`), scalar-array compound literals (`1[(int[]){...}]`), and aggregate array pointer field access (`1[points].y`) with C compiler-oracle coverage. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_c_style_reverse_subscript_expressions -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous const pointer-returning call conversion run. This run closed a pointer const-correctness gap: pointer-returning function calls with `const T *` return types now carry their declared pointee-const metadata through pointer conversion checks, so binding a `const int *` returning call directly to `int *` reports `cannot discard const qualifier from pointer target` instead of silently allowing a mutable view. Coverage includes invalid fixture `tests/fixtures/invalid/pointer_return_call_const_discard.c`, focused RED/GREEN interpreter tests, the full local verification gate, and required Docker verification. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter rejects_const_pointer_returning_call_to_mutable_pointer -- --nocapture
cargo test --test interpreter rejects_pointer_return_const_discard -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous `sizeof` aggregate-expression array-field run. This run closed a non-evaluating-context aggregate parity gap: `sizeof` now reports full array-object sizes for scalar and embedded aggregate-array fields selected from aggregate-valued expression results, including aggregate-returning calls, aggregate assignment results, aggregate-valued conditionals, and nested aggregate field paths such as `sizeof(make_box(40).line.points)`, without evaluating calls, assignments, or unselected branches inside the operand. Coverage includes `tests/fixtures/valid/sizeof_aggregate_expression_array_fields.c`, native compiler-oracle fixture `tests/fixtures/compat/valid/sizeof_aggregate_expression_array_fields.c`, focused interpreter and C compiler-oracle tests, the full local verification gate, and required Docker verification. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_sizeof_array_fields_on_aggregate_valued_expressions -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous `sizeof` aggregate-expression field run. This run closed a concrete aggregate/non-evaluating-context parity gap: `sizeof` can now query fields selected from aggregate-valued expressions such as aggregate assignment results, aggregate-valued conditionals/comma expressions, aggregate-returning calls, and matching union-valued expression results without evaluating assignments, comma-left operands, or function calls inside the `sizeof` operand. Coverage includes `tests/fixtures/valid/sizeof_aggregate_expression_fields.c`, native compiler-oracle fixture `tests/fixtures/compat/valid/sizeof_aggregate_expression_fields.c`, focused interpreter and C compiler-oracle tests, the full local verification gate, and required Docker verification. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_sizeof_fields_on_aggregate_valued_expressions -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous scalar-context diagnostic run. This run closed a concrete diagnostics polish gap from the open parser/error-message track: pointer-returning calls used where a scalar is required now report the function-specific `pointer function '<name>' used as scalar expression`, and union compound literal values used as scalars now report `union value used as scalar` instead of the struct-specific fallback. Coverage includes invalid fixtures `tests/fixtures/invalid/pointer_function_used_as_scalar.c` and `tests/fixtures/invalid/union_value_used_as_scalar.c`, focused interpreter tests, the full local verification gate, and required Docker verification. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter rejects_pointer_function_used_as_scalar_expression -- --nocapture
cargo test --test interpreter rejects_union_values_used_as_scalar_expressions -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous union-valued expression field-access run. This run closed the next aggregate expression parity gap by adding direct coverage for `.` field access after union-valued expression results: union compound literals, union assignment expressions, union-valued conditional/comma expressions, union-returning calls, and aggregate pointer-dereference assignment results such as `((*(&left) = make_number(6))).value`. It also fixed a union diagnostic wording gap so naked union-returning calls used as scalar expressions now report `union function 'make_number' used as scalar expression` instead of the previous struct-specific wording. Coverage includes `tests/fixtures/valid/union_expr_field_access.c`, invalid diagnostic fixture `tests/fixtures/invalid/union_function_used_as_scalar.c`, native compiler-oracle fixture `tests/fixtures/compat/valid/union_expr_field_access.c`, focused interpreter tests, the C compiler-oracle suite, and the full local/Docker verification gate. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_field_access_on_union_valued_expressions -- --nocapture
cargo test --test interpreter rejects_union_function_used_as_scalar_expression -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous aggregate-valued expression field-access run. This run closed the next compound-literal/aggregate parity gap: scalar array fields on aggregate compound literals now have explicit assignment, compound-assignment, prefix, and postfix increment fixture coverage; embedded aggregate-array fields on aggregate compound literals have scalar-field write coverage through indexed elements; and the parser now allows `.` field access after grouped aggregate-valued expression results that `eval_struct_expr` already understands, including aggregate assignment expressions, aggregate pointer dereference assignment results, aggregate-valued conditional/comma expressions, and aggregate-returning calls (`(((struct Line){{...}}).points[0] = replacement).x`, `(left = right).x`, `(cond ? left : replacement).y`, `(marker = marker + 1, right).x`, and `make_point(5).y`). Coverage includes `tests/fixtures/valid/aggregate_compound_literal_array_field_lvalues.c`, `tests/fixtures/valid/aggregate_compound_literal_aggregate_array_field_lvalues.c`, `tests/fixtures/valid/aggregate_expr_field_access.c`, native compiler-oracle fixtures for the C-compatible cases, focused interpreter tests, the C compiler-oracle suite, and the full local/Docker verification gate. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_field_access_on_aggregate_valued_expressions -- --nocapture
cargo test --test interpreter supports_lvalue_writes_to_array_fields_on_aggregate_compound_literals -- --nocapture
cargo test --test interpreter supports_lvalue_writes_to_aggregate_array_fields_on_aggregate_compound_literals -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Previous latest: All passed after the 2026-05-09 autonomous aggregate compound-literal array-field indexing/address-of run. This run closed the next aggregate compound-literal parity gap: scalar array fields on aggregate compound literals can now be indexed directly (`((struct Packet){{1, 2, 3}}).values[1]`), their elements can be addressed and passed to pointer parameters (`&((struct Packet){{4, 5, 6}}).values[2]`), embedded aggregate-array fields can be indexed and followed by field access (`((struct Line){{{1, 2}, {3, 4}}}).points[1].y`), and aggregate-array field elements can be addressed (`&((struct Line){{{5, 6}}}).points[0]`). The parser lowers postfix indexing of aggregate compound-literal array fields through existing pointer-decay/arithmetic/dereference machinery, and pointer arithmetic now classifies literal integer offsets before pointer probing so `pointer + 0` does not look like adding a null pointer. Coverage includes `tests/fixtures/valid/aggregate_compound_literal_array_field_indexing.c`, native compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_compound_literal_array_field_indexing.c`, invalid const-discard fixture `tests/fixtures/invalid/aggregate_compound_literal_array_field_element_const_discard.c`, focused interpreter and compiler-oracle tests, recursion-depth regression, and the full local/Docker verification gate. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

Commands verified:

```bash
cargo test --test interpreter supports_direct_indexing_and_address_of_array_fields_on_aggregate_compound_literals -- --nocapture
cargo test --test interpreter rejects_const_discard_from_array_field_elements_on_aggregate_compound_literals -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

Note: an attempted focused command with a substring that did not match any test (`aggregate_compound_literal_array_field`) ran 0 tests; the two exact focused interpreter test names above were run afterward and passed.

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
- Top-level/local supported aggregate declarations can use brace initializers (including designated struct/union-array elements such as `struct Point points[3] = {[2] = {.x = 5}, [0] = {1, 2}};`), same-type aggregate-returning call expressions such as `struct Point p = make_point(...);` and `union Number n = make_number(5);`, aggregate-valued conditional/comma expressions such as `struct Point p = flag ? high : low;` and `union Number n = (side_effect(), right);`, aggregate assignment expressions such as `struct Point copy = (target = source);` and `union Number picked = (*slot = replacement);`, aggregate-array pointer indexed values such as `struct Point copy = p[i];` / `union Number picked = n[i];`, or aggregate pointer dereference values such as `struct Point copy = *p;` / `union Number picked = *n;`, preserving branch short-circuiting, by-value copies, and const binding enforcement after initialization.
- Typedef enum definitions such as `typedef enum { READY = 1, BUSY } Status;` create parser-only integer aliases while exposing scoped enum constants at runtime, matching the existing named-tag enum typedef model without adding distinct enum runtime storage.
- Top-level `int`/`char` scalar globals, one-dimensional array globals, supported pointer globals (including one-level pointers to supported structs after the struct type is declared), leading `const int` / `const char` scalar and array globals/locals/parameters for a first-pass read-only qualifier milestone, top-level/local enum constant declarations with optional tags, explicit integer values, implicit incrementing values, trailing commas, and block-scope shadowing, direct named-enum type spellings such as `enum State global = READY;`, `const enum State local = BUSY;`, `enum State choose(enum State left, enum State right);`, and `sizeof(enum State)` mapped onto Cust's integer storage model, top-level and block-scoped `typedef` aliases for `int`, `char`, prior named `struct` types, prior named enum tags as integer aliases, anonymous enum typedef definitions as integer aliases, top-level/block-scoped aggregate typedef definitions (including block-local tag shadowing via unique internal type identities) such as `typedef struct Point { int x; int y; } Point;`, `typedef union Number { int value; char tag; } Number;`, and anonymous alias-only forms such as `typedef struct { int x; int y; } Point;` / `typedef union { int value; char tag; } Number;`, and one-level scalar/struct pointer aliases (usable in globals/locals/arrays/pointers/function signatures/`sizeof` without changing runtime storage, with inner-block shadowing and scope expiry), plus preprocessor-free `struct` type declarations (`struct Point { int x; char y; };`, `struct Packet { int values[3]; char tag[2]; };`, `struct Rect { struct Point origin; int width; };`, `struct Node { int value; struct Node *next; int *external; };`) and top-level/local zero-initialized or brace-initialized struct variables and one-dimensional arrays of supported structs (`struct Point points[2] = {{1, 2}, {3, 4}};`) with scalar/array/nested/pointer member reads/writes, same-type copy assignment including array and nested struct field copies plus C-style pointer-value field copies, scalar and array-element field lvalue expressions, struct-array element field lvalues such as `points[i].x += 1`, by-value struct function parameters including array/nested struct-field and struct-array-element arguments, by-value struct function return types, and struct pointer declarations/parameters, plus `int main() { ... }` / `int main(void) { ... }` and additional `int name(...) { ... }` / `char name(...) { ... }` / `void name(...) { ... }` / `struct Name name(...) { ... }` function definitions
- function calls as expressions with scalar, pointer-returning, pointer, and supported struct/union arguments; local parameter scopes including by-value `struct Name param`, by-reference `struct Name *param`, and C-style array parameter spellings such as `int values[3]`, `char text[4]`, `int values[]`, `char text[]`, `struct Point points[]`, and `union Number numbers[]` treated as pointer parameters after a prior aggregate declaration; C-style unnamed parameter declarations in function prototypes such as `int add(int, int);`, `char first(char *);`, `void mutate(int [], struct Point *);`, and `union Number pick(union Number [], int);`; C-style empty `void` parameter lists in definitions and prototypes such as `int main(void)` / `int helper(void);`; direct/mutual recursion support, top-level function prototypes and definitions such as `int helper(int value);` / `char first(char *text);` / `int *choose(int *left, int *right);` / `const int *view(const int *value);` / `struct Point *pick(struct Point *points, int index);` / `union Number *pick(union Number *numbers, int index);` / `void mutate(int *slot);` / `int sum(struct Point p);` / `void set(struct Point *p);` / `struct Point make_point(int x);` with signature compatibility checks, arity diagnostics, undefined-function diagnostics, empty `return;` support for `void` functions, diagnostics for value returns from `void` functions / empty returns from scalar-, pointer-, or struct-returning functions / scalar use of `void`, pointer, or struct calls, targeted mismatched/non-struct argument diagnostics for struct parameters, targeted mismatched struct-return diagnostics, pointer return conversion diagnostics for pointee type or const-discard mismatches, `sizeof` on function calls respecting `int` vs `char` vs pointer return sizes and deterministic Cust struct sizes, and a 32-call-depth safety limit with function-name context
- integer literals (decimal, C-style octal such as `052`, and hexadecimal such as `0x2a` / `0X10`, with standard C integer suffix spellings such as `42u`, `10L`, `7ll`, and `0xffUL` accepted as syntax over Cust's deterministic `i64` storage), character literals with standard simple escapes (`\\a`, `\\b`, `\\f`, `\\n`, `\\r`, `\\t`, `\\v`, `\\0`, `\\\\`, `\\'`, and `\\?`) plus C numeric escape sequences such as `\\101` and `\\x2a`, string literals with the corresponding NUL-terminated byte values plus escaped double quotes and numeric escapes, variables, and one-dimensional `int`/`char` arrays
- Fixed-size `char` arrays can be initialized directly from string literals: `char word[4] = "cat";` includes the NUL terminator when it fits, `char exact[3] = "dog";` accepts C's exact-size non-NUL-terminated form, omitted elements remain zero-filled, static/const char arrays preserve their existing storage semantics, and too-long initializer strings report `initializer string for char array '<name>' is too long`. Struct/union array fields now share the same string-literal initializer path for positional initializers, field designators, nested field path designators, and aggregate-array elements such as `struct Label labels[2] = {{"one", 1}, [1] = {.text = "two"}}`. Native compiler-oracle coverage intentionally avoids exact-size truncation because the repository's `-Werror` flags reject it as `-Wunterminated-string-initialization`.
- Braced scalar initializer expressions are supported in scalar initializer contexts: `int x = {expr};`, scalar array entries/designators such as `int values[3] = {{1}, {2}, [2] = {3}};`, scalar struct/union fields such as `struct Pair p = {{1}, {2}};`, and scalar path designators such as `.field = {expr}` evaluate the contained expression once, preserve side effects, and accept trailing commas. Native `-Werror` compiler-oracle coverage is intentionally skipped for this fixture because GCC/cc warn on braces around scalar initializers.
- declarations: top-level or local initialized `int x = expr;` / `char x = expr;`, default-initialized scalar `int x;` / `char c;`, first-pass const-qualified scalar declarations such as `const int limit = 5;` / `const char marker = 'A';`, supported pointer declarations such as `int *p = &x;`, `int *p;` (defaulting to null), or pointer-typedef declarations such as `IntPtr p = &x;` after `typedef int *IntPtr;`, persistent local `static` declarations for supported scalar/pointer/array/struct forms such as `static int counter = 0;`, `static int values[3] = {1, 2};`, and `static struct Point point = {3, 4};`, zero-initialized or brace-initialized arrays `int xs[N];` / `char cs[N];` / `int xs[N] = {expr, ...};` including designated entries such as `int xs[4] = {[2] = 5, [0] = 1};`, read-only const arrays such as `const int table[N] = {8, 9};`, one-dimensional supported struct/union arrays such as `struct Point points[3] = {[2] = {.y = 6, .x = 5}, [0] = {1, 2}};` / `union Number numbers[3] = {[1] = {.tag = 7}, [2] = {4}};` with omitted trailing or undesignated elements zero/default-filled, enum constant declarations such as `enum State { READY = 1, RUNNING };`, scalar, array-field, nested struct, pointer-field, and designated struct brace initializers and path designators such as `struct Point p = {1, 2};` / `struct Point p = {.y = 2, .x = 1};` / `struct Packet packet = {.values = {[1] = 2}, .anchor = {.y = 4, .x = 3}};` / `struct Packet packet = {.anchor.x = 3, .values[1] = 2};` / `struct Node head = {3, &tail, 0};` / `const struct Config c = {7, 8};` with declaration-order or field-designated initialization, zero-filled omitted trailing fields/elements/pointers defaulting to null, and typedef aliases such as `typedef int Count;`, `typedef char Byte;`, `typedef struct Point Point;`, `typedef enum State State;`, anonymous enum aliases such as `typedef enum { READY = 1 } Status;`, and one-level pointer aliases such as `typedef int *IntPtr;` / `typedef struct Point *PointPtr;` at top level or in block scopes, with inner aliases shadowing outer aliases until block exit; globals initialize before `main()` and remain visible/mutable from helper functions
- `int` and `char` function parameters plus `char` and top-level const-qualified scalar/aggregate function return types such as `const int f(void)`, `const Count f(void)`, `const struct Point make(void)`, and `const Number make(void)` (stored/returned by value in the current interpreter model)
- fixed-size scalar array parameter spellings such as `int values[3]` and `char text[4]` are parsed as pointer parameters just like unsized `int values[]`/`char text[]`; the bracket length is syntax-checked but does not enforce argument length, `sizeof(values)` reports Cust pointer size, and array/string arguments decay to interpreter-owned pointers with existing const/read-only diagnostics
- C99-style aggregate compound literals are supported for existing struct/union types and aggregate typedef aliases in rvalue expression contexts: `(struct Point){1, 2}`, `(Point){.x = 1, .y = 2}`, and `(union Number){.tag = 9}` reuse Cust's aggregate initializer semantics, can initialize/assign/return/pass by-value aggregates, participate in aggregate conditional/comma expressions, allow scalar field reads such as `((Number){12}).tag`, and support scalar-field lvalue operations such as `((struct Point){1, 2}).x = 7`, `((struct Point){3, 4}).y += 5`, `++((struct Point){5, 6}).x`, and `((struct Point){7, 8}).y++` while rejecting writes to const fields. C99-style scalar compound literals are also supported for scalar types and scalar typedef aliases: `(int){expr}`, `(char){expr}`, and `(Count){expr}` evaluate as scalar expressions with optional trailing commas, preserve initializer side effects once, remain non-evaluating under `sizeof`, and now act as modifiable scalar lvalues for direct assignment/compound assignment and prefix/postfix increment/decrement such as `((int){1} = 5)`, `((int){3} += 4)`, `++(int){8}`, and `(int){9}++`. Addressable scalar and aggregate compound literals such as `&(int){7}` and `&(struct Point){.x = 1}` create hidden mutable current-scope storage and return interpreter-owned pointers that participate in existing dereference, pointer-parameter, type-checking, and `->` field-access paths. C99-style scalar-array compound literals are supported as pointer-valued rvalue expressions: `(int[]){1, 2}`, `(char[]){'a', 0}`, `(char[]){"cat"}`, and `(int[3]){...}` allocate mutable interpreter-owned array storage, support positional/designated initializers plus char-array string-literal initializers with inferred unsized lengths or checked fixed lengths, and can initialize pointer variables or flow directly into pointer/array parameters. C99-style aggregate-array compound literals are supported as pointer-valued rvalue expressions: `(struct Point[]){{1, 2}, {.x = 3}}`, `(union Number[]){{7}, [2] = {.value = 9}}`, and `(struct Point[3]){...}` allocate mutable interpreter-owned aggregate-array storage in the current scope, support positional/designated aggregate element initializers with inferred or checked lengths, and can initialize aggregate pointer variables or flow directly into aggregate pointer/array parameters.
- First-pass scalar pointer support from `docs/plans/pointer-model.md`: `int *p = &x;`, `char *p = &c;`, `p = &y;`, `p = 0;`, `*p` reads, and `*p = expr;` writes through interpreter-owned scalar references. Address-of dereference expressions such as `&*p`, `&*(values + 1)`, and `&*null_ptr` now preserve the underlying pointer value without dereferencing the pointee, matching C's pointer-identity idiom while staying inside Cust's safe pointer metadata model. Null dereferences report `null pointer dereference`; pointers to scalar variables whose block/function scope has ended report `pointer to out-of-scope variable '<name>'`.
- Pointer parameters are supported for scalar addresses (`inc(&x)`), struct addresses (`set(&point)` for `struct Point *` parameters), struct-array element addresses (`set(&points[i])`), direct aggregate-array decay to struct/union pointer parameters (`set(points)` for `struct Point *` and `set(numbers)` for `union Number *`), array-to-pointer decay (`sum(values)` for `int *`/`char *` parameters), string-literal decay to read-only `char *` arguments, and array-element pointers (`&values[1]` and `&p[1]` when `p` is an array-backed pointer). Pointer indexing `p[i]` reads/writes array-base and array-element pointer storage with deterministic null/read-only/negative/out-of-bounds diagnostics; array-element pointer indexing is relative to the addressed element. Struct/union array pointers support indexed aggregate field access such as `points[i].x`, `points[i].x += 1`, and `numbers[i].value++` in addition to arrow access after pointer arithmetic. Struct pointers support `p->field` and `(*p).field` scalar field reads/writes plus field lvalue assignment, compound assignment, and increment/decrement; aggregate pointer dereference values and copy assignments such as `struct Point copy = *p;`, `sum(*p);`, `return *p;`, `*p = replacement;`, and `struct Point copy = (*p = replacement);` deep-copy supported structs/unions while preserving const-pointee diagnostics; null struct pointers report `null pointer dereference` and ended-scope targets report `pointer to out-of-scope variable '<name>'`. Scalar struct fields can be addressed with `&point.x`, nested field paths such as `&packet.anchor.y`, and struct-array element fields such as `&points[i].x`; dereferencing those pointers aliases the original field storage and preserves const-discard diagnostics. Array-backed scalar pointer arithmetic is supported for `p + n`, `n + p`, `p - n`, pointer difference between two pointers to the same array/string storage, `p += n`, `p -= n`, and pointer-variable `++`/`--`; struct/union-array element pointers such as `struct Point *p = &points[0]` and `union Number *n = &numbers[0]` support bounded `p + n`, `p - n`, `p += n`, `p -= n`, pointer-variable `++`/`--`, and same-array pointer difference while deliberately continuing to reject one-past results. Pointer equality/truthiness is supported for null, scalar, array, string, struct, union, and aggregate-array element pointers, with array decay equal to the zero-index element pointer. Pointer arithmetic on scalar/null pointers, pointer bitwise operations, pointer ordering comparisons, pointer-vs-nonzero-integer equality, pointer-to-pointer forms, and pointer arrays remain unsupported with targeted diagnostics.
- assignments: `x = expr;`, `xs[index] = expr;`, same-type struct copy assignment (`b = a;`) with value semantics, struct and union assignment expressions returning by-value copies (`struct Point copy = (target = source);`, `union Number picked = (*slot = replacement);`), struct field assignment statements and expressions (`p.x = expr;` / `return p.x = 3;`), aggregate compound-literal scalar-field lvalue expressions (`((struct Point){1, 2}).x = 7`, `((struct Point){3, 4}).y += 5`, `++((struct Point){5, 6}).x`, `((struct Point){7, 8}).y++`), pointer reassignment (`p = &x`/`p = 0`/`p = &xs[index]`/`p = p + n`), scalar/array-element dereference assignment (`*p = expr;`), grouped dereference assignment such as `*(&xs[1]) = expr;`, right-associative assignment expressions for scalar, scalar compound-literal, struct-field, array-index, and dereferenced pointer lvalues such as `return x = 1;`, `((int){1} = 5)`, `xs[0] = (xs[1] = 7);`, and `(*p = 6) != 0`, compound assignment expressions/statements `+=`, `-=`, `&=`, `|=`, `^=`, `<<=`, and `>>=` for scalar, scalar compound-literal, struct-field, array-index/pointer-index, and dereferenced pointer lvalues, `+=`/`-=` for array-backed pointer variables, plus prefix/postfix increment/decrement expressions/statements (`++x`, `x++`, `--x`, `x--`) for scalar, scalar compound-literal, struct-field, array-index/pointer-index, dereferenced pointer lvalues, and array-backed pointer variables; scalar cast expressions such as `(int)expr`, `(char)expr`, and scalar typedef casts such as `(Count)expr` parse at unary precedence and evaluate over Cust's existing integer storage model; const-qualified scalar/parameter bindings reject direct assignment, assignment expressions, compound assignment, increment/decrement, and scalar pointer writes, while const arrays reject indexed/pointer writes via read-only storage
- comma operator `left, right` at the lowest expression precedence, evaluating the left expression for side effects and yielding the right expression; supported in grouped expressions, conditions/truthiness contexts, `for` clauses, pointer-valued expressions, and call arguments (where top-level commas still separate arguments)
- conditional operator `cond ? then_expr : else_expr` with C-style truthiness for scalar, array-decay, and pointer-valued conditions; the middle operand is a full expression, nested conditionals are right-associative, and only the selected branch is evaluated
- array indexing expressions `xs[index]`, pointer indexing expressions `p[index]` and C reverse-subscript spellings such as `index[p]` / `index[values]` / `index["text"]` over supported pointer, array, string, array compound-literal, and aggregate-array pointer expressions, string literal indexing expressions `"text"[index]`, and scalar/array-element pointer dereference expressions `*p` with runtime negative/out-of-bounds/null/out-of-scope/read-only diagnostics as applicable
- `sizeof` expressions for supported Cust types and expressions: `sizeof(int)`, `sizeof(char)`, `sizeof(const int)`, `sizeof(const char)`, one-dimensional array type names such as `sizeof(int[3])`, `sizeof(char[4])`, `sizeof(const int[2])`, `sizeof(struct Point[2])`, and aggregate typedef arrays such as `sizeof(Number[3])`, direct aggregate type spellings such as `sizeof(struct Point)` / `sizeof(union Number)` / `sizeof(const union Number)`, pointer type spellings such as `sizeof(int *)` / `sizeof(char *)` / `sizeof(struct Point *)` / `sizeof(union Number *)` and const-qualified pointer type spellings such as `sizeof(const int *)`, scalar variables, arrays (using declared element type and length), pointer variables/address-of expressions including struct-field address-of expressions, first-pass struct variables, struct-array variables/elements, and scalar/array/nested struct fields (using deterministic Cust field-size sums without native ABI padding and union max-field sizing), typedef aliases including const-qualified aliases in size contexts, string literals (including the NUL terminator), indexed string/array/pointer expressions, and dereferenced pointer-valued expressions such as `sizeof(*(char_ptr + 1))`, `sizeof(*choose_char(...))`, `sizeof(*(&points[0].x))`, `sizeof(*(struct_ptr + 1))`, and `sizeof(*(cond ? left : right))` without evaluating the operand. Cust defines `sizeof(int) == 8`, `sizeof(char) == 1`, and pointer size `8`; `sizeof(void)` and `sizeof(const void)` are rejected with a targeted parser diagnostic.
- Const-qualified pointer declarations and parameters support a scoped subset documented in `docs/plans/const-pointer-model.md`: `const int *p` / `const char *p` / `const struct Point *p` mark writes through that pointer binding as read-only while still allowing pointer reassignment; `int * const p` / `char * const p` / `struct Point * const p` mark the pointer slot read-only while allowing writes to mutable targets; `const int * const p` / `const char * const p` / `const struct Point * const p` combine both. Pointer conversions preserve pointee constness: mutable pointer expressions may flow into const pointer targets, but assigning or passing `const int *` / `const char *` / `const struct Point *` expressions to mutable `int *` / `char *` / `struct Point *` targets reports `cannot discard const qualifier from pointer target`.
- Const-qualified struct variables and by-value parameters are supported for the existing scalar-field struct subset: `const struct Point p;` and `const Point p;` after a typedef create zero-initialized read-only struct bindings, `int f(const struct Point p)` receives a by-value read-only parameter copy, direct field/copy assignment to const struct bindings reports `cannot assign to const variable '<name>'`, and writes through const struct pointers or direct pointers to const struct targets report `cannot assign through pointer to const`.
- Const-qualified scalar fields inside struct definitions are supported for `const int` and `const char` fields. Field reads work through direct variables and struct pointers, mutable sibling fields remain writable, writes to const fields report `cannot assign to const struct field '<field>'`, and whole-struct copy assignment into struct types containing const fields reports `cannot assign to struct '<Type>' with const fields`.
- One-level pointer fields inside structs are supported for scalar and struct pointees, including self-referential links such as `struct Node *next;`, scalar pointer fields such as `int *external;`, pointer-field initializer entries, pointer-field reassignment with concrete pointee type checks, direct pointer-field array-backed pointer arithmetic (`cursor.p - 1`), compound pointer assignment (`cursor.p += 2`), prefix/postfix pointer increments/decrements (`++cursor.p`, `cursor.p--`), struct-pointer arrow access to pointer fields with matching pointer arithmetic and reassignment (`slot->p += 3`, `slot->p--`, `slot->p = slot->p - 2`), chained struct-pointer field access (`head.next->value`), and dereference of scalar pointer fields (`*head.external`). Pointer fields copy pointer values by value during struct copy/parameter/return flows, preserve pointee const metadata for `const T *field`, and reject unsupported pointer-to-pointer or pointer-array fields with targeted diagnostics.
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

- Pointer parameters now accept scalar-array fields by decay for direct structs, struct-array elements, struct pointers, and nested struct paths, so `use(packet.values)`, `use(packets[i].values)`, `use(slot->values)`, `use(one.inner.values)`, `use(boxes[i].inner.values)`, and `use(ptr->inner.values)` create interpreter-owned array-base pointers; `&packet.values[i]`, `&packets[n].values[i]`, `&slot->values[i]`, `&one.inner.values[i]`, `&boxes[n].inner.values[i]`, and `&ptr->inner.values[i]` create array-element pointers that alias the embedded field storage and preserve existing const/read-only diagnostics, including const root structs that make nested array-field decay a pointer-to-const conversion.
- Struct fields can now embed one-dimensional arrays of supported aggregate types such as `struct Line { struct Point points[2]; };`; Cust zero-initializes or brace-initializes each element, reads scalar fields with `line.points[i].x`, mutates scalar fields with assignment/compound assignment such as `line.points[0].y = line.points[1].x` and `line.points[1].x += 2`, deep-clones embedded aggregate-array fields for by-value struct parameters/copies, and reports deterministic Cust sizes without relying on native struct padding. Embedded aggregate-array fields also decay to aggregate pointers in pointer contexts (`mutate(line.points)`), support element address-of (`struct Point *p = &line.points[1]`), pointer arithmetic (`line.points + 2`), and pointer-indexed field access in callees while preserving const-discard diagnostics for const containing structs. The same aggregate-array field pointer behavior now works through struct pointers (`mutate(slot->points)`, `&slot->points[i]`, `slot->points + n`, and `slot->points[i].field`) while preserving const-discard diagnostics for const struct-pointer views.

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
- Parser diagnostics now include targeted missing-`(` messages after function names and `if`/`while`/`for` keywords, targeted missing-semicolon messages after `break`, `continue`, and `for` conditions, targeted missing-`=` messages after variable/pointer declarations and scalar/indexed/dereference assignments, targeted missing-name/type messages for function names, variable/pointer declarations, and parameter lists, unmatched closing delimiter messages for stray `)`/`]` in statements and extra `}` at top level, context-aware unterminated-block messages (for example after a function header or `if` condition), explicit empty-array-length diagnostics before `]` in declarations, negative array-length diagnostics, explicit rejection of `break`/`continue` in non-body `for` clauses, pointer-parameter malformed-list coverage, explicit unsupported pointer-return/pointer-array/parser diagnostics, explicit unsupported pointer-to-pointer parameter/declaration diagnostics, delimiter-aware trailing-comma diagnostics for function parameter/call lists, and duplicate `switch` case/default label diagnostics.

## Verified commands

```bash
cargo test --test interpreter reports_array_compound_literal_sizes_without_evaluating_initializers -- --nocapture
cargo test --test interpreter union_aggregate_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous `sizeof` array compound-literal and union array-field decay fixture run. This run fixed `sizeof((T[]){...})` / `sizeof((T[N]){...})` for scalar and aggregate array compound literals so Cust reports the array object size using inferred or fixed lengths without evaluating initializer side effects, instead of treating those expression forms as pointer-sized in non-evaluating `sizeof` contexts. It also locked in direct and struct-pointer embedded union-array field pointer decay/address-of coverage (`bag.numbers`, `&bag.numbers[i]`, `bag->numbers`, `&bag->numbers[i]`, and `bag->numbers + n`) plus const-discard diagnostics. Coverage includes `tests/fixtures/valid/sizeof_array_compound_literals.c`, `tests/fixtures/valid/union_aggregate_array_field_decay.c`, `tests/fixtures/valid/struct_pointer_union_array_field_decay.c`, invalid fixture `tests/fixtures/invalid/union_array_field_const_discard.c`, native C compiler-oracle fixtures, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter array_compound_literal -- --nocapture
cargo test --test interpreter nested_aggregate_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous char-array compound-literal string initializer and nested aggregate-array field fixture run. This run added `(char[]){"cat"}` parser support by routing string-literal entries in scalar-array compound literals through the existing `ArrayInitializer::StringLiteral` storage path, including fixed-size too-long diagnostics. It also locked in nested embedded aggregate-array field pointer decay coverage for `box->inner.points`, `&box->inner.points[i]`, pointer arithmetic, and const-discard diagnostics. Coverage includes expanded `tests/fixtures/valid/array_compound_literals.c`, invalid fixture `tests/fixtures/invalid/array_compound_literal_string_too_long.c`, valid/invalid nested aggregate-array field fixtures, and native C compiler-oracle fixtures. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter struct_pointer_aggregate_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous struct-pointer embedded aggregate-array field decay run. This run extended embedded `struct T field[N]` aggregate-array field pointer decay from direct structs to struct-pointer paths: `slot->points` decays into aggregate pointer contexts, `&slot->points[i]` returns an aliasing aggregate pointer, `slot->points + n` reuses bounded aggregate-array field pointer arithmetic, and `slot->points[i].field` reads/writes through the containing struct pointer while preserving const-discard diagnostics for const struct-pointer views. Coverage includes `tests/fixtures/valid/struct_pointer_aggregate_array_field_decay.c`, invalid fixture `tests/fixtures/invalid/struct_pointer_aggregate_array_field_const_discard.c`, native fixture `tests/fixtures/compat/valid/struct_pointer_aggregate_array_field_decay.c`, and focused interpreter regressions. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter supports_struct_aggregate_array_fields -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous aggregate-array-fields-in-structs recovery run. This run finished the in-progress embedded `struct T field[N]` work by accepting aggregate-array fields inside supported structs, recursively brace-initializing them, reading and mutating scalar fields through `line.points[i].field`, preserving deep-copy/by-value isolation for containing structs, and adding native C compiler-oracle coverage that compares behavior/exit code only. Coverage includes `tests/fixtures/valid/struct_aggregate_array_fields.c`, native fixture `tests/fixtures/compat/valid/struct_aggregate_array_fields.c`, and the `supports_struct_aggregate_array_fields` interpreter regression. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter supports_address_of_dereference_as_pointer_identity -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous address-of-dereference pointer identity run. This run added C-style `&*pointer_expr` support by lowering address-of applied to a dereference back to the underlying pointer expression, so scalar pointers, array-backed pointer arithmetic expressions such as `&*(values + 1)`, pointer parameters, and null pointer values preserve pointer metadata without dereferencing the target. Coverage includes `tests/fixtures/valid/address_of_dereference.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/address_of_dereference.c`, `tests/interpreter.rs`, and `tests/c_compat.rs`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter nested_struct_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous nested struct array-field decay const-propagation run. This run added a regression for nested scalar array-field decay and element address-of across direct struct variables, struct-array elements, and struct pointers (`one.inner.values`, `boxes[i].inner.values`, `ptr->inner.values`, and matching `&...values[j]` forms), plus negative regressions proving `const struct Box box; mutate(box.inner.values);` and `const struct Box boxes[1]; mutate(boxes[0].inner.values);` reject mutable pointer decay with `cannot discard const qualifier from pointer target`. The implementation adds direct nested array-field and struct-array element const inference while preserving pointer-field `points_to_const` behavior. Coverage includes `tests/fixtures/valid/nested_struct_array_field_decay.c`, invalid fixtures `tests/fixtures/invalid/nested_struct_array_field_const_discard.c` and `tests/fixtures/invalid/nested_struct_array_element_field_const_discard.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/nested_struct_array_field_decay.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter struct_pointer_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous struct-pointer array-field decay/index/address-of parity run. This run added scalar array-field decay and element address-of support for struct pointers reached through `->`: `slot->values` / `slot->label` can bind to `int *`/`char *` parameters, `slot->values[i]` reads/writes embedded elements, and `&slot->values[i]` / `&slot->label[i]` produce mutable interpreter-owned array-element pointers to embedded field storage. Const struct-pointer views preserve pointer-conversion safety by rejecting mutable decay with `cannot discard const qualifier from pointer target`. Coverage includes `tests/fixtures/valid/struct_pointer_array_field_decay.c`, invalid fixture `tests/fixtures/invalid/struct_pointer_array_field_const_discard.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_pointer_array_field_decay.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter supports_struct_array_field_decay_and_element_address_of -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous struct-array field decay/address-of parity run. This run added scalar array-field decay and element address-of support for direct struct variables and struct-array elements: `packet.values` / `packets[i].values` can bind to `int *`/`char *` parameters, and `&packet.values[j]` / `&packets[i].values[j]` produce mutable interpreter-owned array-element pointers to embedded field storage. Coverage includes `tests/fixtures/valid/struct_array_field_decay.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_array_field_decay.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo test --test interpreter struct_char_array -- --nocapture
cargo test --test c_compat -- --nocapture
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-09 autonomous struct-char-array string initializer run. This run reused the fixed-size `char` array string-literal parser path for struct/union array fields, so positional aggregate initializers (`{"cat", 3}`), field designators (`.text = "hi"`), nested field path designators (`.label.text = "A\\x2a"`), and struct-array element initializers can initialize embedded `char[N]` fields from string literals with zero-fill and too-long diagnostics preserved. Coverage includes `tests/fixtures/valid/struct_char_array_string_initializers.c`, invalid `tests/fixtures/invalid/struct_char_array_string_initializer_too_long.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_char_array_string_initializers.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter char_array_string -- --nocapture
cargo test --test interpreter supports_char_arrays_initialized_from_string_literals -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous char-array string initializer run. This run added fixed-size `char` array initialization from string literals, preserving NUL inclusion when it fits, C-compatible exact-size non-NUL-terminated initialization, zero-fill for omitted trailing array elements, static/const array storage behavior, and a targeted too-long initializer diagnostic. Coverage includes `tests/fixtures/valid/char_array_string_initializers.c`, invalid `tests/fixtures/invalid/char_array_string_initializer_too_long.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/char_array_string_initializers.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. The compiler-oracle fixture avoids exact-size NUL truncation because `cc -std=c11 -Wall -Wextra -Werror` rejects that supported C form as `-Wunterminated-string-initialization` on this host; interpreter-only coverage keeps the Cust behavior explicit. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_octal_and_hex_escape_sequences -- --nocapture
cargo test --test interpreter reports_hex_escape_sequences_without_digits -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous numeric-escape-sequence run. This run expanded character and string literal lexing to support C numeric escape sequences: octal escapes such as `\\101` consume up to three octal digits and hexadecimal escapes such as `\\x2a` consume one or more hex digits, with a targeted source-line/caret diagnostic for `\\x` without following hex digits. Coverage includes `tests/fixtures/valid/numeric_escape_sequences.c`, invalid `tests/fixtures/invalid/hex_escape_without_digits.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/numeric_escape_sequences.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter standard_simple_escape -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous standard-escape-sequence run. This run expanded the lexer-supported simple C escape sequences in both character and string literals: `\a`, `\b`, `\f`, `\r`, `\v`, and `\?` now map to their standard scalar byte values alongside the previously supported escapes, while unsupported escape diagnostics remain unchanged for other sequences. Coverage includes `tests/fixtures/valid/standard_escape_sequences.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/standard_escape_sequences.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter integer_literal -- --nocapture
cargo test --test interpreter invalid_octal -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous integer literal base run. This run added C-style octal and hexadecimal integer constants to the lexer: `052`, `0x2a`, and `0X10` now produce scalar integer values usable by the existing expression/initializer/interpreter paths. Invalid octal digits such as `08` report `invalid digit '8' in octal integer literal` with the existing source-line/caret context, and base-prefixed out-of-range literals reuse the established `integer literal out of range` diagnostic. Coverage includes `tests/fixtures/valid/integer_literal_bases.c`, invalid `tests/fixtures/invalid/invalid_octal_integer_literal.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/integer_literal_bases.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_addressable_scalar_and_aggregate_compound_literals -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous addressable compound literal run. This run added a scoped storage/lifetime model for addressable scalar and aggregate compound literals: `&(int){7}`, `&(char){'a'}`, and `&(struct Point){.x = 1}` now allocate hidden mutable storage in the current Cust scope and return interpreter-owned pointers through the existing safe pointer model. Dereference writes, pointer-parameter binding, aggregate `->` field access, concrete pointee type checks, and current-scope lifetime/out-of-scope behavior reuse established scalar/struct pointer paths. Coverage includes `tests/fixtures/valid/addressable_compound_literals.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/addressable_compound_literals.c`, `tests/interpreter.rs`, `tests/c_compat.rs`, and `docs/plans/addressable-compound-literals.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter aggregate_array_compound -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous aggregate-array compound literal run. This run added C99-style aggregate-array compound literals for supported struct/union element types: `(struct Point[]){{1, 2}, {.x = 3}}`, `(union Number[]){{7}, [2] = {.value = 9}}`, and fixed-size forms such as `(struct Point[3]){...}` now parse as pointer-valued rvalue expressions backed by mutable interpreter-owned aggregate-array storage in the current scope. Positional and designated aggregate element initializers work with inferred unsized lengths or checked fixed lengths, fixed-size excess initializers report `too many initializers for aggregate array compound literal`, and existing aggregate-array pointer indexing/mutation/parameter-binding paths allow pointer-variable initialization and direct aggregate pointer/array-parameter calls. Coverage includes `tests/fixtures/valid/aggregate_array_compound_literals.c`, invalid `tests/fixtures/invalid/aggregate_array_compound_literal_too_many_initializers.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_array_compound_literals.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter array_compound -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous array compound literal run. This run added C99-style scalar-array compound literals for supported scalar element types: `(int[]){1, 2}`, `(char[]){'a', 0}`, and fixed-size forms such as `(int[3]){...}` now parse as pointer-valued rvalue expressions backed by mutable interpreter-owned array storage. Positional and designated initializers work with inferred unsized lengths or checked fixed lengths, fixed-size excess initializers report `too many initializers for array compound literal`, and the existing pointer binding/indexing paths allow pointer-variable initialization and direct pointer/array-parameter calls. Coverage includes `tests/fixtures/valid/array_compound_literals.c`, invalid `tests/fixtures/invalid/array_compound_literal_too_many_initializers.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/array_compound_literals.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter scalar_compound -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous scalar compound literal run. This run added C99-style scalar compound literals for supported scalar type spellings and scalar typedef aliases: `(int){expr}`, `(char){expr}`, and `(Count){expr}` now parse as rvalue scalar expressions, accept a trailing comma, reject excess initializer entries with `too many initializers for scalar compound literal`, preserve initializer side effects in evaluated contexts, and remain non-evaluating under `sizeof`. Coverage includes `tests/fixtures/valid/scalar_compound_literals.c`, invalid `tests/fixtures/invalid/scalar_compound_literal_too_many_initializers.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/scalar_compound_literals.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_aggregate_compound_literals_in_expression_contexts -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous aggregate compound literal run. This run added scoped C99-style aggregate compound literals for supported structs/unions and aggregate typedef aliases, including declaration initialization, assignment expressions, by-value function arguments, aggregate returns, conditional/comma aggregate expression contexts, and scalar field reads from compound literals. Coverage includes `tests/fixtures/valid/aggregate_compound_literals.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_compound_literals.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_fixed_array_parameters_as_pointer_parameters -- --nocapture
cargo test --test interpreter supports_one_dimensional_arrays_indexing_and_parameters -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous fixed-array-parameter decay run. This run aligned fixed-size scalar array parameter spellings (`int values[3]`, `char text[2]`) with C parameter adjustment by parsing them as pointer parameters while still syntax-checking bracket lengths. Larger arrays and string literals now decay through the existing safe pointer model, writes through the callee alias caller storage, and `sizeof(values)` inside such parameters reports Cust pointer size. Coverage includes `tests/fixtures/valid/fixed_array_parameters_decay.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/fixed_array_parameters_decay.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_braced_scalar_initializers_in_declarations_and_aggregates -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous braced scalar initializer run. This run added parser support for single-expression braced scalar initializers in scalar declaration contexts (`int scalar = {side_effect(&calls)};`), scalar array initializer entries/designators (`int values[3] = {{1}, {2}, [2] = {3}};`), scalar struct/union fields (`struct Pair pair = {{scalar}, {2}, {{7}, {8}, {9}}};`, `union Number number = {{12}};`), and scalar path designators (`.left = {10}`, `.values[1] = {11}`). The contained expression is evaluated once and trailing commas are accepted. The existing C compiler-oracle suite still passes; the new fixture remains interpreter-only because `cc -std=c11 -Wall -Wextra -Werror` rejects otherwise-compatible braced scalar initializers with warnings promoted to errors. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter struct_pointer_field -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous struct-pointer-field arithmetic run. This run extended struct pointer fields beyond direct reassignment: pointer-valued fields such as `cursor.p` now participate in array-backed pointer arithmetic expressions (`cursor.p - 1`), compound pointer assignments (`cursor.p += 2`), and prefix/postfix pointer increments/decrements (`++cursor.p`, `cursor.p--`) while preserving Cust's interpreter-owned pointer bounds/null/scalar-pointer diagnostics. Direct struct pointer-field assignments now also validate concrete pointee type compatibility at the assignment boundary, so assigning `struct Size *` into a `struct Point *` field reports `cannot convert pointer to struct 'Size' to pointer to struct 'Point'` instead of storing the wrong pointer. Coverage includes `tests/fixtures/valid/struct_pointer_field_arithmetic.c`, invalid `tests/fixtures/invalid/struct_pointer_field_type_mismatch.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/struct_pointer_field_arithmetic.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_direct_enum_type_declarations_parameters_returns_and_sizeof -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous direct-enum-type run. This run added direct named-enum type spellings across declarations and function signatures: `enum State global = READY;`, local `enum State` / `const enum State` variables, `enum State` parameters, prototypes, return types, and `sizeof(enum State)` now parse through Cust's existing integer-backed enum model without requiring a typedef alias. Coverage includes `tests/fixtures/valid/direct_enum_types.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/direct_enum_types.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_const_qualified_return_types -- --nocapture
cargo test --test c_compat -- --nocapture
cargo test --test interpreter -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
cargo test --test fuzz_safety -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous const-return-type run. This run added top-level const-qualified function return spellings for scalar, typedef-scalar, direct aggregate, and typedef-aggregate returns. Cust treats the top-level qualifier as return-type syntax over the existing by-value return model, while preserving `const T *` pointer-return pointee-const semantics. Coverage includes `tests/fixtures/valid/const_return_types.c`, `tests/interpreter.rs`, and `docs/plans/const-return-types.md`. Native compiler-oracle coverage is intentionally not added for this fixture because `cc -std=c11 -Wall -Wextra -Werror` rejects top-level const return qualifiers as `-Werror=ignored-qualifiers`; the finding is recorded in `status/research.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_sizeof_pointer_expressions_without_evaluating_operands -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous sizeof-pointer-expression run. This run made `sizeof(*pointer_expr)` type-aware for non-evaluating pointer-valued operands including pointer arithmetic, pointer-returning calls, conditional pointer expressions, comma expressions, string/array decay, and aggregate-array pointers. `sizeof(*(char_ptr + 1))` now reports `char` size instead of defaulting to `int`, while `sizeof(*side_effect_pointer(...))` remains non-evaluating. Coverage includes `tests/fixtures/valid/sizeof_pointer_expressions.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/sizeof_pointer_expressions.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter aggregate_assignment -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous aggregate-assignment-expression run. This run added aggregate assignment expressions for supported structs and unions: same-type variable assignments such as `struct Point copy = (target = source);` and aggregate pointer dereference assignments such as `union Number picked = (*slot = replacement);` now evaluate as aggregate expressions that return by-value copies of the assigned value while mutating the destination. Runtime assignment clones the RHS into the target and returns a separate deep-cloned aggregate value, preserving existing const-field/const-pointee enforcement and same-type mismatch diagnostics. Coverage includes `tests/fixtures/valid/aggregate_assignment_expressions.c`, invalid `tests/fixtures/invalid/aggregate_assignment_expression_type_mismatch.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_assignment_expressions.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter aggregate_pointer_dereference -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous aggregate-pointer-dereference run. This run added aggregate pointer dereference values and copy assignment for the supported struct/union pointer subset: `struct Point copy = *p;`, `sum(*p);`, `return *p;`, `union Number picked = *n;`, and `*p = replacement;` now deep-copy aggregate values through interpreter-owned pointers, including by-value function argument and return contexts, while writes through `const struct/union *` views report `cannot assign through pointer to const`. Coverage includes `tests/fixtures/valid/aggregate_pointer_dereference.c`, invalid `tests/fixtures/invalid/const_aggregate_pointer_deref_assignment.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_pointer_dereference.c`, and `tests/interpreter.rs` / `tests/c_compat.rs` wiring. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_unnamed_function_prototype_parameters -- --nocapture
cargo test --test interpreter parameter_names -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous unnamed-prototype-parameter run. This run added C-style unnamed parameter support for semicolon-terminated function prototypes, so declarations such as `int add(int, int);`, `char pick(char *);`, `void mutate(int [], struct Point *);`, and `union Number choose(union Number [], int);` parse as signatures without requiring parameter names while function definitions continue to require names and preserve the existing exact missing-name diagnostics. Coverage includes `tests/fixtures/valid/unnamed_prototype_parameters.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/unnamed_prototype_parameters.c`, focused regression coverage in `tests/interpreter.rs`, C-oracle wiring in `tests/c_compat.rs`, and README/status updates. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter anonymous_enum -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous anonymous-enum-typedef run. This run added parser support for C-style anonymous enum typedef definitions such as `typedef enum { READY = 1, BUSY } Status;`, resolving the alias to Cust's existing integer scalar type while emitting the enum constants as scoped runtime enum bindings. Top-level and block-scoped anonymous enum typedefs work in globals, locals, arrays, function parameters/returns, and `sizeof(alias)`; block-local alias and constant shadowing follow existing parser/runtime block scopes. Coverage includes `tests/fixtures/valid/anonymous_enum_typedefs.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/anonymous_enum_typedefs.c`, and typedef-model/README updates. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter anonymous_aggregate -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous anonymous-aggregate-typedef run. This run added parser support for C-style anonymous aggregate typedef definitions such as `typedef struct { int x; int y; } Point;` and `typedef union { int value; char tag; } Number;`. Anonymous definitions receive unique internal type identities without adding source-level tags, the alias remains parser-only metadata, diagnostics display the alias name for the anonymous aggregate type, and alias-spelled declarations/arrays/pointers/functions/returns/`sizeof` reuse the existing safe struct/union runtime model. Coverage includes `tests/fixtures/valid/anonymous_aggregate_typedefs.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/anonymous_aggregate_typedefs.c`, and typedef-model documentation updates. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter aggregate_tag_shadowing -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous aggregate-tag-shadowing run. This run replaced aggregate tag visibility from a set of names with lexical source-tag-to-internal-type maps, allowing block-local aggregate typedef definitions such as `typedef struct Point { ... } Point;` and `typedef union Number { ... } Number;` to shadow outer tags without leaking after block exit or colliding in runtime metadata. Existing top-level duplicate-tag diagnostics still reject redeclarations in the same parser scope, while nested shadows receive unique internal type identities for runtime struct/union field maps, function signatures, pointers, and typedef aliases. Coverage includes `tests/fixtures/valid/aggregate_tag_shadowing.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_tag_shadowing.c`, and typedef-model documentation updates. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter unsized_array -- --nocapture
cargo test --test interpreter rejects_const_scalar_array_decay_to_mutable_unsized_parameter -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous unsized-array-parameter run. This run added C-style unsized array parameter spellings for scalar and supported aggregate arrays: `int values[]`, `char text[]`, `struct Point points[]`, and `union Number numbers[]` now parse as pointer parameters, reuse existing pointer argument binding/decay, preserve callee mutation of original array storage, allow string literals for read-only `char` pointer views, and reject const array decay into mutable unsized parameters with the existing `cannot discard const qualifier from pointer target` diagnostic. Coverage includes `tests/fixtures/valid/unsized_array_parameters.c`, invalid `tests/fixtures/invalid/unsized_array_parameter_const_discard.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/unsized_array_parameters.c`, and README/status updates. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter block_scoped -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-08 autonomous block-scoped aggregate typedef definition run. This run added block-local `typedef struct Name { ... } Alias;` and `typedef union Name { ... } Alias;` definitions by adding parser aggregate-tag visibility scopes alongside the existing block typedef/enum scopes. Block-local aggregate tags and aliases are usable inside their declaring block, expire after block exit, and the parsed runtime type definitions remain available to execute already-parsed block statements; repeated aggregate tag names are still rejected globally to avoid ambiguous runtime metadata until unique internal type identities are designed. Coverage includes `tests/fixtures/valid/block_scoped_aggregate_typedef_definitions.c`, invalid `tests/fixtures/invalid/block_aggregate_typedef_alias_out_of_scope.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/block_scoped_aggregate_typedef_definitions.c`, and updated `docs/plans/typedef-model.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter pointer_return -- --nocapture
cargo test --test interpreter array_initializers -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous pointer-return-function run. This run added one-level pointer-returning functions and prototypes for scalar, char, const-pointee, struct, union, and pointer-typedef return spellings; return flow now carries interpreter-owned pointer metadata plus declared pointee type/const metadata, validates type and const conversions at the return boundary, and keeps pointer-to-pointer return types rejected with exact parser diagnostics. The implementation also replaced speculative pointer-expression probing with a side-effect-safe syntactic pointer-value check so scalar function calls in arithmetic/truthiness contexts are not evaluated while merely checking for pointer operands. Coverage includes `tests/fixtures/valid/pointer_return_functions.c`, invalid `tests/fixtures/invalid/pointer_return_type_mismatch.c` / `tests/fixtures/invalid/pointer_return_const_discard.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/pointer_return_functions.c`, and design notes in `docs/plans/pointer-return-functions.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter pointer_type_mismatch -- --nocapture
cargo test --test interpreter pointer_assignment_type_mismatches -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous pointer-type-compatibility run. This run tightened Cust's safe pointer model so declared pointer slots, pointer parameters, pointer assignment expressions/statements, and pointer-field initializers validate the concrete runtime pointee type before accepting non-null pointer values. Invalid conversions now fail at the conversion boundary instead of mutating wrong-typed storage or producing misleading later diagnostics: `char *` into `int *` reports `cannot convert pointer to char to pointer to int`, `union Number *` into `struct Point *` reports `cannot convert pointer to union 'Number' to pointer to struct 'Point'`, and `struct Size *` assignment into `struct Point *` reports the same targeted shape. Null pointer conversions remain type-compatible, and existing const-discard checks are preserved. Coverage includes `tests/fixtures/invalid/scalar_pointer_type_mismatch.c`, `tests/fixtures/invalid/aggregate_pointer_type_mismatch.c`, `tests/fixtures/invalid/pointer_assignment_type_mismatch.c`, updated pointer model notes, and `references/cust-pointer-type-compatibility.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter aggregate_array_decay -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-array-decay run. This run added direct aggregate-array decay to existing aggregate pointer parameters: `struct Point points[2]; use(points);` and `union Number numbers[2]; use(numbers);` now produce interpreter-owned pointers to element zero for `struct Point *` / `union Number *` parameters, preserving callee mutation of the original aggregate array storage. Const aggregate arrays are treated as pointers to const elements for conversion checks, so passing `const struct Point points[1]` to a mutable `struct Point *` parameter reports `cannot discard const qualifier from pointer target`, while mutable-to-const pointer parameter flow remains valid. Coverage includes `tests/fixtures/valid/aggregate_array_decay_to_pointers.c`, invalid `tests/fixtures/invalid/const_aggregate_array_decay_discard.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_array_decay_to_pointers.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter aggregate_arrays -- --nocapture
cargo test --test interpreter struct_array_designators -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-array-designated-initializer run. This run extended the designated-initializer model to one-dimensional arrays of supported structs and unions: element designators such as `struct Point points[3] = {[2] = {.y = 6, .x = 5}, [0] = {1, 2}};` and `union Number numbers[3] = {[1] = {.tag = 7}, [2] = {4}};` now initialize selected aggregate-array elements, preserve zero/default initialization for omitted elements, support mixed positional continuation after designators, and report targeted out-of-bounds diagnostics such as `array designator index 2 out of bounds for struct array 'points'`. Coverage includes `tests/fixtures/valid/aggregate_array_designated_initializers.c`, invalid `tests/fixtures/invalid/struct_array_designator_out_of_bounds.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_array_designated_initializers.c`, and updated `docs/plans/designated-initializers.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter pointer_indexed_aggregate_values -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-pointer-indexed-value run. This run closed an aggregate/function gap for C-style struct/union array pointers: indexed aggregate pointer expressions such as `p[i]` and `n[i]` can now be used as by-value aggregate expressions in same-type copy assignment, aggregate declaration initializers, and by-value function arguments. Runtime evaluation offsets the interpreter-owned aggregate-array pointer, deep-clones the selected element field map, preserves caller/callee copy isolation, and keeps existing bounds and pointer diagnostics. Coverage includes `tests/fixtures/valid/aggregate_pointer_indexed_values.c` and native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_pointer_indexed_values.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter pointer_indexing -- --nocapture
cargo test --test interpreter aggregate_pointer_index -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-pointer-indexing run. This run completed a C-style indexed aggregate-pointer field milestone: struct/union array pointers such as `struct Point *p = &points[0]` and `union Number *n = &numbers[0]` now support `p[i].field` / `n[i].field` scalar member reads, assignment expressions, compound assignments, and prefix/postfix increment/decrement. Runtime evaluation offsets the interpreter-owned aggregate-array pointer by the index and reuses struct-pointer field helpers, preserving aggregate-array bounds checks, union scalar-field synchronization, and const-pointee write diagnostics (`cannot assign through pointer to const`). Coverage includes `tests/fixtures/valid/aggregate_pointer_indexing.c`, invalid `tests/fixtures/invalid/const_aggregate_pointer_index_write.c`, and native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_pointer_indexing.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter pointer_arithmetic -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-pointer-arithmetic run. This run extended Cust's safe pointer model so struct-array and union-array element pointers created with forms like `struct Point *p = &points[0]` and `union Number *n = &numbers[0]` can be offset with `p + n`, `p - n`, `p += n`, `p -= n`, pointer-variable `++`/`--`, and same-array pointer subtraction. The implementation reuses interpreter-owned `PointerValue::StructElement { scope_id, name, index }` targets, validates live scopes and aggregate-array bounds on every offset, and deliberately continues rejecting one-past aggregate pointers. Coverage includes `tests/fixtures/valid/aggregate_pointer_arithmetic.c`, invalid `tests/fixtures/invalid/struct_pointer_arithmetic_out_of_bounds.c`, native C compiler-oracle fixture `tests/fixtures/compat/valid/aggregate_pointer_arithmetic.c`, and notes in `references/cust-aggregate-pointer-arithmetic.md`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo test --test interpreter supports_sizeof_aggregate_type_names -- --nocapture
cargo test --test c_compat -- --nocapture
docker compose run --rm test
docker compose run --rm cust
```

All passed after the 2026-05-07 autonomous aggregate-sizeof-type run. This run closed a parser gap in `sizeof(...)` type contexts: direct aggregate type spellings such as `sizeof(struct Bytes)`, `sizeof(union Number)`, `sizeof(const union Number)`, `sizeof(struct Bytes *)`, and `sizeof(union Number *)` now parse as type operands instead of falling through to expression parsing. Cust continues to use deterministic interpreter sizes (struct field sums with no native padding, union max-field sizing, pointer size 8) while compiler-oracle coverage avoids ABI-sensitive exact struct sizes by checking stable C size relationships. Coverage includes `tests/fixtures/valid/sizeof_aggregate_types.c` and native C compiler-oracle fixture `tests/fixtures/compat/valid/sizeof_aggregate_types.c`. Docker Compose emitted non-fatal `Docker Compose requires buildx plugin to be installed` warnings and fell back to the classic builder; both required Docker commands exited 0.

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
