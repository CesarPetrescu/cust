# Cust TODO

The autonomous agent should complete a meaningful work package per cron run: usually one full C/interpreter feature or 2-4 tightly related small tasks. Prefer real code and test expansion over status-only churn.

## Next recommended tasks

1. Continue P1 C-subset expansion with logical operators (`&&`, `||`, `!`) and unary plus, including precedence/short-circuit tests.
2. Add `for` loops or `break`/`continue` after logical/unary operators are in place.

## Every-run checklist

- [ ] Pull latest `main` with `git checkout main && git pull --ff-only`
- [ ] Read all files in `status/`
- [ ] Pick one meaningful work package from `status/missing-features.md` or `status/todo.md`
- [ ] Research docs if uncertain
- [ ] Write/update failing tests first for code behavior
- [ ] Implement the full selected C/interpreter work package, not only the smallest possible status/docs change
- [ ] Run local verification
- [ ] Run Docker verification
- [ ] Update `status/current-state.md`
- [ ] Update `status/todo.md`
- [ ] Update `status/stuck.md` if blocked
- [ ] Update `status/research.md` with useful links/findings
- [ ] Commit all verified changes, including status-only updates
- [ ] Push to `origin main` before ending the run
