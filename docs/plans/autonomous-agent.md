# Cust Autonomous Agent Plan

> **For Hermes:** Use this plan as the operating specification for recurring autonomous development on Cust.

**Goal:** Run a safe autonomous coding loop that advances the [product roadmap](../ROADMAP.md) through substantive C-interpreter semantics or demonstrated conformance fixes, while keeping status files, tests, Docker verification, and Git history clean.

**Architecture:** A scheduled maintainer runs against a Cust repository checkout. Each run inspects checkout ownership, synchronizes its clean base, reads `status/`, ideates candidate roadmap-completion work, thinks through impact/safety/testability, selects the highest-impact finishable work package, researches as needed, uses TDD for implementation, verifies locally and in Docker, updates status/backlog files, commits, pushes, and reports results. If blocked, it records the blocker and stops without pushing broken code.

**Tech Stack:** Rust, Cargo, Docker Compose, Git, Hermes scheduling and development tools. Authentication and delivery configuration stay outside the public repository.

---

## Operating Principles

1. **Advance product scope:** treat [`docs/ROADMAP.md`](../ROADMAP.md) as the selection authority; finish one meaningful C-interpreter semantic or demonstrated conformance slice per run when feasible. The status backlog is evidence and candidate inventory, not a sequential obligation to finish diagnostic microtasks.
2. **TDD for behavior:** tests before implementation for code changes.
3. **Docker verification:** no push unless Docker test path passes.
4. **Status-first:** update `status/` every run.
5. **Research when unsure:** use official Rust/Cargo/Docker/GitHub/C docs first; when local command/C/POSIX details are needed or web docs are unavailable, use `man` pages and record the finding.
6. **No risky rewrites:** do not rewrite the interpreter architecture unless the status plan explicitly says so.
7. **No secret leakage:** never commit private keys, tokens, `.env`, or machine secrets.
8. **Honest reporting:** distinguish verified results from attempted/unverified work.
9. **Always version controlled:** every run must start with `git pull --ff-only`; every verified change must be committed and pushed before the run ends. If there are no code/docs changes, still update status files when useful and commit/push those status updates.
10. **Ideate before create:** before coding, generate candidate improvements, judge whether each is worth doing now, and save overflow ideas in `status/todo.md` or `status/missing-features.md`.

## Files owned by the autonomous process

- `status/current-state.md` — concise state snapshot
- `status/missing-features.md` — prioritized feature backlog
- `status/todo.md` — next tasks and every-run checklist
- `status/stuck.md` — blockers and failed attempts
- `status/research.md` — useful external docs/findings
- `docs/plans/autonomous-agent.md` — this operating plan
- `docs/autonomous-agent-prompt.md` — cron prompt template

## Per-run algorithm

### 1. Sync and inspect

```bash
cd /path/to/your/cust-checkout
git status --short
git checkout main
git pull --ff-only
git status --short
```

If the tree is dirty or another worker owns this checkout, do not switch branches or overwrite user work; use an isolated worktree or stop and report the conflict.

### 2. Read status files

Read:

- `status/current-state.md`
- `status/missing-features.md`
- `status/todo.md`
- `status/stuck.md`
- `status/research.md`

### 3. Ideate, evaluate, and choose a work package

Read [`docs/ROADMAP.md`](../ROADMAP.md) alongside `status/`. List several candidates grounded in actual code/fixtures and a concrete C program that currently fails or exposes a correctness gap. Compare user-visible semantic impact, safe representation, dependencies, and testability. Select a coherent vertical slice with a defined supported/excluded boundary and acceptance fixtures; preserve overflow candidates with concrete tests. Reconcile `status/todo.md`'s top recommendation with the roadmap before choosing work; dated/numbered historical "next" statements are not authority.

Priority order:

1. Fix failing tests/builds and active correctness/safety blockers.
2. Implement the highest-value finishable C-semantic or demonstrated conformance slice identified by the roadmap.
3. Do diagnostic/CLI/tooling work when a reproduced user-visible defect blocks confidence in the subset, not simply because an audit is next-numbered.
4. Do documentation/status-only work when safe code work is blocked or already complete.

Native compilers may be used only as external test oracles for supported fixtures, not as implementation helpers. Multiple tightly related TODOs can form one slice; do not manufacture extra microtasks to fill a run.

### 4. Research if needed

Use web search/documentation for uncertain details. If you need local command syntax, system/C/POSIX semantics, or external docs are unavailable, consult local manual pages with `man` (examples: `man cargo`, `man docker`, `man 3 printf`, relevant C/POSIX pages). Record the decision in `status/research.md`, including whether it came from a URL or a `man` page.

### 5. Create/implement with TDD

For behavior changes:

1. Add/modify focused tests. Prefer multiple layers when practical: interpreter regression tests, valid/invalid fixtures, and error/negative tests.
2. Run them and confirm they fail for the expected reason.
3. Implement/create the selected feature or fix completely enough for the documented C subset.
4. Run the focused test.
5. Run the full suite.

### 6. Verification gate

Required before commit/push:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
docker compose run --rm test
docker compose run --rm cust
```

If Docker is unavailable or fails for infrastructure reasons, record the exact blocker in `status/stuck.md` and do not claim Docker verification passed.

### 7. Update status and idea backlog

Update relevant files in `status/`:

- current state changed
- completed/moved TODOs
- blockers
- research findings
- promising ideas discovered during ideation that did not fit this run

### 8. Commit and push

Every successful run must end version-controlled. Only commit/push after verification passes:

```bash
git status --short
git add <changed files>
git commit -m "type: concise description"
git push
```

If code/docs did not change but status files were updated, commit and push the status update. If absolutely nothing changed, report `no changes to commit`.

Commit types: `feat`, `fix`, `test`, `docs`, `ci`, `refactor`, `chore`.

### 9. Report

Report compactly:

- ideation summary and why the selected work package was chosen
- work package selected
- backlog items completed
- C/tooling/product features implemented
- tests added/expanded
- files changed
- tests run + pass/fail
- commit hash if pushed
- blockers if any
- next recommended task

## Stop conditions

Stop without pushing code if:

- tests fail and fix is not obvious
- Docker verification fails
- Git has unexpected uncommitted user changes
- GitHub push/auth fails
- task requires product decision not present in docs/status
- implementation needs unsafe host actions

Record stop reason in `status/stuck.md` where appropriate.
