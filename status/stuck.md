# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after completion of the bounded first `double` runtime-value slice on 2026-08-12. Focused review-driven RED/GREEN covers the double interpreter slice, generic-selection and nested-call arithmetic linearity regressions, valid unselected-double generic associations, literal-wrapper non-evaluating validation, and double-address assignment/call/return/cast/conditional/ordering boundaries; executable inventory is 1,592 tests. No implementation, environment, or research blocker is active; bounded v0.29.0 release closure is next.

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
