# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-18 embedded aggregate-array element field assignment run after clean startup inspection, four focused RED/GREEN cycles, independent review plus aggregate-pointer-field const-pointee/const-slot regression coverage, one-time direct-index/reverse-pointer/RHS evaluation coverage, exact const/bounds/type diagnostics, non-evaluating `sizeof` markers, warning-free Cust/GCC/Clang exit 16, 757 passing interpreter tests, and the full required local and Docker gates. Pointer-valued field assignment through embedded aggregate-array elements is the next concrete backlog item; its reproduced `pointer value used as scalar` failure is a scoped implementation gap, not an external blocker.

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
