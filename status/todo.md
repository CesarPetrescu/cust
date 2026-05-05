# Cust TODO

The autonomous agent should complete a meaningful roadmap work package per cron run: usually one full C/interpreter/tooling feature or 2-4 tightly related backlog items. Prefer finishing concrete unchecked items with code/tests over generic improvement or status-only churn.

## Next recommended tasks

1. [x] Single-statement control-flow bodies and `else if` chains for `if`/`else`/`while`/`do`/`for`, including C dangling-`else` binding and compiler-oracle coverage
2. [x] C-style block comments `/* ... */` as lexer whitespace, including multi-line/inline comments, exact unterminated-comment diagnostics, and compiler-oracle coverage
3. [x] `void` helper functions and `return;` support; acceptance: lexer/parser/interpreter handle `void name(...) { ... }`, empty returns in void functions, side-effect-only void calls, int/void return-shape diagnostics, scalar-use diagnostics for void calls, and C compiler-oracle coverage
4. [ ] Continue parser recovery/error-message expansion only for newly discovered malformed programs not already covered by the exact-error suite; acceptance: add focused exact-output parser tests before implementation for remaining delimiter/list boundary errors, unsupported near-future C forms, or other malformed programs that still fall through to generic messages. Switch missing-case-colon coverage, duplicate switch case/default label coverage, conditional-operator missing-colon, do-while missing-semicolon, increment/decrement non-lvalue coverage, pointer-bitwise operand diagnostics, bitwise-compound pointer diagnostics, invalid shift-count diagnostics, int/void return-shape diagnostics, scalar-use diagnostics for void calls, and unterminated block-comment lexer diagnostics are now included, so future malformed control/expression work should target only new precise failures.
5. Add `LICENSE` file after confirming the project-owner's intended license; acceptance: repository root includes the chosen license text and README license section points to it.
6. Consider additional expression-level C-subset features only with clear acceptance fixtures; assignment expressions, arithmetic compound assignment, bitwise/shift compound assignment, increment/decrement, bitwise/shift operators, the conditional operator, the comma operator, switch statements, do-while loops, block comments, and void helper functions are now covered, so future expression/control work should avoid regressing exact delimiter diagnostics such as missing `]` inside index expressions.

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
