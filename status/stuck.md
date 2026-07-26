# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-26 row-pointer function-return/generic-read run after 1,024 passing tests, warning-free Clippy, compiler-oracle exit-code parity, and both canonical Docker gates. Pointer-to-row typedef/declaration-list forms, generic row-expression lvalue updates, and non-variable 2D roots are roadmap work, not external blockers.

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
