# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-21 derived inner const-pointer callee-return run after 144 valid direct/captured root routes, eleven exact const/bounds/cross-root/type/lifetime diagnostics, the scalar embedded-array owner-lifetime RED/GREEN fix, 62 passing fuzz-safety tests, 852 passing interpreter tests, and all required local/Docker gates. Callee-internal promotion/wrapper/offset placement is a scoped follow-up, not an external blocker.

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
