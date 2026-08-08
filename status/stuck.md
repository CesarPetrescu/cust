# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.24.0 publication on 2026-08-08. Focused version RED/GREEN, synchronized release metadata/docs/status, independent `APPROVED` review, formatting, warning-denied Clippy, all 1,463 local/rebuilt-Docker tests, runtime output `10`, exact local/Cargo/container version checks, the static security scan, `git diff --check`, release-commit-first branch publication, and exact local/remote annotated-tag peeled-target verification pass. No implementation, Docker, Git, research, tag, or environment blocker is currently known.

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
