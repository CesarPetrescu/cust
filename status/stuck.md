# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-23 freshly composed distinct-root result reselection run after 9,216 balanced scalar/aggregate final selector routes, 17 exact bounds/const/cross-root/type/lifetime checks, a warning-free Cust/GCC/Clang fixture returning 63, 78 passing fuzz-safety tests, and 866 passing interpreter tests. The explicit pointer-provenance queue is complete; C99 `__func__` is the next bounded language slice, not an external blocker.

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
