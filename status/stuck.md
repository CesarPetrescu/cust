# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after the standalone scalar object-byte raw-memory slice on 2026-08-08. Focused RED/GREEN, synchronized implementation/tests/fixture/docs/status, independent `APPROVED` review, formatting, warning-denied Clippy, all 1,470 local/rebuilt-Docker tests, compiler-oracle comparisons, runtime output `10`, and `git diff --check` pass. No implementation, Docker, Git, research, or environment blocker is currently known.

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
