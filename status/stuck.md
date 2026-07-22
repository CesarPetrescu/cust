# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-22 post-selection wrapper/offset conformance run after 4,608 scalar/aggregate direct/captured routes, seven exact bounds/cross-root/type/lifetime diagnostics, two const-write diagnostics, a warning-free Cust/GCC/Clang fixture returning 40, 71 passing fuzz-safety tests, and 861 passing interpreter tests. Re-forwarding the composed pointer through another helper/return boundary is a scoped follow-up, not an external blocker.

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
