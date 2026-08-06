# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.20.0 publication on 2026-08-06. Independent review approved the release diff; all 1,400 local/rebuilt-Docker tests, runtime output `10`, strict formatting/Clippy, version and secret checks, release-commit-first push, and exact remote peeled-tag verification pass. Remote `v0.20.0` peels to `8fec9ef9e0f251808cc16b7f43d560686e88d9a1`. No implementation, Docker, Git, research, or environment blocker is currently known.

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
