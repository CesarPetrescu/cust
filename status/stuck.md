# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.32.0 publication on 2026-08-14. Focused version RED/GREEN, exact 1,642-test inventory reconciliation, independent release approval, the full local/rebuilt-Docker gate, runtime output `10`, and local/Cargo/container version probes pass. Release commit `97442a8ed589f167abc7691d190de574154267e0` reached exact `origin/main` before annotated tag object `04358d2ca26243b88e717c45234639fe031c57a5`; local and remote refs peel exactly to that commit. Direct one-dimensional double arrays in supported aggregate fields are next, with no active blocker.

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
