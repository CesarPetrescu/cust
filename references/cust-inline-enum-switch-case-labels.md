# Inline enum definitions in switch case labels

Date: 2026-06-26

## Symptom

A switch `case` label can contain a type-query integer constant expression that defines an inline enum, for example:

```c
switch (sizeof(enum SwitchSize { SWITCH_SIZE = 1 })) {
    case sizeof(enum CaseSize { CASE_SIZE = 7 }):
        return CASE_SIZE;
}
```

Cust parsed the inline enum definition in the case label and could fold the case value, but runtime execution failed with `undefined variable 'CASE_SIZE'` when the case body referenced the generated enumerator.

## Root cause

`parse_switch()` already collected pending inline enum declarations from the switch controlling expression and prepended them before the `Stmt::Switch`. It did not collect pending inline enum declarations produced while parsing each `case` label's integer constant expression, so those generated enumerators were never emitted before the section statements executed.

## Implementation decision

After parsing each `case`/`default` label, call `take_pending_inline_enum_decl()` and, when present, insert the resulting `EnumDecl` as the first statement in that switch section. This makes the generated constants available both when the switch jumps directly to that case and when execution falls through from an earlier section, without changing case-value folding or switch-expression enum hoisting.

## Verification

Focused:

```bash
cargo test --test interpreter inline_enum_switch_case_label -- --nocapture
cargo test --test c_compat -- --nocapture
```

Full gate from the run should include `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and both required Docker Compose commands.
