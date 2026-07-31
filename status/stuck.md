# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-31 bounded `strstr` run: eight focused interpreter tests, adjacent `strpbrk`/string-span regressions, the registered strict native/compiler-oracle fixture, formatting, strict Clippy, all 1,211 local tests, and both Docker Compose gates pass. Independent review found nested `strstr` calls under scalar `sizeof` wrappers bypassed declaration/arity/type validation; focused RED/GREEN generalized the recursive intrinsic validator, and final re-review approved with no blocking/high/medium issue. The runtime image prints `10`.

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
