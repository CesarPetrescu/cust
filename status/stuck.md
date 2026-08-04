# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after tracked `char **` reassignment closure on 2026-08-04. Final independent review passed without findings; formatting, warning-denied Clippy, all 1,358 local tests, the canonical Docker test gate, runtime output `10`, all 45 focused character-pointer-object tests, the compiler-oracle harness, recursion-depth coverage, and `git diff --check` pass. No implementation, Docker, Git, research, or environment blocker exists.

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
