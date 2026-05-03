# Cust TODO

The autonomous agent should pick only one small task per cron run unless the task is documentation-only.

## Next recommended tasks

1. Add source location tracking to lexer errors.
2. Add parser error location tracking.
3. Add fixtures under `tests/fixtures/valid` and `tests/fixtures/invalid`.
4. Add block scoping with TDD.
5. Improve local Docker test ergonomics if repeated cron runs expose issues.

## Every-run checklist

- [ ] Pull latest `main` with `git checkout main && git pull --ff-only`
- [ ] Read all files in `status/`
- [ ] Pick one small TODO from `status/missing-features.md` or `status/todo.md`
- [ ] Research docs if uncertain
- [ ] Write/update failing tests first for code behavior
- [ ] Implement minimal change
- [ ] Run local verification
- [ ] Run Docker verification
- [ ] Update `status/current-state.md`
- [ ] Update `status/todo.md`
- [ ] Update `status/stuck.md` if blocked
- [ ] Update `status/research.md` with useful links/findings
- [ ] Commit all verified changes, including status-only updates
- [ ] Push to `origin main` before ending the run
