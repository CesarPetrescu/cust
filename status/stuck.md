# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-25 bounded six-token adjacent-string conformance run after 420 unique programs, exact valid token/AST/runtime NUL checks, every first-error position 0 through 4, balanced five-boundary trivia/family/route checks, 97 passing fuzz-safety tests, 875 passing interpreter tests (983 total), a warning-free full Clippy gate, and both Docker gates. The v0.2 release-metadata package is next, not an external blocker.

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
