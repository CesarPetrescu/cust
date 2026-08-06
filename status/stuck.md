# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.21.0 release preparation on 2026-08-06. Final independent read-only review returned `APPROVED`; local/remote tag preflight is empty; focused version RED/GREEN, formatting, warning-denied Clippy, all 1,437 local tests, the rebuilt 1,437-test Docker gate, runtime output `10`, local/Cargo/container version `cust 0.21.0`, secret scan, and `git diff --check` pass. Release-commit-first push/tag publication and remote peeled-target verification remain workflow closure steps, not blockers. No implementation, Docker, Git, research, or environment blocker is currently known.

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
