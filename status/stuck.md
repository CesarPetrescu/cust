# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after the bounded `memset` run on 2026-08-06. Independent review found no blocking correctness or safety issue; its low documentation-drift finding was resolved. All 1,437 local/rebuilt-Docker tests, runtime output `10`, strict formatting/Clippy, nine focused `memset` regressions, the registered warning-free native compiler oracle, and `git diff --check` pass. No implementation, Docker, Git, research, or environment blocker is currently known.

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
