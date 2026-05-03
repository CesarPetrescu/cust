# Cust Autonomous Agent Plan

> **For Hermes:** Use this plan as the operating specification for recurring autonomous development on Cust.

**Goal:** Run a safe autonomous coding loop that improves Cust incrementally while keeping status files, tests, Docker verification, and Git history clean.

**Architecture:** A Hermes cron job runs against `/root/hermes-workspace/cust`. Each run reads `status/`, selects one small task, researches as needed, uses TDD for implementation, verifies locally and in Docker, updates status files, commits, pushes, and reports results. If blocked, it records the blocker and stops without pushing broken code.

**Tech Stack:** Rust, Cargo, Docker Compose, Git/GitHub SSH deploy key, Hermes cron, Hermes web/search + file + terminal toolsets.

---

## Operating Principles

1. **Small safe increments:** one feature/fix per run.
2. **TDD for behavior:** tests before implementation for code changes.
3. **Docker verification:** no push unless Docker test path passes.
4. **Status-first:** update `status/` every run.
5. **Research when unsure:** use official Rust/Cargo/Docker/GitHub/C docs first.
6. **No risky rewrites:** do not rewrite the interpreter architecture unless the status plan explicitly says so.
7. **No secret leakage:** never commit private keys, tokens, `.env`, or machine secrets.
8. **Honest reporting:** distinguish verified results from attempted/unverified work.

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
cd /root/hermes-workspace/cust
git checkout main
git pull --ff-only
git status --short
```

If the tree is dirty before changes, inspect carefully. Do not overwrite user work.

### 2. Read status files

Read:

- `status/current-state.md`
- `status/missing-features.md`
- `status/todo.md`
- `status/stuck.md`
- `status/research.md`

### 3. Choose one task

Priority order:

1. Fix failing tests/builds
2. Resolve active blocker if possible
3. P0 item from `missing-features.md`
4. First item from `todo.md`
5. Documentation improvement if no safe code task exists

### 4. Research if needed

Use web search/documentation for uncertain details. Record the decision in `status/research.md`.

### 5. TDD implementation

For behavior changes:

1. Add/modify a focused test.
2. Run it and confirm it fails for the expected reason.
3. Implement the minimum code needed.
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

### 7. Update status

Update relevant files in `status/`:

- current state changed
- completed/moved TODOs
- blockers
- research findings

### 8. Commit and push

Only after verification passes:

```bash
git status --short
git add <changed files>
git commit -m "type: concise description"
git push
```

Commit types: `feat`, `fix`, `test`, `docs`, `ci`, `refactor`, `chore`.

### 9. Report

Report compactly:

- task selected
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
