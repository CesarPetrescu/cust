# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-24 deterministic whole-program mixed-trivia layout conformance run after 20 unique valid programs, 555 balanced trivia assignments, public lexer/parser/runtime panic guards, baseline token-kind/AST/result parity, exact character-based token-plus-EOF locations, exact dimension counters, 85 passing fuzz-safety tests, 875 passing interpreter tests (971 total), a warning-free Clippy gate, and both Docker gates. Composed-trivia-run conformance is the next bounded tooling slice, not an external blocker.

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
