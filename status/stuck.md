# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-23 distinct-root selected-result carry run after 4,608 balanced scalar/aggregate wrapper/offset/helper/return routes, 17 exact bounds/const/cross-root/type/lifetime checks, a warning-free Cust/GCC/Clang fixture returning 31, 75 passing fuzz-safety tests, and 865 passing interpreter tests. Final copied-parameter reselection of the carried result against the preserved alternate root is a scoped follow-up, not an external blocker.

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
