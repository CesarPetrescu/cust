# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-19 nested adjusted-aggregate-parameter alias-modeling run after 48 generated two-writer/const-reader cases across five outer-array path families, 23 targeted diagnostic checks, warning-free Cust/GCC/Clang exit 32, 43 passing fuzz-safety tests, 834 passing interpreter tests, and the full required local and Docker gates. Aggregate-compound-literal outer-array arguments are a scoped follow-up, not an external blocker.

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
