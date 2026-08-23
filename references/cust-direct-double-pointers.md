# Direct `double *` review closure

Date: 2026-08-19

## Final independent review closure

- Keep interpreted execution depth (`MAX_CALL_DEPTH = 24`) separate from bounded call-summary analysis depth (`MAX_DOUBLE_STORAGE_CALL_DEPTH = 16`). Reusing the lower analyzer ceiling for runtime calls contracts established behavior.
- For pointer-returning calls whose later arguments mutate aliases, compute returned aggregate targets from per-argument source-order snapshots. Capturing a target only after all argument effects can redirect a result that selected an earlier argument.
- `StructCompoundSet` over aggregate-pointer fields must advance the stored aggregate target for `+=`/`-=`; evaluating only the RHS loses element selection and can hide later `double` storage from bounded-memory checks.
- `_Generic` controls are unevaluated, not unvalidated. Lexical classification of `DerefSet` must still reject writes through `const` pointers and pointer-valued assignments to scalar `double` lvalues.
- Scope-owned array-compound-literal metadata was introduced for direct `double[]` provenance only. Character compound literals remain ownerless until a separately selected lifetime-policy feature deliberately expands that boundary.

## Scope

The first safe direct `double *` slice reuses Cust-owned scalar and one-dimensional-array storage. It supports direct scalar addresses, array decay/indexing/arithmetic, pointer fields, function parameters/returns, compatible `void *` forwarding, const/lifetime/bounds checks, and non-evaluating `sizeof`/`_Alignof`/`_Generic` classification. It does not enable deeper pointers, pointer-to-row or multidimensional double storage, aggregate-field double addresses/decay, typedef-double forms, or raw-memory access to double object bytes.

## Review-driven root causes

1. **Host overflow in non-evaluating analysis.** Composing aggregate-array pointer indexes used unchecked `usize` addition. Guest code beneath `sizeof` could therefore panic the Rust host. Use `checked_add`; an overflowed index becomes unknown so wildcard field facts preserve conservative validation.
2. **Runtime lookup from lexical analysis.** Scoped aggregate field types are already present in `DoubleStorageFact::declared_type`. Once a field expression is statically typed, do not fall through to runtime `find_variable`/pointer evaluation for an automatic object that does not exist in the active interpreter scope.
3. **Function-boundary provenance loss.** A scalar “may return double storage” bit cannot represent aggregate pointer fields. Parameterized call summaries must carry returned aggregate field paths and changed struct-pointer parameter fields, then apply callee effects back to the caller target. Record writes separately from final field-set differences: unioned conditional inputs can otherwise hide an effect. Conditional arguments denote multiple possible targets: merge dangerous effects into each and do not use a possible safe overwrite to clear unselected storage.
4. **Control-flow shape matters.** A `do` body runs at least once, so its post-body state replaces the incoming state before any later loop join. Returning branch state must not contaminate fallthrough state. `return`, `break`, and `continue` all stop the current statement sequence, even though only `return` exits the function. Preserve break/continue alias states separately until their enclosing loop consumes them so later fallthrough statements cannot launder stopped paths.
5. **Runtime arithmetic is also hostile input.** Direct pointer indexing must use checked signed addition; a large guest offset from a nonzero base cannot be allowed to panic a debug Rust host.
6. **Effects need target identity and order.** Traverse setter/update operands, replace stale aggregate targets after assignment/`_Generic`, retain adjusted struct-array element identity, and make function-summary cache keys plus ordered write effects aware of aliased parameters.
7. **Globals and storage duration are part of the model.** Seed lexical global facts, export/apply global writes across calls, and reject static pointer initializers whose evaluated owner has automatic duration even when the pointer is never observed.
8. **Switch exits are structural.** A switch with `default` whose every reachable section returns has no continuation; preserve that stop/return fact instead of merging unreachable entry aliases.
9. **All repeating loops need fixed points.** `for` and `do-while` can launder provenance across a second iteration if analyzed once. Reuse monotone widening while preserving `for`'s zero-iteration path and `do-while`'s mandatory first pass.
10. **Argument values are captured in source order.** Snapshot aliases after each argument and use that snapshot for the matching parameter and effect target; a later assignment argument must not retarget an earlier argument retroactively.
11. **Global aliases share one callee fact.** When a pointer parameter targets a global aggregate, retain the global target so global-name and parameter writes occur on one fact in body order rather than as conflicting replay summaries.
12. **Parameter summaries use parameter identity.** Return snapshots can contain a same-named block local; read the fixed parameter scope instead of reverse-searching by name.
13. **Dynamic current-storage indexes are conservative.** Inspect all current aggregate elements without evaluating a dynamic index before falling back to globals-only lexical facts.
14. **Conditional generic types stay lexical.** Combine lexically derived branch types with the ordinary conditional type rules; callee-local controlling operands beneath `sizeof` cannot use runtime lookup.
15. **Call results retain aggregate targets.** A call summary that returns a struct pointer must carry the selected caller aggregate target, not only field facts. Pointer-field assignment through `identity(&box)->pointer` can then update `box` without evaluating the call.
16. **Supported unary generic controls stay lexical.** Unary `+`/`-` preserve `double` and otherwise apply Cust's integer promotion; `~` keeps its existing double diagnostic; `!` yields `int` for supported scalar/pointer operands. None may look up a callee parameter at runtime beneath `sizeof`.
17. **Recursive fallback follows the declared return type.** A recursive edge remains conservative for scalar `double`, `void *`, `double *`, aggregate pointers, and double row pointers, but statically incompatible `int *`/`char *`/`_Bool *` returns are not double storage.
18. **Aggregate parameter element indexes use one coordinate system.** Parameterized non-evaluating summaries retain root-absolute aggregate-array indexes. Rebase neither seeded element facts nor callee targets to zero: backward arithmetic from a nonzero argument must remain representable, and caller replay must not add the base a second time.
19. **Returned aggregate targets are already absolute.** A returned parameter target replaces the caller target's selected index; adding the caller base again misattributes `identity(&boxes[1])` and similar writes.
20. **Nested pointer fields preserve target shape.** Synthetic targets for aggregate pointer fields forwarded through parameters remain `Direct` for standalone objects and `Element` with the original absolute index for array elements. This keeps both by-value direct-object writes and relative pointer arithmetic exact.
21. **Unknown selected elements merge conservatively.** When a callee writes through an aggregate pointer whose caller index is dynamic, merge the changed element fields into wildcard storage so any possible selected element remains protected.
22. **Recursive fallback must retain effects.** Use a temporary recursion sentinel to bound one direct body summary, then extract ordinary parameter, nested aggregate-target, and global writes. Do not reduce a recursive edge to declared return taint only or cache the approximation as a completed call.
23. **Aggregate values carry pointer-target maps.** Aggregate literals, returned aggregate calls, and nested aggregate-valued field expressions must preserve embedded aggregate-pointer identities independently from double-storage field paths so later writes still reach caller-owned objects.
24. **Aggregate arrays decay in generic controls.** A local aggregate array is lexically a pointer to its element aggregate after `_Generic` controlling-expression conversion; retain element constness in the pointer metadata rather than storing the array as a scalar aggregate value type.
25. **Binary generic controls stay lexical.** Derive every supported binary operator family's result category and diagnostics from lexical operand facts. Runtime lookup cannot resolve callee locals beneath `sizeof` and other non-evaluating routes.
26. **Call arguments snapshot the value, not only evaluation order.** Keep both pre- and post-effect aliases for each source-ordered argument. Postfix increment and nested call-like values classify from the pre-effect state, while prefix increment classifies from the updated target; apply the same rule in direct call analysis and nested effect replay.
27. **Sequenced values may need an intermediate snapshot.** A comma-right postfix value observes comma-left effects but precedes its own increment. Replay the left operand into a cloned pre-effect state, then classify the right operand; use the same value snapshot for calls, declarations, returns, assignments, and pointer lvalue writes. Keep plain aggregate-target wrapper recursion in parity with cached lookup.

## TDD closure

Independent review first reproduced:

- unchecked aggregate pointer-index panic;
- a pre-existing nested aggregate-array sibling false failure (`undefined variable 'shelf'`);
- provenance laundering through a struct-pointer parameter, an aggregate-valued return call, and a callee mutation;
- false rejection after a guaranteed `do` overwrite and after an unreachable second return;
- control-flow laundering after `break`/`continue` plus contamination from a returning branch; and
- lost callee effects when a struct pointer argument is conditional;
- laundering after conditional `break`/`continue`; and
- hidden writes when unioned conditional input facts equal the callee's output facts.
- runtime direct-pointer index overflow;
- setter-index side-effect laundering;
- stale aggregate targets after pointer assignment and `_Generic` selection;
- aliased-parameter write-order/cache reuse;
- adjusted struct-array element collapse across pointer arithmetic;
- missing global aggregate/slot facts and callee effects;
- false switch continuation after all sections return; and
- automatic-storage addresses accepted as static pointer initializers.
- second-iteration provenance laundering in `for` and `do-while`;
- later argument side effects retargeting earlier pointer parameters;
- global/parameter alias writes replayed in reverse order;
- same-named block locals hiding parameter effects on return paths;
- dynamic local aggregate indexes skipped by current-storage validation; and
- conditional `_Generic` typing falling through to absent runtime locals.
- aggregate field writes through a struct-pointer call result being dropped;
- unary `_Generic` control over a callee parameter reaching absent runtime state; and
- recursive `int *` forwarding being classified as double storage; and
- direct aggregate-pointer compound assignment and increment leaving the non-evaluating target on the prior array element.

The first three review rounds produced eleven dedicated regressions plus the pre-existing sibling regression. A fourth fresh review produced eleven additional focused tests for its eight grouped defects. The later blocking review produced six more focused regressions. In the second review-fix cycle, blocker 1 was outside Cust's supported syntax and was removed without edits; blockers 2–4 each produced one strict RED/GREEN regression for aggregate call-result targets, unary lexical `_Generic`, and recursive fallback typing. The prior `StructPtrGet`/`void *` aggregate-field finding was invalid because Cust rejects `void *` aggregate fields and supported substitutes stop earlier, so no code fix is claimed for it. Later reviews drove regressions for aggregate-pointer mutation offsets, root-absolute parameter coordinates, recursive effects, aggregate-value pointer targets, aggregate-array generic conversion, binary lexical generic typing, and prefix call snapshots. The next value-snapshot review cycle reproduced ten strict tests as `Ok(8)` across prefix pointer writes and comma/postfix values consumed by calls, declarations, assignments, returns, and assignment-wrapped writes. Shared intermediate snapshots and comma-target recursion made every route GREEN. The latest review added three strict RED/GREEN regressions for aggregate by-value dereference facts, postfix aggregate-dereference snapshots, and host-safe depth bounding of a 500-operand comma expression inside a summarized callee. The first canonical run exposed three valid nested-intrinsic regressions from sharing the 32-level validator state; dedicated 128-level validator and effect-walker states restored those tests while retaining the deep diagnostic, and independent correction review approved the result. After the corrected analyzer fixes, `cargo test --test interpreter non_evaluating_memory_ -- --nocapture` passes all 189 selected interpreter tests, the matching CLI safety regression passes for 190 total non-evaluating-memory tests, `cargo test --test interpreter direct_double_pointer -- --nocapture` passes all 65 selected tests, and `cargo test --test c_compat -- --nocapture` passes its one compiler-oracle test. The corrected canonical gate passes formatting, warning-denied Clippy, all 1,977 local tests, rebuilt Docker tests, runtime smoke output `10`, and diff checks.

## Verification checklist

- Keep call-analysis cache keys parameterized by relevant input storage facts.
- Preserve recursion/depth limits and linear scaling tests.
- Test both dangerous provenance preservation and safe sibling/overwrite precision.
- Run the actual `cargo test --test c_compat -- --nocapture`; fixture-name filtering runs zero tests.
- Obtain fresh independent review before the canonical gate, then run every required local and Docker command after the final code/test edit.
