# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after the struct-backed bounded character-memory extension on 2026-08-07. Focused aggregate-memory and preserved two-dimensional string-row regressions, the registered compiler oracle, direct GCC/Clang warnings-as-errors fixture checks, a fresh independent `APPROVED` complete-diff review, all 1,460 local/rebuilt-Docker tests, and runtime image output `10` pass. No implementation, Docker, Git, research, or environment blocker is currently known.

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
