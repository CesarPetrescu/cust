# Bounded C11 `#error` Directives

## Standard and native behavior

- C11 N1570 §6.10.5 requires `# error pp-tokensopt` to emit a diagnostic containing the specified preprocessing-token sequence.
- The message is not macro-expanded. GCC strict-C11 probes retain a macro name literally.
- Translation-phase comment replacement contributes token separation. Normalize whitespace/comments to one space outside string and character literal spelling, but preserve literal contents and escapes verbatim.
- `%:error` is the spelling-equivalent digraph route. Physical backslash-newline deletion occurs before directive handling.
- Existing inactive-group routing must inspect the directive name for structure and then ignore the `#error` operand entirely.

## Implementation pattern

1. Route active `error` after the inactive-directive guard in `process_preprocessor_directive()`.
2. Skip leading preprocessing whitespace/comments, then scan the remaining logical directive line without macro expansion.
3. Preserve quote/escape regions while collapsing outside whitespace and line/block comments.
4. Return the diagnostic at the directive marker so primary and nested-header paths retain existing source-origin wrapping.
5. Treat an empty normalized sequence as an explicit empty-error diagnostic.

## Resource-safety pitfall

Bounding only the normalized message is insufficient. `lexer_error_with_context()` copies the complete physical source line and creates caret padding proportional to the source column, so a rejected multi-megabyte one-line message can still produce multi-megabyte stderr.

For the 1 MiB `#error` message overflow path, use a bounded source window and bounded caret offset. The regression must assert both the error kind and a hard upper bound on the complete formatted diagnostic length, not just `starts_with(...)`.

## Verification checklist

- RED/GREEN direct active message proving macro names stay unexpanded.
- RED/GREEN comment/whitespace normalization and escaped literal contents.
- Empty, `%:error`, physical-splice, multiline-comment, and inactive routes.
- Exact malformed block-comment location.
- UTF-8 byte boundary plus complete diagnostic output bound.
- Nested included-header origin through the CLI.
- Independent review before the canonical local/Docker gate.
