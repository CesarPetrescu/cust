# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-03 safe two-level character-pointer slice. Focused RED/GREEN, three rounds of independent read-only review with all findings resolved, formatting, strict Clippy, all 1,295 local tests, the rebuilt Docker test gate, runtime output `10`, and `git diff --check` pass. No active implementation, Docker, Git, research, or environment blocker exists. The completed `char **endptr` foundation unblocks bounded `strtol`/`strtoul` as the next roadmap package.

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
