# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-02 bounded `strcoll`/`strxfrm` implementation. Focused RED/GREEN, compiler-oracle parity, independent review, formatting, strict Clippy, all 1,279 local tests, the rebuilt Docker test gate, runtime output `10`, and `git diff --check` pass. No active implementation, Docker, Git, research, or environment blocker exists. `strtol`/`strtoul` remain a documented roadmap dependency on unsupported pointer-to-pointer `endptr`, not an active blocker for v0.13.0 release closure.

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
