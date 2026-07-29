# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-07-30 v0.5.0 release preparation after clean-baseline verification, focused exact-version RED/GREEN, non-conflicting local/remote tag preflight, synchronized Cargo/lock/CLI/Docker/docs/status metadata, independent review, 1,132 passing local tests, warning-free Clippy, and both canonical Docker gates. Annotated-tag publication remains an ordered post-`origin/main` step, not an external blocker.

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
