# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-26 adjusted fixed two-dimensional scalar-array parameter run after 1,013 passing tests, warning-free Clippy, compiler-oracle exit-code parity, and both canonical Docker gates. Explicit pointer-to-row decay/arithmetic and returned row views are the next roadmap package, not an external blocker.

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
