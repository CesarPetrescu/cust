# Cust TODO

The autonomous agent should complete a meaningful roadmap work package per cron run: usually one full C/interpreter/tooling feature or 2-4 tightly related backlog items. Prefer finishing concrete unchecked items with code/tests over generic improvement or status-only churn.

## Next recommended tasks

1. Continue parser recovery/error-message expansion only for newly discovered malformed programs not already covered by the exact-error suite; acceptance: add focused exact-output parser tests before implementation for nested block brace diagnostics, missing type/identifier diagnostics in declarations/parameters, or other delimiter/list boundary errors that still fall through to generic messages.
2. Add release notes/tags once the current v0.1 surface is judged ready; acceptance: changelog/release notes summarize CLI flags, diagnostics, arrays/strings/pointers, conformance fixtures, and Docker verification.
3. Add a `LICENSE` file after confirming the project-owner's intended license; acceptance: repository root includes the chosen license text and README license section points to it.
4. Add `return;`/void design notes only after deciding whether the v0.1 subset should support `void` functions.

## Every-run checklist

- [ ] Pull latest `main` with `git checkout main && git pull --ff-only`
- [ ] Read all files in `status/`
- [ ] Ideate candidate roadmap-completion tasks from status/backlog/current code
- [ ] Think through impact, safety, dependencies, and testability; choose the best meaningful work package
- [ ] Record good overflow ideas in `status/todo.md` or `status/missing-features.md`
- [ ] Research docs if uncertain
- [ ] Write/update failing tests first for code behavior
- [ ] Create/implement the full selected C/interpreter/tooling work package and close concrete backlog items, not vague improvements
- [ ] Run local verification
- [ ] Run Docker verification
- [ ] Update `status/current-state.md`
- [ ] Update `status/todo.md`
- [ ] Update `status/stuck.md` if blocked
- [ ] Update `status/research.md` with useful links/findings
- [ ] Commit all verified changes, including status-only updates
- [ ] Push to `origin main` before ending the run
