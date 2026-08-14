# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during bounded v0.32.0 release preparation on 2026-08-14. Focused version RED/GREEN, exact 1,642-test inventory reconciliation, local/Cargo version probes, and empty local/remote tag preflight pass; independent release review, the canonical gate, release-commit push, and tag publication remain procedural work, not blockers.

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
