# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after the two-dimensional non-character scalar-row object-byte feature on 2026-08-09. Focused direct/route/safety/non-evaluating tests and the actual compiler-oracle harness are GREEN; independent review returned `APPROVED`; formatting, warning-denied Clippy, all 1,479 local/rebuilt-Docker tests, runtime output `10`, and diff checks pass. No implementation, Docker, Git, research, or environment blocker is currently known.

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
