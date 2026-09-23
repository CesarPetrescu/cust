# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Direct double aggregate-field row addresses have focused/interpreter-oracle GREEN and independent pre-gate review CLEAR; aggregate double row-pointer fields remain a separately scoped candidate, not an active blocker.

### 2026-09-24 — Direct aggregate double row addresses

- Before-state: `double (*row)[2] = &table.rows[1];` failed at the existing unsupported double row-address boundary, although direct `table.rows` decay already yielded a typed row pointer.
- Cause: direct `AddressOfStructArrayField` lacked the 2D-double row case and field-owner attachment, and the blanket double-address classifier rejected it. Typed field-backed base plus row offset preserves the root; `sizeof` stays metadata-only.
- RED/GREEN: focused interpreter test first failed and now passes; a second test covers const/width/bounds/lifetime and non-evaluation. Registered C11 fixture agrees at exit 14. Read-only review returned `AI_REVIEW:CLEAR`. No external blocker remains.

### 2026-09-23 — TODO 445 comma-operator RHS diagnostics

- Failure: `(1, /)` and `values[0, /]` reached generic primary parsing and reported `expected expression, found Slash` instead of comma-operator context.
- Root cause: `parse_comma_expr()` lacked invalid binary/assignment-start validation, while `parse_index_expr()` maintained an independent comma loop that could bypass any general-loop fix.
- RED/GREEN: exact focused tests first failed for the ordinary and index-loop forms. `reject_invalid_comma_operator_rhs()` now serves both loops and preserves structural, invalid-operator, and keyword precedence.
- Stack closure: an initial in-frame check made `sizeof_base_integer_string_conversion_endptr_validation_remains_linear` abort with stack overflow; moving complete diagnostic construction into the helper returns the normal-stack regression to GREEN.
- Review/gate: initial independent review identified the index-loop omission; the review-fix regression went RED then GREEN, and fresh re-review returned `AI_REVIEW:CLEAR`. Formatting, strict Clippy, full local tests, Docker test exit 0, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-17 — TODO 441 leading comma in control conditions

- Failure: `if (,)`, `while (,)`, and `do { } while (,);` reached generic primary-expression parsing and reported `expected expression, found Comma`.
- Root cause: `reject_missing_control_condition_expr()` classified structural delimiters and postfix-only starts but omitted `Token::Comma`, unlike the completed switch-specific structural guard.
- RED/GREEN: the three-case exact source-location regression first failed at the `if` case. Adding `Token::Comma` to the shared structural guard makes all three GREEN; a separate execution regression proves commas after valid primaries still parse and run.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,671 local tests, Docker test exit 0 after the foreground Compose client timeout, rebuilt Docker runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-17 — TODO 440 leading comma in `switch` expressions

- Failure: `switch (,)` reached generic primary-expression parsing and reported `expected expression, found Comma`.
- Root cause: `reject_missing_switch_expr()` intentionally covered structural delimiters, selectors, and conditional markers but omitted `Token::Comma`; unlike all remaining impossible binary/assignment operators, comma was not classified by `reject_invalid_control_condition_expr()`.
- RED/GREEN: the exact source-location regression first failed with the generic Comma message. Adding `Token::Comma` to the switch-only structural guard makes it GREEN; all remaining impossible operator starts are already contextual, and valid unary prefixes retain their dedicated missing-operand diagnostics.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,670 local tests, Docker test exit 0 confirmed from the retained container after the foreground client timeout, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-17 — TODO 439 selector-token starts in `switch` expressions

- Failure: `switch (.)` and `switch (->field)` reached generic primary-expression parsing and reported `expected expression, found Dot/Arrow`.
- Root cause: the switch-specific structural guard covered delimiters, braces, brackets, and conditional markers but omitted the two postfix-only selector tokens.
- RED/GREEN: the exact two-case source-location matrix first failed at `Dot`; adding `Token::Dot` and `Token::Arrow` to `reject_missing_switch_expr()` makes it GREEN. `switch (choice.value)` remains accepted, proving postfix member access after a primary expression is unchanged.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,670 local tests, Docker test exit 0 after the foreground client wait, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-17 — TODO 438 malformed keyword starts in `switch` expressions

- Failure: `switch (int)`, `switch (struct)`, `switch (return)`, and `switch (if)` reached generic primary-expression parsing and reported `expected expression, found <Token>`.
- Root cause: switch used its route-specific delimiter guard and the new invalid-operator guard but omitted the established keyword-start guard invoked by the other control condition routes.
- RED/GREEN: the four-case exact source-location matrix first failed at `Int`; adding `reject_keyword_start_expression("switch")` after switch delimiter validation makes it GREEN. Direct CLI coverage confirms legal `_Generic` selectors remain valid and non-evaluating.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,670 local tests, Docker test exit 0 after the foreground client time window, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-16 — TODO 437 malformed nonempty `switch` condition diagnostics

- Failure: `switch (/)`, `switch (=)`, `switch (==)`, `switch (&&)`, `switch (%=)`, and `switch (<)` all reported generic `expected expression` diagnostics.
- Root cause: `parse_switch()` ran only its structural missing-expression guard and then delegated invalid operator starts to generic primary-expression parsing, omitting the shared invalid-control-condition guard used by `if`, `while`, and `do-while`.
- RED/GREEN: a six-case exact source-location matrix first failed with `expected expression, found Slash`. Calling the shared guard after structural validation makes the matrix GREEN and retains legal unary, grouped, scalar, and enum switch conditions.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,669 local tests, Docker test exit 0 after the foreground client timeout, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-16 — TODO 436 malformed nonempty `if` condition diagnostics

- Failure: `if (/)`, `if (=)`, `if (==)`, `if (&&)`, `if (%=)`, and `if (<)` all reported generic `expected expression` diagnostics.
- Root cause: `parse_if()` ran the shared structural/keyword condition guard and then delegated operator starts to generic primary-expression parsing; the shared invalid-operator condition guard was used by `while` and `do-while` but omitted from the if route.
- RED/GREEN: a six-case exact source-location matrix failed as expected. Calling the shared guard after structural/keyword validation makes the matrix GREEN and retains legal unary, grouped, and scalar if conditions.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,667 local tests, Docker test exit 0 after the foreground client window, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-16 — TODO 435 malformed nonempty `do-while` condition diagnostics

- Failure: `do { } while (/)`, `do { } while (=)`, `do { } while (==)`, `do { } while (&&)`, `do { } while (%=)`, and `do { } while (<)` all reported generic `expected expression` diagnostics.
- Root cause: `parse_do_while()` ran the shared structural/keyword condition guard and then delegated operator starts to generic primary-expression parsing; the earlier while-specific guard was not reusable.
- RED/GREEN: a six-case exact source-location matrix failed as expected. Parameterizing the narrow operator guard by control context makes the matrix GREEN and retains existing `while` behavior plus legal unary, grouped, and scalar do-while conditions.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; canonical local/Docker verification is recorded with this run. No external blocker remains.

### 2026-09-16 — TODO 434 malformed nonempty `while` condition diagnostics

- Failure: `while (/)`, `while (=)`, `while (==)`, `while (&&)`, `while (%=)`, and `while (<)` all reported generic `expected expression` diagnostics.
- Root cause: the shared control-condition guard covered structural/EOF and keyword starts but intentionally did not classify binary-only, assignment, equality, relational, or shift tokens; `parse_while()` therefore reached generic primary-expression parsing.
- RED/GREEN: a six-case exact source-location matrix failed as expected against generic messages. A narrow post-shared-guard `while` helper now reports `expected expression after while, found <Token>` while legal unary, grouped, and scalar conditions remain GREEN.
- Review/gate: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,665 local tests, Docker test exit 0 after a retained container outlived the foreground client window, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-16 — TODO 432 malformed nonempty-`return` expression diagnostics

- Failure: `return /;`, `return =;`, `return ==;`, `return &&;`, `return %=;`, and `return <;` all reported generic `expected expression` diagnostics.
- Root cause: `reject_missing_return_expr()` covered structural punctuation/EOF and delegated keyword starts to the contextual shared helper, but omitted operators which cannot legally begin a Cust expression.
- RED/GREEN: a six-case exact source-location matrix failed as expected against the generic messages. The narrow token guard now produces `expected expression after return, found <Token>` for all six while preserving bare void return plus valid scalar, pointer, and aggregate return behavior.
- Review/gate: final independent read-only Codex review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,664 local tests, Docker test exit 0 after the foreground wait window, rebuilt runtime output `10`, and diff hygiene pass. No external blocker remains.

### 2026-09-16 — TODO 431 malformed-`for` expression-start diagnostics

- Failure: malformed initializer, condition, and increment starts such as `for (=; ; )`, `for (; ==; )`, and `for (; ; ])` reached generic clause or primary-expression diagnostics.
- Root cause: `parse_for()` had separate partial route-local guards; conditions delegated straight to `parse_expr()`, while initializer/increment coverage omitted several operator and delimiter tokens.
- RED/GREEN: the 15-case exact source-location matrix first exposed the generic fallbacks. One shared clause-start guard now covers impossible expression starts while deliberately retaining unary `~`, empty clauses, declaration initialization, and pre-existing specific control-flow/integer-constant diagnostics.
- Review/gate: final independent Codex review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,663 local tests, Docker test exit 0, rebuilt runtime output `10`, and diff hygiene pass. No blocker remains.

### 2026-09-16 — TODO 430 contextual missing-control-body diagnostics

- Failure: missing `if`, `while`, `do`, and `for` bodies fell from shared control-body parsing into generic `unexpected token in statement: RBrace` diagnostics.
- Root cause: `parse_control_body_after()` selected blocks specially but otherwise delegated directly to generic statement parsing, so the control context was discarded before an impossible body start was rejected.
- RED/GREEN: a 15-case matrix first failed at `if (1) }`; a shared guard now produces exact context and locations for `RBrace`, EOF, and comma starts for all four statement-bodied controls. `switch` was confirmed to require `{` and already reports its own contextual opening-brace error.
- Review/gate: independent Codex review found one rustfmt-only issue, corrected before the final test gate. Formatting, strict Clippy, all 2,662 local tests, Docker test exit 0, rebuilt runtime output `10`, and diff hygiene pass. No blocker remains.

### 2026-09-16 — TODO 429 dangling-`else` missing-statement diagnostics

- Failure: `int main(void) { if (1) { return 0; } else }` reported generic `unexpected token in statement: RBrace` at the body closing brace.
- Root cause: after consuming `else`, `parse_if()` delegated directly to generic control-body/statement parsing with no context-aware check for a missing or impossible statement start.
- RED/GREEN: exact `RBrace`, EOF, comma, assignment, and division starts first failed against the required `expected statement after else, found <Token>` diagnostics. A narrow post-`else` guard makes them GREEN while valid empty/braced/expression/control bodies and nearest-`if` binding remain GREEN.
- Closure: fresh independent Codex review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,661 local tests, Docker test exit 0 after the foreground wait window, rebuilt runtime output `10`, and diff hygiene pass. No blocker remains.

## Resolved this run

### 2026-09-15 — TODO 428 v0.63.0 publication evidence recovery

- Evidence: local `origin/main` reflog records `fe64b66b4ac5faa3fa824f4d847fbdf1195f7a80` as `update by push` at `2026-09-15T19:24:02+03:00`; the local annotated `v0.63.0` tag object `d997dce6aac3454076bf19e7e66f89167f8cb1ac` has tagger timestamp `2026-09-15T19:24:03+03:00` and peels to that release commit.
- Verification: a fresh remote query reports the same unpeeled tag object and peeled release target; remote main has advanced to later branch commit `f005cff302f6b3d073072d470e40f1dc31cbd438`. This status-only evidence commit is separate from and untagged by v0.63.0; it does not move or recreate the tag. No blocker remains.

### 2026-09-15 — TODO 427 explicit CLI end-of-options delimiter

- Failure: `cust -- -program.c` returned stderr `unknown option '--'`; a bare `cust --` returned the same unknown-option diagnostic instead of the missing-source usage contract.
- Root cause: command-line dispatch treated every top-level dash-prefixed first argument as an option and separately rejected every dash-prefixed mode source operand, with no representation for an intentionally literal operand.
- RED/GREEN: exact subprocess tests first failed in normal, token, AST, and max-step source modes plus bare-delimiter usage. A shared delimiter-aware source-operand helper and a top-level delimiter route make all tests GREEN without changing help/version or undelimited unknown-option dispatch.
- Closure: independent read-only review returned `AI_REVIEW:CLEAR`; formatting, strict Clippy, all 2,660 local tests, Docker test container exit 0 after the foreground observation window, rebuilt runtime output `10`, and diff hygiene pass. No blocker remains.

### 2026-09-15 — TODO 426 bounded v0.63.0 release preparation

- RED/GREEN: exact CLI version and Docker Compose image assertions failed at `0.62.0` and pass after Cargo/lock plus both image tags moved to `0.63.0`.
- Verification: fresh independent read-only release review returned `AI_REVIEW:CLEAR`; formatting, warning-denied Clippy, all 2,658 local tests, all 2,658 Docker tests, rebuilt runtime output `10`, and diff hygiene pass. The first foreground Docker observation reached its tool time window while the image-build test process still ran; no failure was inferred, and the retained process was later observed to exit 0 through the required `docker compose run --rm test` gate.
- Publication guard: local and remote `v0.63.0` preflight refs were absent. The tag remains uncreated until the verified release commit is accepted by `origin/main`. No blocker remains.

### 2026-09-15 — TODO 425 stable CLI help and unknown-option contract

- Failure: `cust --help` was interpreted as a source path and failed with I/O status 66; option-like source positions after `--tokens`, `--ast`, and `--max-steps N` followed the same path.
- Root cause: command-line dispatch recognized only `--version` before treating its first unrecognized argument as a filename, with no shared option-like path guard.
- RED/GREEN: exact subprocess regressions first observed the status-66 behavior, then pass with stdout-only byte-stable help, stderr-only unknown-option diagnostics, and status 64. Existing no-argument usage remained immediate-GREEN preservation coverage.
- Closure: fresh independent Codex review returned `AI_REVIEW:CLEAR` after 42 read-only CLI contract probes. Formatting, strict Clippy, all local tests, Docker tests, rebuilt runtime output `10`, and diff hygiene pass. No blocker remains.

### 2026-09-15 — TODO 423 aggregate-field-array folded classifier parity

- Failure: the expanded matrix reached parser-folded `sizeof` through aggregate-valued conditional, function-return, and assignment-result bases. Conditional aggregate values were accepted only as struct pointers, while calls and assignments had no direct aggregate-result classification, so valid selected output-array fields failed metadata analysis.
- Root cause: `integer_constant_aggregate_field_info()` and function-call type inference omitted `DeclType::Struct` result paths even though runtime aggregate evaluation already supported them.
- RED/GREEN: the focused conditional regression and generated matrix failed before production changes and pass after bounded metadata-only aggregate-value inference. The final matrix runs 1,456 success and 160 exact boundary programs across all pointees/spellings without host addresses or operand evaluation.
- Closure: two independent reviews returned `AI_REVIEW:CLEAR`; review suggestions restored prior routes and strengthened exact/independent test oracles. Formatting, warning-denied Clippy, all 2,653 local and Docker tests, runtime output `10`, compiler oracle, and diff hygiene pass. No blocker remains.

### 2026-09-15 — v0.61.0 publication closure

- Ordering: verified release commit `ff76d129fb4a24bdd05bbfbe1d79516ec490290e` reached exact `origin/main` while local and remote `v0.61.0` refs were absent; the annotated tag was then created and explicitly pushed.
- Verification: local and remote tag object `7bd7eac8d28dda9a6eb76759c1cbad4a2073c8fa` peels exactly to the release commit, while the remote branch still identified that commit at tag verification. This later status-only evidence update is separate from the tagged release commit and does not move or recreate `v0.61.0`; no blocker remains.

### 2026-09-15 — TODO 421 ordinary-first mixed parser-depth host-stack overflow

- Evidence: independent re-review found 44 ordinary grouped expressions followed by `sizeof(int[1])` aborted a 2 MiB Rust thread with SIGABRT before the nested type-name budget activated; the prior 40-group reverse regression missed this boundary.
- Root cause: the ordinary parser retained its historical 128-level ceiling until entering integer/type/array parsing. With larger current parser frames, the host stack could be exhausted before a later route activated mixed-budget checks.
- RED/GREEN: the exact 44-group child-process reproducer failed by SIGABRT. Lowering the ordinary unary/grouping ceiling to 40 makes it return a source-located nesting diagnostic, preserves acceptance at exactly 40 groups, and keeps the hostile CLI operator matrix GREEN. A first 32-level ceiling caused nine existing stress regressions; 40 preserves five unchanged and four adjusted tests retain their original downstream assertions.
- Closure state: fresh independent final review returned `APPROVED`; formatting, warning-denied Clippy, all 2,652 local tests, all 2,652 Docker tests, rebuilt runtime output `10`, and diff hygiene pass. TODO 421 is complete.

### 2026-09-15 — TODO 421 inherited review blockers and resource-safety closure

- Recovery: preserved the inherited tracked scalar-output aggregate-field array diff and promoted every review proof to a focused regression before editing production code.
- Root causes: initializer shape checks occurred after array-only runtime dispatch; embedded aggregate-valued routes were wrapped as pointers in metadata analysis; element-size and whole-array assignment classifiers were incomplete; batched dereferences and integer-constant unary/conditional recursion were not fully charged to deterministic parser depth limits; folded aggregate-literal field metadata omitted `AggregateFieldGet`.
- Closure: braced-shape validation, aggregate-aware route metadata, complete non-evaluating operation classification, bounded ordinary/integer-constant unary and conditional parsing, and folded aggregate-literal metadata make 51 focused interpreter tests, ten pointer-output parity tests, three hostile CLI subprocess tests, and the compiler oracle GREEN. Reviews then reproduced 2 MiB-stack aborts across integer-constant, nested ordinary-operand, and ordinary-first type-query routes plus skipped static aggregate-copy validation. Dedicated RED/GREEN regressions lower integer-constant parsing to 64, use mixed stack-unit accounting, cap ordinary parsing globally at 40, and recurse through pointer-output array fields. The complete 2,495-test interpreter suite is GREEN after preserving meaningful downstream stress assertions within the safe parser ceiling. Fresh independent final review returned `APPROVED`; all 2,652 local and Docker tests pass with runtime output `10`. No external blocker remains.

### 2026-09-14 — v0.60.0 publication closure

- Ordering: verified release commit `de129577da875e2d7a96d973430464e3e6fff7d4` reached exact `origin/main` before the annotated tag was created and explicitly pushed.
- Verification: local and remote tag object `97be4e5d9a7f22aa432a91bd70c82f2d4729151c` peels exactly to the release commit. This later status-only evidence update does not move or recreate `v0.60.0`; no blocker remains.

### 2026-09-14 — Right-associated call timing-gate false positive

- Failure: the first v0.60.0 canonical run stopped before Docker when the 8-term/32-term timing ratio measured 5.40x against a 5x ceiling.
- Investigation: release changes did not touch interpreter production code, and five consecutive focused reruns were stable and GREEN. The same unchanged test produced a one-off 5.90x whole-suite excursion during v0.59.0 preparation, then passed focused and complete reruns.
- Root cause: a 5x limit leaves only 25% headroom over the 4x work increase expected from linear traversal, so parallel whole-suite scheduler contention can inflate one sample enough to fail without a semantic or complexity regression.
- Closure: the ceiling is 8x, consistent with the repository's existing timing policy; this tolerates bounded run noise but still rejects quadratic 16x growth. Focused GREEN, fresh complete-diff `APPROVED` review, and the complete post-edit 2,596-test local/no-cache-rebuilt-Docker gate pass. No active blocker remains.

### 2026-09-14 — TODO 419 tracked scalar-output array review closure

- Failure: inherited final review reproduced pre-guard recursive AST clones, deep scalar compound-literal traversal, and loss of block-static initializer/identity across direct switch jumps; final re-review also caught an evaluated aggregate-index diagnostic regression and a warning-denied `Vec<Box<Stmt>>` lint.
- Root causes: depth checks followed rather than preceded clone/traversal sites; skipped-switch declaration synthesis erased storage-class and enum semantics; and strict non-evaluating index validation was reused before the established evaluated aggregate diagnostic path.
- RED/GREEN: focused regressions first reproduced each semantic or bounded-resource defect. Pre-clone validation, bounded literal traversal, original static/enum analysis and execution, evaluated/non-evaluating index separation, and unboxed synthetic declarations make all focused tests and strict Clippy GREEN.
- Closure: the 135-test tracked-array filter, all 2,596 local tests, Docker test container exit 0, rebuilt runtime output `10`, formatting, strict Clippy, diff hygiene, and fresh independent `AI_REVIEW:CLEAR` pass. No active blocker remains.

### 2026-09-10 — v0.59.0 publication evidence recovery

- Recovery state: the release commit and annotated tag had already reached the remote, while authoritative status queues still described publication as pending.
- Ordering evidence: the `origin/main` reflog records the release commit push at 2026-09-10T15:18:28+03:00, before the tagger timestamp 2026-09-10T15:18:31+03:00.
- Closure: fresh remote checks report `origin/main` and peeled `v0.59.0` at release commit `19e451c2a38644e78bca096bd6c3a158ff3e0d1f`, with annotated tag object `dc40e8523f06a9fb578f965e89ac29b617388d4c`. The evidence update does not move or recreate the tag; no blocker remains.

### 2026-09-10 — v0.59.0 release gate timing retry

- Failure: the first local `cargo test` gate failed only `right_associated_double_call_arithmetic_evaluation_is_bounded` at a 5.90× measured ratio, before Docker execution.
- Investigation: release changes contain no interpreter production code. The exact test then passed five consecutive focused runs with stable total duration, while the next unchanged complete canonical run passed all 2,449 local tests and all 2,449 tests in the no-cache-rebuilt Docker image.
- Classification: the one-off ratio excursion is consistent with transient scheduler contention during a parallel timing sample, but its external cause was not proven. Five focused reruns and both complete canonical reruns reproduced neither the excursion nor any deterministic semantic or scaling regression.
- Closure: no test threshold or production code changed. Successive documentation/reference corrections received fresh independent `APPROVED` review over the complete 17-file release diff. The final canonical rerun passed formatting, strict Clippy, all 2,449 local tests, a no-cache image rebuild, all 2,449 rebuilt-Docker tests, runtime output `10`, local CLI `cust 0.59.0`, image tag `cust:v0.59.0`, and diff hygiene. No active blocker remains.

### 2026-09-10 — TODO 417 tracked scalar-output aggregate fields

- Failure: runtime and parser-folded metadata disagreed across addressed/composed/reverse aggregate bases, field operations, brace-elided pointer subobjects, qualification, output-only operators, and nested initialization; several unselected `_Generic` routes could hide constraints.
- Root causes: aggregate type, const ancestry, pointer-slot qualification, pointee qualification, and actual subscript base did not travel together through every metadata helper, while child traversal was sometimes mistaken for validating the enclosing operation.
- RED/GREEN: all-four-pointee regressions first reproduced each review finding before bounded shared metadata and operation validation were corrected. The final focused filter runs 135 tests and covers valid controls, exact diagnostics, non-evaluation, and deterministic work limits.
- Closure: all 135 focused tests, all eight output-parity tests, the registered compiler oracle, formatting, strict Clippy, all 2,449 local and rebuilt-Docker tests, runtime output `10`, and diff hygiene pass. No active blocker remains.

### 2026-09-08 — Tracked scalar-output aggregate fields review closure

- Task attempted: numbered TODO 417, the first safe tracked `char **` / `int **` / `_Bool **` / `double **` aggregate-field slice.
- What failed: successive reviews found recursive aggregate graph overflow, indexed recursive-const bypass, mixed-pointee union identity, recursive `sizeof(call)` assignment validation, field reverse indexing, host-stack call analysis, lexical equality/void-pointer classification, callee row metadata, const-diagnostic precedence, invalid historical test setup, discarded output updates, and unevaluated static aggregate initializer gaps.
- Root causes: recursive metadata and callee analysis lacked independent bounded states; several non-evaluating routes used runtime rather than lexical metadata; assignment/update validation did not uniformly run for discarded statements; and static output restrictions lived only in runtime initialization.
- Closure: cycle-safe memoized graph inspection, dedicated call/comma bounds, lexical output/array/row/enum facts, recursive const-first validation, explicit union restrictions, shared discarded-expression checks, and metadata-only static initializer validation now keep classifier/evaluator behavior in parity without host addresses or evaluation.
- RED/GREEN: review probes and focused tests first reproduced every defect. Five first-cycle and three second-cycle regressions cover volatile/wrong-pointee/nonzero discarded assignments, increment/decrement/compound updates, enum-zero indexed assignments, equality/void conditionals, row parameters, and direct/nested/array static initializers across all four pointees.
- Verification checkpoint: 41 focused aggregate-field tests, all eight output-parity tests, the compiler oracle, formatting, strict Clippy, diff hygiene, and all 2,355 local tests pass. Final fresh review and canonical Docker verification remain pending before commit/push.

### 2026-09-05 — Complete tracked-output alias parser closure

- Failure: complete aliases such as `typedef IntPtr *IntOutput; IntOutput output` stopped at the pointer-to-pointer typedef boundary even though direct `int **` and inner-alias `IntPtr *` tracked outputs were supported. Independent review also found `_Atomic(IntOutput)` needed an explicit targeted rejection and that an older safety test no longer reached the intended third pointer level.
- Root cause: ordinary pointer-alias metadata represented only one-level pointers, so flattening a complete output alias either lost one level or triggered generic rejection; `_Atomic` and retained-boundary dispatch had no semantic variant for the complete tracked shape.
- RED/GREEN: the complete-alias object/parameter regression first failed at typedef parsing. Dedicated tracked-output alias metadata made it GREEN across all four scalar kinds. The review-driven `_Atomic` type-query regression then failed before explicit `PointerOutput` rejection and passed afterward; the older double boundary now constructs a genuine third-level alias.
- Verification: eight new interpreter regressions, the expanded registered compiler-oracle fixture, fresh final `APPROVED` review, formatting, strict Clippy, all 2,231 local/rebuilt-Docker tests, runtime output `10`, and diff hygiene pass.

### 2026-09-04 — Pointer-output parity false-positive resistance

- Failure: the first independent review found six coverage gaps: no direct `&slot` output-value route, no equality false branch, no null truthiness case, no explicit assignment-result side-effect assertion, a constant `_Generic` control that could not prove non-evaluation, and a lifetime program whose eventual dereference obscured whether earlier `sizeof` or an unrelated call observed the dangling value. A second review found that division-by-zero traps could be translated into the same initializer diagnostic expected by the test.
- Root cause: the initial matrix proved broad cross-type success but several consumers lacked a negative or side-effect witness, and one diagnostic trap shared an error-translation path with the expected result.
- Closure: the final 216-cell matrix adds direct address classification, true/false equality, non-null/null truthiness, assignment-result scratch checks, marker-producing generic controls, and split positive/negative lifetime programs. Diagnostic programs now use an independently witnessed dangling dereference whose error survives pointer-output translation. Focused tests pass and fresh final review returned `AI_REVIEW:CLEAR`.

### 2026-09-04 — Double pointer-output unrelated-call observation

- Failure: independent review showed that a `double **` retaining an unobserved pointer to expired block storage failed at a later unrelated function call, and two persistent outputs could report either expired local depending on `HashMap` iteration order.
- Root cause: a new double-only post-call sweep read every tracked output object in every live scope and static-local entry rather than validating only the ordered pointer-output arguments associated with the call.
- RED/GREEN: `double_pointer_outputs_do_not_observe_expired_values_at_unrelated_call_boundaries` first failed with `pointer to out-of-scope variable 'local'`. Removing the broad sweep made it GREEN while explicit later dereference and output-argument escape regressions continue to reject expired targets.
- Verification: all 87 double-pointer tests, the compiler oracle, fresh independent `APPROVED` re-review, formatting, strict Clippy, all 2,216 local/Docker tests, runtime output `10`, static scan, and diff hygiene pass.

### 2026-09-04 — Boolean pointer-output diagnostic parity

- Failure: inherited `_Bool **` conditional coverage reported `integer pointer output` instead of `boolean`; after that fix, independent review reproduced `character pointer output equality` for aggregate operands beneath `sizeof` and `_Generic`.
- Root cause: two non-evaluating diagnostic branches retained binary `char`/non-`char` assumptions from before tracked pointer outputs carried explicit scalar pointee types.
- RED/GREEN: the inherited conditional test failed before `CType::pointer_output_kind()` replaced the binary label. A dedicated equality regression then failed on `sizeof(output == box)` and passed after the binary output-kind classifier supplied the actual pointee label; the same test covers `_Generic`.
- Verification: nine boolean-output tests, all 82 pointer-output tests, the compiler oracle, fresh independent `APPROVED` re-review, formatting, strict Clippy, all 2,208 local/rebuilt-Docker tests, runtime output `10`, static scan, and diff hygiene pass.

### 2026-09-03 — Integer pointer-output aggregate conditions and null `void *`

- Failure: a conditional RHS with an aggregate condition reached a side-effecting assignment LHS before reporting the invalid scalar condition, while `(void *)0` was rejected for `int **` automatic/static initialization, reassignment, and arguments despite strict GCC/Clang acceptance.
- Root cause: conditional pointer-write prevalidation used general `sizeof_expr()` instead of the scalar-condition validator, and tracked output validation recognized integer null-pointer constants but not the C null-pointer-constant form cast to `void *`.
- RED/GREEN: the ordering regression first reported `division by zero`, and the automatic initializer first rejected `(void *)0`. Shared scalar-condition validation now rejects the aggregate before LHS evaluation; a narrow null-`void *` classifier maps direct `(void *)` casts of integer null constants to tracked null output state and static-constant admission. Const-qualified `void *` initializer and assignment regressions first received generic pointer-object diagnostics; exact qualification errors now survive both translation layers. Five regressions cover automatic, file-global, block-static, reassignment, call, and qualification routes; all 64 pointer-output tests and the compiler oracle pass.
- Closure: final complete-diff re-review and canonical verification are pending.

### 2026-09-03 — Integer null-pointer constant review closure

- Failure: a selected zero arm could hide an incompatible pointer type or a non-constant variable in an unselected logical/conditional arm, while sufficiently deep folded-zero trees reached recursive classification before Cust's iterative nesting guard and overflowed the Rust test thread stack.
- Root cause: one recursive helper combined integer-constant-expression admissibility with short-circuit value evaluation and had no depth budget; pointer-output null conversion consulted its selected value before proving the complete expression's scalar type and constant-expression validity.
- RED/GREEN: focused regressions first returned `Ok(0)` for `0 && variable` / `1 ? 0 : variable`, accepted an incompatible `char *` arm, and reproduced a host stack overflow. Separate bounded validity and value passes now inspect every logical/conditional operand for constant-expression eligibility while evaluating only the selected arm, and scalar-null classification excludes pointer-typed conditionals. The 59-test pointer-output filter is GREEN. The native fixture now compares only same-type `sizeof` expressions, and status/reference text distinguishes supported unqualified scalar-pointee aliases from unsupported pointer typedef aliases.
- Closure: final independent re-review and canonical verification are pending.

### 2026-09-03 — Integer pointer-output parameter-scope escape

- Failure: a nested conditional write through `int **` could store `&value` for a by-value callee parameter and return successfully when the caller did not dereference the dangling result.
- Root cause: the post-call output check ran after the function body scope was gone but before the parameter scope was popped, so `ensure_pointer_value_live()` still considered the parameter target alive.
- RED/GREEN: `integer_pointer_output_writes_reject_nested_escaping_parameter_owners_at_call_boundary` first panicked because interpretation returned `Ok(0)`. Output validation now runs after callee scope teardown and reports `pointer to out-of-scope variable 'value'`; the complete 59-test pointer-output filter remains GREEN.
- Closure: prior complete-diff review and delta review passed before the later null-constant review findings; current final re-review and the 2,183-test canonical gate are pending.

### 2026-09-02 — Pointer classifier/evaluator parity review closure

- Failure: inherited pointer parity tests were GREEN, but independent probes exposed aggregate-array element addresses with missing pointee/const metadata and incompatible pointer assignments/static-local initializers that evaluated `trap()` before rejecting their types.
- Root causes: `AddressOfArray` metadata handled scalar arrays but omitted `Value::StructArray`; evaluated assignment and static-local initialization paths performed runtime pointer evaluation before the metadata-only type conversion check already used by automatic declarations and `sizeof`.
- RED/GREEN: the aggregate-array route first reported `division by zero`, and its const route reported the wrong struct-to-int mismatch. Ordinary, five field-backed assignment routes, and static-local initialization likewise first reported `division by zero`. Struct-array pointee/read-only metadata plus shared pre-evaluation conversion checks now produce exact incompatible-pointer or const-discard diagnostics without running side effects.
- Verification: final independent review returned `APPROVED`; six pointer-parity tests, all 2,137 local/rebuilt-Docker tests, formatting, warning-denied Clippy, runtime output `10`, and diff hygiene pass.

### 2026-09-01 — Aggregate classifier/evaluator route divergence

- Failure: the new exact 88-route property matrix first stopped on a `_Generic` aggregate argument with `expected struct expression`; after that narrow fix, it stopped on a pointer-reached embedded aggregate-array element with the same diagnostic.
- Root causes: `eval_struct_argument()` omitted `Expr::GenericSelection` although aggregate type inference and runtime aggregate evaluation supported it; separately, `aggregate_expr_type_name()` accepted `Expr::StructPtrArrayGet` while `eval_struct_expr()` omitted that variant.
- RED/GREEN: argument classification now accepts `_Generic`, and aggregate evaluation resolves `StructPtrArrayGet` through its existing pointer once before deep-cloning the selected fields. All 22 variants pass declaration, argument, return, and exact mismatch contexts with one-time evaluation and by-value isolation.
- Review closure: direct selected-association recursion initially let an undefined function in an unselected `_Generic` association return `Ok(7)`; routing through `eval_selected_generic()` restores shared semantic validation and nesting-depth handling. A later review showed iteration-position counters could not detect duplicate/omitted route identities; stable enum IDs now index all route, context, and cell totals.

### 2026-09-01 — Carrierless union matrix exposed missing embedded-array aggregate evaluation

- Failure: the new exact property matrix stopped at `union Candidate snapshot = holder.items[0];` with `expected struct expression`; the same focused regression reproduced independently without a bounded-memory call.
- Root cause: `aggregate_expr_type_name()` already classified `Expr::StructArrayGet` as the aggregate element type, but `eval_struct_expr()` had no matching runtime arm. Embedded aggregate-array elements worked as assignment targets and pointer roots but not as by-value initializer expressions.
- RED/GREEN: a focused interpreter test failed first. Runtime aggregate evaluation now resolves the existing `AddressOfStructArrayField` pointer target and deep-clones its field map, preserving one-time index evaluation and by-value isolation. The 90-route model and expanded native fixture then pass.

### 2026-09-01 — Persistent carrierless scalar-union object bytes

- Recovery state: the run inherited uncommitted production, focused-test, and native-fixture changes for the queue-leading carrierless scalar-union package. The original feature RED is unavailable and is not re-claimed.
- Root gap: the all-`_Bool` predicate installed hidden bytes only when every member normalized. Mixed-width layouts such as `union { const int wide; char low; _Bool truth; }` still lacked a non-const full-width language member even though the same hidden maximum-layout storage can preserve their complete deterministic representation.
- Closure: persistent-byte admission now covers every nonempty mutable scalar-only non-`double` layout precisely when no non-boolean mutable full-width carrier exists. Canonical routing selects an actual mutable member and carries its real `CType`; hidden bytes remain authoritative and synchronize every typed view after language or intrinsic writes.
- Verification: two focused carrierless tests, the broader 16-test scalar-union filter, direct GCC/Clang warnings-as-errors execution, and the actual compiler-oracle test pass. Fresh independent review exercised overlap, by-value copy, and const-owner probes and found no blockers. Formatting, strict Clippy, all 2,126 local/Docker tests, runtime output `10`, and diff hygiene pass.

### 2026-08-31 — Current Clippy rejected persistent-byte initialization shape

- Failure: the first v0.46.0 canonical attempt stopped at `clippy::collapsible-if` in `sync_union_scalar_fields_from_active()` before tests or Docker ran.
- Root cause: the previous all-`_Bool` feature commit nested the hidden-storage absence guard around an `if let`; current warning-denied Clippy requires the equivalent Rust 2024 let-chain already used elsewhere in the interpreter.
- RED/GREEN: `cargo clippy -- -D warnings` reproduced the exact warning. Collapsing only those two conditions preserves selection and mutation semantics; strict Clippy and formatting then pass. Fresh independent review returned `APPROVED`, and the complete local/no-cache-rebuilt-Docker canonical gate passes.

### 2026-08-31 — Persistent all-`_Bool` scalar-union object bytes

- Failure: the focused all-`_Bool` union regression stopped at `function 'memcpy' does not yet support union-backed scalar object storage for argument 1` because no normalized `_Bool` member could retain raw byte `2`.
- Root cause: carrier-backed union storage assumes one language-visible scalar can losslessly encode every admitted object byte. `_Bool` canonicalization destroys that representation even though alias identity, capacity, and safety metadata were already available.
- RED/GREEN: all-`_Bool` unions with at least one mutable member now receive hidden interpreter-owned character-byte storage. Initializers and language assignments update those bytes before synchronizing normalized member views; selected and whole bounded-memory reads/writes use the persistent bytes symmetrically. Three focused tests cover all five intrinsics, typed identity, raw bytes, member bounds, const, non-evaluation, zero count, copy isolation, and aggregate-array elements.
- Review closure: independent review exposed stale whole-object and character-view all-`_Bool` rejection tests. Replacing them with current positive behavior and all-const boundaries made the full suite GREEN; fresh current-diff review returned `APPROVED`.
- Verification: formatting, warning-denied Clippy, all 2,124 local tests, all 2,124 rebuilt-Docker tests, runtime output `10`, direct GCC/Clang warning-denied fixture execution, compiler-oracle comparison, and diff hygiene pass.

### 2026-08-31 — Whole-object scalar-union byte ranges

- Failure: whole admitted scalar-union pointers were rejected by whole-struct validation before the existing shared carrier could serve byte operations.
- Root cause: whole-aggregate traversal still modeled every union as sequential struct fields, so maximum-layout capacity, nested writeability, and typed byte-result canonicalization did not share the selected-member carrier semantics.
- RED/GREEN: focused tests first reported `function 'memcpy' does not yet support union-backed whole-struct object storage for argument 1`. Whole-union validation/read/write now delegates to the admitted full-width writable carrier while preserving synchronization and all existing unsupported layouts.
- Review closure: independent review found sequential typed-result lookup, nested member-order-dependent const rejection, stale whole-struct rejection tests, and then a same-type const member selected before the writable carrier. Exact regressions failed before expected-type-aware union lookup, nested admitted-union writeability, boundary-test reconciliation, and mutable-member preference made them GREEN. Fresh re-review returned `APPROVED`.
- Verification: formatting, warning-denied Clippy, all 2,121 local tests, all 2,121 rebuilt-Docker tests, runtime output `10`, direct GCC/Clang strict fixture execution, compiler-oracle comparison, and diff hygiene pass.

### 2026-08-30 — Scalar-only shared union object bytes

- Failure: distinct scalar member pointers compared unequal because identity included the selected field path, and bounded-memory validation rejected every union-backed scalar root.
- Root cause: union initialization/assignment copied one numeric value into independent member slots, while field offsets and pointer equality followed struct-style declaration paths instead of union offset-zero storage identity.
- RED/GREEN: the first focused test returned `1` before the intrinsic call. Canonical union offsets/equality exposed the expected runtime rejection, then scalar-only layout admission and deterministic byte synchronization made low-byte writes visible through every member. Review of the full interpreter target exposed stale rejection assertions and one non-scalar initializer regression; focused fixes retained array/pointer/double/nested boundaries. Independent review then found normalized `_Bool` views hid raw byte `2`, a native padding-byte assumption, and missing array-element/layout-wide veto coverage. A dedicated raw-byte test returned `2` before carrier-backed intrinsic access; array-element/isolation and selected-scalar-with-array-sibling tests plus an unsigned-character zero oracle close those findings. Fresh re-review then exposed const-carrier restoration corruption; a focused negative case first returned `cannot assign through pointer to const`, and admission now requires a non-const carrier before any validation mutation.
- Final review closure: a narrow mutable carrier behind a wider const member could fabricate unavailable upper bytes. The exact regression first returned `Ok(0)`; admission now requires a non-const carrier exactly as wide as the largest member. Const-member, unsupported-layout equality, same-width alias/memmove, and deterministic seed checks were added. Fresh re-review returned `APPROVED`; all 2,117 local/Docker tests and runtime output `10` pass.

### 2026-08-30 — Selected-row short-circuit bounded-memory composition

- Failure: a valid `memset(*row, ...) != *row || memcmp(*row, ...) != 0` expression failed with `two-dimensional array 'combined_values' does not decay to a scalar pointer`, while equivalent nested statements isolated the failure to non-evaluating validation of the combined expression.
- Root cause: runtime double-storage alias analysis called `pointer_value_type()` for `PointerValue::Array2DRow`. That generic type path intentionally rejects scalar-pointer decay, but alias analysis only needed to prove live double-backed row storage.
- RED/GREEN: the focused interpreter regression failed with the exact decay diagnostic before production changes. `current_pointer_has_double_storage()` now validates row-owner liveness and checks the backing element type directly; combined/sequential results, source order, one-time markers, and adjacent-row preservation pass.
- Closure: the registered fixture now uses the combined expression and passes Cust, GCC, Clang, and the compiler-oracle harness. Independent review returned `APPROVED`; formatting, strict Clippy, all 2,111 local/Docker tests, runtime output `10`, and diff hygiene pass.

### 2026-08-30 — Double-row cast, conditional-width, and native-oracle review closure

- Failure: fresh independent review showed that `(int *)rows` inherited double-row metadata beneath `sizeof`, conditional branches with different row widths were accepted in evaluated and non-evaluating contexts, and the native fixture read a `double` after byte-zeroing it.
- Root causes: `array2d_row_pointer_element_type()` propagated row metadata through every non-void pointer cast; conditional validation compared only scalar pointee kinds and discarded `Array2DPointer` column counts; the fixture coupled deterministic Cust bytes to a representation not guaranteed by ISO C.
- RED/GREEN: three focused tests first returned pointer size or success. Row metadata now survives only same-element scalar pointer casts, and conditional validation compares complete row types through `DeclType::Array2DPointer` before evaluation. The fixture compares the zeroed object representation against an unsigned-character byte array and never reads the byte-zeroed double value.
- Full-gate correction: the initial conditional fix eagerly inferred both branches for every ternary and regressed the existing nested `strtol` non-evaluating linearity test with `cannot determine generic selection pointer type`. Guarding complete branch inference behind positive row-shape detection restored that exact regression while both width-mismatch tests remained GREEN.
- Follow-up: combining the row-returning `memset` comparison and byte `memcmp` in one `||` expression exposed a separate composed-expression classifier limitation, so the warning-free oracle uses equivalent source-ordered `if` statements and the combined form is recorded as a concrete post-release regression package.

### 2026-08-30 — Selected double-row bounds, qualification, and address boundaries

- Failure: the inherited selected-row package admitted the expected direct/pointer/parameter/field bounded-memory routes, but independent review showed that `&row[columns]` could expose adjacent backing storage, leading `const` incorrectly froze an adjusted pointer slot, bracket `const` failed to freeze that slot, and an aggregate compound-literal row address returned pointer size instead of retaining the unsupported pointer-to-row boundary.
- Root causes: `checked_pointer_value_index()` validated total backing allocation but omitted row-local width; runtime parameter binding conflated `points_to_const` with `is_const`; direct `Array2D` parameter lowering ignored bracket qualification; and generic address-of-subscript lowering erased `&` into pointer addition before double-row classification.
- RED/GREEN: focused regressions first returned success or the wrong qualification result for `memset`/`memcmp` one-past row addresses, leading-const and bracket-const adjusted parameters, and `&((struct T){...}).rows[0]`. Row-local index validation, separated slot/pointee qualification, bracket-const propagation, and metadata-aware aggregate-literal address lowering restore exact diagnostics. The existing one-dimensional aggregate compound-literal indexed-address fixture remains GREEN.
- Verification: the 27-test two-dimensional-double filter, adjacent aggregate-literal indexed addresses, registered fixture, actual compiler oracle, direct GCC/Clang warnings-as-errors execution, formatting, strict Clippy, all 2,107 local tests, all 2,107 rebuilt-Docker tests, runtime output `10`, and diff hygiene pass.

### 2026-08-29 — Aggregate-field binary64 admission and non-evaluating provenance

- Failure: all five bounded raw-memory intrinsics rejected direct scalar and one-dimensional-array `double` fields; after opening runtime admission, non-evaluating direct/arrow roots still failed. A first metadata shortcut then incorrectly accepted pointer-valued fields targeting 2D rows and union-backed direct array fields beneath `sizeof`.
- Root cause: runtime admission categorized all aggregate-backed array pointers as unsupported and omitted scalar field pointer variants. Non-evaluating validation initially inferred only the resulting `double *` pointee type, losing whether the aggregate field was a direct fixed array, a pointer with current storage provenance, or union-backed storage.
- RED/GREEN: direct scalar, array/nested, and non-evaluating field tests failed before runtime pointer admission and metadata-only field classification were added. Independent review supplied exact pointer-field-to-2D-row and union-array reproducers; both returned `Ok(8)` before `StructFieldType::Array(CType::Double, _)` classification plus union ancestry restored the exact unsupported double-storage diagnostic.
- Boundary evidence: deterministic little-endian binary64 identity and partial writes, field-local capacity/overlap, recursive const, expired owners, whole unsupported layouts, union storage, 2D rows, and runtime/non-evaluating parity now have focused regressions; the registered native fixture uses only ABI-independent relationships.
- Pre-commit review closure: `sizeof(memset(&((struct S){...}).scalar, ...))` and matching direct array-field decay were rejected despite evaluated parity. Two focused tests failed first, then aggregate-literal field metadata admitted only direct scalar/1D-array fields outside union ancestry. The same review found the native fixture read a `double` after all-zero `memset`; removing that typed read retains byte-level `memchr` coverage without assuming a portable all-zero floating representation. Fresh re-review confirmed both blockers closed and found only stale six-versus-eight focused-test wording, corrected before the canonical gate.
- Follow-up review closure: after the wording correction, re-review showed that generic pointer-provenance analysis did not mark direct union compound literals, so `sizeof(memset(((union Choice){{...}}).values, ...))` returned `8`. A dedicated scalar/array union-compound-literal test failed first; direct aggregate type/path ancestry now guards both compound-literal admission branches and restores the exact unsupported double-storage diagnostic.
- Final review closure: `sizeof(memset(&((struct Cell){&value}).value, ...))` returned `8` because `sizeof_expr(AddressOfAggregateField)` returned pointer size without validating the aggregate initializer. A focused test failed first; the address branch now sizes the aggregate expression non-evaluatingly, restoring `cannot assign pointer expression to double value` while preserving side-effect suppression.

### 2026-08-29 — Nested non-evaluating standalone `double` roots

- Failure: the inherited binary64 object-byte implementation passed evaluated standalone scalar/array routes, but `sizeof(memcpy(...))` rejected supported conditional, comma, selected `_Generic`, and assignment-wrapped roots as unsupported double storage.
- Root cause: structural validation correctly detected double storage recursively but its positive supported-root classifier covered only direct literals/variables/casts/addition, so wrapper expressions fell through to the conservative rejection.
- RED/GREEN: the focused regression failed first at the conditional route; conditional, comma, selected `_Generic`, and assignment value were added one vertical step at a time, with the same focused test rerun after each change until all wrappers passed without evaluating side effects. Existing helper-returned and aggregate/two-dimensional/union-backed boundaries remain rejected.

### 2026-08-28 — Two-dimensional double typedef return panic and parameter bypass

- Failure: opening `typedef double Matrix[R][C]` made direct objects work, but alias-spelled array parameters silently adjusted into supported row pointers and alias-spelled function returns reached an internal `unreachable!` panic.
- Root cause: the direct double-parameter guard recognized only explicit bracket syntax, while `parse_function_return_type()` rejected only `DeclType::Array` before calling the return-type lowering helper.
- RED/GREEN: a focused parameter test first returned `0`; a `catch_unwind` return regression reproduced the Rust-host panic. `parse_params()` now rejects `DeclType::Array2D(CType::Double, ...)` at the alias token, and return parsing rejects both one- and two-dimensional array aliases with the existing source-located diagnostic.
- Verification: all three focused alias tests, direct GCC/Clang warnings-as-errors execution, the actual compiler oracle, fresh independent complete-diff re-review, formatting, strict Clippy, all 2,076 local/rebuilt-Docker tests, runtime output `10`, and diff hygiene pass.

### 2026-08-27 — Direct 2D double pointer, const, and resource boundaries

- Failure: the initial slice evaluated row-pointer arithmetic instead of retaining the double pointer-to-row boundary; `sizeof` accepted row addresses and const element increments; source-controlled integer constant arithmetic and huge 2D allocation could panic the host.
- Root cause: the unsupported-double-pointer classifier did not identify dimensioned double roots or row addresses across binary/non-evaluating wrappers, the `sizeof(Increment)` route omitted 2D mutability validation, integer constant folding used unchecked Rust arithmetic, and 2D zero storage used infallible `vec![0; len]` after only checking the element-count product.
- RED/GREEN: focused tests first returned values or panicked under `catch_unwind`. Dimensioned-root/row-address classification, `ensure_two_dimensional_array_mutable()`, checked constant arithmetic, and fallible reserve-before-resize now preserve exact diagnostics. A proposed zero-row overflow reproducer was invalid because non-positive lengths are parser-rejected; a proposed scalar `sizeof(values[0][0] + 1.0)` regression was run and exits `0`.
- Verification: all six focused feature tests, integer-constant and two-dimensional filters, the compiler oracle, formatting, strict Clippy, all local and rebuilt-Docker tests, runtime output `10`, and diff hygiene pass.

### 2026-08-25 — Double-array compound-literal parser guards and review closure

- Failure: direct and typedef-backed one-dimensional double-array compound literals stopped at explicit parser diagnostics even though typed hidden scalar-array storage already supported their runtime and metadata behavior.
- Root cause: two feature-stage rejection guards remained in the direct scalar-array and alias-expanded `DeclType::Array` compound-literal branches.
- RED/GREEN: one direct and one typedef-backed focused test each failed on its own guard before the guard was removed. Review-driven tests then proved typedef const discard/write and expired storage plus evaluated/non-evaluating multidimensional, whole-array/pointer-to-row, union-backed, and raw-memory boundaries.
- Verification: fresh independent review returned `APPROVED`; eight focused tests, the actual compiler oracle, GCC/Clang warnings-as-errors, all local and Docker tests, runtime output `10`, formatting, Clippy, and diff hygiene pass.

### 2026-08-24 — Direct-double typedef alias boundary bypasses

- Failure: alias-spelled pointer-to-row function returns and aggregate fields collapsed to scalar pointers/arrays; `_Atomic(RealPtr)` bypassed the direct-double pointer boundary; `((Row){...})` enabled a forbidden double-array compound literal; and top-level pointer-slot `const` on alias-spelled function returns/casts incorrectly qualified the pointee.
- Root cause: derived declarator parsing reused `decl_type_to_pointee()` before rejecting array aliases followed by `*`, while the atomic and array-compound-literal paths did not reapply direct-double boundaries after alias expansion. Function-return and cast lowering also merged top-level pointer-slot qualification with pointee qualification for an already-pointer `DeclType`.
- RED/GREEN: focused safety and pointer-slot-const regressions reproduced every route. Narrow parser guards retain the existing unsupported shapes, and already-pointer return/cast lowering now preserves only alias pointee qualification. All focused tests are GREEN.
- Verification: the registered native fixture passes GCC/Clang warnings-as-errors and the actual compiler oracle; final independent review returned `APPROVED`; all 2,060 local/rebuilt-Docker tests and runtime output `10` pass.

### 2026-08-24 — Static aggregate-pointer provenance laundering

- Failure: `sizeof(store_then_read(&choice.item)->values)` returned eight instead of rejecting a direct double pointer derived from union-backed aggregate storage when one analyzed helper call stored the pointer in a static local and a later call read it.
- Root cause: branch provenance joining retained a concrete safe `aggregate_target` when the other path carried `None` for a mixed/unknown union-backed target. A later parameter writeback to that concrete safe target replaced the static binding and cleared its union taint.
- RED/GREEN: the inherited strict regression failed with `Ok(8)`. `merge_union_pointer_provenance()` now clears target identity whenever the two branch targets differ, including unknown versus concrete; all 42 union-provenance tests pass.
- Verification: independent complete-diff review returned `APPROVED`; all focused tests, all 2,057 local tests, rebuilt Docker tests, and runtime output `10` pass.

### 2026-08-19 — Direct-double-pointer non-evaluating provenance closure

- Three independent review rounds found one interpreter panic, one existing suite failure, aggregate-provenance losses at function and conditional-pointer boundaries, and control-flow laundering/false positives in the inherited direct `double *` work.
- Dedicated RED regressions reproduced each route. Focused GREEN now covers checked index arithmetic, static scoped aggregate-field typing, pointer-parameter and aggregate-return field facts, explicit callee-write summaries, single- and multi-target mutation effects, guaranteed first `do` execution, returning-branch fallthrough precision, unreachable-return termination, and independently propagated `break`/`continue` states.
- A fourth fresh review found unchecked runtime pointer-index addition, skipped setter/update effects, stale assignment/`_Generic` aggregate targets, aliased-parameter write-order/cache defects, adjusted struct-array element collapse, absent global provenance/effects, fully-returning-switch imprecision, and acceptance of static pointer initializers from automatic storage. Eleven focused RED/GREEN tests close those routes.
- A later blocking review found one-pass `for`/`do-while` alias propagation, argument targets captured after later side effects, reversed global/parameter alias writes, same-named block locals selected as parameter return facts, dynamic aggregate indexes skipped by current-storage validation, and conditional `_Generic` types falling back to absent runtime locals. Six focused tests reproduced those misses before the root fixes.
- The final complete-diff review found that direct struct-pointer compound assignment updated runtime storage identity but not the non-evaluating aggregate target; the same visitor pattern affected pointer increment/decrement. Focused `selected += 1` and `selected++` regressions first returned `Ok(8)`, then both produced the required double-storage diagnostic after target offsets were propagated.
- In the second review-fix cycle, blocker 1 used unsupported syntax and required no edit. Blockers 2–4 went RED respectively with dropped aggregate call-result field provenance (`Ok(8)`), `undefined variable 'marker'` for unary lexical `_Generic`, and a false double-storage diagnostic for recursive `int *`; all three exact tests are GREEN after the narrow root fixes. A prior `StructPtrGet`/`void *` aggregate-field finding was invalid because Cust rejects such fields and supported substitute spellings stop earlier; no fix is claimed. The settled executable filters pass 76 non-evaluating-memory tests, 59 direct-pointer tests, and the one `c_compat` test.

### 2026-08-16 — Reverse aggregate subscript and non-evaluating pointer-field const ancestry

- Failure: fresh recovery review reproduced `i[holders].items[0].values[0]` mutating through `const struct Item *items`; assignment, compound assignment, and increment beneath `sizeof` also returned eight instead of rejecting the write. A later review found `sizeof(holder.items[0].values[0] = 2.0)` also returned eight when the mutable aggregate pointee contained a direct const array field or nested const aggregate ancestor. Native `cc -std=c11 -Wall -Wextra -Werror` rejected both lvalues as read-only.
- Root causes: `pointer_expr_points_to_const()` matched `Expr::StructElementArrayGet` with non-contextual `struct_element_field_metadata(name, fields)`, but reverse syntax stores the aggregate root in `index`; separately, non-evaluating mutability validation returned immediately after proving a pointer field's pointee mutable and skipped the pointee's remaining field path.
- RED/GREEN: expression-aware reverse metadata restored `points_to_const`. A second 12-case matrix covers direct const arrays and nested const ancestors across evaluated/`sizeof` assignment, compound assignment, and increment; resolving the mutable pointer field's aggregate pointee type and validating the remaining path restored exact const diagnostics.
- Current state: fresh complete-diff re-review returned `APPROVED`; focused direct-double, compiler-oracle, formatting, strict Clippy, all local tests, rebuilt Docker tests, runtime output `10`, and the diff check pass.

## Blocker template

```markdown
### YYYY-MM-DD — Short blocker title

- Task attempted:
- What failed:
- Evidence / command output:
- Hypothesis:
- What was tried:
- Needed from user:
- Next safe step:
```

## Rules

- Do not silently skip blockers.
- If Docker tests fail, do not push feature code.
- If GitHub push fails, leave commit local and report the exact auth/permission problem.
- If internet research contradicts current design, document the source in `status/research.md` before changing architecture.
