# Cust Physical-Line Splicing

## Scope completed on 2026-07-28

Cust now applies C11 translation-phase-2 deletion to every backslash immediately followed by LF or CRLF before preprocessing-token decomposition. This is global rather than directive-only: splices can join directive names, macro names/replacements, identifiers, integer literals and suffixes, operators, comment delimiters, character/string literals, and numeric escapes.

## Implementation model

- `SplicedSourceChars` stores the logically spliced `Vec<char>` separately from a parallel physical `(line, column)` position for every retained character plus EOF.
- `SplicedSourceChars::new()` deletes splice pairs without inserting whitespace or a sentinel. This is required for forms such as `+\\\n=`, `/\\\n/`, `0x4\\\n1UL`, and `B\\\nB`.
- Lexer position advancement uses the parallel map. Do not reconstruct physical positions from the spliced character sequence.
- `slice(start, end)` copies both values and positions. Macro replacement-list and `#if`/`#elif` sub-lexing must use `lex_spliced_with_context()` over such slices; converting a slice back to `String` loses internal physical-line jumps.
- Do not use a reserved Unicode sentinel. A sentinel either behaves as whitespace instead of deletion or collides with valid source/literal text.

## Regression coverage

Focused interpreter tests cover:

- LF and CRLF continuations in `#define` and `#if`;
- joining directive names, macro names, macro replacement identifiers, decimal/hex/suffixed numbers, and ordinary identifiers;
- compound operators and line/block comment delimiters formed across splices;
- character/string literals and numeric escapes;
- inactive ordinary text, line comments, and quoted text hiding apparent directives on continued logical lines;
- exact diagnostics on later physical lines; and
- preservation of literal U+E000 source text.

`tests/fixtures/compat/valid/object_like_macro_line_continuations.c` provides warning-free native C11 oracle parity.

## Verification pitfall

The initial sentinel implementation passed whitespace-separated directive cases but failed true phase-2 token joining, macro replacement spelling/redefinition, and real sentinel source text. Independent review probes were essential. Keep at least one all-boundaries regression rather than testing only conventional `#define X ... \\` formatting.
