# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-24 deterministic numeric-escape adjacent-string concatenation run after 240 unique programs, four fragments on both boundary sides, four trivia forms, three declaration/expression routes, 192 exact valid token/location/AST/runtime checks, 48 exact malformed diagnostic/caret checks, 91 passing fuzz-safety tests, 875 passing interpreter tests (977 total), a warning-free full Clippy gate, and both Docker gates. Three-token numeric-escape concatenation is the next bounded tooling slice, not an external blocker.

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
