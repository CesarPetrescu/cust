# sizeof array compound literals and union array-field decay

2026-05-09 autonomous run.

## Completed behavior

- `sizeof((int[]){...})`, `sizeof((char[5]){"dog"})`, and `sizeof((struct Point[2]){...})` now report the full array object size instead of Cust's deterministic pointer size.
- `sizeof` remains non-evaluating for array and aggregate-array compound literal initializers: side effects inside initializer expressions are not executed.
- The inferred unsized compound-literal length reuses the runtime storage helpers:
  - scalar arrays: `Interpreter::infer_array_initializer_len`
  - aggregate arrays: `Interpreter::infer_struct_array_initializer_len`
- Fixed-size compound literals use the written length.
- Added fixture coverage for embedded union-array field pointer decay through direct struct values and struct-pointer paths:
  - `bag.numbers`
  - `&bag.numbers[i]`
  - `bag->numbers`
  - `&bag->numbers[i]`
  - `bag->numbers + n`
- Const containing structs still reject mutable union-array field decay with `cannot discard const qualifier from pointer target`.

## Implementation notes

- The production change is localized to `Interpreter::sizeof_expr` in `src/lib.rs`.
- Do not evaluate array compound literal initializers from `sizeof`; infer length and element size syntactically/metadata-only.
- Native compiler-oracle coverage is safe for `char` array compound literal sizes because `sizeof(char[N])` is ABI-stable. Avoid exact native `int` or aggregate object sizes in oracle fixtures because Cust defines deterministic sizes (`int = 8`, no aggregate padding).

## Focused commands

```bash
cargo test --test interpreter reports_array_compound_literal_sizes_without_evaluating_initializers -- --nocapture
cargo test --test interpreter union_aggregate_array_field -- --nocapture
cargo test --test c_compat -- --nocapture
```
