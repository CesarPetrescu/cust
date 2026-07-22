# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-22 const-parameter return-selection conformance run after 32 direct/captured scalar/aggregate routes, seven exact const-write/cross-root/type/lifetime diagnostics, a warning-free Cust/GCC/Clang fixture returning 24, 69 passing fuzz-safety tests, 859 passing interpreter tests, and all required local/Docker gates. Nested parameter-return selection is a scoped follow-up, not an external blocker.

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
