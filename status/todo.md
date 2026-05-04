# Cust TODO

The autonomous agent should complete a meaningful work package per cron run: usually one full C/interpreter feature or 2-4 tightly related small tasks. Prefer real code and test expansion over status-only churn.

## Next recommended tasks

1. Add `break` and `continue` for loop/while control flow now that `for` loops are in place.
2. Add empty statements and expression statements so loop bodies and headers can cover more C-compatible statement forms.
3. Add function definitions/calls and local parameters once control-flow coverage is stronger.

## Every-run checklist

- [ ] Pull latest `main` with `git checkout main && git pull --ff-only`
- [ ] Read all files in `status/`
- [ ] Ideate candidate improvements from status/backlog/current code
- [ ] Think through impact, safety, dependencies, and testability; choose the best meaningful work package
- [ ] Record good overflow ideas in `status/todo.md` or `status/missing-features.md`
- [ ] Research docs if uncertain
- [ ] Write/update failing tests first for code behavior
- [ ] Create/implement the full selected C/interpreter work package, not only the smallest possible status/docs change
- [ ] Run local verification
- [ ] Run Docker verification
- [ ] Update `status/current-state.md`
- [ ] Update `status/todo.md`
- [ ] Update `status/stuck.md` if blocked
- [ ] Update `status/research.md` with useful links/findings
- [ ] Commit all verified changes, including status-only updates
- [ ] Push to `origin main` before ending the run
