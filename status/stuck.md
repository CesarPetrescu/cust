# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-30 v0.6.0 release-preparation run after a clean 1,144-test baseline, exact CLI/Compose version RED/GREEN, synchronized Cargo/lock/Docker/docs/status surfaces, and empty local/remote `v0.6.0` tag preflight. Independent review, the canonical gate, release-commit push, annotated-tag publication, and remote peeled-target verification remain mandatory closure steps rather than blockers.

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
