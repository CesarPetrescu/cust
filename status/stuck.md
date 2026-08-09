# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.26.0 release preparation on 2026-08-09. Focused exact-version tests are GREEN at `0.26.0`; independent complete-diff review returned `APPROVED`; formatting, warning-denied Clippy, all 1,475 local/rebuilt-Docker tests, compiler-oracle comparisons, runtime output `10`, version checks, and safety/diff checks pass. Local/remote tag preflight confirms `v0.26.0` is absent. Release-commit-first publication is the remaining workflow step, not a blocker. No implementation, Docker, Git, research, or environment blocker is currently known.

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
