# Cust predefined `__FILE__` / `__LINE__` macros

Date: 2026-07-29

## Scope

Cust treats `__FILE__` and `__LINE__` as dynamic predefined object-like macros. They expand in ordinary source, included headers, object/function-like macro rescanning, argument prescan, `#if`/`#elif`, and `defined`/`#ifdef`/`#ifndef` contexts. `#define` and `#undef` reject both reserved names at the macro-name token.

## Expansion context

- `__LINE__` uses the physical line carried by the invocation `LocatedToken`, including the physical-position map retained through backslash-newline splicing.
- `__FILE__` uses `<input>` for string-only APIs and the normalized project-relative logical path at the top of the file include stack for file-aware APIs.
- Replacement-list tokens are created at the outer use site, so a macro such as `#define LINE __LINE__` reports the line where `LINE` is expanded, not where it was defined.
- Raw preprocessing lexers must not eagerly expand predefined names. Direct expansion is enabled only for translation-unit lexing; replacement lists, condition operands, invocation slices, and paste relexing preserve the identifier for the shared macro expander.

## Spelling and values

`__FILE__` stores the logical name as the decoded string-token value while separately building valid C preprocessing spelling. Backslash, quote, standard control characters, and remaining bounded control characters are escaped. This distinction is required for two-level stringification: a logical filename containing a newline has runtime value `odd\nname.c`, while `STR(__FILE__)` contains the spelling `"odd\\nname.c"` rather than an invalid raw newline.

## Bounds and metadata

Each predefined expansion consumes one shared macro-work unit and one shared emitted-token unit, and preserves invocation separation, line/column, and source-origin metadata. It uses the same 128-depth, 65,536-work, and 8,192-token limits as user-defined macros.

## Verification notes

- RED/GREEN regressions cover direct values, use-site lines, forwarding, raw versus expanded stringification, `defined`, `#ifndef`, and exact redefinition/undefinition diagnostics.
- Linux file-backed tests cover exact primary/header logical names and unusual newline-containing filenames.
- The warning-free native C11 fixture uses suffix checks because native compilers retain the command-line path in primary `__FILE__`, while Cust deliberately normalizes to the project-relative logical name.
- Independent review exposed the control-character spelling edge and a Clippy argument-count gate; both received focused fixes before final approval and the canonical local/Docker gate.
