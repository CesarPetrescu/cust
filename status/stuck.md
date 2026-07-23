# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-23 fixed-seed `_Bool` conversion-context run after focused RED/GREEN evidence, a warning-free compiler-oracle fixture returning 27 under Cust/GCC/Clang, 81 passing fuzz-safety tests, 875 passing interpreter tests (967 total), and both Docker gates. Deterministic arbitrary-byte/string lexer/parser robustness expansion is the next bounded tooling slice, not an external blocker.

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
