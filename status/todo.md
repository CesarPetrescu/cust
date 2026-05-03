# Cust TODO

The autonomous agent should pick only one small task per cron run unless the task is documentation-only.

## Next recommended tasks

1. Add GitHub Actions CI.
   - Create `.github/workflows/ci.yml`
   - Run `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`
   - Run Docker build/test if possible
2. Add source location tracking to lexer errors.
3. Add parser error location tracking.
4. Add fixtures under `tests/fixtures/valid` and `tests/fixtures/invalid`.
5. Add block scoping with TDD.

## Every-run checklist

- [ ] Pull latest `main`
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
- [ ] Commit and push only if verification passes
