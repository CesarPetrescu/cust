# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.26.0 publication on 2026-08-09. Focused exact-version tests are GREEN at `0.26.0`; independent complete-diff review returned `APPROVED`; formatting, warning-denied Clippy, all 1,475 local/rebuilt-Docker tests, compiler-oracle comparisons, runtime output `10`, version checks, and safety/diff checks pass. Release commit `9bb4e0222904a5029cfbc5ada389e33c38b34063` is on `origin/main`; local and remote annotated tag object `eeeaeb69967188b558d3211e76b877da5d62d8de` peels exactly to it. No implementation, Docker, Git, research, or environment blocker is currently known.

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
