# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after completing ordinary tracked unqualified `char **` objects on 2026-08-03. Focused review-driven RED/GREEN, final independent `APPROVED`, formatting, strict Clippy, all 1,333 local tests, the rebuilt Docker test gate, runtime output `10`, compiler-oracle coverage, recursion-depth regression, security scan, and `git diff --check` pass. No active implementation, Docker, Git, research, or environment blocker exists.

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
