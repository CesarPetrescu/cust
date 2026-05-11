# Cust `_Thread_local` storage-class syntax parity

2026-05-11 autonomous run.

## Scope

Cust accepts C11 `_Thread_local` as parser-level no-op storage metadata for supported object declarations:

- top-level globals such as `_Thread_local int global;`
- top-level `static _Thread_local` globals
- `static _Thread_local` local declarations, reusing Cust's existing persistent local `static` storage

Runtime behavior remains Cust's deterministic single-thread interpreter model; `_Thread_local` does not create per-host-thread storage or alter pointer identity, lifetime, `sizeof`, `_Alignof`, or initialization order.

## Implementation notes

- Lexer maps `_Thread_local` to `Token::ThreadLocal`.
- Top-level parser consumes `_Thread_local` around existing `static`/`extern` storage-class handling before routing through existing function/global declaration paths.
- Local statement parser routes leading `_Thread_local` through `parse_thread_local_local_decl`; `static _Thread_local` locals consume the specifier inside `parse_static_local_decl` and reuse `Stmt::StaticLocal`.

## Fixture pitfall

Native `cc -std=c11 -Wall -Wextra -Werror` rejects block-scope `_Thread_local int local;` without `static` or `extern` as an implicitly automatic thread-local declaration. Compiler-oracle fixtures should use warning-free `static _Thread_local int local;` at block scope.

## Verification commands

```bash
cargo test --test interpreter supports_thread_local_storage_class_specifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
