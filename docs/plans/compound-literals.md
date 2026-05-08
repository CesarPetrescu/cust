# Aggregate Compound Literal Model

Cust supports a deliberately scoped C99-style aggregate compound literal subset:

- Supported syntax: `(struct Name){...}`, `(union Name){...}`, and aggregate typedef aliases such as `(Point){.x = 1}`.
- The brace body reuses the existing aggregate initializer parser, including positional entries, field designators, nested aggregate initializers, scalar array-field initializers, and union first/active-field behavior.
- Compound literals are rvalue aggregate expressions in Cust's interpreter model. They can initialize aggregate variables, feed same-type aggregate assignment expressions, be passed to by-value aggregate parameters, be returned from aggregate-returning functions, appear in aggregate `?:` / comma expressions, and expose scalar/nested struct fields through `.` field access.
- Cust does not model C compound literal object address/lifetime yet. Taking the address of a compound literal, mutating it as an lvalue, or using scalar/array compound literals is intentionally out of scope until a storage/lifetime design is written.
- Native C compilers remain test oracles only. Compatibility fixtures compare exit codes for supported aggregate expression behavior without relying on native ABI padding or object addresses.
