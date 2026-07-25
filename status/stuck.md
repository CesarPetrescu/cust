# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-25 deterministic five-token multiple-malformed numeric-escape precedence run after 420 unique programs, four pair masks plus three triple masks, two balanced malformed-family rotations, ten balanced cyclic trivia quadruples, three routes, exact first-error source-context and counter checks, 96 passing fuzz-safety tests, 875 passing interpreter tests (982 total), a warning-free full Clippy gate, and both Docker gates. Generic bounded long-chain adjacent-string conformance is the next tooling slice, not an external blocker.

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
