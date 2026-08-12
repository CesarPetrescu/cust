# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.30.0 publication on 2026-08-13. Before this status-only evidence update, release commit `86d0d470fc22825e096a3dbca638dde84ca5800b` was exact local `HEAD` and `origin/main`; remote annotated tag object `3aaf5af050ca4867e7e928e86d2516370aae3b61` peels exactly to it. No implementation, Docker, Git, research, or environment blocker is active.

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
