# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-21 direct aggregate-array compound-literal derived inner-pointer qualification-transition run after 108 generated identity/read cases across every pointee-kind/promotion-stage/wrapper/offset/helper-depth route, 12 exact mutable-rebinding/write diagnostics, warning-free Cust/GCC/Clang exit 21, 59 passing fuzz-safety tests, 850 passing interpreter tests, and all required local/Docker gates. Post-promotion const re-forwarding is a scoped follow-up, not an external blocker.

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
