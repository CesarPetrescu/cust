# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-28 bounded variadic-macro closure after targeted review-driven RED/GREEN work, lazy unused-argument prescan with reserved-token validation, bounded pre-rescan substitution, inactive nested-conditional operand suppression, exact empty-invocation arity, warning-free native C11 parity, 1,094 passing local tests, warning-free Clippy, and both canonical Docker gates. Stringification is an actionable next slice, not an external blocker.

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
