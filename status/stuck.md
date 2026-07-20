# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-20 wrapped nonzero-offset direct aggregate-array literal adjusted-parameter run after 54 generated two-writer/const-reader cases, nine const-typedef wrapper combinations, 13 targeted diagnostics, warning-free Cust/GCC/Clang exit 37, 47 passing fuzz-safety tests, 838 passing interpreter tests, and the full required local and Docker gates. Applying the offset matrix to captured aggregate-compound-literal array fields is a scoped follow-up, not an external blocker.

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
