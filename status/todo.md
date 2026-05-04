# Cust TODO

The autonomous agent should complete a meaningful roadmap work package per cron run: usually one full C/interpreter/tooling feature or 2-4 tightly related backlog items. Prefer finishing concrete unchecked items with code/tests over generic improvement or status-only churn.

## Next recommended tasks

1. Add `&array[index]` array-element pointers and dereference assignment/read support; acceptance: `*(&values[1])` reads/writes exactly that element, array-element pointers can be passed to `int *`/`char *` parameters where C-compatible, out-of-bounds diagnostics match the pointer model, and fixtures include optional compiler-oracle comparisons where supported.
2. Add fuzz/property tests for lexer/parser safety; acceptance: generate random byte/string inputs, assert lexing/parsing never panics, and keep the test deterministic enough for Docker cron verification.
3. Continue parser recovery/error-message expansion only for newly discovered malformed programs not already covered by the exact-error suite (remaining examples: nested block brace diagnostics where useful and other delimiter-list boundary errors).
4. Add `return;`/void design notes only after deciding whether the v0.1 subset should support `void` functions.
5. Add CLI flags such as `--ast`, `--tokens`, or `--max-steps` after the core C-subset expansion stabilizes.

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
