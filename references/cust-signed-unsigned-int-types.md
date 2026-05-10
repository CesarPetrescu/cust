# Signed/unsigned integer type spellings

2026-05-10 autonomous run: Cust now accepts `signed` and `unsigned` integer type spellings as parser-level aliases for the existing deterministic `int` storage model.

## Scope

- `signed`, `signed int`, `unsigned`, and `unsigned int` parse as `CType::Int`.
- Works in globals, locals, static locals, `for` declaration initializers, function returns, parameters/prototypes, pointer declarations/parameters, typedef aliases, scalar casts, and `sizeof` type operands including one-dimensional array type names.
- `const unsigned int` reuses the existing leading-const path.
- Native compiler-oracle coverage is safe because the fixture only compares behavior/exit code and `sizeof` ratios, not ABI-specific exact native integer widths.

## Implementation notes

- Lexer adds dedicated `Token::Signed` / `Token::Unsigned` keywords instead of treating them as identifiers.
- `parse_decl_type` consumes an optional following `int`; bare `signed` / `unsigned` are accepted as C shorthand for signed/unsigned int.
- Start-token lookahead must be updated anywhere scalar declarations or type operands are recognized: top-level declarations, local/static/for declarations, function-definition detection, cast detection, and `sizeof` type detection.
- Cust intentionally keeps runtime scalar storage unchanged (`i64`, deterministic `sizeof(int) == 8`); this is syntax/conformance parity, not native unsigned wraparound semantics.
