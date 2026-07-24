# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-24 deterministic composed literal-fragment-run robustness run after 320 unique string/character sources, 25 ordered pairs, five selected triples, LF/CRLF, four expression/initializer boundaries, 280 exact valid value/token/location checks, 40 exact malformed opening-quote caret checks, 89 passing fuzz-safety tests, 875 passing interpreter tests (975 total), a warning-free full Clippy gate, and both Docker gates. Numeric-escape/literal-fragment robustness is the next bounded tooling slice, not an external blocker.

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
