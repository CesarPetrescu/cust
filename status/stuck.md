# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.25.0 release preparation on 2026-08-08. Exact version tests went RED against `0.24.0` and GREEN at `0.25.0`; package/lock, CLI, Docker, README/changelog/status, and the 1,470-test inventory are synchronized; fresh complete-diff re-review returned `APPROVED`; formatting, warning-denied Clippy, all local/rebuilt-Docker tests, runtime/version output, safety and diff checks, and local/remote tag preflight pass. Release-commit-first publication remains pending by design, not blocked. No implementation, Docker, Git, research, or environment blocker is currently known.

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
