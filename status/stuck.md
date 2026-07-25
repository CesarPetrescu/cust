# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-07-25 deterministic multiple-malformed-token numeric-escape precedence run after 600 unique programs, three pair masks plus one triple mask, two balanced malformed-family rotations, 25 independently composed trivia pairs, three routes, exact first-error source-context and counter checks, 94 passing fuzz-safety tests, 875 passing interpreter tests (980 total), a warning-free full Clippy gate, and both Docker gates. Four-token precedence that can make position 2 the first malformed literal is the next bounded tooling slice, not an external blocker.

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
