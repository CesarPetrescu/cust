# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during bounded v0.24.0 release preparation on 2026-08-08. Focused version RED/GREEN, empty local/remote tag preflight, synchronized release metadata/docs/status, independent `APPROVED` review, formatting, warning-denied Clippy, all 1,463 local/rebuilt-Docker tests, runtime output `10`, exact local/Cargo/container version checks, the static security scan, and `git diff --check` pass. No implementation, Docker, Git, research, tag, or environment blocker is currently known; publication is intentionally waiting for release-commit acceptance on `origin/main`.

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
