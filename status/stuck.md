# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-18 embedded aggregate-array element field increment/decrement run after clean startup inspection, focused direct/reverse RED/GREEN tests, one-time index/pointer evaluation coverage, exact const/bounds/type diagnostics, non-evaluating `sizeof` markers, warning-free Cust/GCC/Clang exit 14, 749 passing interpreter tests, and the full required local and Docker gates. The full Clang `c_compat` sweep's pre-existing `-Wconstant-logical-operand` warning is documented but does not affect the canonical/default-compiler gate or the new fixture. Embedded aggregate-array element field replacement/compound-assignment parity is the next concrete backlog item.

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
