# Safe tracked boolean pointer objects and output parameters

Date: 2026-09-04

## Supported slice

Cust's bounded unqualified `_Bool **` slice reuses the tracked pointer-output representation already used by `char **` and `int **`. The representation stores interpreter-owned pointer-slot identity plus an explicit scalar `CType`; no host address is introduced.

The slice supports:

- unqualified local, file-global, and block-static `_Bool **` objects;
- null/default initialization and mutable unqualified `_Bool *` slot addresses;
- compatible object forwarding through `_Bool **` parameters;
- `*output` pointer reads/writes and `**output` scalar reads/writes;
- C `_Bool` normalization of every nonzero indirect scalar write to `1`;
- equality, truthiness, aliases, and non-evaluating `sizeof`;
- caller-owned pointer lifetime, constness, static-storage, and array-owner identity.

Qualified slots/pointees, incompatible scalar pointees, deeper pointers, pointer arrays, aggregate fields, pointer-return types, address-taking of tracked output objects, arithmetic, relational ordering, and compound updates remain targeted unsupported boundaries.

## Implementation fan-out

Parser declaration and parameter routing must admit `DeclType::Scalar(CType::Bool)` wherever the tracked `char`/`int` pointer-output path is selected. Runtime lookup must likewise recognize mutable unqualified `_Bool *` slots in ordinary and static storage. Keep parameter binding, slot liveness checks, indirect typed storage, static initializer checks, output equality/truthiness, and `sizeof` on the shared tracked representation.

Do not bypass the normal scalar write path: `_Bool` normalization belongs to typed target storage and must apply equally through `**output` at local, global, and static roots.

## Diagnostic pitfall

Before `_Bool **`, several pointer-output diagnostics assumed only two categories:

```text
char => character
anything else => integer
```

That assumption silently mislabels boolean failures. Use `CType::pointer_output_kind()` for unary/contextual diagnostics and `pointer_output_binary_kind_label()` when either binary operand may provide the tracked output type. Audit evaluated and non-evaluating routes separately: this package exposed stale labels in conditional `sizeof` and aggregate equality beneath both `sizeof` and `_Generic`.

## Verification pattern

```bash
cargo test --test interpreter boolean_pointer -- --nocapture
cargo test --test interpreter pointer_output -- --nocapture
cargo test --test c_compat -- --nocapture
```

The registered native fixture should restrict itself to defined pointer-slot identity, forwarding, null, normalization, and `sizeof` relationships. Native compilers remain external oracles only; Cust retains interpreter-owned identities and deterministic sizes.
