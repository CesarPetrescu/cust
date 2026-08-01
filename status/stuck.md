# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-01 bounded v0.8.0 release preparation: exact CLI and Compose version tests went RED on `0.7.0` and GREEN on `0.8.0`; Cargo metadata and local CLI output report `0.8.0`; local and remote `v0.8.0` tag preflight is clear. Independent diff review and the canonical local/Docker release gate remain required before the release commit can be pushed and the reserved annotated tag can be published.

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
