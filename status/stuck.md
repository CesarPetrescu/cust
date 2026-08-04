# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.17.0 release verification on 2026-08-04. Exact CLI and Compose version assertions went RED against `0.16.0` and GREEN at `0.17.0`; local/remote tag preflight is empty; independent re-review returned `APPROVED`; formatting, strict Clippy, all 1,358 local/rebuilt-Docker tests, runtime output `10`, local/Cargo/container version `cust 0.17.0`, and `git diff --check` pass. No implementation, Docker, Git, research, or environment blocker is currently known.

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
