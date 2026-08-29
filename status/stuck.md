# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed 2026-08-29 after aggregate-field binary64 object-byte implementation and review closure. Focused RED/GREEN passes direct/arrow/nested/struct-array-element routes and exact 2D-row/union/non-evaluating boundaries; bounded v0.41.0 release closure is the one next package.

## Resolved this run

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
