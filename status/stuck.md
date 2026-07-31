# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-31 bounded `strpbrk` run: six focused interpreter tests, the registered compiler-oracle fixture, strict formatting/Clippy, all 1,196 local tests, and both Docker Compose gates pass. Independent review found and drove focused fixes for two nested non-evaluating type-query constraint gaps, then approved the final code/test diff with no blocking/high/medium issue. The runtime image prints `10`.

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
