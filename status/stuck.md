# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after bounded v0.19.0 publication on 2026-08-05. Independent read-only review returned `APPROVED`; focused version RED/GREEN, all 1,394 local/rebuilt-Docker tests, runtime output `10`, and local/Cargo/container version `cust 0.19.0` pass. Release commit `19e8117688d2ee21323bab4705f9de8fd974b1e2` reached `origin/main` before annotated tag publication, and remote `v0.19.0` peels exactly to it. No implementation, Docker, Git, research, or environment blocker is currently known.

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
