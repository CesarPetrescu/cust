# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-17 relational-ordering classification model run after clean startup inspection, 96 fixed-seed generated cases, a RED/GREEN mixed scalar/pointer ordering fix, exact same-root/cross-root/null/mixed diagnostics, side-effect counts, warning-free GCC/Clang execution, focused Rust verification, and the full required local and Docker gates.

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
