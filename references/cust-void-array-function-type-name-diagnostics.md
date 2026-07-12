# Void array/function cast and type-query diagnostics

## 2026-07-12 autonomous run

A deterministic suffix-matrix probe found that `void` took dedicated early branches in both `parse_cast()` and `parse_sizeof_like_type_name()`. Those branches checked `void *`, but did not inspect following `(` or `[` suffixes. Consequently `(void(void))0` fell through to a generic closing-parenthesis error, while `sizeof(void(void))` and `sizeof(void[2])` misleadingly reported only the base `sizeof(void)` rejection.

Keep `void *` detection first so all immediate and parenthesized void-pointer forms retain `void pointers are not supported`. After consuming `void`, reject a function suffix through the shared `reject_function_type_suffix(...)` helper and reject an array suffix at `[` with route-specific `void array casts/<operator> types are not supported`. Only a bare closing parenthesis should continue to `(void)expr` cast behavior or the established `sizeof(void)` / `_Alignof(void)` diagnostic.

Focused verification should include the new void array/function test plus existing void-pointer, bare-void type-query, and ordinary function-type tests.
