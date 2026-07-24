# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-25 deterministic three-token numeric-escape concatenation run after 1,350 unique programs, four fragments in all three positions, 25 independently composed trivia pairs, three routes, 900 exact valid token/location/AST/runtime checks, 450 exact malformed diagnostic/caret checks, 92 passing fuzz-safety tests, 875 passing interpreter tests (978 total), a warning-free full Clippy gate, and both Docker gates. Simultaneous multi-token numeric-escape composition is the next bounded tooling slice, not an external blocker.

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
