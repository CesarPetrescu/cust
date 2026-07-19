# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-19 adjusted-aggregate-parameter alias-mutation run after 48 generated two-writer/const-reader cases, direct nested-array decay RED/GREEN, warning-free Cust/GCC/Clang exit 40, 42 passing fuzz-safety tests, 833 passing interpreter tests, and the full required local and Docker gates. Nested named/anonymous/union path expansion is a scoped follow-up, not an external blocker.

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
