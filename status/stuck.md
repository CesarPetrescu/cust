# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed after the bounded `memcmp` run on 2026-08-06. Independent review returned `APPROVED` with no blocking or material findings. All 1,428 local/rebuilt-Docker tests, runtime output `10`, strict formatting/Clippy, focused `memcmp` regressions, the registered warning-free native compiler oracle, and `git diff --check` pass. The first Docker build exposed a pre-existing linearity-test threshold false positive; the depth-8/depth-40 detector now allows expected 5x linear work while retaining an 8x ceiling, passed 20 focused repetitions, and passed the rebuilt Docker gate. No implementation, Docker, Git, research, or environment blocker is currently known.

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
