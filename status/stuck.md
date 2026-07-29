# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-29 predefined `__FILE__`/`__LINE__` closure after focused RED/GREEN, exact direct/forwarded/stringified/conditional and primary/header logical-source coverage, exact reserved-name diagnostics, warning-free native parity, independent review, 1,121 passing local tests, warning-free Clippy, and both canonical Docker gates. Bounded active `#error` diagnostics are actionable next work, not an external blocker.

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
