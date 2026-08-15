# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

### 2026-08-15 — Aggregate `double` array review closure exhausted

- Task attempted: finish the inherited direct one-dimensional `double` aggregate-field package.
- What failed: after the allowed review/fix/reverify cycles, final independent review found a remaining const-integrity bypass, so the package is not safe to commit or push.
- Evidence: with either `const struct Item items[1]` or `const struct Item *items`, `(1 ? index[items].values : index[items].values)[0] = 3.0`, `(0, index[items].values)[0] += 2.0`, and `_Generic(0, int: index[items].values, default: index[items].values)[0]++` currently succeed. Their `sizeof(...)` forms return `8` instead of rejecting the mutation. Independent probes confirmed all six direct const-root mutations and the earlier reverse pointer-boundary/non-evaluating scalar defects are now closed; the wrapper-specific qualification loss is distinct.
- What was tried: focused RED/GREEN added direct reverse const-root, reverse address/pointer escape, wrapped reverse read, and non-evaluating condition/unary/cast regressions; all 83 `direct_double_` tests, four focused reverse tests, the focused scalar-operator test, and the actual compiler oracle are GREEN. Review traced the remaining gap to contextual reverse `StructElementGet` qualification in `pointer_expr_points_to_const` and the evaluated/non-evaluating wrapped-base mutability traversal.
- Next safe step: add an evaluated plus `sizeof` matrix for conditional/comma/`_Generic` reverse bases across assignment/compound/increment and const-array/pointer-to-const roots; preserve the contextual owner/index through shared qualification and mutability helpers; then obtain fresh independent approval before the canonical gate.

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
