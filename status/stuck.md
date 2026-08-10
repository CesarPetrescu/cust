# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.28.0 publication on 2026-08-10. Focused version RED/GREEN, fresh independent `APPROVED` re-review, formatting, warning-denied Clippy, all 1,534 local/rebuilt-Docker tests, runtime output `10`, local/Cargo/container version `cust 0.28.0`, and the diff check pass. Release commit `02561fb29abd65339d5d01612649636855041955` reached exact `origin/main` before annotated tag object `b12de927c124c163962e43bd5b56b30e46d9557d`; local and remote refs peel exactly to the release commit. No implementation, Docker, Git, research, or environment blocker is active.

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
