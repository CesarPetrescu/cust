# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-19 deterministic adjusted-aggregate-parameter identity-model run after 60 generated model cases, 14 targeted diagnostic checks, warning-free Cust/GCC/Clang exit 35, 41 passing fuzz-safety tests, 830 passing interpreter tests, and the full required local and Docker gates. Ordered two-writer/const-reader model expansion is a scoped follow-up, not an external blocker.

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
