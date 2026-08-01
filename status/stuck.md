# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-01 bounded v0.9.0 release closure: exact CLI and Compose expectations went RED against `0.8.0` and GREEN at `0.9.0`; Cargo/lock, Docker, README, changelog, status, and the 1,252-test inventory are synchronized; independent review and the canonical local/Docker gate pass. Release commit `9eaf5d266e5330547c999be6e59061141ef26fbe` reached `origin/main` before the annotated tag, and remote `v0.9.0` peels exactly to it.

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
