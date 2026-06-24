# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed in the 2026-06-24 autonomous inline enum aggregate field run after baseline `cargo test`, focused RED/GREEN interpreter coverage, C compiler-oracle coverage, full local verification, and required Docker verification passed. Docker Compose emitted non-fatal missing-buildx warnings but exited 0 for both required Docker commands.

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
