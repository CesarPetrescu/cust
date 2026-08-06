# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during bounded v0.20.0 release preparation on 2026-08-06. Exact CLI and Compose version assertions went RED at `0.19.0` and GREEN at `0.20.0`; local and remote tag preflight found no `v0.20.0`, and the independently reconciled inventory remains 1,400 tests. No implementation, Docker, Git, research, or environment blocker is currently known.

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
