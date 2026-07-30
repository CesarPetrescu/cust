# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-30 first bounded v0.7 standard-library run after a clean 1,144-test baseline, focused `abs`/`labs`/`llabs` RED/GREEN, compiler-oracle GREEN, independent approval, all 1,150 local tests, and both Docker gates passed. The previously reserved annotated `v0.6.0` tag is published and its remote peeled target matches release commit `40f721ad0a88a545443eb991002369d89732912e`. Commit and push remain closure steps rather than blockers.

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
