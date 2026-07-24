# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-24 deterministic composed-trivia-run conformance run after all 64 ordered trivia pairs, eight selected triples, 1,760 unique programs, 888 whole-layout boundary assignments, public lexer/parser/runtime panic guards, baseline token-kind/AST/result parity, exact character-based token-plus-EOF locations, exact dimension counters, 86 passing fuzz-safety tests, 875 passing interpreter tests (972 total), a warning-free full Clippy gate, and both Docker gates. Malformed composed-comment-run robustness is the next bounded tooling slice, not an external blocker.

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
