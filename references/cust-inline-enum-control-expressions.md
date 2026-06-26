# Inline enum definitions in control expressions

Date: 2026-06-26

## Gap

Inline enum definitions in expression type specifiers can appear inside control-flow headers, for example:

```c
while ((enum Limit { LIMIT = 3 })i < LIMIT) { ... }
for ((void)(enum Init { INIT = 2 })0;
     (enum Bound { BOUND = 5 })i < BOUND;
     (void)(enum Step { STEP = 4 })0, i = i + STEP - 3) { ... }
switch ((enum Kind { MATCH = 22 })MATCH) { case MATCH: ... }
```

The parser already made the enumerators visible to later parsing, but runtime execution only emitted pending inline `EnumDecl`s for declarations, expression statements, and returns. Control expressions therefore parsed but failed at runtime with `undefined variable '<ENUMERATOR>'` when the condition/body/case/increment referenced the enumerator.

## Implementation notes

- After parsing `if`, `while`, `do while`, and `switch` controlling expressions, call `take_pending_inline_enum_decl()` before parsing the body/sections so later body parsing does not clear the pending constants.
- Prepend that enum declaration to the resulting control statement via a small helper, matching the existing runtime wrapper pattern for declarations/expression statements/returns.
- For `for` loops, move enum declarations discovered in the condition and the leading enum declaration produced by an increment expression into the loop init statement. This executes them once in the loop scope before the first condition/body/increment evaluation; leaving the increment enum declaration inside the increment statement re-declares it every iteration and fails with `variable '<ENUMERATOR>' already declared in this scope`.

## Verification

Focused RED/GREEN:

```bash
cargo test --test interpreter inline_enum_control_expr_definitions -- --nocapture
cargo test --test c_compat -- --nocapture
```

Full required gate was run by the autonomous maintainer after status updates.
