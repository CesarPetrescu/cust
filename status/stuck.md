# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after completion of one-level `void *` function return/prototype/call-result support on 2026-08-06. Review-driven focused RED/GREEN, all 1,400 local/rebuilt-Docker tests, runtime output `10`, strict formatting/Clippy, and `git diff --check` pass. The initial widening suggestion for incompatible erased storage was rejected against Cust's existing documented safe compatible-referent boundary; policy-aware final independent re-review returned `APPROVED`. No implementation, Docker, Git, research, or environment blocker is currently known.

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
