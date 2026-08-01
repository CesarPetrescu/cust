# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-01 bounded `strncat` run: six focused interpreter tests, the registered strict native/compiler-oracle fixture, formatting, strict Clippy, all 1,235 local tests, and both Docker Compose gates pass. Runtime count-shape diagnostics received focused RED/GREEN coverage; independent review found one blocking early-source-NUL test gap and no implementation defect, the added exact-capacity regression passes, and final re-review reported no blocking findings. The runtime image prints `10`.

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
