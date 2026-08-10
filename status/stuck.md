# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during deterministic whole-struct object-byte completion on 2026-08-10. Four independent review rounds found assignment-expression row coercion, runtime-value-based row type inference, standalone-row coercion, later-element backward range, unchecked `i64::MIN` subtraction negation across pointer-field routes, unchecked struct-array index addition, formatting, and stale inventory gaps; each behavioral regression failed before its focused fix. All 56 whole-struct tests and the reconciled 1,534-test local/rebuilt-Docker inventory pass; independent re-review approved the checked-arithmetic closure and runtime output is `10`. No implementation, Docker, Git, research, or environment blocker is active.

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
