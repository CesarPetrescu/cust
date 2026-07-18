# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-18 embedded aggregate-array element pointer-field update run after focused RED/GREEN coverage, warning-free Cust/native exit 9, 769 passing interpreter tests, and the full required local and Docker gates. The newly minimized ordinary `nodes[i].cursor` pointer-field read gap is a scoped interpreter backlog item, not an external blocker.

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
