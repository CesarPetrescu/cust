# Long/short type spellings

2026-05-10 autonomous run: Cust accepts a deliberately scoped C type-spelling parity slice for `long` and `short` integer forms.

## Supported syntax

- `long`, `long int`, `short`, and `short int` lower to Cust's existing deterministic `int` storage.
- `signed long`, `signed long int`, `unsigned long`, `unsigned long int`, `signed short`, `signed short int`, `unsigned short`, and `unsigned short int` also lower to deterministic `int`.
- These spellings are accepted anywhere ordinary scalar `int` type syntax is routed through `parse_decl_type`: globals, locals, static locals, `for` declarations, function returns/prototypes/parameters, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` scalar/pointer/one-dimensional array type operands.

## Semantics and test boundaries

- Runtime storage remains Cust's `i64` integer model; this is syntax/conformance parity, not native width/range/wraparound behavior.
- `sizeof(long)` and `sizeof(short)` report Cust's deterministic integer size (`8`) because both lower to `CType::Int`.
- C compiler-oracle fixtures avoid exact `sizeof(long)`/`sizeof(short)` assertions because native C sizes are ABI-dependent and `short` commonly differs from Cust's deterministic integer size.
- `long long` remains outside this scoped slice unless a later roadmap item deliberately adds it.

## Implementation notes

- Lexer keywords: `Token::Long` and `Token::Short`.
- Parser updates include top-level/block/static/for declaration type-start checks, function-definition lookahead, cast type starts, and `sizeof` type starts.
- `parse_decl_type` consumes optional trailing `int` after `long`/`short`, and after signed/unsigned long/short forms.
