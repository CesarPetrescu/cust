# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-24 deterministic malformed composed-comment-run robustness run after eight mutation families, both LF/CRLF endings, 384 unique sources, 48 exact unterminated-comment caret checks, bounded parser locations, exact dimension counters, 87 passing fuzz-safety tests, 875 passing interpreter tests (973 total), a warning-free full Clippy gate, and both Docker gates. Comment-delimiter/literal-boundary robustness is the next bounded tooling slice, not an external blocker.

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
