# Cust bounded C11 line directives

## Scope

Cust supports active `#line` and `%:line` directives with:

- decimal line values in C11's required `1..=2147483647` range;
- an optional ordinary string-literal presumed source name;
- object-like, function-like, nested, and predefined-macro expansion in the operand;
- use by subsequent `__LINE__` and `__FILE__` expansions;
- source-local state across nested quoted includes;
- physical source positions retained for diagnostics.

## Architecture

`PreprocessorState` carries a current presumed-source override and a signed offset from physical lines. A line directive computes that offset from the physical line after the complete logical directive. This must use `chars.position(line_end)` rather than the directive introducer's line because phase-2 backslash-newline deletion can join multiple physical lines.

Quoted inclusion saves and clears both values before lexing a header, then restores them on success or failure. The include path stack continues to provide the default logical header name. Do not overwrite `LocatedToken.line` or `.column`: parser and lexer source snippets intentionally remain canonical physical coordinates.

The shared macro rescanner accepts the line offset alongside the current presumed source name. This keeps direct uses, forwarding, argument prescan, rescanning, and `#if`/`#elif` expansion consistent without evaluating expressions.

## Operand and resource rules

Lex the complete operand as preprocessing tokens, then apply the translation unit's shared macro expansion budget. Validate the final preprocessing spelling rather than prematurely converting large decimal pp-numbers to Cust runtime integers.

- The first token must contain decimal ASCII digits only.
- The optional second token must be one ordinary unprefixed string literal.
- A third token is an exact trailing-token error.
- Operand lexing is capped at 8,192 tokens.
- The decoded presumed source name is capped at 4,096 UTF-8 bytes.
- Source-name overflow uses bounded source context so diagnostic size cannot scale with an attacker-controlled directive line.

Decode the optional ordinary string literal to the logical source-name value. Later `__FILE__` expansion reuses the shared preprocessing-string escaping helper, so quotes, backslashes, control characters, and embedded escapes remain a valid string token.

## TDD and verification

High-value focused regressions:

```bash
cargo test --test interpreter line_directives_ -- --nocapture
cargo test --test c_compat -- --nocapture
cargo test --test interpreter reports_function_name_when_recursive_calls_exceed_depth_limit -- --nocapture
```

Coverage should include direct and filename-retaining remaps, function/object macro operands, condition and forwarding use sites, physical splicing, exact malformed/range/wide/trailing diagnostics, bounded source names, physical error locations, nested-header remapping, and restoration in the parent file. A warning-free native fixture should compare `__FILE__`/`__LINE__` behavior without relying on ABI sizes.
