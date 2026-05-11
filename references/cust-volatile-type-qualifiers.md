# Volatile type qualifier syntax parity

Date: 2026-05-11

Cust accepts C `volatile` type qualifiers as parser-level syntax over the existing deterministic interpreter model. `volatile` is intentionally a no-op for runtime behavior: it does not introduce native optimization/observable-access semantics, and `const` remains the only qualifier that affects write enforcement.

Implemented coverage:

- Lexer keyword: `volatile` maps to `Token::Volatile`.
- Declaration/type contexts consume repeated `const`/`volatile` qualifiers before the base type, preserving existing const enforcement when `const` appears and ignoring `volatile` otherwise.
- Supported contexts include globals, locals, static locals, `auto`/`register` locals, `for` initializer declarations, parameters/prototypes, function return type parsing for ordinary scalar returns, pointer declarations and parameters including post-star qualifiers, aggregate fields, typedef aliases, casts, and `sizeof` type operands.
- Native compiler-oracle fixture avoids warning-prone forms under `-Wall -Wextra -Werror`: top-level volatile function return qualifiers trigger `-Wignored-qualifiers`, and initializing non-volatile pointer targets from volatile arrays triggers discarded-qualifier warnings.

Non-goals:

- No memory-mapped I/O or C optimizer semantics.
- No new write restrictions beyond existing `const` handling.
- Volatile pointer-target metadata is not tracked separately; Cust treats volatile-qualified pointees like mutable pointees unless `const` is also present.

Verification:

```bash
cargo test --test interpreter supports_volatile_type_qualifiers -- --nocapture
cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture
```
