# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-30 bounded `atoi`/`atol`/`atoll` run after focused RED/GREEN, review-driven pointer-owner and unevaluated-argument fixes, compiler-oracle GREEN, final independent approval, all 1,158 local tests, and both Docker gates passed. Commit and push remain closure steps rather than blockers.

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
