# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-28 bounded quoted-header closure after targeted RED/GREEN work, independent security/correctness review, Linux `openat2` containment-before-open and non-regular-file fixes, warning-free native C11 parity, 1,089 passing local tests, warning-free Clippy, and both canonical Docker gates. The next preprocessing slice is a roadmap choice, not an external blocker.

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
