# Safe tracked integer pointer objects and output parameters

Date: 2026-09-03

## Supported slice

Cust's first bounded `int **` slice reuses the existing tracked pointer-output representation rather than introducing host addresses. The representation now carries an explicit scalar pointee (`char` or `int`) and can identify either null or a mutable interpreter-owned pointer slot.

The slice supports:

- unqualified local, file-global, and block-static `int **` objects;
- null/default initialization, null constants cast to `void *`, and initialization from mutable unqualified `int *` slot addresses;
- object forwarding through unqualified `int **` parameters;
- `*output` reads and writes, `**output` scalar access, equality, truthiness, and non-evaluating `sizeof`;
- compatible null, `void *`, and object-byte-to-`int *` assignment conversions;
- caller-owned pointer lifetime, constness, and array-owner identity.

Qualified pointees/slots, `_Bool **`, `double **`, deeper pointers, pointer arrays, aggregate fields, pointer typedef aliases, pointer-return types, address-taking of an `int **` object, arithmetic, relational ordering, and compound updates remain targeted unsupported boundaries. Unqualified scalar-pointee aliases such as `typedef int Integer; Integer **output` are supported because they resolve to the same tracked `int **` representation.

## Parser and runtime fan-out

A safe two-level pointer slice is not only a declaration-parser change. Keep these paths in parity:

1. top-level/local/static declaration routing and mixed declarator lists;
2. function definitions, prototypes, signatures, parameter preflight, and source-ordered binding;
3. pointer-output object initialization/reassignment and indirect storage reads/writes;
4. pointer/scalar classifier paths for equality, truthiness, logical expressions, discard contexts, and `sizeof`;
5. null-pointer constant folding, including enum shadowing and undefined signed-shift rejection;
6. slot storage duration, pointee owner/lifetime, and qualification checks.

Prevalidation must reject incompatible pointer types before evaluating either the rejected RHS or an earlier side-effecting LHS/call argument. Conditional and comma wrappers need structural validation so a nested assignment cannot hide an incompatible type. Conditional RHS conditions must use scalar-condition validation rather than generic `sizeof`: aggregates have a size but are not valid C scalar controlling expressions.

Generic selections require use-specific output handling. Tracked `int **` values are valid association expressions (including beneath `sizeof`) while remaining unsupported as controlling expressions. Validate every association structurally, preserve output assignment diagnostics from unselected arms, and carry a validation-only pointer-output category rather than pretending the value is an ordinary one-level pointer type.

Null `void *` admission is deliberately narrow. Recognize `(void *)` only when its operand independently folds to a valid integer null-pointer constant, validate the cast non-evaluatingly, and map it to the typed tracked null state. Do not treat arbitrary `void *` expressions as `int **` objects.

## Lifetime review pitfall

Function-call output validation must run after the callee parameter scope is removed. Running `ensure_pointer_value_live()` while that scope is still present catches block locals (the body scope has already been popped) but misses a pointer to a by-value parameter:

```c
void publish(int **output, int value, int choose) {
    *output = choose ? &value : 0;
}

int main(void) {
    int *saved = 0;
    publish(&saved, 4, 1);
    return 0;
}
```

Read the caller-owned output slot only after `pop_scope()` has removed the callee parameter scope, then validate the stored pointer. Keep return-stack and call-depth cleanup before this final check so success and diagnostic exits leave interpreter state balanced. Preserve function parameter declaration order when collecting output slots; never iterate a map for diagnostic/validation order.

## Verification pattern

Use one shared focused substring and the real compiler-oracle test function:

```bash
cargo test --test interpreter pointer_output -- --nocapture
cargo test --test c_compat -- --nocapture
```

The warning-free native fixture should use only defined pointer identity and mutation relationships. Native compilers remain test oracles only; the interpreter runtime must retain Cust-owned identities and deterministic sizes.
