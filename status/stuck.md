# Cust Stuck Situations

Use this file to log blockers that need user input or deeper research.

## Active blockers

None. Last reviewed during the 2026-08-01 bounded C11 character-classification slice: focused runtime, exact declaration, arity/shape/value, EOF/unsigned-character-domain, nested non-evaluating, and native-oracle coverage passes; independent final review found no blocking security, logic, C-semantics, precedence, or dispatch errors. The prior v0.8.0 release and annotated tag remain published and remotely verified.

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
