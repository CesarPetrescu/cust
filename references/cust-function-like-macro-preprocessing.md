# Cust bounded function-like macro slice

Date: 2026-07-28

## Scope

Cust now accepts bounded function-like `#define` directives with named parameters. It collects invocation arguments with balanced parentheses, permits zero-parameter and empty arguments, macro-expands arguments before substitution, substitutes every parameter occurrence, and rescans the replacement with the current macro table. Rescanning composes with object-like aliases, nested function-like calls, following parenthesized token groups, ordinary translation-unit code, and `#if`/`#elif` expressions.

The implementation retains Cust's interpreter-owned preprocessing path. Native compilers remain warning-free compatibility oracles only.

## C semantics preserved

- A function-like definition requires `(` immediately after the macro name; invocation permits preprocessing whitespace/comments before `(`.
- Commas separate arguments only at invocation-parenthesis depth one.
- Argument tokens are fully prescanned before parameter substitution.
- The replacement list is rescanned after substitution, including calls formed by aliases or following source tokens.
- A macro remains disabled while rescanning its own replacement. The `PRESERVE(PRESERVE)(1)` oracle route therefore leaves the inner name available as an ordinary C function designator, matching GCC/Clang behavior.
- Identical definitions are accepted and conflicting object/function definitions retain deterministic diagnostics.

## Bounds and explicit exclusions

The shared translation-unit expansion work and emitted-token quotas apply to both macro kinds. Expansion nesting is capped at 128 independently of macro-name recursion tracking, so deeply nested arguments fail deterministically before unbounded host recursion. Direct/indirect replacement recursion remains source-located.

The first slice deliberately excludes stringification (`#`) and token pasting (`##`). Variadic parameters are implemented by the follow-up slice in `references/cust-variadic-function-like-macro-preprocessing.md`; header inclusion is documented separately.

## Coverage

- `tests/fixtures/compat/valid/function_like_macros.c` covers declarations, nested/empty arguments, argument prescan, object/function aliases, replacement rescanning across following calls, temporary self-disable behavior, and function macros in `#if`; the fixture returns 0 under Cust and the warning-free native C11 compiler oracle.
- Invalid fixtures cover duplicate parameters, invocation arity, and recursive expansion with exact source context; follow-up variadic fixtures cover malformed ellipsis placement and omitted variable arguments.
- Interpreter regressions cover the valid fixture and excessive nested invocation depth.
- Existing object-macro depth/token/work and exact diagnostic regressions remain green.

## Verification

`cargo fmt --check`, warning-free Clippy, all 1,078 local tests, the native compiler-oracle harness, `docker compose run --rm test`, and `docker compose run --rm cust` pass. Independent pre-commit review found and drove fixes for a stale unsupported-feature test, nested-argument stack growth, and temporary macro-disable rescanning; final review reported no blocking/high findings.
