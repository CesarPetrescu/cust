# Cust bounded variadic function-like macro preprocessing

Date: 2026-07-28

## Scope

Cust accepts C11 function-like macro parameter lists ending in `...`, including both `#define F(...)` and `#define F(named, ...)`. At invocation, trailing arguments and their separating comma preprocessing tokens are merged into one variable argument, prescanned with the ordinary macro table, substituted for every `__VA_ARGS__` occurrence, and rescanned with the surrounding replacement list. The existing translation-unit work/token quotas and 128-level expansion bound remain shared with object-like and named-parameter macros.

## C11 semantics and boundaries

- WG14 N1570 6.10.3p4 requires at least one variable argument item after the named parameters. `F(value,)` supplies an empty variable argument; `F(value)` omits it and is rejected. For a macro with no named parameters, `F()` supplies one empty argument.
- N1570 6.10.3p12 merges all trailing arguments, including their separator commas. Cust retains those comma tokens so forwarding forms such as `CALL(function, __VA_ARGS__)` preserve nested call arity.
- N1570 6.10.3.1 treats `__VA_ARGS__` as a parameter. Cust prescans the merged variable argument before substitution and then uses the existing replacement rescanner.
- N1570 6.10.3p1-p2 makes whitespace separation part of compatible macro-definition identity. `MacroReplacementToken` now records whether each token was separated from its predecessor, while treating every nonempty trivia run as the same separation.
- N1570 6.10.3p5 reserves `__VA_ARGS__` for variadic replacement lists. Exact diagnostics reject it as an ordinary source identifier, macro/directive name, `defined` operand, invocation argument, or named macro parameter.
- Direct and indirect replacement recursion retain Cust's established deterministic diagnostic boundary instead of leaving disabled recursive macro names in the output. Stringification (`#`) and token pasting (`##`) remain separate unsupported slices.

## Implementation notes

`parse_macro_arguments()` receives the number of fixed parameters for a variadic definition. Top-level commas split only until that many fixed arguments have been collected; later commas remain inside the single variable-argument token vector. Nested parentheses continue to protect commas from top-level splitting.

Argument prescan is lazy and replacement-driven: only named parameters and `__VA_ARGS__` occurrences that are actually substituted consume expansion work/token budget. Unused arguments are not expanded, matching C11 and preventing malformed or recursive unused arguments from producing diagnostics. Raw argument tokens are still checked for reserved `__VA_ARGS__` misuse before this lazy-prescan decision. Empty invocations still count as one empty argument when a macro declares named parameters, so rejected calls report the C argument count.

Every ordinary replacement token and substituted argument slice is capacity-checked before it is pushed or cloned into the intermediate replacement vector. This applies the 8,192-token bound before replacement rescanning, preventing repeated named or variadic parameter occurrences from allocating an unbounded amplified vector even when a later recursive token or collapsing macro would otherwise stop final emission.

Conditional groups nested below an inactive parent process directive names for structural depth while ignoring their operands and trailing preprocessing tokens. Their text still flows through comment-aware inactive scanning, so unterminated comments remain exact lexical errors.

Replacement-token spelling now covers every supported punctuator as well as identifiers, integer/character/string literals. The parser uses exact spelling lengths and source token starts to derive only the separation bit needed for compatible redefinition checks; expansion still uses semantic `Token` values and interpreter-owned source locations.

## Coverage

- `tests/fixtures/compat/valid/variadic_function_like_macros.c` covers fixed-plus-variable and variable-only definitions, direct calls, nested forwarding, object/function macro argument prescan, comma preservation, empty and explicitly empty trailing arguments, and `#if` expansion. It is warning-free under strict native C11 and returns 0 under Cust.
- Interpreter regressions cover omitted-variable-argument arity, malformed/final ellipsis placement, reserved `__VA_ARGS__` contexts, whitespace-sensitive conflicting redefinitions, pre-rescan substitution amplification, and the existing recursion/depth/token/work bounds.
- Independent review prompted reserved-identifier, whitespace-identity, generic resource-diagnostic, unused-argument prescan, inactive nested-conditional, empty-invocation arity, unused-argument reserved-token, pre-rescan allocation-bound, and documentation fixes; final review reported no blocking/high/medium findings.
