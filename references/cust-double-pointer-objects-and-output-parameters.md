# Safe tracked double pointer objects and output parameters

Date: 2026-09-04

## Supported slice

Cust's bounded unqualified `double **` slice reuses the tracked pointer-output representation used by `char **`, `int **`, and `_Bool **`. It stores interpreter-owned pointer-slot identity plus `CType::Double`; it never exposes a host address.

The slice supports local, file-global, and block-static objects; null/default initialization; mutable unqualified `double *` slot addresses; compatible parameter forwarding; `*output` pointer access; binary64-preserving `**output` scalar access; equality, truthiness, and non-evaluating `sizeof`; and the shared owner/lifetime/const/static rules.

Qualified or incompatible slots, deeper pointers, pointer arrays, aggregate fields, pointer returns, address-taking of tracked objects, arithmetic, ordering, and compound updates remain exact unsupported boundaries.

## Implementation fan-out

Admit `CType::Double` in all three pointer-output parser routes: parameters, first declarators, and additional declarators. Runtime mutable-slot discovery must recognize ordinary and static `double *` values. Keep the tracked-output evaluator, declaration/assignment validation, indirect pointer/scalar operations, equality/truthiness, and `sizeof` classifiers shared rather than adding a second representation.

`expr_is_unsupported_double_pointer()` must exempt expressions only after `expr_is_character_pointer_output_value()` has proved they are tracked output values. This keeps direct `double **` support narrow while retaining all established direct-double safety boundaries.

## Review pitfall: do not sweep persistent outputs after every call

A broad post-call scan of all tracked `double **` objects is incorrect. It observes dangling-but-unused values at unrelated call boundaries, diverges from the established tracked-output contract, adds work proportional to all program storage on every call, and can produce nondeterministic diagnostics when iterating `HashMap` state.

Keep post-call validation limited to pointer-output arguments in function declaration order, after the callee parameter scope is removed. Other expired targets diagnose when the program observes them. A regression should retain a dead pointer through an unrelated `ping()` call and return successfully, while a paired regression dereferences the same expired value and receives `pointer to out-of-scope variable`.

## Verification pattern

```bash
cargo test --test interpreter double_pointer -- --nocapture
cargo test --test c_compat -- --nocapture
```

The native fixture should use only defined pointer identity, forwarding, mutation, null, and `sizeof` relationships. Run the actual one-test compiler-oracle harness; filtering by fixture name runs zero tests.