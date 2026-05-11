# Extern Global Declarations

2026-05-11 autonomous run.

Cust now treats uninitialized top-level `extern` object declarations as parser-only declarations in the single-file interpreter model. This lets later ordinary definitions provide the runtime storage without tripping duplicate-global diagnostics.

Covered forms:

- Scalar declarations such as `extern int total;` and `extern char marker;`.
- One-dimensional scalar arrays such as `extern int values[3];`.
- Aggregate variables and arrays such as `extern struct Point origin;`, `extern struct Point points[2];`, and `extern union Number number;` after the aggregate type is defined.
- Named enum variables such as `extern enum Status status;`.
- One-level pointer declarations such as `extern int *cursor;`.

Implementation notes:

- `Parser::parse_program` records whether the consumed top-level storage-class was `extern`.
- Declaration parsers set `last_decl_had_initializer` while parsing supported variable/aggregate declarations.
- If a top-level `extern` object declaration has no initializer, the parsed declaration is skipped instead of being added to `Program::globals`; initialized declarations still become globals.
- Function `extern` handling remains unchanged and continues to route through the existing prototype/definition signature checks.

Fixture notes:

- The compiler-oracle fixture keeps aggregate tags defined before the `extern` object declarations so both Cust and native C have complete object types available.
- The native fixture uses ordinary later definitions (`int total = 4;`, `struct Point origin = {5, 6};`, etc.) and avoids initialized `extern` globals, which can be warning-prone under `-Wall -Wextra -Werror`.
