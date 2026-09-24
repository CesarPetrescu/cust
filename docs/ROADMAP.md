# Cust product scope and roadmap

Cust's product is an independently executed, deterministic **C subset** with explicit supported shapes and safe, intelligible failure for unsupported ones. It is not an ISO C implementation, a host-ABI emulator, a general libc, or a security sandbox. The [README](../README.md) describes the current public surface; [`status/missing-features.md`](../status/missing-features.md) and [`status/todo.md`](../status/todo.md) are the mutable evidence/backlog ledger. Historical checked items are not the product strategy.

## Selection rule

Choose work by user-visible semantic value, not by the next number in a diagnostic queue. Fix a failing build/test or an active correctness/safety blocker first. Otherwise prefer a coherent missing C behavior, type/storage boundary, or demonstrated conformance mismatch that can be finished and verified as a vertical slice. A completed queue item or exact-error audit alone is not evidence that Cust gained a capability. The status queue is a candidate pool, **not** an obligation to exhaust sequentially; when it disagrees with this roadmap, revise its top recommendation and preserve the old record as history.

Before implementation, name (1) an example C program currently rejected or handled incorrectly, (2) the intended supported and excluded shapes, (3) the ownership/const/lifetime and non-evaluation consequences, and (4) the tests that would establish completion. Compare portable behavior with a C11 compiler when useful, but never execute user programs by compiling them as Cust's runtime. Reject a candidate that only inventories more syntax tokens without a real fallback or capability gap. A diagnostic fix is appropriate when a concrete malformed input currently produces a misleading, generic, or unsafe result; it should not become a sequence of speculative token-by-token audits.

## First investigation: `double` pointer-to-row typedefs (completed)

The previously rejected `typedef double (*Row)[2];` over an existing fixed two-dimensional `double` object is now supported:

```c
typedef double (*Row)[2];
int main(void) {
    double values[2][2] = {{1.0, 2.0}, {3.0, 4.0}};
    Row row = values;
    return (int)row[1][0];
}
```

Cust previously reported `double pointer-to-row typedef aliases are not supported` at the typedef; a C11 compiler accepts this sample and it returns 3. The existing typed row-pointer metadata now admits `double` aliases without host addresses. Local objects, parameter/return aliases, row-scaled indexing/arithmetic, const pointees and slots, bounds/lifetime checks, and non-evaluating `sizeof` have focused tests; a warning-clean native fixture is registered. Direct double row-pointer function declarators are completed below; aggregate fields remain a separate investigation, while recursive pointer derivatives retain targeted rejection.

The direct `double` row-pointer **function** declarator slice is also complete: `int read(double (*rows)[2])` and `double (*advance(double (*rows)[2]))[2]` reuse the same row-width/const/lifetime metadata and interpreter-owned storage without any host pointer path. Focused RED/GREEN tests prove direct parameters and returns, named/unnamed prototypes, binary64 writes, unevaluated `sizeof`, width/qualification/bounds/expired-owner rejection; a strict C11 compiler-oracle fixture matches Cust. Named 2D-double row addresses `&values[i]` now reuse the same row-pointer identity as `values+i`, including const, width, owner, bounds, and non-evaluating `sizeof` checks. Direct aggregate-field 2D-double row addresses `&table.rows[i]` preserve row width and owner/path through the existing field-backed array pointer, including evaluated indexing and non-evaluating `sizeof`; arrow-backed `&owner->rows[i]` also offsets that typed field base instead of a scalar element pointer. Indexed aggregate-array and temporary aggregate row addresses, aggregate double row-pointer fields and recursive pointer derivatives remain separate investigations. The next high-value candidate is an aggregate field of pointer-to-double-row type: first reproduce a rejected `struct View { double (*rows)[2]; };` over a live 2D owner and assess whether distinct field storage can retain row width, const, and lifetime through aggregate copies. If its representation is disproportionate, investigate `&tables[j].rows[i]` as a bounded alternate with exact indexed owner/path. Do not lift the parser guard without classifier/evaluator parity and explicit boundaries.

## Outcomes, in dependency order (not version promises)

| Stage | Outcome | Exit gate |
| --- | --- | --- |
| 1. Semantic boundary inventory | Choose one consequential unsupported C program family, e.g. a currently excluded pointer/output-slot operation, `double` declarator/storage combination, or aggregate object-byte layout. Document why it matters and the minimum viable storage/type representation; do not treat every possible C form as one feature. | Reproducible before-state, narrow support/rejection contract, implementation-ready valid/invalid fixtures and portable oracle expectations where possible. If the representation is not safe/tractable, record the blocker and choose another family. |
| 2. Implement a complete vertical slice | Extend parser, type/classifier, evaluator and storage paths together for the selected family. Preserve existing pointer identity, bounds, const/lifetime, value-copy, evaluation-order, and unevaluated-expression rules. | Focused tests first fail for the intended reason and then pass; valid programs execute correctly, adjacent unsupported forms fail deliberately, and any admitted expression has evaluated/non-evaluated parity. Register native-oracle fixtures only where behavior is portable and warning-free. |
| 3. Conformance and release readiness | Exercise the selected slice through declarations, calls, aggregates, aliases, conditionals, and error paths that actually apply. Update public capability/limit claims after verification, not before. | `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, `docker compose run --rm test`, `docker compose run --rm cust`, and `git diff --check` pass or the precise unverified gate is reported. Only then consider a release/version change; no release is implied by this roadmap. |

Repeat these stages for the next highest-value family. Cross-cutting correctness or parser-depth regressions can preempt a feature when they block safe execution or testing. Additional library intrinsics and CLI polish are subordinate to semantic coverage unless a specific user-facing need or blocker makes them higher impact. Success is a documented and tested increase in the executable C subset (or closure of a demonstrated correctness defect), not a growing inventory of tests, diagnostics, tags, or status entries.

## Boundaries to keep visible

Existing narrow tracked `T **` output slots are not a general multi-level pointer model. Supported selected byte roots are not arbitrary memory access; pointer-containing aggregates, general union layouts, and whole two-dimensional `double` objects are not automatically admitted. Deterministic Cust sizes/layout intentionally differ from many native C ABIs. System headers, general variadics/function pointers, VLAs, `float`/`long double`, and host libc/stdio are outside current scope. Expanding any of these requires an explicit representation decision, adjacent rejection tests, and a viable verification oracle—not an unqualified promise to implement full C.
