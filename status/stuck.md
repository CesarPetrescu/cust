# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-23 C `_Bool` conversion/storage-normalization run after five focused regressions, canonical compiler-oracle coverage returning 27 under Cust/GCC/Clang, 78 passing fuzz-safety tests, 874 passing interpreter tests, and both Docker gates. Deterministic model-based `_Bool` conversion-context coverage is the next bounded correctness/tooling slice, not an external blocker.

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
