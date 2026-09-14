# Fixed tracked scalar-output arrays

This note records the bounded v0.60.0 implementation and release contract for fixed one-dimensional arrays of tracked scalar outputs such as `int **outputs[N]`.

## Supported slice

- Pointees: unqualified `char`, `int`, `_Bool`, and `double`.
- Element spellings: direct `T **`, inner pointer aliases, complete output aliases, and chained complete aliases.
- Storage: automatic, file-global, and block-static object arrays.
- Initialization: zero/default, positional, designated, and compatible element-to-element references within declaration order.
- Use: indexed reads and reassignment, function forwarding of selected elements, equality/truthiness, typed indirect pointer/scalar access, and deterministic `sizeof` for the full array and selected elements.
- Metadata: each element retains interpreter-owned output-slot identity, scalar pointee type, pointee owner/lifetime/read-only state, pointer-slot qualification, and static-storage eligibility.
- Scope: comma declaration lists, `for` initializers, shadowing, enum/integer-constant folding, and direct `switch` jumps preserve the same lexical binding and storage semantics.

## Non-evaluating contract

Runtime evaluation, parser-folded integer constant expressions, and callee-summary analysis all validate tracked-output array initializers, indexes, assignments, calls, scalar consumers, and update operations without executing `sizeof` operands. Nested expression, call, metadata, and clone sites are independently depth/work bounded before recursive traversal or AST cloning.

A skipped automatic declaration after an always-stopping switch statement contributes lexical type metadata without running its initializer. A skipped block-static declaration retains its original stable id and initializer, and skipped enum declarations retain their values and validation dependencies.

## Retained boundaries

The bounded slice does not enable:

- array-to-pointer decay or adjusted tracked-output array parameters;
- `&outputs`, `&outputs[i]`, or other whole-array/element address exposure;
- aggregate fields containing tracked-output arrays;
- deeper or multidimensional tracked-output arrays;
- tracked-output casts;
- element pointer arithmetic, relational ordering, compound updates, or increment/decrement;
- qualified scalar pointees or qualified tracked output slots.

Keep evaluated and non-evaluating diagnostics separate where established behavior differs. In particular, aggregate-valued indexes beneath `sizeof` are rejected without evaluation, while evaluated aggregate indexes continue through the ordinary scalar evaluator so their existing aggregate-specific diagnostic is preserved.

## Acceptance evidence

- Focused interpreter filter: 135 tests.
- Registered warning-free C11 compiler oracle: `tests/fixtures/compat/valid/tracked_scalar_output_arrays.c`.
- Generated alias-boundary parity: all direct/inner/complete/chained output spellings accept fixed object arrays while aggregate-field arrays and deeper shapes remain rejected.
- Executable v0.60.0 inventory: 2,596 tests = 2,444 interpreter + 101 fuzz-safety + 33 CLI + 6 pointer-classifier parity + 8 pointer-output parity + 2 Docker metadata + 1 compiler-oracle harness + 1 repository-license test.

Native C compilers remain external test oracles only; Cust uses no host addresses or native runtime path.
