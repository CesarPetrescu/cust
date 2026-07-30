# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-30 C11 null-directive run after focused RED/GREEN, strict native parity, independent review, 1,135 passing local tests, warning-free Clippy, and both canonical Docker gates. The annotated `v0.5.0` tag is published and its remote peeled target matches release commit `4a7ec79486b4c86ecdfea2524aa3e5c854ceba70`.

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
