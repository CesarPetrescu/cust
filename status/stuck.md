# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-24 deterministic comment-delimiter/literal-boundary robustness run after 160 unique string/character sources, ten delimiter/escape/multibyte/malformed mutations, LF/CRLF, four expression/initializer boundaries, 104 exact valid token/location checks, 56 exact malformed opening-quote caret checks, 88 passing fuzz-safety tests, 875 passing interpreter tests (974 total), a warning-free full Clippy gate, and both Docker gates. Composed literal-fragment-run robustness is the next bounded tooling slice, not an external blocker.

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
