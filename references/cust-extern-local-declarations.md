# Block-scope `extern` object declarations

2026-05-12 autonomous run.

## Implemented behavior

Cust now accepts block-scope `extern` object declarations as parser-only declarations over the existing single-file global storage model:

```c
int total = 4;
int values[3] = {1, 2, 3};

int read(void) {
    extern int total;
    extern int values[3];
    return total + values[1];
}
```

Supported forms reuse existing declaration parsing for scalar, char, enum, one-level pointer, one-dimensional array, struct, struct-array, and union object declarations. The parsed declaration is discarded as `Stmt::Empty`, so it does not create a local variable that would shadow the real global; runtime lookup continues to reach the existing global binding.

## Boundary

Block-scope `extern` declarations with initializers are rejected with:

```text
extern local declarations cannot have initializers
```

That avoids silently discarding side-effecting initializer expressions from declarations that should not define local storage.

## Coverage

- `tests/fixtures/valid/extern_local_declarations.c`
- `tests/fixtures/compat/valid/extern_local_declarations.c`
- `tests/fixtures/invalid/extern_local_initializer.c`
- Focused test: `cargo test --test interpreter extern_local -- --nocapture`
- Compiler oracle: `cargo test --test c_compat supported_programs_match_c_compiler_exit_codes -- --nocapture`
