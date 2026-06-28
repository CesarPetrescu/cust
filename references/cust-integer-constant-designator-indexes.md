# Cust integer constant expression designator indexes

Date: 2026-06-28

Cust originally accepted only non-negative numeric literal tokens inside C array/designator brackets such as `[2] = value`, even though C designator indexes are integer constant expressions. This showed up as `expected array designator index, found Ident("SLOT_INDEX")` for warning-free C forms like:

```c
enum { BASE_INDEX = 1, SLOT_INDEX = BASE_INDEX + 1 };
int values[4] = {[SLOT_INDEX] = 5, [sizeof(char)] = 7};
```

Implementation notes:

- Route both fixed-length and inferred/unbounded array designator indexes through `parse_integer_constant_expr(&HashMap::new(), "expected array designator index")`.
- Convert the folded `i64` through a shared `array_designator_value_to_index` helper so negative values and huge constants remain targeted diagnostics.
- Check for `Token::Comma` after folding and before `]` so `[1, 2]` keeps the existing `comma operator is not allowed in integer constant expression` boundary rather than a generic closing-bracket error.
- This automatically covers scalar array declarations, fixed and inferred scalar array compound literals, scalar array fields inside aggregate initializers, aggregate-array designators, path designators, and struct/union array initializers because those paths share `parse_array_designator_index_with_context` or `parse_unbounded_array_designator_index`.
- Native compiler-oracle fixtures must keep the final `return` in the 0..255 range because `tests/c_compat.rs` compares Cust's integer result with the native process exit code.

Focused verification used:

```bash
cargo test --test interpreter integer_constant_expressions_for_designator_indexes -- --nocapture
cargo test --test c_compat -- --nocapture
```
