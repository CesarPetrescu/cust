# Aggregate forward declaration diagnostics

2026-06-21 autonomous run.

Cust still requires complete `struct`/`union` definitions before use and does not add incomplete aggregate type support. When malformed or unsupported C source contains a forward declaration such as:

```c
struct Point;
union Number;
```

parser routing should detect the exact aggregate-keyword/name/semicolon pattern at top level before falling through to aggregate variable declaration parsing. This preserves a targeted unsupported-subset diagnostic (`forward struct declarations are not supported` / `forward union declarations are not supported`) instead of the misleading downstream `undefined struct type '<name>'` error.

Focused command:

```bash
cargo test --test interpreter rejects_aggregate_forward_declarations_with_context -- --nocapture
```
