# Cust object-like macro preprocessing

2026-07-27 autonomous run.

## Scope

Cust supports one-line object-like `#define` directives as the first bounded v0.4 preprocessing slice. Definitions affect subsequent tokens in the translation unit and can expand recursively through other object-like macro names. Replacement tokens may participate in declarations, enum/array integer constant expressions, initializers, and runtime expressions.

Function-like macros, `#include`, line continuations, `#undef`, conditional compilation, token pasting, and stringification remain unsupported.

## Implementation

- `lex()` owns a translation-unit macro table and recognizes `#` only when no ordinary token precedes it on that source line.
- `process_preprocessor_directive()` parses one physical directive line, tokenizes each replacement list without inheriting the outer macro table, and stores token kinds rather than source text.
- Ordinary identifier lexing calls `expand_object_macro()`. Expansion recursively rescans replacement tokens against the current table, so later definitions can satisfy names retained in earlier replacement lists.
- Expansion clones replacement token kinds at the invocation location. Comments are already consumed as whitespace and string/character literals are single lexer tokens, so identifier-like text inside them is never considered for macro replacement.
- An explicit expansion stack reports direct or indirect recursion. Identical object-like definitions are accepted; different replacement token sequences report a conflicting redefinition at the second macro name.
- Replacement-list lexing disables directive recognition, so unsupported raw `#`/`##` operators receive an exact diagnostic instead of being executed as nested directives. Line comments, block comments, vertical tabs, and form feeds act as directive whitespace within the one-physical-line boundary.
- Translation-unit-wide counters bound expansion depth (128), emitted tokens (8,192), and recursive expansion work (65,536 calls). The separate work counter includes empty macro expansion and prevents exponential empty-replacement graphs from bypassing the emitted-token quota; counters do not reset between source-level invocations.

## TDD and verification

Focused RED:

```bash
cargo test --test interpreter object_like_macro -- --nocapture
```

All three selected tests failed on the former `preprocessor directives are not supported` lexer diagnostic. GREEN covers nested declaration/constant/runtime expansion, comments and literals, recursive expansion, and conflicting redefinitions. A separate focused test covers exact `#include`, function-like, malformed-definition, and line-continuation diagnostics.

Independent review then produced additional focused REDs: valid line-comment/vertical-tab/form-feed directive whitespace was rejected; a raw replacement `#` was incorrectly executed as a nested directive; a 300-name chain lacked a depth diagnostic; binary token expansion lacked an emitted-token limit; exponentially branching empty macros bypassed that token limit; and repeated invocations reset the quota. Focused GREEN now locks all six seams with exact source context and panic-free bounded behavior.

The warning-free compiler oracle is `tests/fixtures/compat/valid/object_like_macros.c`; it returns 0 under Cust and native C11 compilation.

## Standards basis

WG14 N1570 §6.10.3 paragraphs 1-3 and 7-10 define object-like replacement lists and redefinition constraints; §6.10.3.4 requires replacement-list rescanning; footnote 171 notes that character and string literals are preprocessing tokens and are not scanned internally for macro names. Cust deliberately diagnoses recursive expansion rather than retaining the standard's disabled self-reference token because the bounded roadmap requires an explicit recursion error.
