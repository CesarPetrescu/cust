# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-24 deterministic token-preserving trivia-splice conformance run after 555 unique valid programs with public lexer/parser/runtime panic guards, baseline token-kind/AST/result parity, bounded 1-based lexer locations, exact dimension counters, 84 passing fuzz-safety tests, 875 passing interpreter tests (970 total), a warning-free Clippy gate, and both Docker gates. Whole-program mixed-trivia layout conformance is the next bounded tooling slice, not an external blocker.

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
