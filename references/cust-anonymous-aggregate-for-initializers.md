# Anonymous aggregate declarations in `for` initializers

2026-06-23 autonomous run.

## Scope

Cust supports anonymous aggregate object declarations in ordinary local/global declaration statements. This run extended the same declaration parser route to `for` initializer declarations:

```c
for (struct { int x; } point = {1}; point.x < 4; point.x++) { ... }
for (union { int value; char tag; } number = {5}; number.value < 7; number.value++) { ... }
```

## Implementation note

`parse_for` already routed scalar/enum/alias declaration starts through `parse_var_decl()`, but omitted plain `Token::Struct | Token::Union`. Add a separate `parse_aggregate_var_decl()` branch in the initializer path so anonymous/named aggregate declarations consume the initializer separator semicolon naturally.

## Verification

- RED: `cargo test --test interpreter supports_anonymous_aggregate_for_initializers -- --nocapture` failed with `unexpected token in for initializer: Struct`.
- GREEN: same focused test passed after the parser routing change.
- Oracle: `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture` passed with `tests/fixtures/compat/valid/anonymous_aggregate_for_initializers.c`.

## Pitfalls

- `for` initializer declaration parsers consume the first semicolon themselves. Do not call a no-semicolon declaration helper unless the surrounding `parse_for` semicolon handling is changed deliberately.
- Keep native oracle fixtures warning-free; simple anonymous aggregate `for` initializers compile cleanly under `cc -std=c11 -Wall -Wextra -Werror`.
