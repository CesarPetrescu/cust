# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-02 deterministic `rand`/`srand` completion. Focused RED/GREEN, final independent review, formatting, warning-denied Clippy, all 1,259 local tests, rebuilt Docker test execution, runtime output `10`, and `git diff --check` pass. Published `v0.10.0` remains verified at peeled commit `366f24934b6ed4a6b12f97d46069c4a48da9cd42`; no active implementation, Docker, Git, or research blocker exists.

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
