# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-18 ordinary named aggregate-array pointer-field subscript/address run after focused RED/GREEN coverage, warning-free Cust/native exit 80, 797 passing interpreter tests, and the full required local and Docker gates. Aggregate-valued consumer-context parity for the newly enabled `StructElementArrayGet`/`Set` routes is a scoped follow-up, not an external blocker.

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
