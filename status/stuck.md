# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-31 v0.7.0 release-preparation run: focused version assertions are GREEN, package/CLI/Compose metadata agrees on `0.7.0`, the 1,170-test inventory is reconciled, and local/remote tag preflight is empty. Independent review, the canonical gate, release-commit push, annotated-tag publication, and peeled-target verification are required closure steps rather than blockers.

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
