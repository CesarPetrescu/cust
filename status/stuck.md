# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None.

## Resolved this run

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
