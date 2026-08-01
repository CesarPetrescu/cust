# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-02 bounded v0.10.0 release preparation: exact CLI and Compose expectations went RED against `0.9.0` and GREEN at `0.10.0`; Cargo/lock, Docker, README, changelog, status, and the 1,255-test inventory are synchronized. The non-conflicting `v0.10.0` tag remains reserved until the release diff is independently approved, the canonical local/Docker gate passes, and the verified commit is accepted by `origin/main`; no publication is claimed yet.

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
