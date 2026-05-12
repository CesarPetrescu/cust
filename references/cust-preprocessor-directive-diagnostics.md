# Cust unsupported preprocessor directive diagnostics

2026-05-12 autonomous run.

## Scope

Cust remains a preprocessor-free C-subset interpreter. It does not expand `#include`, `#define`, conditional compilation, macros, line markers, or any other preprocessing directive.

## Implementation note

The lexer now special-cases `#` before the generic unexpected-character fallback and reports:

```text
preprocessor directives are not supported at line <line>, column <column>
<source line>
^
```

This is intentionally lexer-local: adding a token would let directive syntax drift into parser contexts where diagnostics become less direct. The diagnostic preserves the existing source-line/caret helper used by lexer errors.

## Coverage

- Invalid fixture: `tests/fixtures/invalid/preprocessor_directive.c`
- Focused test: `cargo test --test interpreter rejects_preprocessor_directives_with_context -- --nocapture`
- RED failure before implementation: generic `unexpected character '#'` at line 1, column 1.
- GREEN result after implementation: exact unsupported preprocessor diagnostic with source context.

## Future boundary

If Cust ever gains preprocessing, it should be a separate documented feature/tooling layer. Native compilers may remain test oracles only; they must not become Cust's preprocessing/runtime path.
