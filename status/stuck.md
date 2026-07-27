# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-27 v0.3.0 release run after 1,039 passing tests, warning-free Clippy, exact local/container `cust 0.3.0` output, and both canonical Docker gates. The fixed two-dimensional scalar-array/pointer-to-row roadmap and release package are complete; object-like macro preprocessing is the next bounded v0.4 task, not an external blocker.

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
