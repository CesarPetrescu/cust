# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.27.0 release preparation on 2026-08-09. Release RED/GREEN, fresh independent complete-diff review, formatting, warning-denied Clippy, all 1,479 local/rebuilt-Docker tests, doc tests, focused version/license/compiler-oracle checks, runtime output `10`, local/Cargo/container version `cust 0.27.0`, empty local/remote tag preflight, and diff checks pass. No implementation, Docker, Git, research, or environment blocker is currently known; publication remains ordered behind exact `origin/main` acceptance of the release commit.

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
