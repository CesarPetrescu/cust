# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-18 reverse-subscript lvalue-result `sizeof` run after clean startup inspection, four focused RED/GREEN regressions, eleven assignment/compound/increment type relationships, pointer/index/RHS non-evaluation markers, warning-free GCC exit 13, 744 passing interpreter tests, and the full required local and Docker gates. Clang's `-Wunevaluated-expression` under `-Werror` is a documented native-oracle boundary, not a Cust blocker. Embedded aggregate-array element field increment/decrement parity is the next concrete backlog item.

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
