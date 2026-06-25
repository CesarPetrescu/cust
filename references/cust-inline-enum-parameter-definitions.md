# Inline enum definitions in function parameters

Date: 2026-06-25

Context: direct inline enum definitions in function parameter type specifiers, e.g. `int f(enum Mode { READY = 3 } value) { return READY; }`.

Root cause: `parse_params()` already parses inline enum specifiers and queues their constants in `pending_inline_enum_constants`, but `parse_function_declaration()` cleared that pending state after parameter parsing to avoid leaking parameter-list enum constants into file scope. That made function-body references to those parameter-declared enumerators fail with `undefined variable '<name>'`.

Implementation note: after `parse_params()`, take the pending parameter enum declaration and, for function definitions only, insert it as the first statement in the function body. For prototypes, discard it because there is no executable body and the constants must not become file-scope declarations. Keep return-type inline enum declarations on the existing file-scope path.

Verification: `cargo test --test interpreter inline_enum_parameter -- --nocapture` covers the RED/GREEN path. Native `cc -std=c11 -Wall -Wextra -Werror` is intentionally not used as a compiler-oracle fixture for this shape because it warns that `enum Mode` declared inside the parameter list will not be visible outside the declaration, and the repository's `-Werror` flags promote that warning to a hard error.
