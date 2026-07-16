# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-16 field-backed pointer-return/forwarding model run after clean startup inspection, 128 fixed-seed generated pointer expressions, exact cross-field/root/const/type/bounds diagnostic regressions, warning-free GCC/Clang C11 execution, focused Rust verification, and pre-gate formatting/clippy checks. The full local and Docker gates passed before commit.

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
