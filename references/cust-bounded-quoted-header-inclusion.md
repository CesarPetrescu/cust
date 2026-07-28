# Cust bounded quoted-header inclusion

## Completed slice

Cust file entry points support project-relative `#include "..."` on Linux without delegating execution to a native compiler.

- Search the logical including-file directory first, then the primary source's project root.
- Keep logical paths separate from canonical file identities so symlink spelling controls C-compatible relative lookup while canonical identities control containment and cycle detection.
- Share macro definitions and expansion budgets across the translation unit.
- Preserve normalized project-relative source names in formatted tokens and parser/runtime diagnostics.
- Permit conventional include guards, including guarded self-inclusion, while rejecting unguarded cycles.
- Reject system headers and macro-expanded include operands explicitly.

## Security and resource model

- Maximum nested included-header depth: 32.
- Maximum cumulative included source: 1 MiB.
- Canonicalize a candidate only to derive an in-root relative path, then perform the actual Linux open from the project-root directory descriptor with `openat2(RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS)` plus `O_NONBLOCK`; containment therefore applies before the target is opened.
- Read included source from the already-open handle and compare its Unix device/inode to the selected logical pathname so a later path race fails closed.
- Reject symlink escapes and symlink loops with source-context diagnostics.
- Non-Linux quoted inclusion fails closed until an equivalent containment-preserving open is available. Ordinary primary source interpretation remains available, and platform-gated tests assert the fail-closed boundary.

## TDD and review regressions

Focused CLI coverage should include:

1. direct and nested includes with shared object/function macros;
2. logical-path lookup through a symlinked primary source and in-root symlinked header;
3. duplicate basename headers with distinct project-relative token origins;
4. inactive includes and guarded self-inclusion;
5. unguarded cycles, depth, and cumulative-byte limits;
6. missing/system/macro-expanded/path-traversal/symlink-loop diagnostics;
7. primary and included FIFO rejection under a process deadline;
8. parser and runtime errors originating in headers;
9. AST debug-output compatibility and the existing 32-call recursion boundary.

The added `io_error` flag initially enlarged `CustError` enough to trigger host stack overflow at the prior recursion boundary. Store the message as `Box<str>` and box `Stmt::Break`/`Stmt::Continue` token payloads so the deterministic `MAX_CALL_DEPTH = 32` contract remains intact.

## Verification

Run focused CLI tests, both recursion-boundary regressions, and the full compiler-oracle harness before the canonical gate. The native C fixture should use only warning-free project-relative headers and return 0 under Cust and `cc -std=c11 -Wall -Wextra -Werror`.
