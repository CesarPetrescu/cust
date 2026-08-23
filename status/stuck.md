# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during bounded v0.34.0 release preparation on 2026-08-23. Version-first CLI and Compose expectations failed against `0.33.0` and pass after package/lock/image metadata moved to `0.34.0`; Cargo metadata and the built CLI agree, the exact 1,977-test inventory reconciles, local/remote `v0.34.0` tag preflight is empty, fresh complete-diff review is `APPROVED`, and the canonical local/no-cache-rebuilt-Docker gate passes. Release-commit-first push, tag publication, and exact remote peeled-target verification remain ordinary required steps rather than blockers.

## Resolved this run

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
