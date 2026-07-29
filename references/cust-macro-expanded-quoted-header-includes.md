# Cust bounded macro-expanded quoted-header operands

Date: 2026-07-29

## Scope

Linux file entry points accept active `#include` and `%:include` operands that macro-expand to exactly one unprefixed string-literal preprocessing token. Object-like aliases, function-like forwarding, nested macros, stringification, macros defined in included headers, and logical sibling lookup all reuse the existing interpreter-owned preprocessing and secure quoted-header path.

Direct string-only APIs still reject includes because they do not install an `IncludeContext`; non-Linux file entry points continue to fail closed.

## Expansion and validation

1. Keep the existing direct `"header.h"` parser path unchanged.
2. For any other active operand, tokenize the complete directive remainder in preprocessing-token mode without executing directives.
3. Expand it with `expand_macro_tokens()` using the translation unit's existing macro table and `MacroExpansionBudget`.
4. Diagnose expanded `<...>` forms as unsupported system headers.
5. Require exactly one `Token::StringLiteral` or quote-leading `Token::RawPreprocessor`; reject empty, wide/prefixed, non-string, and multi-token results.
6. Strip only the outer quote spelling and pass the retained raw inner spelling to `lex_quoted_header()`.

Retaining raw preprocessing spelling matters. GCC and Clang strict-C11 probes treat escapes in a macro-produced include spelling as header-name characters rather than decoded runtime string bytes (for example, `"foo\\x2eh"` searches for that literal spelling). Do not normalize the token through ordinary expression-string decoding before path lookup.

## Safety and bounds

- Macro operand expansion shares the existing depth, emitted-token, work, stringification-byte, and token-pasting-byte budgets; no include-local reset is allowed.
- Inactive include operands are not tokenized or expanded. Coverage uses both malformed syntax and a recursive macro under `#if 0`.
- Successful operands flow through the existing `lex_quoted_header()` implementation, preserving logical including-directory search, project-root containment, `openat2(RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS)`, regular-file checks, opened dev/inode identity, include depth/source-byte bounds, cycle detection, and normalized origins.
- Unsafe macro-produced paths must fail through the same directive-located containment diagnostic as direct paths.

## TDD and review coverage

- Initial focused RED: `%:include FORWARD_HEADER` reported `macro-expanded include operands are not supported`.
- Focused GREEN covers object/function/nested/stringifying macros, macros defined in headers, inactive malformed/recursive operands, and direct plus digraph directive spellings.
- Exact diagnostics cover non-string, adjacent/multi-token strings, empty strings, wide strings, system headers, malformed function invocations, unsafe paths, and failures originating inside included headers.
- A file-backed 8,192-token regression proves prior translation-unit expansion exhausts the budget before a later include operand can emit another token.
- `quoted_header_includes.c` retains direct no-whitespace include coverage and adds warning-free macro-expanded primary/nested include routes for Cust and native C11.
- Independent review found no blocking/high implementation defect; its medium shared-budget coverage finding and low nested-location/direct-route/docs findings were resolved before the canonical gate.

## Verification

Run the focused CLI tests, actual `c_compat` harness, strict native fixture, recursion-depth regression, then the full local and Docker canonical gates. Filtering `c_compat` by fixture name runs zero tests and is not valid verification.
