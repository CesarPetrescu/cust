# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-29 bounded C11 `#line` closure after focused RED/GREEN, review-driven remapped-name hardening, exact operand/location/include-state coverage, official/native semantics probes, independent review, 1,132 passing local tests, warning-free Clippy, and both canonical Docker gates. The v0.5.0 release package is actionable next work, not an external blocker.

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
