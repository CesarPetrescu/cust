# Array typedef compound literals

Date: 2026-06-24

Cust supports one-dimensional array typedef aliases such as `typedef int Scores[3];`, `typedef char Word[4];`, and `typedef struct Point Points[2];`. C99 compound literals can use those aliases as type names:

```c
typedef int Scores[3];
typedef struct Point Points[2];
int *values = (Scores){1, 2, 3};
struct Point *points = (Points){{1, 2}, {.x = 3, .y = 4}};
```

Implementation notes:

- `parse_cast()` receives array aliases as `DeclType::Array(PointeeType, len)` from `parse_decl_type("cast type")`.
- If the array alias type name is followed by `{`, lower scalar element aliases to `Expr::ArrayLiteral { len: Some(len), ... }` and aggregate element aliases to `Expr::AggregateArrayLiteral { len: Some(len), ... }`.
- If there is no `{` after `(AliasArray)`, preserve the existing unsupported cast diagnostic (`pointer casts are not supported`) instead of treating array aliases as pointer casts.
- Compiler-oracle fixtures should keep the alias one-dimensional and warning-free under `cc -std=c11 -Wall -Wextra -Werror`; char array aliases may use string literal initializers such as `(Word){"cat"}`.

Verification added in this run:

```bash
cargo test --test interpreter array_typedef_compound_literals -- --nocapture
cargo test --test c_compat -- --nocapture
```
