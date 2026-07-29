# Cust bounded macro token-pasting maintenance lessons

Date: 2026-07-29

## Standards boundary

ISO C11 draft N1570 §6.10.3.3 defines `##` token concatenation. A parameter adjacent to `##` is substituted with its raw, unprescanned preprocessing-token sequence. Empty arguments become placemarkers; concatenating a placemarker with a token keeps that token, two placemarkers remain a placemarker, and all placemarkers disappear before rescanning. Two non-placemarker operands must form exactly one valid preprocessing token. The result is then rescanned with the rest of the replacement list.

Cust supports both `##` and the `%:%:` digraph in function-like and object-like replacement lists. It deliberately diagnoses leading, trailing, operand-less, invalid-result, generated-byte-limit, recursion, and existing depth/token/work boundaries rather than invoking a host preprocessor.

## Representation and expansion

- Keep `MacroReplacementToken::Paste { spelling, separated }` distinct from ordinary `#` stringification tokens. Preserve both spelling (`##` versus `%:%:`) and preceding whitespace separation because GCC/Clang strict-C11 probes diagnose redefinitions that change either one.
- Recognize only exact adjacent `#` + `#` or `%:` + `%:` spellings. Mixed `#%:` and comment/whitespace-separated hashes remain separate stringification tokens.
- Cache an expanded argument only when that parameter has at least one ordinary, non-`#`/non-`##` occurrence. An occurrence adjacent to paste always clones the raw argument instead.
- Lower substituted replacement elements to `MacroSubstitutionItem::{Token, Paste, Placemark}`. A left-to-right reduction naturally handles multi-token arguments (only the nearest boundary tokens concatenate), chains, and placemarkers.
- Re-lex concatenated spelling in preprocessing-token mode and require one non-EOF token. Retain the left operand's separator on the result and rescan the reduced replacement through the existing macro expander.
- Account for valid C11 punctuators that the ordinary Cust lexer intentionally decomposes. Generated `<:`, `:>`, `<%`, and `%>` lower to semantic bracket/brace tokens; `%:` lowers to `Hash`; generated `##` and `%:%:` remain one opaque preprocessing token so a later macro can stringify their original spelling.
- Charge every generated concatenation, including intermediate chain results, against a translation-unit-wide 1 MiB byte budget before allocation. Existing replacement-vector, emitted-token, work, and depth checks remain shared.

## TDD and oracle coverage

Focused RED/GREEN regressions covered:

- raw direct arguments versus indirect prescan and result rescanning;
- identifiers, keywords, integer preprocessing numbers, punctuators, multi-token arguments, chained operators, object-like macros, variadic arguments, `%:%:`, `#if`, and stringification after paste;
- left/right/both empty arguments and chained placemarkers;
- leading/trailing/consecutive operators and invalid two-token results;
- replacement-list whitespace and `##`/`%:%:` spelling in macro redefinition equality;
- generated bracket/brace/hash/paste digraph punctuators, including later stringification of `%:`, `%:%:`, and `##`;
- the generated-byte bound before concatenation allocation.

The warning-free compiler-oracle fixture is `tests/fixtures/compat/valid/macro_token_pasting.c`; invalid result coverage is `tests/fixtures/invalid/macro_token_pasting_invalid_result.c`. Run the actual compiler-oracle harness with `cargo test --test c_compat -- --nocapture`; filtering it by fixture name runs zero tests.

## Pitfalls

- Do not prescan every argument eagerly: an invalid/recursive macro inside a paste-only argument must remain raw unless the pasted result later names it.
- Do not represent `##` as two generic hash tokens after definition parsing; doing so confuses stringification and loses redefinition metadata.
- Do not concatenate semantic token renderings. Use retained preprocessing spellings so suffixes, digraphs, opaque pp-numbers, and literal escapes survive.
- Do not validate by merely checking that lexing succeeds; a spelling such as `+*` lexes successfully as two tokens and must be rejected.
- Do not reject every spelling that the ordinary lexer decomposes into multiple semantic tokens: C11 digraph/hash/paste punctuators are still one preprocessing token and need an explicit spelling-to-token bridge.
- Do not allocate the concatenated `String` before checking cumulative byte capacity.
