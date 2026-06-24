# Enum compound literals

Date: 2026-06-25

Cust's cast/compound-literal parser already lowers direct named-enum declaration types to `DeclType::Scalar(CType::Int)` via `parse_decl_type("cast type")`; the gap was the cast lookahead. `starts_cast_type_after_lparen()` omitted `Token::Enum`, so `(enum State){READY}` was parsed as a parenthesized expression and failed at `Enum` before `parse_cast()` could run.

Implementation checklist:

1. Add a RED fixture using direct named enum type names in scalar and one-dimensional array compound literals, e.g. `(enum State){READY + 2}` and `(enum State[3]){READY, [2] = BUSY}`.
2. Use enum compound-literal arrays through pointer initialization (`enum State *values = (enum State[3]){...};`), not array-object initialization; native C does not initialize arrays from assignment expressions.
3. Include a native compiler-oracle fixture when the program is warning-free under `cc -std=c11 -Wall -Wextra -Werror`.
4. Implement by adding `Token::Enum` to `starts_cast_type_after_lparen()`. The existing `parse_decl_type()` and scalar/array compound-literal lowering then handle direct enum spellings without new runtime storage semantics.
5. Verify with `cargo test --test interpreter enum_compound_literals -- --nocapture` and `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture` before the full gate.
