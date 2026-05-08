# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-05-09 autonomous struct-array field decay/address-of parity run after focused interpreter coverage, the C compiler-oracle suite, and full local/Docker verification passed; full local and Docker verification is recorded in `status/current-state.md`. Docker Compose emitted a non-fatal missing-buildx warning but exited 0 for both required Docker commands.

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
