# Cust TODO

The autonomous agent should complete a meaningful work package per cron run: usually one full C/interpreter feature or 2-4 tightly related small tasks. Prefer real code and test expansion over status-only churn.

## Next recommended tasks

1. Add `char` literal/type support as the next P2 data-type expansion now that core integer functions/control flow and first-pass parser diagnostics are stable.
2. Continue improving parser recovery/error messages for additional malformed programs (for example trailing commas in function parameter lists, missing semicolons after declarations/assignments, and unmatched delimiters).
3. Add `return;`/void design notes only after deciding whether the v0.1 subset should support `void` functions.
4. Add CLI flags such as `--version`, `--ast`, `--tokens`, or `--max-steps` after the core C-subset expansion stabilizes.

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
