# Tracked scalar-output aggregate-field arrays

This note records the bounded v0.61.0 implementation and release contract for fixed one-dimensional tracked scalar-output arrays embedded in supported struct fields, such as `struct Box { int **outputs[N]; }`.

## Supported slice

- Pointees: unqualified `char`, `int`, `_Bool`, and `double`.
- Element spellings: direct `T **`, inner pointer aliases, complete output aliases, and chained complete aliases.
- Containing routes: direct objects, aggregate arrays, nested fields, struct pointers, embedded containing-object arrays, reverse subscripts, aggregate-valued temporaries, aggregate copies, and aggregate returns.
- Initialization: zero/default, positional, designated, incremental designators, whole-field replacement, and compatible selected-element references. The array field requires nested braces such as `struct Box box = {{0}};`; flat `struct Box box = {0};` is rejected with `pointer output array fields require a braced initializer`.
- Use: indexed reads and reassignment, forwarding of selected elements through functions and returns, equality/truthiness, typed indirect pointer/scalar access, and deterministic `sizeof` for the complete field array and selected elements.
- Metadata: each element retains interpreter-owned output-slot identity, scalar pointee type, referenced-pointee owner/lifetime/read-only state, containing-object owner/lifetime, recursive const ancestry, pointer-slot qualification, and static-storage eligibility.

## Runtime and non-evaluating parity

Runtime evaluation, parser-folded integer constant expressions, `_Generic`, and callee-summary analysis validate containing-object indexes, field-array initializers, selected-element assignments, calls, scalar consumers, and unsupported updates without executing `sizeof` operands. Aggregate-valued temporary bases are captured once so initializer side effects and hidden lexical owners are preserved.

A full field-array query keeps array-object type and returns `N * sizeof(T *)`; a selected element returns Cust's fixed pointer size. Whole-array use in a scalar or pointer-value context does not decay. Aggregate copies deep-clone the array container while preserving each selected caller-owned output-slot identity.

## Resource-safety closure

Source-controlled parser recursion is bounded before the host stack can abort:

- ordinary unary and grouping syntax has a 40-level ceiling;
- integer-constant-expression unary and conditional syntax has a 64-level ceiling;
- mixed ordinary/integer/type-query routes share conservative stack-unit accounting;
- nested array type names, inline enum definitions, array compound literals, field selections, and assignments are exercised on a 2 MiB child thread stack;
- 50,000-prefix CLI inputs return source-located diagnostics instead of terminating the host process.

When changing these limits, retain the exact accepted 40-level ordinary boundary and the child-process regressions in both mixed nesting directions. Stress tests above the ordinary parser ceiling must not be used to claim downstream validator behavior.

## Retained boundaries

The bounded slice does not enable:

- tracked-output arrays as union fields or nested aggregate storage inside unions;
- array-to-pointer decay or adjusted tracked-output array parameters;
- `&box.outputs`, `&box.outputs[i]`, or other whole-array/element address exposure;
- deeper, flexible, or multidimensional tracked-output field arrays;
- qualified scalar pointees or qualified tracked output slots;
- tracked-output casts;
- whole-array assignment;
- selected-element pointer arithmetic, relational ordering, compound updates, or increment/decrement.

Keep evaluated and non-evaluating diagnostics in parity where the language constraint is shared, while preserving established source-location differences between parser and runtime semantic diagnostics.

## Acceptance evidence

- Focused interpreter filter: 51 tests (`cargo test --test interpreter tracked_scalar_output_field_arrays -- --nocapture`).
- Generated pointer-output parity: 128 positive route/spelling/consumer programs plus 32 union/deeper-shape boundary programs.
- Hostile CLI coverage: three parser-depth subprocess tests, including 50,000-token unary and integer-constant inputs.
- Registered warning-free C11 compiler oracle: `tests/fixtures/compat/valid/tracked_scalar_output_field_arrays.c`.
- Executable v0.61.0 inventory: 2,652 tests = 2,495 interpreter + 101 fuzz-safety + 36 CLI + 6 pointer-classifier parity + 10 tracked pointer-output parity + 2 Docker metadata + 1 compiler-oracle harness + 1 repository-license test.

Native C compilers remain external test oracles only; Cust uses no host addresses or native runtime path.
