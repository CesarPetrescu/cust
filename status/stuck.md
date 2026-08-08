# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during bounded v0.23.0 release preparation on 2026-08-08. Focused version RED/GREEN, exact metadata and executable-inventory reconciliation, final independent `APPROVED` review, formatting, warning-denied Clippy, all 1,460 local/rebuilt-Docker tests, runtime image output `10`, container version `cust 0.23.0`, and `git diff --check` pass. No implementation, Docker, Git, research, or environment blocker is currently known.

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
