# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-16 pointer-return function boundary model run after clean startup inspection, fixed-seed generated model verification, warning-free GCC/Clang C11 execution, the full local Rust gate, and both required Docker commands.

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
