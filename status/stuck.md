# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-02 bounded v0.12.0 release preparation. Focused CLI and Compose version assertions went RED on `0.11.0` and GREEN on `0.12.0`; local/remote tag preflight is empty; independent review and the complete local/rebuilt-Docker gate pass. No active implementation, Docker, Git, research, or environment blocker exists. `strtol`/`strtoul` remain a documented roadmap dependency on unsupported pointer-to-pointer `endptr`, not an active blocker for the next deterministic C-locale `strcoll`/`strxfrm` package.

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
