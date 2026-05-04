# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-05-05 autonomous run after local and Docker verification passed for array-element pointers (`&values[index]`), direct/grouped dereference reads and writes (`*(&values[1])`), array-element pointers passed to `int *`/`char *` parameters, relative pointer indexing diagnostics, and compiler-oracle pointer compatibility fixtures. Docker Compose emitted a non-fatal missing-buildx warning but exited 0 for both required Docker commands.

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
