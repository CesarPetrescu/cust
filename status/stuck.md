# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.31.0 release publication on 2026-08-13. Version and contextual-diagnostic RED/GREEN, independent approval, executable inventory reconciliation, local/rebuilt-Docker gates, metadata probes, release-commit-first branch acceptance, annotated-tag publication, and exact remote object/peeled verification pass.

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
