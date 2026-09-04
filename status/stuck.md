# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed 2026-09-04 during bounded v0.52.0 release preparation. Exact CLI/Compose version tests are GREEN at `0.52.0`, the 2,208-test executable inventory is reconciled, README/changelog/status expose the completed `_Bool **` package and exact boundaries, a fresh independent re-review remains pending after correcting premature approval wording, and the canonical local/no-cache-rebuilt-Docker/runtime/version/static-scan/diff gate passes. Repeated local/remote tag preflight is empty. Release-commit-first publication and exact annotated-tag checks are normal remaining acceptance steps, not blockers.

## Resolved this run

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
