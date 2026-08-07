# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.22.0 publication on 2026-08-07. Exact CLI and Compose assertions went RED against `0.21.0` and GREEN at `0.22.0`; local/rebuilt-Docker version output agrees, the 1,447-test inventory is reconciled, independent re-review returned `APPROVED`, and the canonical local/rebuilt-Docker gate passes. Release commit `86b5a5be1ea73e5cf1ba8eef0c77d9b0cf3f66e9` reached `origin/main` before remote annotated tag object `68fbdd70f8cb7edf1b4212272ba1900ac968f7df`, which peels exactly to that commit. No implementation, Docker, Git, research, or environment blocker is currently known.

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
