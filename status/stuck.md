# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.15.0 release preparation on 2026-08-04. Focused version assertions are GREEN after expected RED against v0.14.0, local/remote tag preflight is empty, independent re-review returned `APPROVED`, and all 1,333 local/rebuilt-Docker tests plus both canonical Docker gates pass. No active implementation, Docker, Git, research, or environment blocker exists; the tag remains reserved for publication only after the verified release commit reaches `origin/main`.

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
